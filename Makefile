# Makefile for zstd_compress

# Compiler
CC = gcc

# Compiler flags
CFLAGS = -Wall -Wextra -O2 -std=c99

# Linker flags
LDFLAGS = -lhdfs -lzstd

# Source files
SRCS = main.c hdfs.c compression.c

# Header files
HDRS = hdfs.h compression.h

# Object files
OBJS = $(SRCS:.c=.o)

# Target executable
TARGET = zstd_compress

.PHONY: all clean

all: $(TARGET)

$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)

%.o: %.c $(HDRS)
	$(CC) $(CFLAGS) -c -o $@ $<

clean:
	rm -f $(OBJS) $(TARGET)

install: $(TARGET)
	cp $(TARGET) /usr/local/bin/