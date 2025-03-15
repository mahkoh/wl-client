use std::{
    io,
    os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, OwnedFd},
};

#[cfg(test)]
mod tests;

pub struct Eventfd {
    fd: OwnedFd,
}

impl Eventfd {
    pub fn new() -> io::Result<Self> {
        // SAFETY: eventfd is a safe function when called with these flags.
        let fd = unsafe { libc::eventfd(0, libc::EFD_CLOEXEC | libc::EFD_NONBLOCK) };
        if fd == -1 {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: eventfd returns a valid file descriptor or -1.
        let fd = unsafe { OwnedFd::from_raw_fd(fd) };
        Ok(Self { fd })
    }

    pub fn bump(&self) -> io::Result<()> {
        let buf = 1u64.to_ne_bytes();
        // SAFETY: buf is valid for buf.len() bytes.
        let res = unsafe { libc::write(self.fd.as_raw_fd(), buf.as_ptr().cast(), buf.len()) };
        if res == -1 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    pub fn clear(&self) -> io::Result<()> {
        let mut buf = 0u64.to_ne_bytes();
        // SAFETY: buf is valid for buf.len() bytes.
        let res = unsafe { libc::read(self.fd.as_raw_fd(), buf.as_mut_ptr().cast(), buf.len()) };
        if res == -1 {
            let err = io::Error::last_os_error();
            if err.kind() == io::ErrorKind::WouldBlock {
                return Ok(());
            }
            return Err(err);
        }
        Ok(())
    }
}

impl AsFd for Eventfd {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.fd.as_fd()
    }
}
