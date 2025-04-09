use std::os::fd::RawFd;

pub trait Channel {
    fn as_raw_fd(&self) -> RawFd;
}
