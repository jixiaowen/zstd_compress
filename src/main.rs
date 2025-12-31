use std::thread;
use clap::Parser;
use log::{debug, error, info, LevelFilter};
use env_logger::Builder;

mod libhdfs;
mod hdfs;
mod compression;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// HDFS file path to compress
    #[arg(required = true)]
    input_path: String,
    
    /// Number of threads to use for compression (default: number of CPU cores)
    #[arg(short, long, default_value_t = 0)]
    threads: usize,
}

fn main() -> Result<(), anyhow::Error> {
    // Initialize logger
    Builder::new()
        .filter_level(LevelFilter::Info)
        .parse_env("LOG_LEVEL")
        .init();
    
    // Parse command line arguments
    let args = Args::parse();
    debug!("Parsed arguments: {:?}", args);
    
    // Determine number of threads to use
    let num_threads = if args.threads > 0 {
        args.threads
    } else {
        thread::available_parallelism()?.get()
    };
    info!("Using {} threads for compression", num_threads);
    
    // Validate input path
    if !args.input_path.starts_with("hdfs://") {
        error!("Input path must start with 'hdfs://': {}", args.input_path);
        std::process::exit(1);
    }
    
    // Create output path with .zst suffix
    let output_path = format!("{}.zst", args.input_path);
    info!("Output path: {}", output_path);
    
    // Initialize HDFS handler
    let hdfs_handler = match hdfs::HdfsHandler::new() {
        Ok(handler) => handler,
        Err(e) => {
            error!("Failed to initialize HDFS handler: {:?}", e);
            std::process::exit(1);
        }
    };
    
    // Check if input file exists
    if !hdfs_handler.file_exists(&args.input_path)? {
        error!("Input file does not exist: {}", args.input_path);
        std::process::exit(1);
    }
    
    // Read input file
    let data = hdfs_handler.read_file(&args.input_path)?;
    
    // Compress data
    let compressed_data = compression::compress_data(&data, num_threads)?;
    
    // Write compressed data to output file
    hdfs_handler.write_file(&output_path, &compressed_data)?;
    
    info!("Compression completed successfully!");
    info!("Input: {}", args.input_path);
    info!("Output: {}", output_path);
    info!("Original size: {} bytes", data.len());
    info!("Compressed size: {} bytes", compressed_data.len());
    info!("Compression ratio: {:.2}x", data.len() as f64 / compressed_data.len() as f64);
    
    Ok(())
}
