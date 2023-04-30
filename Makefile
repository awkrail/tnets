APPS = 

DRIVERS = driver/dummy.o \
					driver/loopback.o \

OBJS = util.o \
			 net.o \
			 ip.o \

TESTS = test/step0.exe \
				test/step1.exe \
				test/step2.exe \
				test/step3.exe \
				test/step4.exe \

CFLAGS := $(CFLAGS) -g -W -Wall -Wno-unused-parameter -iquote .

BASE = platform/linux
CFLAGS := $(CFLAGS) -pthread -iquote $(BASE)
OBJS := $(OBJS) $(BASE)/intr.o

.SUFFIXES:
.SUFFIXES: .c .o

.PHONY: all clean

all: $(APPS) $(TESTS)

$(APPS): %.exe : %.o $(OBJS) $(DRIVERS)
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)

$(TESTS): %.exe : %.o $(OBJS) $(DRIVERS) test/test.h
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)

.c.o:
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -rf $(APPS) $(APPS:.exe=.o) $(OBJS) $(DRIVERS) $(TESTS) $(TESTS:.exe=.o)
