#include <stdio.h>
#include <stdarg.h>
#include <ctype.h>
#include <time.h>
#include <sys/time.h>

#include "util.h"

/*
 * Logging
 */

int
lprintf(FILE *fp, int level, const char *file, int line, const char *func,
        const char *fmt, ...)
{
  struct timeval tv;
  struct tm tm;
  char timestamp[32];
  int n = 0;
  va_list ap;

  flockfile(fp);
  gettimeofday(&tv, NULL);
  strftime(timestamp, sizeof(timestamp), "%T", localtime_r(&tv.tv_sec, &tm));
  n += fprintf(fp, "%s.%03d [%c] %s: ", timestamp, (int)(tv.tv_usec / 1000), 
               level, func);
  va_start(ap, fmt);
  n += vfprintf(fp, fmt, ap);
  va_end(ap);
  n += fprintf(fp, " (%s:%d)\n", file, line);
  funlockfile(fp);
  return n;
}

void
hexdump(FILE *fp, const void *data, size_t size)
{
  unsigned char *src;
  int offset, index;

  flockfile(fp);
  src = (unsigned char *)data;
  fprintf(fp, "+------+-------------------------------------------------+------------------+\n");
  for(offset = 0; offset < (int)size; offset += 16) {
    fprintf(fp, "| %04x | ", offset);
    for(index = 0; index < 16; index++) {
      if(offset + index < (int)size) {
        fprintf(fp, "%02x ", 0xff & src[offset + index]);
      } else {
        fprintf(fp, "   ");
      }
    }
    fprintf(fp, "| ");
    for(index = 0; index < 16; index++) {
      if(offset + index < (int)size) {
        if(isascii(src[offset + index]) && isprint(src[offset + index])) {
          fprintf(fp, "%c", src[offset + index]);
        } else {
          fprintf(fp, ".");
        }
      } else {
        fprintf(fp, " ");
      }
    }
    fprintf(fp, " |\n");
  }
  fprintf(fp, "+------+-------------------------------------------------+------------------+\n");
  funlockfile(fp);
}
