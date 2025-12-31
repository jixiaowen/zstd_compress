use std::io::Write;
use log::{debug, info};
use zstd::stream::Decoder;

pub fn compress_data(data: &[u8], num_threads: usize) -> Result<Vec<u8>, anyhow::Error> {
    info!("Compressing data with {} threads", num_threads);
    debug!("Input data size: {} bytes", data.len());
    
    // Use zstd::bulk::compress which automatically uses multiple threads when zstdmt feature is enabled
    let compressed = zstd::bulk::compress(data, 0)?;
    
    info!("Compression completed: {} bytes -> {} bytes (ratio: {:.2}x)", 
          data.len(), compressed.len(), 
          data.len() as f64 / compressed.len() as f64);
    
    Ok(compressed)
}

pub fn decompress_data(data: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
    info!("Decompressing data");
    debug!("Input data size: {} bytes", data.len());
    
    let decompressed = decode_all(data)?;
    
    info!("Decompression completed: {} bytes -> {} bytes", 
          data.len(), decompressed.len());
    
    Ok(decompressed)
}

fn decode_all(data: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
    let mut decoder = Decoder::new(data)?;
    let mut buffer = Vec::new();
    std::io::copy(&mut decoder, &mut buffer)?;
    Ok(buffer)
}