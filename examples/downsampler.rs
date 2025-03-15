use anyhow::Result;
use clap::Parser;
use hound::{WavSpec, WavWriter};
use libavcodec::{
    AVMediaType, AVSampleFormat, Codec, CodecContext, EAGAIN, FormatContext, Frame, Packet,
    SwrContext,
};
use std::path::PathBuf;

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
    let mut format_ctx = FormatContext::open(&args.input)?;

    // Find audio stream
    let audio_stream = format_ctx
        .streams()
        .find(|s| matches!(s.codec_type(), AVMediaType::Audio))
        .expect("no audio stream found");

    // Get decoder
    let decoder = Codec::find_decoder(audio_stream.codec_id()).expect("failed to find decoder");

    // Create decoder context
    let mut codec_ctx = CodecContext::new(&decoder)?;
    audio_stream.apply_parameters_to_context(&mut codec_ctx)?;

    // Open codec
    codec_ctx.open(&decoder)?;

    // Get input format details
    let in_sample_rate = codec_ctx.sample_rate();
    let in_sample_fmt = codec_ctx.sample_format();
    let in_channels = codec_ctx.channel_count();

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

    // Set up WAV writer
    let spec = WavSpec {
        channels: args.channels,
        sample_rate: args.sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut wav_writer = WavWriter::create(&args.output, spec)?;

    // Read packets
    while format_ctx.read_packet(&mut packet)? {
        if packet.stream_index() == audio_stream.index() {
            // Send packet to decoder
            codec_ctx.send_packet(&packet)?;

            // Receive frames from decoder
            loop {
                match codec_ctx.receive_frame(&mut input_frame) {
                    Ok(()) => {
                        // Calculate output frame size based on input frame and resampling ratio
                        let out_samples = swr_ctx.get_out_samples(input_frame.sample_count());

                        // Set up output frame
                        // output_frame.set_nb_samples(out_samples);
                        output_frame.allocate_audio_buffer(
                            1,
                            out_sample_rate,
                            out_samples as usize,
                            out_sample_fmt,
                        )?;

                        // Convert audio
                        swr_ctx.convert_frame(Some(&input_frame), &mut output_frame)?;

                        // Calculate timestamps
                        let pts = if input_frame.pts() != -1 {
                            let pts = input_frame.pts();
                            let time_base = audio_stream.time_base();
                            let out_pts = unsafe {
                                libavcodec_sys::av_rescale_q(
                                    pts,
                                    libavcodec_sys::AVRational {
                                        num: time_base.num(),
                                        den: time_base.den(),
                                    },
                                    libavcodec_sys::AVRational {
                                        num: 1,
                                        den: out_sample_rate as i32,
                                    },
                                )
                            };
                            output_frame.set_pts(out_pts);
                            Some(out_pts)
                        } else {
                            None
                        };

                        // Write output frame data to WAV file
                        if let Some(data) = output_frame.data(0) {
                            // Get actual samples written
                            let samples_written =
                                output_frame.sample_count() as usize * out_channels;

                            // Convert raw bytes to i16 samples
                            let samples = data[..samples_written * 2]
                                .chunks_exact(2)
                                .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]));

                            // Write samples to WAV file
                            for sample in samples {
                                wav_writer.write_sample(sample)?;
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
    codec_ctx.send_packet(&Packet::new()?)?;
    loop {
        match codec_ctx.receive_frame(&mut input_frame) {
            Ok(()) => {
                // Calculate output frame size based on input frame and resampling ratio
                let out_samples = swr_ctx.get_out_samples(input_frame.sample_count());

                // Set up output frame
                // output_frame.set_nb_samples(out_samples);
                output_frame.allocate_audio_buffer(
                    1,
                    out_sample_rate,
                    out_samples as usize,
                    out_sample_fmt,
                )?;

                // Convert audio
                swr_ctx.convert_frame(Some(&input_frame), &mut output_frame)?;

                // Write output frame data to WAV file
                if let Some(data) = output_frame.data(0) {
                    // Get actual samples written
                    let samples_written = output_frame.sample_count() as usize * out_channels;

                    // Convert raw bytes to i16 samples
                    let samples = data[..samples_written * 2]
                        .chunks_exact(2)
                        .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]));

                    // Write samples to WAV file
                    for sample in samples {
                        wav_writer.write_sample(sample)?;
                    }
                }
            }
            Err(e) if e.code == EAGAIN => break,
            Err(e) => return Err(e.into()),
        }
    }

    // Flush the resampler
    loop {
        // Set up output frame
        // output_frame.set_nb_samples(1024); // Use a reasonable buffer size
        output_frame.allocate_audio_buffer(
            1,
            out_sample_rate,
            1024,
            out_sample_fmt,
        )?;

        // Convert any remaining samples
        match swr_ctx.convert_frame(None, &mut output_frame) {
            Ok(()) => {
                if output_frame.sample_count() > 0 {
                    // Write output frame data to WAV file
                    if let Some(data) = output_frame.data(0) {
                        // Get actual samples written
                        let samples_written = output_frame.sample_count() as usize * out_channels;

                        // Convert raw bytes to i16 samples
                        let samples = data[..samples_written * 2]
                            .chunks_exact(2)
                            .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]));

                        // Write samples to WAV file
                        for sample in samples {
                            wav_writer.write_sample(sample)?;
                        }
                    }
                } else {
                    break;
                }
            }
            Err(_) => break,
        }
    }

    // Finalize WAV file
    wav_writer.finalize()?;

    println!(
        "Conversion complete! Output saved to: {}",
        args.output.display()
    );
    Ok(())
}
