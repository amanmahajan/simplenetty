use crate::channel::Channel;
use crate::channel::*;
use crate::tcp_channel::TcpChannel;
use std::collections::HashMap;
use std::net::TcpListener;
use std::os::fd::RawFd;

trait EventLoop {
    fn register_channel(&mut self, channel: Box<dyn Channel>);
    fn run(&mut self);
    fn stop(&mut self);
}

struct EpollEventLoop {
    epoll_fd: i32,
    acceptor_fd: i32,
    listener: TcpListener,
    channels: HashMap<RawFd, Box<dyn Channel>>,
    running: bool,
}

impl EventLoop for EpollEventLoop {
    fn register_channel(&mut self, channel: Box<dyn Channel>) {
        // Register the channel with the epoll instance
        self.channels.insert(channel.as_raw_fd(), channel);
    }

    fn run(&mut self) {
        // Run the event loop
        loop {
            let mut events = vec![epoll::Event::new(epoll::Events::empty(), 0); 1024];

            let num_events = epoll::wait(self.epoll_fd, -1, &mut events).unwrap();

            for i in 0..num_events {
                let event = events[i];
                let fd = event.data as RawFd;

                if fd == self.acceptor_fd {
                    // Handle acceptor events
                    println!("Acceptor event: {:?}", event);
                    // Accept new connections
                    if let Ok((mut stream, addr)) = self.listener.accept() {
                        println!("Accepted connection from: {:?}", addr);

                        let channel = TcpChannel::new(stream);
                        self.register_channel(Box::new(channel));
                    } else {
                        println!("Failed to accept connection");
                    }
                    continue;
                }

                match event.events {
                    e if e & epoll::Events::EPOLLIN.bits() != 0 => {
                        // Handle input event
                        println!("Input event on channel: {:?}", event.data);
                    }

                    e if e & epoll::Events::EPOLLOUT.bits() != 0 => {
                        // Handle output event
                        println!("Output event on channel: {:?}", event.data);
                    }

                    e if e & epoll::Events::EPOLLERR.bits() != 0 => {
                        // Handle error event
                        println!("Error event on channel: {:?}", event.data);
                    }

                    e if e & epoll::Events::EPOLLHUP.bits() != 0 => {
                        // Handle hangup event
                        println!("Hangup event on channel: {:?}", event.data);
                    }

                    _ => {
                        // Handle unknown event
                        println!("Unknown event on channel: {:?}", event.data);
                    }
                }
            }

            if !self.running {
                break;
            }
        }
    }

    fn stop(&mut self) {
        // Stop the event loop
        self.running = false;
    }
}
