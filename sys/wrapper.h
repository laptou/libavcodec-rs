#include "libavcodec/avcodec.h"
#include "libavutil/avutil.h"
#include "libavutil/opt.h"
#include "libavutil/mem.h"
#include "libavutil/imgutils.h"
#include "libavutil/pixdesc.h"

const int AVErrorEAgain = AVERROR(EAGAIN);
const int AVErrorEof = AVERROR_EOF;
