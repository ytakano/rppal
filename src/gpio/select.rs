use super::{
    epoll::{Epoll, EventFd},
    Error, Result,
};
use futures::task::{waker_ref, ArcWake};
use libc::{epoll_event, EPOLLIN, EPOLLPRI};
use std::{
    collections::{BTreeMap, VecDeque},
    sync::mpsc::{channel, Receiver, Sender},
    sync::{Arc, Mutex},
    task::Waker,
    thread::{self, JoinHandle},
};

const ID_EVENTFD: u64 = u64::MAX;

#[derive(Debug)]
enum Msg {
    Add { fd: i32, pin: u8, waker: Waker },
    Remove { fd: i32 },
    Quit,
}

#[derive(Debug)]
pub struct Select {
    tx: Sender<Msg>,
    eventfd: EventFd,
    handler: Option<JoinHandle<()>>, // Waker thread
}

impl Select {
    pub fn new(capacity: usize) -> Result<Select> {
        let eventfd = EventFd::new()?;
        let epoll = Epoll::new()?;
        let (tx, rx) = channel();

        epoll.add(eventfd.fd(), ID_EVENTFD, EPOLLIN | EPOLLPRI)?;

        Ok(Select {
            tx,
            eventfd,
            handler: Some(thread::spawn(move || worker(epoll, rx, capacity))),
        })
    }

    fn send(&mut self, msg: Msg) -> bool {
        if let Ok(_) = self.tx.send(msg) {
            self.eventfd.notify().unwrap();
            true
        } else {
            false // thread should be exited
        }
    }

    pub fn stop(self) {}
}

impl Drop for Select {
    fn drop(&mut self) {
        if !thread::panicking() {
            if self.send(Msg::Quit) {
                if let Some(handler) = self.handler.take() {
                    let _ = handler.join();
                }
            }
        }
    }
}

fn worker(epoll: Epoll, rx: Receiver<Msg>, capacity: usize) {
    let mut wakers = BTreeMap::new(); // pin to waker
    let mut events = vec![epoll_event { events: 0, u64: 0 }; capacity];

    while let Ok(n) = epoll.wait(&mut events, None) {
        for i in 0..n {
            match events[i].u64 {
                ID_EVENTFD => {
                    while let Ok(msg) = rx.try_recv() {
                        match msg {
                            Msg::Quit => return,
                            Msg::Add { fd, pin, waker } => {
                                wakers.insert(pin, waker);
                                epoll.add(fd, pin as u64, EPOLLIN | EPOLLPRI).unwrap();
                            }
                            Msg::Remove { fd } => {
                                epoll.delete(fd).unwrap();
                            }
                        }
                    }
                }
                pin => {
                    if let Some(waker) = wakers.get(&(pin as u8)) {
                        waker.clone().wake();
                    }
                }
            }
        }
    }
}
