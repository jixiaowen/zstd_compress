use std::io::Write;
use zstd::stream::{Encoder, encode_all, Decoder};
use zstd::zstdmt::EncStreamBuilder;
use log::{debug, info};

pub fn compress_data(data: &[u8], num_threads: usize) -> Result<Vec<u8>, anyhow::Error> {
    info!("Compressing data with {} threads", num_threads);
    debug!("Input data size: {} bytes", data.len());
    
    let mut encoder = EncStreamBuilder::new()
        .threads(num_threads)
        .build(Vec::new())?;
    
    encoder.write_all(data)?;
    let compressed = encoder.finish()?;
    
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