#include "libavcodec/avcodec.h"
#include "libavformat/avformat.h"
#include "libavutil/avutil.h"
#include "libswresample/swresample.h"
#include "libswscale/swscale.h"

#include "libavutil/opt.h"
#include "libavutil/mem.h"
#include "libavutil/rational.h"
#include "libavutil/imgutils.h"
#include "libavutil/pixdesc.h"

const int AVErrorEAgain = AVERROR(EAGAIN);
const int AVErrorEof = AVERROR_EOF;
