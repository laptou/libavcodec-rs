#include <stdio.h>
#include <stdlib.h>
#include <libavcodec/avcodec.h>
#include <libavformat/avformat.h>
#include <libavutil/opt.h>
#include <libavutil/channel_layout.h>
#include <libswresample/swresample.h>

int main(int argc, char **argv) {
    // Check command line arguments
    if (argc != 3) {
        fprintf(stderr, "Usage: %s <input_file> <output_file>\n", argv[0]);
        return 1;
    }
    
    // Input and output file names
    const char *input_file = argv[1];
    const char *output_file = argv[2];
    
    // Initialize libavformat and register all formats and codecs
    avformat_network_init();
    
    // Define variables
    int ret;
    AVFormatContext *input_format_ctx = NULL;
    AVFormatContext *output_format_ctx = NULL;
    SwrContext *swr_ctx = NULL;
    AVStream *input_stream = NULL;
    AVStream *output_stream = NULL;
    AVCodecContext *input_codec_ctx = NULL;
    AVCodecContext *output_codec_ctx = NULL;
    const AVCodec *input_codec = NULL;
    const AVCodec *output_codec = NULL;
    AVFrame *frame = NULL;
    AVFrame *resampled_frame = NULL;
    AVPacket packet;
    int stream_index = -1;
    int64_t src_ch_layout, dst_ch_layout;
    const int dst_sample_rate = 16000; // Target: 16kHz
    
    // Open input file
    if ((ret = avformat_open_input(&input_format_ctx, input_file, NULL, NULL)) < 0) {
        fprintf(stderr, "Could not open input file '%s'\n", input_file);
        goto end;
    }
    
    // Retrieve stream information
    if ((ret = avformat_find_stream_info(input_format_ctx, NULL)) < 0) {
        fprintf(stderr, "Could not find stream information\n");
        goto end;
    }
    
    // Dump information about file onto standard error
    av_dump_format(input_format_ctx, 0, input_file, 0);
    
    // Find the audio stream
    for (int i = 0; i < input_format_ctx->nb_streams; i++) {
        if (input_format_ctx->streams[i]->codecpar->codec_type == AVMEDIA_TYPE_AUDIO) {
            stream_index = i;
            break;
        }
    }
    
    if (stream_index == -1) {
        fprintf(stderr, "Could not find audio stream in input file\n");
        goto end;
    }
    
    input_stream = input_format_ctx->streams[stream_index];
    
    // Find the decoder for the audio stream
    input_codec = avcodec_find_decoder(input_stream->codecpar->codec_id);
    if (!input_codec) {
        fprintf(stderr, "Could not find codec\n");
        goto end;
    }
    
    // Allocate codec context for the decoder
    input_codec_ctx = avcodec_alloc_context3(input_codec);
    if (!input_codec_ctx) {
        fprintf(stderr, "Could not allocate codec context\n");
        goto end;
    }
    
    // Copy codec parameters from input stream to input codec context
    if ((ret = avcodec_parameters_to_context(input_codec_ctx, input_stream->codecpar)) < 0) {
        fprintf(stderr, "Failed to copy codec parameters to decoder context\n");
        goto end;
    }
    
    // Open the decoder
    if ((ret = avcodec_open2(input_codec_ctx, input_codec, NULL)) < 0) {
        fprintf(stderr, "Could not open codec\n");
        goto end;
    }
    
    // Set up the output format context for WAV
    if ((ret = avformat_alloc_output_context2(&output_format_ctx, NULL, "wav", output_file)) < 0) {
        fprintf(stderr, "Could not allocate output format context\n");
        goto end;
    }
    
    // Find the WAV encoder
    output_codec = avcodec_find_encoder(AV_CODEC_ID_PCM_S16LE);
    if (!output_codec) {
        fprintf(stderr, "Could not find WAV encoder\n");
        goto end;
    }
    
    // Create a new audio stream in the output file
    output_stream = avformat_new_stream(output_format_ctx, NULL);
    if (!output_stream) {
        fprintf(stderr, "Could not create new stream\n");
        goto end;
    }
    
    output_codec_ctx = avcodec_alloc_context3(output_codec);
    if (!output_codec_ctx) {
        fprintf(stderr, "Could not allocate an encoding context\n");
        goto end;
    }
    
    // Set output codec parameters
    output_codec_ctx->channels = 1; // Mono
    output_codec_ctx->channel_layout = AV_CH_LAYOUT_MONO;
    output_codec_ctx->sample_rate = dst_sample_rate;
    output_codec_ctx->sample_fmt = AV_SAMPLE_FMT_S16;
    output_codec_ctx->bit_rate = dst_sample_rate * 16; // 16 bits per sample
    
    // Set the time base
    output_stream->time_base = (AVRational){1, dst_sample_rate};
    
    // Some container formats (like WAV) require global headers
    if (output_format_ctx->oformat->flags & AVFMT_GLOBALHEADER)
        output_codec_ctx->flags |= AV_CODEC_FLAG_GLOBAL_HEADER;
    
    // Open the encoder for the audio stream
    if ((ret = avcodec_open2(output_codec_ctx, output_codec, NULL)) < 0) {
        fprintf(stderr, "Could not open output codec\n");
        goto end;
    }
    
    // Copy the codec parameters to the stream
    if ((ret = avcodec_parameters_from_context(output_stream->codecpar, output_codec_ctx)) < 0) {
        fprintf(stderr, "Could not copy codec parameters to stream\n");
        goto end;
    }
    
    // Set up SWR context for resampling
    AVChannelLayout in_ch_layout;
    AVChannelLayout out_ch_layout = AV_CHANNEL_LAYOUT_MONO;
    
    av_channel_layout_default(&in_ch_layout, input_codec_ctx->channels);
    
    swr_ctx = swr_alloc();
    if (!swr_ctx) {
        fprintf(stderr, "Could not allocate resampler context\n");
        goto end;
    }
    
    // Set options
    av_opt_set_chlayout(swr_ctx, "in_chlayout", &in_ch_layout, 0);
    av_opt_set_chlayout(swr_ctx, "out_chlayout", &out_ch_layout, 0);

    // Set sample rate and format options
    av_opt_set_int(swr_ctx, "in_sample_rate", input_codec_ctx->sample_rate, 0);
    av_opt_set_int(swr_ctx, "out_sample_rate", dst_sample_rate, 0);
    av_opt_set_sample_fmt(swr_ctx, "in_sample_fmt", input_codec_ctx->sample_fmt, 0);
    av_opt_set_sample_fmt(swr_ctx, "out_sample_fmt", AV_SAMPLE_FMT_S16, 0);
    
    // Initialize the resampling context
    if ((ret = swr_init(swr_ctx)) < 0) {
        fprintf(stderr, "Failed to initialize the resampling context\n");
        goto end;
    }
    
    // Dump output format
    av_dump_format(output_format_ctx, 0, output_file, 1);
    
    // Open the output file
    if (!(output_format_ctx->oformat->flags & AVFMT_NOFILE)) {
        if ((ret = avio_open(&output_format_ctx->pb, output_file, AVIO_FLAG_WRITE)) < 0) {
            fprintf(stderr, "Could not open output file '%s'\n", output_file);
            goto end;
        }
    }
    
    // Write the stream header
    if ((ret = avformat_write_header(output_format_ctx, NULL)) < 0) {
        fprintf(stderr, "Error writing header to output file\n");
        goto end;
    }
    
    // Allocate frame for input
    frame = av_frame_alloc();
    if (!frame) {
        fprintf(stderr, "Could not allocate frame\n");
        ret = AVERROR(ENOMEM);
        goto end;
    }
    
    // Allocate frame for resampled output
    resampled_frame = av_frame_alloc();
    if (!resampled_frame) {
        fprintf(stderr, "Could not allocate resampled frame\n");
        ret = AVERROR(ENOMEM);
        goto end;
    }
    
    // Initialize packet
    av_init_packet(&packet);
    packet.data = NULL;
    packet.size = 0;
    
    // Read all packets
    while (av_read_frame(input_format_ctx, &packet) >= 0) {
        if (packet.stream_index == stream_index) {
            // Send the packet to the decoder
            ret = avcodec_send_packet(input_codec_ctx, &packet);
            if (ret < 0) {
                fprintf(stderr, "Error submitting the packet to the decoder\n");
                break;
            }
            
            // Get all available frames from the decoder
            while (ret >= 0) {
                ret = avcodec_receive_frame(input_codec_ctx, frame);
                if (ret == AVERROR(EAGAIN) || ret == AVERROR_EOF) {
                    // We need more packets
                    break;
                } else if (ret < 0) {
                    // Error during decoding
                    fprintf(stderr, "Error during decoding\n");
                    goto end;
                }
                
                // Setup resampled frame
                resampled_frame->format = AV_SAMPLE_FMT_S16;
                av_channel_layout_copy(&resampled_frame->ch_layout, &out_ch_layout);
                resampled_frame->sample_rate = dst_sample_rate;
                resampled_frame->nb_samples = av_rescale_rnd(swr_get_delay(swr_ctx, input_codec_ctx->sample_rate) + frame->nb_samples,
                                                            dst_sample_rate, input_codec_ctx->sample_rate, AV_ROUND_UP);
                
                // Allocate buffer for resampled data
                ret = av_frame_get_buffer(resampled_frame, 0);
                if (ret < 0) {
                    fprintf(stderr, "Could not allocate output frame samples\n");
                    goto end;
                }
                
                // Resample the frame
                ret = swr_convert(swr_ctx, resampled_frame->data, resampled_frame->nb_samples,
                                (const uint8_t **)frame->data, frame->nb_samples);
                if (ret < 0) {
                    fprintf(stderr, "Error while converting\n");
                    goto end;
                }
                
                // Set the correct PTS
                resampled_frame->pts = av_rescale_q(frame->pts, 
                                                  input_stream->time_base, 
                                                  output_stream->time_base);
                
                // Encode the resampled frame
                ret = avcodec_send_frame(output_codec_ctx, resampled_frame);
                if (ret < 0) {
                    fprintf(stderr, "Error sending frame to encoder\n");
                    goto end;
                }
                
                while (ret >= 0) {
                    AVPacket enc_packet;
                    av_init_packet(&enc_packet);
                    enc_packet.data = NULL;
                    enc_packet.size = 0;
                    
                    ret = avcodec_receive_packet(output_codec_ctx, &enc_packet);
                    if (ret == AVERROR(EAGAIN) || ret == AVERROR_EOF) {
                        break;
                    } else if (ret < 0) {
                        fprintf(stderr, "Error during encoding\n");
                        goto end;
                    }
                    
                    // Rescale output packet timestamp values from codec to stream timebase
                    enc_packet.stream_index = 0;
                    av_packet_rescale_ts(&enc_packet, output_codec_ctx->time_base, output_stream->time_base);
                    
                    // Write the packet to the output file
                    ret = av_interleaved_write_frame(output_format_ctx, &enc_packet);
                    if (ret < 0) {
                        fprintf(stderr, "Error writing packet to output file\n");
                        goto end;
                    }
                    
                    av_packet_unref(&enc_packet);
                }
                
                av_frame_unref(resampled_frame);
                av_frame_unref(frame);
            }
        }
        
        av_packet_unref(&packet);
    }
    
    // Flush encoder
    ret = avcodec_send_frame(output_codec_ctx, NULL);
    if (ret >= 0) {
        while (1) {
            AVPacket enc_packet;
            av_init_packet(&enc_packet);
            enc_packet.data = NULL;
            enc_packet.size = 0;
            
            ret = avcodec_receive_packet(output_codec_ctx, &enc_packet);
            if (ret < 0)
                break;
            
            // Rescale output packet timestamp values from codec to stream timebase
            enc_packet.stream_index = 0;
            av_packet_rescale_ts(&enc_packet, output_codec_ctx->time_base, output_stream->time_base);
            
            // Write the packet to the output file
            ret = av_interleaved_write_frame(output_format_ctx, &enc_packet);
            if (ret < 0) {
                fprintf(stderr, "Error writing packet to output file\n");
                goto end;
            }
            
            av_packet_unref(&enc_packet);
        }
    }
    
    // Write the trailer
    av_write_trailer(output_format_ctx);
    
end:
    // Clean up
    if (input_codec_ctx)
        avcodec_free_context(&input_codec_ctx);
    if (output_codec_ctx)
        avcodec_free_context(&output_codec_ctx);
    if (frame)
        av_frame_free(&frame);
    if (resampled_frame)
        av_frame_free(&resampled_frame);
    if (swr_ctx)
        swr_free(&swr_ctx);
    if (input_format_ctx)
        avformat_close_input(&input_format_ctx);
    if (output_format_ctx && !(output_format_ctx->oformat->flags & AVFMT_NOFILE))
        avio_closep(&output_format_ctx->pb);
    if (output_format_ctx)
        avformat_free_context(output_format_ctx);
    
    avformat_network_deinit();
    
    if (ret < 0) {
        fprintf(stderr, "Error occurred: %s\n", av_err2str(ret));
        return 1;
    }
    
    printf("Conversion completed successfully! Output file: %s\n", output_file);
    return 0;
}
