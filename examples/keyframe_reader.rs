use anyhow::Context;
use libavcodec::*;
use std::env;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    // Check command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        return Ok(());
    }

    let input_file = Path::new(&args[1]);
    
    // Open input file and find stream info
    let mut format_ctx = FormatContext::open(input_file)?;
    
    // Find the first video stream
    let video_stream = format_ctx
        .streams()
        .find(|s| s.codec_type() == AVMediaType::Video)
        .context("Could not find video stream")?;

    // Get the decoder for the video stream
    let decoder = Codec::find_decoder(video_stream.codec_id())
        .context("Unsupported codec")?;
    
    // Create and configure decoder context
    let mut codec_ctx = CodecContext::new(&decoder)?;
    video_stream.apply_parameters_to_context(&mut codec_ctx)?;
    codec_ctx.open(&decoder)?;

    // Allocate frames
    let mut frame = Frame::new()?;
    let mut packet = Packet::new()?;

    // Get timebase for PTS conversion
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
                        // Check if this is a keyframe
                        if frame.is_key_frame() {
                            // Get PTS and convert to seconds
                            let pts = frame.pts();
                            let pts_time = pts as f64 * time_base.as_f64();
                            
                            println!("Keyframe PTS: {} ({:.3} seconds)", pts, pts_time);
                            
                            // Here you can process the keyframe data
                            // frame.data(0) contains Y plane
                            // frame.data(1) contains U plane
                            // frame.data(2) contains V plane
                            process_keyframe(&frame);
                        }
                    }
                    Err(FFmpegError { code: EAGAIN, .. }) => break,
                    Err(e) => return Err(e.into()),
                }
            }
        }
        packet.unref();
    }

    Ok(())
}

fn process_keyframe(frame: &Frame) {
    // Get frame dimensions
    let width = frame.width();
    let height = frame.height();
    
    println!("Processing keyframe: {}x{}", width, height);
    
    // Here you can implement your keyframe processing logic
    // For example, you can access the raw frame data:
    let y_plane = frame.data(0);
    let u_plane = frame.data(1);
    let v_plane = frame.data(2);
    
    // Get the line sizes for each plane
    let y_linesize = frame.linesize(0);
    let u_linesize = frame.linesize(1);
    let v_linesize = frame.linesize(2);
    
    println!("Y plane linesize: {}", y_linesize);
    println!("U plane linesize: {}", u_linesize);
    println!("V plane linesize: {}", v_linesize);
} 
