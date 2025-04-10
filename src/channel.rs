use std::os::fd::RawFd;


/*
A "channel" is an abstraction—a common interface—that represents a unit of I/O
(like a network socket or connection) that can be registered with an event loop for asynchronous event handling.
*/
pub trait Channel {
    fn as_raw_fd(&self) -> RawFd;
    fn write(&mut self);
    fn read(&mut self);
}
