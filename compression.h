#ifndef COMPRESSION_H
#define COMPRESSION_H

#include <stdint.h>

int compress_data(const uint8_t *input, size_t input_size, uint8_t **output, size_t *output_size, int num_threads);
int decompress_data(const uint8_t *input, size_t input_size, uint8_t **output, size_t *output_size);

#endif /* COMPRESSION_H */