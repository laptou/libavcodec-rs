#include "libavcodec/avcodec.h"
#include "libavformat/avformat.h"
#include "libavutil/avutil.h"
#include "libswresample/swresample.h"
#include "libswscale/swscale.h"

#include "libavutil/imgutils.h"
#include "libavutil/log.h"
#include "libavutil/mem.h"
#include "libavutil/opt.h"
#include "libavutil/pixdesc.h"
#include "libavutil/rational.h"

const int AVErrorEAgain = AVERROR(EAGAIN);
const int AVErrorEof = AVERROR_EOF;
enum AVError {
  BSF_NOT_FOUND = AVERROR_BSF_NOT_FOUND,
  BUG = AVERROR_BUG,
  BUFFER_TOO_SMALL = AVERROR_BUFFER_TOO_SMALL,
  DECODER_NOT_FOUND = AVERROR_DECODER_NOT_FOUND,
  DEMUXER_NOT_FOUND = AVERROR_DEMUXER_NOT_FOUND,
  ENCODER_NOT_FOUND = AVERROR_ENCODER_NOT_FOUND,
  EEOF = AVERROR_EOF,
  EXIT = AVERROR_EXIT,
  EXTERNAL = AVERROR_EXTERNAL,
  FILTER_NOT_FOUND = AVERROR_FILTER_NOT_FOUND,
  INVALIDDATA = AVERROR_INVALIDDATA,
  MUXER_NOT_FOUND = AVERROR_MUXER_NOT_FOUND,
  OPTION_NOT_FOUND = AVERROR_OPTION_NOT_FOUND,
  PATCHWELCOME = AVERROR_PATCHWELCOME,
  PROTOCOL_NOT_FOUND = AVERROR_PROTOCOL_NOT_FOUND,
};

void avrs_format_msg(char *buf, int buf_size, const char *fmt, va_list args);
