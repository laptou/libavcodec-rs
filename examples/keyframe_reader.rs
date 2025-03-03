use anyhow::Result;
use image::{ImageBuffer, Rgb};
use libavcodec::{
    AVDiscard, AVMediaType, AVPixelFormat, Codec, CodecContext, FormatContext, Frame, Packet,
    SwsContext,
};
use libavcodec_sys;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    // Check command line arguments
    // let args: Vec<String> = std::env::args().collect();
    // if args.len() != 2 {
    //     eprintln!("Usage: {} <input_video>", args[0]);
    //     std::process::exit(1);
    // }

    // let input_file = args[1].clone();
    let input_file = "C:\\Users\\ibiyemi\\Videos\\nokia.mov";

    // Create output directory
    let output_dir = Path::new("keyframes");
    fs::create_dir_all(&output_dir)?;

    // Open input file
    let mut format_ctx = FormatContext::open(&input_file)?;

    // Find video stream
    let video_stream = format_ctx
        .streams()
        .find(|s| matches!(s.codec_type(), AVMediaType::Video))
        .expect("no video stream found");

    // Get decoder
    let decoder = Codec::find_decoder(video_stream.codec_id()).expect("failed to find decoder");

    // Create decoder context
    let mut codec_ctx = CodecContext::new(&decoder)?;
    video_stream.apply_parameters_to_context(&mut codec_ctx)?;

    // Only decode keyframes
    codec_ctx.set_skip_frame(AVDiscard::NonKey);

    // Open codec
    codec_ctx.open(&decoder)?;

    // Create frames
    let mut frame = Frame::new()?;
    let mut rgb_frame = Frame::new()?;

    // Get frame dimensions
    let width = codec_ctx.width();
    let height = codec_ctx.height();
    let src_pix_fmt = codec_ctx.pix_fmt();

    // Allocate buffer for RGB frame
    rgb_frame.allocate_buffer_ffmpeg(
        width,
        height,
        AVPixelFormat::Rgb24,
        1, // align to 1-byte boundary since we're using it with image crate
    )?;

    // Create scaler context for YUV to RGB conversion
    let mut sws_ctx = SwsContext::get_context(
        width,
        height,
        src_pix_fmt,
        width,
        height,
        AVPixelFormat::Rgb24,
        0,
    )?;

    // Create packet for reading
    let mut packet = Packet::new()?;

    // Get stream timebase for PTS conversion
    let time_base = video_stream.time_base();

    // Read frames
    while format_ctx.read_packet(&mut packet)? {
        if packet.stream_index() == video_stream.index() {
            // Send packet to decoder
            codec_ctx.send_packet(&packet)?;

            // Receive frames from decoder
            loop {
                match codec_ctx.receive_frame(&mut frame) {
                    Ok(()) => {
                        // Since we set skip_frame to NonKey, we know this is a keyframe
                        let pts = frame.pts();
                        let pts_time = pts as f64 * time_base.as_f64();

                        println!("Saving keyframe PTS: {} ({:.3} seconds)", pts, pts_time);

                        // Convert to RGB
                        sws_ctx.copy(&frame, &mut rgb_frame)?;

                        // Create RGB image from frame data
                        let rgb_data = rgb_frame.data(0).unwrap();
                        let rgb_image = image::ImageBuffer::<Rgb<u8>, _>::from_raw(
                            width as u32,
                            height as u32,
                            rgb_data,
                        )
                        .unwrap();

                        // Save as JPEG
                        let output_path = output_dir.join(format!("frame_{}.jpg", pts));
                        rgb_image.save(output_path)?;
                    }
                    Err(e) if e.code == libavcodec::EAGAIN => break,
                    Err(e) => return Err(e.into()),
                }
            }
        }
        packet.unref();
    }

    Ok(())
}
