use crate::channel::Channel;
use std::os::fd::{AsRawFd, RawFd};


pub struct SocketChannelHandler{


}

impl SocketChannelHandler {
    pub fn new() -> Self {
        SocketChannelHandler{

        }
    }
}

/// The `ChunkIterator` struct represents an iterator over data chunks.
pub struct ChunkIterator {
    handler: SocketChannelHandler,
    idx: usize,

}

impl ChunkIterator {
    pub fn new(handler: SocketChannelHandler) -> Self {
        ChunkIterator {
            handler, idx: 0,
        }
    }

}

pub struct TcpChannel {
    stream: std::net::TcpStream,
    channel_handler: Arc<SocketChannelHandler>,
    chunk_iterator: Box<ChunkIterator>,
}
impl TcpChannel {
    pub(crate) fn new(stream: std::net::TcpStream, channel_handler: Arc<SocketChannelHandler>) -> Self {

        let chunk_iterator = Box::new(ChunkIterator::new(channel_handler.clone()));
        TcpChannel { stream , channel_handler, chunk_iterator }
    }
}

impl Channel for TcpChannel {
    fn as_raw_fd(&self) -> RawFd {
        self.stream.as_raw_fd()
    }

    fn write(&mut self) {
        // Provide the logic for writing data over the TCP channel.
        println!("Writing to TcpChannel with fd ");
    }

    fn read(&mut self) {
        // Provide the logic for reading data from the TCP channel.
        println!("Reading from TcpChannel with fd ");
    }
}
