#ifndef HDFS_H
#define HDFS_H

#include <stdint.h>

typedef struct {
    void *fs;
} HdfsHandler;

HdfsHandler *hdfs_handler_new();
void hdfs_handler_free(HdfsHandler *handler);

int hdfs_read_file(HdfsHandler *handler, const char *path, uint8_t **buffer, size_t *buffer_size);
int hdfs_write_file(HdfsHandler *handler, const char *path, const uint8_t *buffer, size_t buffer_size);
int hdfs_file_exists(HdfsHandler *handler, const char *path);

#endif /* HDFS_H */