use std::path::Path;
use std::sync::Arc;
use hdfs::client::Client;
use hdfs::error::Error;
use log::{debug, info};

pub struct HdfsHandler {
    client: Arc<Client>,
}

impl HdfsHandler {
    pub fn new() -> Result<Self, Error> {
        let config_path = Path::new("/etc/hadoop");
        debug!("Reading HDFS configuration from {:?}", config_path);
        
        let client = Client::from_conf_dir(config_path)?;
        info!("Successfully connected to HDFS");
        
        Ok(Self {
            client: Arc::new(client),
        })
    }
    
    pub async fn read_file(&self, path: &str) -> Result<Vec<u8>, Error> {
        info!("Reading file from HDFS: {}", path);
        let mut file = self.client.open_file(path).await?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;
        info!("Successfully read {} bytes from {}", buffer.len(), path);
        Ok(buffer)
    }
    
    pub async fn write_file(&self, path: &str, data: &[u8]) -> Result<(), Error> {
        info!("Writing file to HDFS: {}", path);
        let mut file = self.client.create_file(path).await?;
        file.write_all(data).await?;
        info!("Successfully wrote {} bytes to {}", data.len(), path);
        Ok(())
    }
    
    pub async fn file_exists(&self, path: &str) -> Result<bool, Error> {
        self.client.exists(path).await
    }
}