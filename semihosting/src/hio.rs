use ufmt::{derive::uDebug, uWrite};

use crate::nr;

/// Unbuffered host standard output
#[derive(uDebug)]
pub struct HStdout {
    fd: usize,
}

impl HStdout {
    pub fn write(&self, buffer: &[u8]) -> Result<(), ()> {
        if unsafe { syscall!(WRITE, self.fd, buffer.as_ptr(), buffer.len()) } == 0 {
            Ok(())
        } else {
            Err(())
        }
    }
}

impl uWrite for HStdout {
    type Error = ();

    fn write_str(&mut self, s: &str) -> Result<(), ()> {
        self.write(s.as_bytes())
    }
}

impl uWrite for &'_ HStdout {
    type Error = ();

    fn write_str(&mut self, s: &str) -> Result<(), ()> {
        self.write(s.as_bytes())
    }
}

pub fn hstdout() -> Result<HStdout, ()> {
    open(":tt\0", nr::open::W_TRUNC).map(|fd| HStdout { fd })
}

/// Unbuffered host standard error
#[derive(uDebug)]
pub struct HStderr {
    fd: usize,
}

impl HStderr {
    pub fn write(&self, buffer: &[u8]) -> Result<(), ()> {
        if unsafe { syscall!(WRITE, self.fd, buffer.as_ptr(), buffer.len()) } == 0 {
            Ok(())
        } else {
            Err(())
        }
    }
}

impl uWrite for HStderr {
    type Error = ();

    fn write_str(&mut self, s: &str) -> Result<(), ()> {
        self.write(s.as_bytes())
    }
}

impl uWrite for &'_ HStderr {
    type Error = ();

    fn write_str(&mut self, s: &str) -> Result<(), ()> {
        self.write(s.as_bytes())
    }
}

pub fn hstderr() -> Result<HStderr, ()> {
    open(":tt\0", nr::open::W_APPEND).map(|fd| HStderr { fd })
}

fn open(name: &str, mode: usize) -> Result<usize, ()> {
    let name = name.as_bytes();
    match unsafe { syscall!(OPEN, name.as_ptr(), mode, name.len() - 1) } as isize {
        -1 => Err(()),
        fd => Ok(fd as usize),
    }
}
