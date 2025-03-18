#include <stdio.h>
#include <stdarg.h>

extern "C" void avrs_format_msg(char *buf, int buf_size, const char *fmt, va_list args) {
  vsnprintf(buf, buf_size, fmt, args);
}
