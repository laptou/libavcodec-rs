int main(int argc, char *argv[]) {
    if (argc < 2) {
        fprintf(stderr, "Usage: %s <input_file>\n", argv[0]);
        return 1;
    }

    const char *input_file = argv[1];
    
    // Initialize libavformat and register all formats and codecs
    avformat_network_init();
    
    // Open input file and find stream info
    AVFormatContext *format_ctx = NULL;
    if (avformat_open_input(&format_ctx, input_file, NULL, NULL) != 0) {
        fprintf(stderr, "Could not open file %s\n", input_file);
        return 1;
    }
    
    if (avformat_find_stream_info(format_ctx, NULL) < 0) {
        fprintf(stderr, "Could not find stream information\n");
        return 1;
    }
    
    // Find the first video stream
    int video_stream_idx = -1;
    for (unsigned i = 0; i < format_ctx->nb_streams; i++) {
        if (format_ctx->streams[i]->codecpar->codec_type == AVMEDIA_TYPE_VIDEO) {
            video_stream_idx = i;
            break;
        }
    }
    
    if (video_stream_idx == -1) {
        fprintf(stderr, "Could not find video stream\n");
        return 1;
    }
    
    // Get the video stream
    AVStream *video_stream = format_ctx->streams[video_stream_idx];
    
    // Find the decoder for the video stream
    const AVCodec *codec = avcodec_find_decoder(video_stream->codecpar->codec_id);
    if (!codec) {
        fprintf(stderr, "Unsupported codec\n");
        return 1;
    }
    
    // Allocate a codec context for the decoder
    AVCodecContext *codec_ctx = avcodec_alloc_context3(codec);
    if (!codec_ctx) {
        fprintf(stderr, "Failed to allocate codec context\n");
        return 1;
    }
    
    // Copy parameters from the video stream to codec context
    if (avcodec_parameters_to_context(codec_ctx, video_stream->codecpar) < 0) {
        fprintf(stderr, "Failed to copy codec parameters to context\n");
        return 1;
    }
    
    // Open the codec
    if (avcodec_open2(codec_ctx, codec, NULL) < 0) {
        fprintf(stderr, "Failed to open codec\n");
        return 1;
    }
    
    // Allocate frames
    AVFrame *frame = av_frame_alloc();
    AVFrame *rgb_frame = av_frame_alloc();
    if (!frame || !rgb_frame) {
        fprintf(stderr, "Failed to allocate frames\n");
        return 1;
    }
    
    // Determine required buffer size and allocate buffer for RGB frame
    int num_bytes = av_image_get_buffer_size(AV_PIX_FMT_RGB24, 
                                            codec_ctx->width,
                                            codec_ctx->height, 
                                            1);
    uint8_t *rgb_buffer = (uint8_t *)av_malloc(num_bytes);
    if (!rgb_buffer) {
        fprintf(stderr, "Failed to allocate RGB buffer\n");
        return 1;
    }
    
    // Assign buffer to RGB frame
    av_image_fill_arrays(rgb_frame->data, rgb_frame->linesize, rgb_buffer,
                        AV_PIX_FMT_RGB24, codec_ctx->width, codec_ctx->height, 1);
    
    // Initialize SWS context for software scaling/conversion
    struct SwsContext *sws_ctx = sws_getContext(
        codec_ctx->width, codec_ctx->height, codec_ctx->pix_fmt,
        codec_ctx->width, codec_ctx->height, AV_PIX_FMT_RGB24,
        SWS_BILINEAR, NULL, NULL, NULL
    );
    
    if (!sws_ctx) {
        fprintf(stderr, "Failed to initialize SwsContext\n");
        return 1;
    }
    
    // Timebase for PTS conversion
    AVRational time_base = video_stream->time_base;
    
    // Packet for reading data
    AVPacket *packet = av_packet_alloc();
    if (!packet) {
        fprintf(stderr, "Failed to allocate packet\n");
        return 1;
    }
    
    // Read frames
    while (av_read_frame(format_ctx, packet) >= 0) {
        // Check if this packet belongs to the video stream
        if (packet->stream_index == video_stream_idx) {
            // Send packet to decoder
            int ret = avcodec_send_packet(codec_ctx, packet);
            if (ret < 0) {
                fprintf(stderr, "Error sending packet to decoder\n");
                break;
            }
            
            // Receive frame from decoder
            while (ret >= 0) {
                ret = avcodec_receive_frame(codec_ctx, frame);
                if (ret == AVERROR(EAGAIN) || ret == AVERROR_EOF) {
                    break;
                } else if (ret < 0) {
                    fprintf(stderr, "Error receiving frame from decoder\n");
                    goto end;
                }
                
                // Check if this is a keyframe
                if (frame->key_frame) {
                    // Convert frame to RGB24 format
                    sws_scale(sws_ctx, (const uint8_t * const*)frame->data, 
                             frame->linesize, 0, codec_ctx->height,
                             rgb_frame->data, rgb_frame->linesize);
                    
                    // Get PTS and convert to seconds
                    int64_t pts = frame->pts;
                    double pts_time = pts * av_q2d(time_base);
                    
                    printf("Keyframe PTS: %"PRId64" (%.3f seconds)\n", pts, pts_time);
                    
                    // Process the keyframe with your function
                    process_keyframe(rgb_buffer, codec_ctx->width, codec_ctx->height, 
                                   rgb_frame->linesize[0], pts, pts_time);
                }
            }
        }
        
        // Free the packet
        av_packet_unref(packet);
    }
    
end:
    // Clean up
    av_packet_free(&packet);
    av_frame_free(&frame);
    av_frame_free(&rgb_frame);
    av_free(rgb_buffer);
    avcodec_free_context(&codec_ctx);
    avformat_close_input(&format_ctx);
    sws_freeContext(sws_ctx);
    avformat_network_deinit();
    
    return 0;
}
