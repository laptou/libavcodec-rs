use anyhow::Result;
use clap::Parser;
use libavcodec::{
    AVMediaType, AVSampleFormat, Codec, CodecContext, EAGAIN, FormatContext, Frame, Packet,
    SwrContext
};
use libavcodec_sys::{self as sys, AVIO_FLAG_WRITE};
use std::{ffi::CString, path::PathBuf, ptr};


/// Audio downsampler that converts any audio file to WAV with specified sample rate
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input audio file path
    #[arg(default_value = "input.mp3")]
    input: PathBuf,

    /// Output WAV file path
    #[arg(default_value = "output.wav")]
    output: PathBuf,

    /// Target sample rate in Hz
    #[arg(short, long, default_value_t = 16000)]
    sample_rate: u32,

    /// Number of channels (1 for mono, 2 for stereo)
    #[arg(short, long, default_value_t = 1)]
    channels: u16,
}

fn main() -> Result<()> {
    let args = Args::parse();

    tracing_subscriber::fmt().init();
    libavcodec::setup_tracing();

    // Open input file
    let mut input_format_ctx = FormatContext::open(&args.input)?;

    // Find audio stream
    let audio_stream = input_format_ctx
        .streams()
        .find(|s| matches!(s.codec_type(), AVMediaType::Audio))
        .expect("no audio stream found");

    // Get decoder
    let decoder = Codec::find_decoder(audio_stream.codec_id()).expect("failed to find decoder");

    // Create decoder context
    let mut input_codec_ctx = CodecContext::new(&decoder)?;
    audio_stream.apply_parameters_to_context(&mut input_codec_ctx)?;

    // Open decoder
    input_codec_ctx.open(&decoder)?;

    // Get input format details
    let in_sample_rate = input_codec_ctx.sample_rate();
    let in_sample_fmt = input_codec_ctx.sample_format();
    let in_channels = input_codec_ctx.channel_count();

    println!(
        "Input: {} Hz, {} channels, format: {:?}",
        in_sample_rate, in_channels, in_sample_fmt
    );

    // Set output format details
    let out_sample_rate = args.sample_rate as usize;
    let out_channels = args.channels as usize;
    let out_sample_fmt = AVSampleFormat::S16; // 16-bit signed PCM for WAV

    println!(
        "Output: {} Hz, {} channels, format: {:?}",
        out_sample_rate, out_channels, out_sample_fmt
    );

    // Create output format context for WAV
    let mut output_format_ctx = unsafe {
        let path_cstr = CString::new(args.output.to_str().unwrap()).unwrap();
        let mut ctx = ptr::null_mut();
        let ret = sys::avformat_alloc_output_context2(
            &mut ctx,
            ptr::null_mut(),
            b"wav\0".as_ptr() as *const i8,
            path_cstr.as_ptr(),
        );
        if ret < 0 {
            return Err(anyhow::anyhow!("Failed to create output context"));
        }
        ctx
    };

    // Find WAV encoder
    let encoder = unsafe {
        let encoder = sys::avcodec_find_encoder(sys::AVCodecID_AV_CODEC_ID_PCM_S16LE);
        if encoder.is_null() {
            return Err(anyhow::anyhow!("Failed to find WAV encoder"));
        }
        encoder
    };

    // Create output stream
    let output_stream = unsafe {
        let stream = sys::avformat_new_stream(output_format_ctx, encoder);
        if stream.is_null() {
            return Err(anyhow::anyhow!("Failed to create output stream"));
        }
        stream
    };

    // Set output codec parameters
    unsafe {
        let codec_params = (*output_stream).codecpar;
        (*codec_params).codec_type = sys::AVMediaType_AVMEDIA_TYPE_AUDIO;
        (*codec_params).codec_id = sys::AVCodecID_AV_CODEC_ID_PCM_S16LE;
        (*codec_params).sample_rate = out_sample_rate as i32;
        (*codec_params).format = sys::AVSampleFormat_AV_SAMPLE_FMT_S16 as i32;
        (*codec_params).bit_rate = out_sample_rate as i64 * 16; // 16 bits per sample
        sys::av_channel_layout_default(&mut (*codec_params).ch_layout, out_channels as i32);
    }

    // Set stream time base
    unsafe {
        (*output_stream).time_base = sys::AVRational {
            num: 1,
            den: out_sample_rate as i32,
        };
    }

    // Open output file
    let path_cstr = CString::new(args.output.to_str().unwrap()).unwrap();
    let ret = unsafe {
        sys::avio_open2(
            &mut (*output_format_ctx).pb,
            path_cstr.as_ptr(),
            AVIO_FLAG_WRITE,
            ptr::null(),
            ptr::null_mut(),
        )
    };
    if ret < 0 {
        return Err(anyhow::anyhow!("Failed to open output file"));
    }

    // Write header
    let ret = unsafe { sys::avformat_write_header(output_format_ctx, ptr::null_mut()) };
    if ret < 0 {
        return Err(anyhow::anyhow!("Failed to write header"));
    }

    // Create resampler context
    let mut swr_ctx = SwrContext::get_context(
        in_sample_rate,
        in_sample_fmt,
        in_channels,
        out_sample_rate,
        out_sample_fmt,
        out_channels,
    )?;

    // Create frames
    let mut input_frame = Frame::new()?;
    let mut output_frame = Frame::new()?;

    // Set up output frame parameters
    output_frame.set_channel_count(out_channels as i32);
    output_frame.set_format(out_sample_fmt as i32);
    output_frame.set_sample_rate(out_sample_rate as i32);

    // Create packet for reading
    let mut packet = Packet::new()?;

    // Read packets
    while input_format_ctx.read_packet(&mut packet)? {
        if packet.stream_index() == audio_stream.index() {
            // Send packet to decoder
            input_codec_ctx.send_packet(&packet)?;

            // Receive frames from decoder
            loop {
                match input_codec_ctx.receive_frame(&mut input_frame) {
                    Ok(()) => {
                        // Calculate output frame size based on input frame and resampling ratio
                        let out_samples = swr_ctx.get_out_samples(input_frame.sample_count());

                        // Set up output frame
                        output_frame.allocate_audio_buffer(
                            out_channels,
                            out_sample_rate,
                            out_samples as usize,
                            out_sample_fmt,
                        )?;

                        // Convert audio
                        swr_ctx.convert_frame(Some(&input_frame), &mut output_frame)?;

                        // Calculate timestamps
                        if input_frame.pts() != -1 {
                            let pts = input_frame.pts();
                            let in_time_base = unsafe { (*audio_stream).time_base };
                            let out_time_base = unsafe { (*output_stream).time_base };
                            let out_pts = unsafe {
                                sys::av_rescale_q(
                                    pts,
                                    in_time_base,
                                    out_time_base,
                                )
                            };
                            output_frame.set_pts(out_pts);
                        }

                        // Encode the resampled frame
                        unsafe {
                            let ret = sys::avcodec_send_frame(
                                output_stream.as_mut_ptr().codecpar.codec,
                                output_frame.as_ptr(),
                            );
                            if ret < 0 {
                                return Err(FFmpegError::new(ret).into());
                            }
                        }

                        // Get encoded packets
                        loop {
                            let mut enc_packet = Packet::new()?;
                            let ret = unsafe {
                                sys::avcodec_receive_packet(
                                    output_stream.as_mut_ptr().codecpar.codec,
                                    enc_packet.as_mut_ptr(),
                                )
                            };
                            match ret {
                                0 => {
                                    // Set stream index
                                    enc_packet.set_stream_index(0);

                                    // Rescale packet timestamps
                                    unsafe {
                                        sys::av_packet_rescale_ts(
                                            enc_packet.as_mut_ptr(),
                                            (*output_stream.as_mut_ptr().codecpar.codec).time_base,
                                            output_stream.time_base(),
                                        );
                                    }

                                    // Write the packet
                                    let ret = unsafe { sys::av_interleaved_write_frame(output_format_ctx, enc_packet.as_ptr()) };
                                    if ret < 0 {
                                        return Err(anyhow::anyhow!("Failed to write frame"));
                                    }
                                }
                                err if err == EAGAIN => break,
                                err => return Err(FFmpegError::new(err).into()),
                            }
                        }
                    }
                    Err(e) if e.code == EAGAIN => break,
                    Err(e) => return Err(e.into()),
                }
            }
        }
        packet.unref();
    }

    // Flush the decoder
    input_codec_ctx.send_packet(&Packet::new()?)?;
    loop {
        match input_codec_ctx.receive_frame(&mut input_frame) {
            Ok(()) => {
                // Calculate output frame size based on input frame and resampling ratio
                let out_samples = swr_ctx.get_out_samples(input_frame.sample_count());

                // Set up output frame
                output_frame.allocate_audio_buffer(
                    out_channels,
                    out_sample_rate,
                    out_samples as usize,
                    out_sample_fmt,
                )?;

                // Convert audio
                swr_ctx.convert_frame(Some(&input_frame), &mut output_frame)?;

                // Encode the resampled frame
                unsafe {
                    let ret = sys::avcodec_send_frame(
                        output_stream.as_mut_ptr().codecpar.codec,
                        output_frame.as_ptr(),
                    );
                    if ret < 0 {
                        return Err(FFmpegError::new(ret).into());
                    }
                }

                // Get encoded packets
                loop {
                    let mut enc_packet = Packet::new()?;
                    let ret = unsafe {
                        sys::avcodec_receive_packet(
                            output_stream.as_mut_ptr().codecpar.codec,
                            enc_packet.as_mut_ptr(),
                        )
                    };
                    match ret {
                        0 => {
                            // Set stream index
                            enc_packet.set_stream_index(0);

                            // Rescale packet timestamps
                            unsafe {
                                sys::av_packet_rescale_ts(
                                    enc_packet.as_mut_ptr(),
                                    (*output_stream.as_mut_ptr().codecpar.codec).time_base,
                                    output_stream.time_base(),
                                );
                            }

                            // Write the packet
                            let ret = unsafe { sys::av_interleaved_write_frame(output_format_ctx, enc_packet.as_ptr()) };
                            if ret < 0 {
                                return Err(anyhow::anyhow!("Failed to write frame"));
                            }
                        }
                        err if err == EAGAIN => break,
                        err => return Err(FFmpegError::new(err).into()),
                    }
                }
            }
            Err(e) if e.code == EAGAIN => break,
            Err(e) => return Err(e.into()),
        }
    }

    // Flush the encoder
    unsafe {
        let ret = sys::avcodec_send_frame(output_stream.as_mut_ptr().codecpar.codec, std::ptr::null());
        if ret < 0 {
            return Err(FFmpegError::new(ret).into());
        }
    }
    loop {
        let mut enc_packet = Packet::new()?;
        let ret = unsafe {
            sys::avcodec_receive_packet(
                output_stream.as_mut_ptr().codecpar.codec,
                enc_packet.as_mut_ptr(),
            )
        };
        match ret {
            0 => {
                // Set stream index
                enc_packet.set_stream_index(0);

                // Rescale packet timestamps
                unsafe {
                    sys::av_packet_rescale_ts(
                        enc_packet.as_mut_ptr(),
                        (*output_stream.as_mut_ptr().codecpar.codec).time_base,
                        output_stream.time_base(),
                    );
                }

                // Write the packet
                let ret = unsafe { sys::av_interleaved_write_frame(output_format_ctx, enc_packet.as_ptr()) };
                if ret < 0 {
                    return Err(anyhow::anyhow!("Failed to write frame"));
                }
            }
            err if err == EAGAIN => break,
            err => return Err(FFmpegError::new(err).into()),
        }
    }

    // Write trailer
    let ret = unsafe { sys::av_write_trailer(output_format_ctx) };
    if ret < 0 {
        return Err(anyhow::anyhow!("Failed to write trailer"));
    }

    println!(
        "Conversion complete! Output saved to: {}",
        args.output.display()
    );

    Ok(())
}
