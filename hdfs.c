#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <hdfs/hdfs.h>
#include "hdfs.h"

HdfsHandler *hdfs_handler_new() {
    HdfsHandler *handler = (HdfsHandler *)malloc(sizeof(HdfsHandler));
    if (!handler) {
        fprintf(stderr, "Failed to allocate memory for HdfsHandler\n");
        return NULL;
    }

    // Connect to HDFS using the default configuration from /etc/hadoop
    handler->fs = hdfsConnect(NULL, 0);
    if (!handler->fs) {
        fprintf(stderr, "Failed to connect to HDFS\n");
        free(handler);
        return NULL;
    }

    printf("Successfully connected to HDFS\n");
    return handler;
}

void hdfs_handler_free(HdfsHandler *handler) {
    if (handler) {
        if (handler->fs) {
            hdfsDisconnect(handler->fs);
            printf("HDFS connection disconnected\n");
        }
        free(handler);
    }
}

int hdfs_read_file(HdfsHandler *handler, const char *path, uint8_t **buffer, size_t *buffer_size) {
    if (!handler || !handler->fs || !path || !buffer || !buffer_size) {
        fprintf(stderr, "Invalid parameters for hdfs_read_file\n");
        return -1;
    }

    printf("Reading file from HDFS: %s\n", path);

    hdfsFile file = hdfsOpenFile(handler->fs, path, O_RDONLY, 0, 0, 0);
    if (!file) {
        fprintf(stderr, "Failed to open file: %s\n", path);
        return -1;
    }

    // Read file content
    size_t buffer_capacity = 8192;
    *buffer = (uint8_t *)malloc(buffer_capacity);
    if (!*buffer) {
        fprintf(stderr, "Failed to allocate memory for buffer\n");
        hdfsCloseFile(handler->fs, file);
        return -1;
    }

    *buffer_size = 0;
    char temp_buf[8192];
    int bytes_read;

    while ((bytes_read = hdfsRead(handler->fs, file, temp_buf, sizeof(temp_buf))) > 0) {
        if (*buffer_size + bytes_read > buffer_capacity) {
            buffer_capacity *= 2;
            uint8_t *new_buffer = (uint8_t *)realloc(*buffer, buffer_capacity);
            if (!new_buffer) {
                fprintf(stderr, "Failed to reallocate memory for buffer\n");
                free(*buffer);
                hdfsCloseFile(handler->fs, file);
                return -1;
            }
            *buffer = new_buffer;
        }

        memcpy(*buffer + *buffer_size, temp_buf, bytes_read);
        *buffer_size += bytes_read;
    }

    hdfsCloseFile(handler->fs, file);
    printf("Successfully read %zu bytes from %s\n", *buffer_size, path);
    return 0;
}

int hdfs_write_file(HdfsHandler *handler, const char *path, const uint8_t *buffer, size_t buffer_size) {
    if (!handler || !handler->fs || !path || !buffer) {
        fprintf(stderr, "Invalid parameters for hdfs_write_file\n");
        return -1;
    }

    printf("Writing file to HDFS: %s\n", path);

    hdfsFile file = hdfsOpenFile(handler->fs, path, O_WRONLY | O_CREAT | O_TRUNC, 0, 0, 0);
    if (!file) {
        fprintf(stderr, "Failed to create file: %s\n", path);
        return -1;
    }

    int bytes_written = hdfsWrite(handler->fs, file, buffer, buffer_size);
    if (bytes_written < 0 || (size_t)bytes_written != buffer_size) {
        fprintf(stderr, "Failed to write to file: %s\n", path);
        hdfsCloseFile(handler->fs, file);
        return -1;
    }

    hdfsCloseFile(handler->fs, file);
    printf("Successfully wrote %zu bytes to %s\n", buffer_size, path);
    return 0;
}

int hdfs_file_exists(HdfsHandler *handler, const char *path) {
    if (!handler || !handler->fs || !path) {
        fprintf(stderr, "Invalid parameters for hdfs_file_exists\n");
        return -1;
    }

    return hdfsExists(handler->fs, path) == 0 ? 1 : 0;
}