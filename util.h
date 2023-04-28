#ifndef UTIL_H
#define UTIL_H

#include <stdio.h>
#include <stdint.h>
#include <unistd.h>

/*
 * Logging
 */
#define errorf(...) lprintf(stderr, 'E', __FILE__, __LINE__, __func__, __VA_ARGS__)
#define warnf(...) lprintf(stderr, 'W', __FILE__, __LINE__, __func__, __VA_ARGS__)
#define infof(...) lprintf(stderr, 'I', __FILE__, __LINE__, __func__, __VA_ARGS__)
#define debugf(...) lprintf(stderr, 'D', __FILE__, __LINE__, __func__, __VA_ARGS__)

#ifdef HEXDUMP
#define debugdump(...) hexdump(stderr, __VA_ARGS__)
#else
#define debugdump(...)
#endif

extern int
lprintf(FILE *fp, int level, const char *file, int line, const char *func, const char *fmt, ...);
extern void
hexdump(FILE *fp, const void *data, size_t size);

/*
 * Queue
 */
struct queue_entry;

struct queue_head {
  struct queue_entry *head;
  struct queue_entry *tail;
  unsigned int num;
};

extern void
queue_init(struct queue_head *queue);
extern void *
queue_push(struct queue_head *queue, void *data);
extern void *
queue_pop(struct queue_head *queue);
extern void *
queue_peek(struct queue_head *queue);
extern void
qeuue_foreach(struct queue_head *queue, void (*func)(void *arg, void *data), void *arg);

#endif
