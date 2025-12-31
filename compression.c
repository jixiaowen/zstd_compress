#include <stdio.h>
#include <stdlib.h>
#include <zstd.h>
#include <zstdmt.h>
#include "compression.h"

int compress_data(const uint8_t *input, size_t input_size, uint8_t **output, size_t *output_size, int num_threads) {
    if (!input || !output || !output_size) {
        fprintf(stderr, "Invalid parameters for compress_data\n");
        return -1;
    }

    printf("Compressing data with %d threads\n", num_threads);
    printf("Input data size: %zu bytes\n", input_size);

    // Create zstd multi-threaded compression context
    ZSTDMT_CCtx *cctx = ZSTDMT_createCCtx();
    if (!cctx) {
        fprintf(stderr, "Failed to create ZSTDMT_CCtx\n");
        return -1;
    }

    // Set number of threads
    if (ZSTDMT_CCtx_setParameter(cctx, ZSTD_c_nbWorkers, num_threads) != 0) {
        fprintf(stderr, "Failed to set number of threads\n");
        ZSTDMT_freeCCtx(cctx);
        return -1;
    }

    // Calculate maximum output size
    size_t max_output_size = ZSTD_compressBound(input_size);
    *output = (uint8_t *)malloc(max_output_size);
    if (!*output) {
        fprintf(stderr, "Failed to allocate memory for output buffer\n");
        ZSTDMT_freeCCtx(cctx);
        return -1;
    }

    // Compress data
    *output_size = ZSTDMT_compressCCtx(cctx, *output, max_output_size, input, input_size, ZSTD_CLEVEL_DEFAULT);
    if (ZSTD_isError(*output_size)) {
        fprintf(stderr, "Failed to compress data: %s\n", ZSTD_getErrorName(*output_size));
        free(*output);
        ZSTDMT_freeCCtx(cctx);
        return -1;
    }

    ZSTDMT_freeCCtx(cctx);

    printf("Compression completed: %zu bytes -> %zu bytes (ratio: %.2fx)\n", 
           input_size, *output_size, (double)input_size / (double)*output_size);

    return 0;
}

int decompress_data(const uint8_t *input, size_t input_size, uint8_t **output, size_t *output_size) {
    if (!input || !output || !output_size) {
        fprintf(stderr, "Invalid parameters for decompress_data\n");
        return -1;
    }

    printf("Decompressing data\n");
    printf("Input data size: %zu bytes\n", input_size);

    // Get decompressed size
    size_t decompressed_size = ZSTD_getFrameContentSize(input, input_size);
    if (decompressed_size == ZSTD_CONTENTSIZE_ERROR) {
        fprintf(stderr, "Invalid zstd frame\n");
        return -1;
    }
    if (decompressed_size == ZSTD_CONTENTSIZE_UNKNOWN) {
        fprintf(stderr, "Unknown decompressed size\n");
        return -1;
    }

    // Allocate buffer for decompressed data
    *output = (uint8_t *)malloc(decompressed_size);
    if (!*output) {
        fprintf(stderr, "Failed to allocate memory for decompressed data\n");
        return -1;
    }

    // Decompress data
    *output_size = ZSTD_decompress(*output, decompressed_size, input, input_size);
    if (ZSTD_isError(*output_size)) {
        fprintf(stderr, "Failed to decompress data: %s\n", ZSTD_getErrorName(*output_size));
        free(*output);
        return -1;
    }

    printf("Decompression completed: %zu bytes -> %zu bytes\n", input_size, *output_size);

    return 0;
}