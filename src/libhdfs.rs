use libc::{c_char, c_int, c_long, c_short, c_void, size_t};

#[repr(C)]
pub struct hdfsFS {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hdfsFile {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hdfsFileInfo {
    pub mName: *mut c_char,
    pub mKind: c_short,
    pub mSize: c_long,
    pub mReplication: c_short,
    pub mBlockSize: c_long,
    pub mModificationTime: c_long,
    pub mAccessTime: c_long,
    pub mOwner: *mut c_char,
    pub mGroup: *mut c_char,
    pub mPermissions: c_short,
    pub mLast: *mut hdfsFileInfo,
}

#[link(name = "hdfs")]
unsafe extern "C" {
    pub fn hdfsConnectAsUser(
        nn: *const c_char,
        port: c_int,
        user: *const c_char,
    ) -> *mut hdfsFS;
    
    pub fn hdfsConnect(
        nn: *const c_char,
        port: c_int,
    ) -> *mut hdfsFS;
    
    pub fn hdfsDisconnect(fs: *mut hdfsFS) -> c_int;
    
    pub fn hdfsOpenFile(
        fs: *mut hdfsFS,
        path: *const c_char,
        flags: c_int,
        bufferSize: c_int,
        replication: c_short,
        blocksize: c_long,
    ) -> *mut hdfsFile;
    
    pub fn hdfsCloseFile(fs: *mut hdfsFS, file: *mut hdfsFile) -> c_int;
    
    pub fn hdfsRead(
        fs: *mut hdfsFS,
        file: *mut hdfsFile,
        buffer: *mut c_void,
        length: size_t,
    ) -> c_int;
    
    pub fn hdfsWrite(
        fs: *mut hdfsFS,
        file: *mut hdfsFile,
        buffer: *const c_void,
        length: size_t,
    ) -> c_int;
    
    pub fn hdfsExists(fs: *mut hdfsFS, path: *const c_char) -> c_int;
    
    pub fn hdfsGetPathInfo(fs: *mut hdfsFS, path: *const c_char) -> *mut hdfsFileInfo;
    
    pub fn hdfsFreeFileInfo(info: *mut hdfsFileInfo);
}

pub const O_RDONLY: c_int = 0;
pub const O_WRONLY: c_int = 1;
pub const O_RDWR: c_int = 2;
pub const O_CREAT: c_int = 64;
pub const O_EXCL: c_int = 128;
pub const O_TRUNC: c_int = 512;
pub const O_APPEND: c_int = 1024;
pub const O_SYNC: c_int = 2048;