use mio::unix::pipe;
use mio::unix::pipe::Receiver;
use mio::unix::pipe::Sender;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Write;
use std::os::fd::AsFd;
use std::os::fd::BorrowedFd;

#[cfg(test)]
mod tests;

pub struct Eventfd {
    sender: Sender,
    receiver: Receiver,
}

impl Eventfd {
    pub fn new() -> io::Result<Self> {
        let (sender, receiver) = pipe::new()?;
        Ok(Self { sender, receiver })
    }

    pub fn bump(&self) -> io::Result<()> {
        loop {
            if let Err(e) = (&self.sender).write(&[0]) {
                match e.kind() {
                    ErrorKind::WouldBlock => {}
                    ErrorKind::Interrupted => continue,
                    _ => return Err(e),
                }
            }
            return Ok(());
        }
    }

    pub fn clear(&self) -> io::Result<()> {
        let mut buf = [0; 128];
        loop {
            match (&self.receiver).read(&mut buf) {
                Ok(0) => return Err(ErrorKind::NotConnected.into()),
                Ok(_) => {}
                Err(e) if e.kind() == ErrorKind::WouldBlock => return Ok(()),
                Err(e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
    }
}

impl AsFd for Eventfd {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.receiver.as_fd()
    }
}
