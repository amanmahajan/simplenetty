use crate::channel::Channel;
use std::os::fd::{AsRawFd, RawFd};
use std::ffi::CStr;
use std::ptr::null;
use std::sync::Arc;

pub struct SocketChannelHandler {}

impl SocketChannelHandler {
    pub fn new() -> Self {
        SocketChannelHandler {}
    }
}


pub struct Chunk {
    pub bytes: *const u8,
    pub size: usize,
}

impl Chunk {
    // Returns true if there is no data in the chunk.
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    // Change the signature to borrow the bytes rather than moving them.
    pub fn bytes(&self) -> &u8 {
        &self.bytes
    }
}


/// The `ChunkIterator` struct represents an iterator over data chunks.
pub struct ChunkIterator {
    handler: Arc<SocketChannelHandler>,
    idx: usize,
}

impl ChunkIterator {
    pub fn new(handler:Arc<SocketChannelHandler>) -> Self {
        ChunkIterator { handler, idx: 0 }
    }
    pub fn next(&mut self) -> Chunk {
        if self.idx == 0 {
            Chunk {
                // Sample data. It wil be changed.
                bytes: "sample bytes".as_ptr(),
                size: "sample byte".len(),
            }
        } else {
            Chunk {
                bytes: null(),
                size: 0,
            }
        }
    }

    /// Update internal state after bytes have been sent.
    pub fn commit(&mut self, _bytes_sent: usize) {
        // For demonstration, update the index.
        // A real implementation might adjust a buffer slice or similar.
        self.idx += 1;
    }
}

pub struct TcpChannel {
    stream: std::net::TcpStream,
    channel_handler: Arc<SocketChannelHandler>,
    chunk_iterator: Box<ChunkIterator>,
}
impl TcpChannel {
    pub(crate) fn new(
        stream: std::net::TcpStream,
        channel_handler: Arc<SocketChannelHandler>,
    ) -> Self {
        let chunk_iterator = Box::new(ChunkIterator::new(channel_handler.clone()));
        TcpChannel {
            stream,
            channel_handler,
            chunk_iterator,
        }
    }
}

impl Channel for TcpChannel {
    fn as_raw_fd(&self) -> RawFd {
        self.stream.as_raw_fd()
    }

    fn write(&mut self) {
        // Get the raw file descriptor from the TcpStream.
        let fd = self.stream.as_raw_fd();
        loop {
            // Get the next data chunk.
            let chunk = self.chunk_iterator.next();
            if chunk.is_empty() {
                // Nothing to write.
                break;
            }

            // Call the low-level send system call.
            let bytes_written = unsafe {
                libc::send(fd, chunk.bytes().as_ptr() as *const libc::c_void, chunk.size, 0);
            };

            if bytes_written <= 0 {
                // Fetch the errno from libc.
                let err = std::io::Error::last_os_error().raw_os_error().unwrap();
                match err {
                    libc::EWOULDBLOCK => {
                        println!("EWOULDBLOCK: send buffer full, retry later");
                        break;
                    }
                    libc::EINTR => {
                        println!("EINTR: write interrupted, retrying");
                        continue; // Retry sending.
                    }
                    libc::ECONNRESET => {
                        println!("ECONNRESET: connection reset by peer");
                        unsafe { libc::close(fd) };
                        break;
                    }
                    libc::EPIPE => {
                        println!("EPIPE: broken pipe, connection closed");
                        unsafe { libc::close(fd) };
                        break;
                    }
                    libc::ENOBUFS => {
                        println!("ENOBUFS: no buffer space available, retrying later");
                        break;
                    }
                    _ => {
                        // Use strerror to obtain the error string.
                        /// The whole snippet takes an operating system error code (err),
                        /// gets a human-readable error message for it from the C library
                        /// , and converts it into a Rust String.
                        let err_str = unsafe {
                            CStr::from_ptr(libc::strerror(err))
                                .to_string_lossy()
                                .into_owned()
                        };
                        println!("write error: {}", err_str);
                        unsafe { libc::close(fd) };
                        return;
                    }
                }
            } else {
                // On success, commit the number of bytes written.
                let bytes_written_usize: usize = bytes_written
                    .try_into()
                    .expect("bytes_written should be non-negative");

                self.chunk_iterator.commit(bytes_written_usize);
            }
        }
    }

    fn read(&mut self) {
        // Provide the logic for reading data from the TCP channel.
        println!("Reading from TcpChannel with fd ");
    }
}
