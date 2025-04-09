use crate::channel::Channel;
use std::os::fd::{AsRawFd, RawFd};

pub struct TcpChannel {
    stream: std::net::TcpStream,
}
impl TcpChannel {
    pub(crate) fn new(stream: std::net::TcpStream) -> Self {
        TcpChannel { stream }
    }
}

impl Channel for TcpChannel {
    fn as_raw_fd(&self) -> RawFd {
        self.stream.as_raw_fd()
    }
}
