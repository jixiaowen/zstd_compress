#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/sysinfo.h>

#include "hdfs.h"
#include "compression.h"

void print_usage(const char *program_name) {
    printf("Usage: %s [OPTIONS] <HDFS_FILE_PATH>\n\n", program_name);
    printf("Compress a file on HDFS using zstd compression algorithm.\n\n");
    printf("Options:\n");
    printf("  -t, --threads <NUM>   Number of threads to use for compression (default: number of CPU cores)\n");
    printf("  -h, --help            Show this help message and exit\n");
    printf("  -V, --version         Show version information and exit\n\n");
    printf("Example:\n");
    printf("  %s hdfs:///path/to/file\n", program_name);
}

void print_version() {
    printf("zstd_compress 1.0.0\n");
    printf("Compress files on HDFS using zstd compression algorithm.\n");
    printf("Supports multi-threading and Kerberos authentication.\n");
}

int main(int argc, char *argv[]) {
    int num_threads = 0;
    int opt;
    char *input_path = NULL;

    // Parse command line arguments
    while ((opt = getopt(argc, argv, "t:hV")) != -1) {
        switch (opt) {
            case 't':
                num_threads = atoi(optarg);
                if (num_threads <= 0) {
                    fprintf(stderr, "Invalid number of threads: %s\n", optarg);
                    print_usage(argv[0]);
                    return EXIT_FAILURE;
                }
                break;
            case 'h':
                print_usage(argv[0]);
                return EXIT_SUCCESS;
            case 'V':
                print_version();
                return EXIT_SUCCESS;
            default:
                print_usage(argv[0]);
                return EXIT_FAILURE;
        }
    }

    // Check if input path is provided
    if (optind >= argc) {
        fprintf(stderr, "Error: No input path provided\n\n");
        print_usage(argv[0]);
        return EXIT_FAILURE;
    }

    input_path = argv[optind];

    // Validate input path
    if (strncmp(input_path, "hdfs://", 7) != 0) {
        fprintf(stderr, "Error: Input path must start with 'hdfs://': %s\n", input_path);
        return EXIT_FAILURE;
    }

    // Determine number of threads to use
    if (num_threads == 0) {
        num_threads = get_nprocs();
    }
    printf("Using %d threads for compression\n", num_threads);

    // Create output path with .zst suffix
    char *output_path = (char *)malloc(strlen(input_path) + 5); // +4 for ".zst" +1 for null terminator
    if (!output_path) {
        fprintf(stderr, "Failed to allocate memory for output path\n");
        return EXIT_FAILURE;
    }
    sprintf(output_path, "%s.zst", input_path);
    printf("Output path: %s\n", output_path);

    // Initialize HDFS handler
    HdfsHandler *hdfs_handler = hdfs_handler_new();
    if (!hdfs_handler) {
        fprintf(stderr, "Failed to initialize HDFS handler\n");
        free(output_path);
        return EXIT_FAILURE;
    }

    // Check if input file exists
    if (!hdfs_file_exists(hdfs_handler, input_path)) {
        fprintf(stderr, "Error: Input file does not exist: %s\n", input_path);
        hdfs_handler_free(hdfs_handler);
        free(output_path);
        return EXIT_FAILURE;
    }

    // Read input file
    uint8_t *input_buffer = NULL;
    size_t input_size = 0;
    if (hdfs_read_file(hdfs_handler, input_path, &input_buffer, &input_size) != 0) {
        fprintf(stderr, "Failed to read input file\n");
        hdfs_handler_free(hdfs_handler);
        free(output_path);
        return EXIT_FAILURE;
    }

    // Compress data
    uint8_t *compressed_buffer = NULL;
    size_t compressed_size = 0;
    if (compress_data(input_buffer, input_size, &compressed_buffer, &compressed_size, num_threads) != 0) {
        fprintf(stderr, "Failed to compress data\n");
        free(input_buffer);
        hdfs_handler_free(hdfs_handler);
        free(output_path);
        return EXIT_FAILURE;
    }

    // Write compressed data to output file
    if (hdfs_write_file(hdfs_handler, output_path, compressed_buffer, compressed_size) != 0) {
        fprintf(stderr, "Failed to write compressed file\n");
        free(compressed_buffer);
        free(input_buffer);
        hdfs_handler_free(hdfs_handler);
        free(output_path);
        return EXIT_FAILURE;
    }

    // Clean up resources
    free(compressed_buffer);
    free(input_buffer);
    hdfs_handler_free(hdfs_handler);
    free(output_path);

    printf("Compression completed successfully!\n");
    return EXIT_SUCCESS;
}