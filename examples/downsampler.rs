use anyhow::Result;
use clap::Parser;
use libavcodec::{
    AVCodecId, AVMediaType, AVSampleFormat, Codec, CodecContext, EAGAIN, FormatContext, Frame,
    Packet, ResampleAlgorithm, SwrContext,
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
    let mut input_format_ctx = FormatContext::open(&args.input)?;

    // Find audio stream
    let mut audio_stream = input_format_ctx
        .streams()
        .find(|s| matches!(s.codec_type(), AVMediaType::Audio))
        .ok_or_else(|| anyhow::anyhow!("no audio stream found"))?;

    // Get decoder
    let decoder = Codec::find_decoder(audio_stream.codec_id())
        .ok_or_else(|| anyhow::anyhow!("failed to find decoder"))?;

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
    let out_sample_rate = args.sample_rate as i32;
    let out_channels = args.channels as i32;
    let out_sample_fmt = AVSampleFormat::S16; // 16-bit signed PCM for WAV

    println!(
        "Output: {} Hz, {} channels, format: {:?}",
        out_sample_rate, out_channels, out_sample_fmt
    );

    // Create output format context for WAV
    let mut output_format_ctx = FormatContext::output(&args.output, Some("wav"))?;

    // Create output stream
    let mut output_stream = output_format_ctx.new_stream()?;

    // Set output codec parameters
    output_stream.set_audio_codec_params(
        AVMediaType::Audio,
        AVCodecId::PcmS16le,
        out_sample_rate as usize,
        out_channels as usize,
        out_sample_fmt,
    )?;

    // Get encoder and create encoder context
    let encoder = Codec::find_encoder(AVCodecId::PcmS16le)
        .ok_or_else(|| anyhow::anyhow!("failed to find PCM encoder"))?;
    let mut output_codec_ctx = CodecContext::new(&encoder)?;

    // Set encoder parameters
    output_codec_ctx.set_sample_rate(out_sample_rate);
    output_codec_ctx.set_sample_format(out_sample_fmt);
    output_codec_ctx.set_channel_count(out_channels);
    output_codec_ctx.set_time_base(output_stream.time_base());

    // Open encoder
    output_codec_ctx.open(&encoder)?;

    // Write header
    output_format_ctx.write_header()?;

    // Create frames
    let mut input_frame = Frame::new()?;
    let mut output_frame = Frame::new()?;

    // Set up output frame parameters
    output_frame.set_channel_count(out_channels);
    output_frame.set_format(out_sample_fmt as i32);
    output_frame.set_sample_rate(out_sample_rate);

    // Create resampler context with proper channel layouts
    let mut swr_ctx = SwrContext::get_context_with_algorithm(
        in_sample_rate as usize,
        in_sample_fmt,
        in_channels as usize,
        out_sample_rate as usize,
        out_sample_fmt,
        out_channels as usize,
        ResampleAlgorithm::Sinc { quality: 5 }, // Use higher quality resampling
    )?;

    // Create packet for reading
    let mut packet = Packet::new()?;

    // Read packets
    while input_format_ctx.read_packet(&mut packet)? {
        tracing::trace!("got packet: {packet:?}");

        if packet.stream_index() == audio_stream.index() {
            // Send packet to decoder
            input_codec_ctx.send_packet(&packet)?;

            // Receive frames from decoder
            loop {
                match input_codec_ctx.receive_frame(&mut input_frame) {
                    Ok(()) => {
                        // Calculate output frame size based on input frame and resampling ratio
                        let out_samples = swr_ctx.get_out_samples(input_frame.sample_count());

                        // Set up output frame - allocate buffer BEFORE conversion
                        output_frame.allocate_audio_buffer(
                            out_channels as usize,
                            out_sample_rate as usize,
                            out_samples as usize,
                            out_sample_fmt,
                        )?;

                        // Ensure frame is writable before conversion
                        output_frame.make_writable()?;

                        // Convert audio
                        swr_ctx.convert_frame(Some(&input_frame), &mut output_frame)?;

                        // Calculate timestamps
                        // if input_frame.pts() != -1 {
                        //     let pts = input_frame.pts();
                        //     let in_time_base = audio_stream.time_base();
                        //     let out_time_base = output_stream.time_base();
                        //     let out_pts =
                        //         pts * (out_time_base.den() as i64) * (in_time_base.num() as i64)
                        //             / ((in_time_base.den() as i64) * (out_time_base.num() as i64));
                        //     output_frame.set_pts(out_pts);
                        // }

                        // Send frame to encoder
                        output_codec_ctx.send_frame(Some(&output_frame))?;

                        // Receive packets from encoder
                        loop {
                            let mut enc_packet = Packet::new()?;
                            match output_codec_ctx.receive_packet(&mut enc_packet) {
                                Ok(()) => {
                                    enc_packet.set_stream_index(0);
                                    // Write the packet
                                    // output_format_ctx.write_frame_interleaved(&mut enc_packet)?;
                                    output_format_ctx.write_frame(&mut enc_packet)?;

                                }
                                Err(e) if e.code == EAGAIN => break,
                                Err(e) => return Err(e.into()),
                            }
                        }
                    }
                    Err(e) if e.code == EAGAIN => break,
                    Err(e) => return Err(e.into()),
                }
            }
        }

        packet = Packet::new()?;
    }

    // Flush the decoder
    input_codec_ctx.send_packet(&Packet::new()?)?;
    loop {
        match input_codec_ctx.receive_frame(&mut input_frame) {
            Ok(()) => {
                // Calculate output frame size based on input frame and resampling ratio
                let out_samples = swr_ctx.get_out_samples(input_frame.sample_count());

                // Set up output frame - allocate buffer BEFORE conversion
                output_frame.allocate_audio_buffer(
                    out_channels as usize,
                    out_sample_rate as usize,
                    out_samples as usize,
                    out_sample_fmt,
                )?;

                // Ensure frame is writable before conversion
                output_frame.make_writable()?;

                // Convert audio
                swr_ctx.convert_frame(Some(&input_frame), &mut output_frame)?;

                // Calculate timestamps
                if input_frame.pts() != -1 {
                    let pts = input_frame.pts();
                    let in_time_base = audio_stream.time_base();
                    let out_time_base = output_stream.time_base();
                    let out_pts = pts * (out_time_base.den() as i64) * (in_time_base.num() as i64)
                        / ((in_time_base.den() as i64) * (out_time_base.num() as i64));
                    output_frame.set_pts(out_pts);
                }

                // Send frame to encoder
                output_codec_ctx.send_frame(Some(&output_frame))?;

                // Receive packets from encoder
                loop {
                    let mut enc_packet = Packet::new()?;
                    match output_codec_ctx.receive_packet(&mut enc_packet) {
                        Ok(()) => {
                            enc_packet.set_stream_index(0);
                            // Write the packet
                            output_format_ctx.write_frame_interleaved(&mut enc_packet)?;
                        }
                        Err(e) if e.code == EAGAIN => break,
                        Err(e) => return Err(e.into()),
                    }
                }
            }
            Err(e) if e.code == EAGAIN => break,
            Err(e) => return Err(e.into()),
        }
    }

    // Flush the encoder
    output_codec_ctx.send_frame(None)?;
    loop {
        let mut enc_packet = Packet::new()?;
        match output_codec_ctx.receive_packet(&mut enc_packet) {
            Ok(()) => {
                enc_packet.set_stream_index(0);
                // Write the packet
                output_format_ctx.write_frame_interleaved(&mut enc_packet)?;
            }
            Err(e) if e.code == EAGAIN => break,
            Err(e) => return Err(e.into()),
        }
    }

    // Write trailer
    output_format_ctx.write_trailer()?;

    println!(
        "Conversion complete! Output saved to: {}",
        args.output.display()
    );

    Ok(())
}
