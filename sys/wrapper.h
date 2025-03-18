#include "libavcodec/avcodec.h"
#include "libavformat/avformat.h"
#include "libavutil/avutil.h"
#include "libswresample/swresample.h"
#include "libswscale/swscale.h"

#include "libavutil/log.h"
#include "libavutil/opt.h"
#include "libavutil/mem.h"
#include "libavutil/rational.h"
#include "libavutil/imgutils.h"
#include "libavutil/pixdesc.h"

const int AVErrorEAgain = AVERROR(EAGAIN);
const int AVErrorEof = AVERROR_EOF;

void avrs_format_msg(char *buf, int buf_size, const char *fmt, va_list args);
