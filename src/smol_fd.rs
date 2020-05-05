use std::ffi::c_void;
use std::io::{Error, Read, Result, Write};
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};

use num_traits::{PrimInt, Signed, Zero};

pub fn libc_check_error<T: Signed + PrimInt + Zero>(val: T) -> Result<T> {
    if val < T::zero() {
        Err(Error::last_os_error())
    } else {
        Ok(val)
    }
}

#[derive(Debug)]
pub struct SmolFd {
    pub raw: RawFd,
}

impl SmolFd {
    pub fn new(fd: RawFd) -> SmolFd {
        SmolFd { raw: fd }
    }

    pub fn close(&mut self) -> Result<()> {
        libc_check_error(unsafe { libc::close(self.raw) })?;

        Ok(())
    }
}

impl FromRawFd for SmolFd {
    unsafe fn from_raw_fd(fd: RawFd) -> SmolFd {
        SmolFd { raw: fd }
    }
}

impl AsRawFd for SmolFd {
    fn as_raw_fd(&self) -> RawFd {
        self.raw
    }
}

impl Write for SmolFd {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let written = libc_check_error(unsafe {
            libc::write(self.raw, buf.as_ptr() as *const c_void, buf.len())
        })?;

        Ok(written as usize)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Read for SmolFd {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let read_len = libc_check_error(unsafe {
            libc::read(self.raw, buf.as_mut_ptr() as *mut c_void, buf.len())
        })?;

        Ok(read_len as usize)
    }
}
