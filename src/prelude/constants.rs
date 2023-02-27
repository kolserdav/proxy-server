#[allow(unused_variables)]
#[cfg(feature = "chunk_1KB")]
pub const CHUNK_SIZE: usize = 1024;

#[allow(unused_variables)]
#[cfg(feature = "chunk_5KB")]
pub const CHUNK_SIZE: usize = 5_120;

#[allow(unused_variables)]
#[cfg(feature = "chunk_10KB")]
pub const CHUNK_SIZE: usize = 10_240;

#[allow(unused_variables)]
#[cfg(feature = "chunk_50KB")]
pub const CHUNK_SIZE: usize = 51_200;

#[allow(unused_variables)]
#[cfg(feature = "chunk_100KB")]
pub const CHUNK_SIZE: usize = 102_400;

#[allow(unused_variables)]
#[cfg(feature = "chunk_500KB")]
pub const CHUNK_SIZE: usize = 512_000;

#[allow(unused_variables)]
#[cfg(feature = "chunk_1MB")]
pub const CHUNK_SIZE: usize = 1_024_000;

#[allow(unused_variables)]
#[cfg(feature = "chunk_5MB")]
pub const CHUNK_SIZE: usize = 5_120_000;

#[allow(unused_variables)]
#[cfg(feature = "chunk_10MB")]
pub const CHUNK_SIZE: usize = 10_240_000;

/// Address of proxy target
pub const TARGET_ADDRESS: &str = "127.0.0.1:3001";

/// Count of working threads
pub const THREADS: usize = 4;

/// Proxy server listen address
pub const PROXY_ADDRESS: &str = "127.0.0.1:3000";
