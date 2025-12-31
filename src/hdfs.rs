use std::ffi::CString;
use std::ptr;
use libc::c_void;
use log::{debug, info};

use crate::libhdfs::*;

pub struct HdfsHandler {
    fs: *mut hdfsFS,
}

impl Drop for HdfsHandler {
    fn drop(&mut self) {
        if !self.fs.is_null() {
            unsafe {
                hdfsDisconnect(self.fs);
            }
            debug!("HDFS connection disconnected");
        }
    }
}

impl HdfsHandler {
    pub fn new() -> Result<Self, anyhow::Error> {
        // For Kerberos authentication, we don't need to specify a user
        // The authentication is handled by the Kerberos ticket cache
        unsafe {
            let fs = hdfsConnect(ptr::null(), 0);
            if fs.is_null() {
                return Err(anyhow::anyhow!("Failed to connect to HDFS"));
            }
            
            info!("Successfully connected to HDFS");
            
            Ok(Self {
                fs,
            })
        }
    }
    
    pub fn connect_as_user(user: &str) -> Result<Self, anyhow::Error> {
        let user_cstr = CString::new(user)?;
        
        unsafe {
            let fs = hdfsConnectAsUser(ptr::null(), 0, user_cstr.as_ptr());
            if fs.is_null() {
                return Err(anyhow::anyhow!("Failed to connect to HDFS as user: {}", user));
            }
            
            info!("Successfully connected to HDFS as user: {}", user);
            
            Ok(Self {
                fs,
            })
        }
    }
    
    pub fn read_file(&self, path: &str) -> Result<Vec<u8>, anyhow::Error> {
        let path_cstr = CString::new(path)?;
        info!("Reading file from HDFS: {}", path);
        
        unsafe {
            let file = hdfsOpenFile(
                self.fs,
                path_cstr.as_ptr(),
                O_RDONLY,
                0,
                0,
                0,
            );
            
            if file.is_null() {
                return Err(anyhow::anyhow!("Failed to open file: {}", path));
            }
            
            let mut buffer = Vec::new();
            let mut temp_buf = [0u8; 8192];
            
            loop {
                let bytes_read = hdfsRead(
                    self.fs,
                    file,
                    temp_buf.as_mut_ptr() as *mut c_void,
                    temp_buf.len(),
                );
                
                if bytes_read <= 0 {
                    break;
                }
                
                buffer.extend_from_slice(&temp_buf[..bytes_read as usize]);
            }
            
            hdfsCloseFile(self.fs, file);
            info!("Successfully read {} bytes from {}", buffer.len(), path);
            Ok(buffer)
        }
    }
    
    pub fn write_file(&self, path: &str, data: &[u8]) -> Result<(), anyhow::Error> {
        let path_cstr = CString::new(path)?;
        info!("Writing file to HDFS: {}", path);
        
        unsafe {
            let file = hdfsOpenFile(
                self.fs,
                path_cstr.as_ptr(),
                O_WRONLY | O_CREAT | O_TRUNC,
                0,
                0,
                0,
            );
            
            if file.is_null() {
                return Err(anyhow::anyhow!("Failed to create file: {}", path));
            }
            
            let bytes_written = hdfsWrite(
                self.fs,
                file,
                data.as_ptr() as *const c_void,
                data.len(),
            );
            
            if bytes_written < 0 {
                hdfsCloseFile(self.fs, file);
                return Err(anyhow::anyhow!("Failed to write to file: {}", path));
            }
            
            hdfsCloseFile(self.fs, file);
            info!("Successfully wrote {} bytes to {}", data.len(), path);
            Ok(())
        }
    }
    
    pub fn file_exists(&self, path: &str) -> Result<bool, anyhow::Error> {
        let path_cstr = CString::new(path)?;
        
        unsafe {
            let result = hdfsExists(self.fs, path_cstr.as_ptr());
            match result {
                0 => Ok(true),
                -1 => Ok(false),
                _ => Err(anyhow::anyhow!("Failed to check if file exists: {}", path)),
            }
        }
    }
}