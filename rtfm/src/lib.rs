#![allow(warnings)]
#![no_std]

use core::{ops, time::Duration};

use riscv::csr::mtime;
pub use riscv_rtfm_macros::app;
use rt as _;
pub use rtfm_core::{Exclusive, Mutex};

#[doc(hidden)]
pub mod export;
mod tq;

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct Instant {
    inner: u64,
}

impl Instant {
    pub fn now() -> Self {
        Instant {
            inner: mtime::read(),
        }
    }

    #[doc(hidden)]
    pub fn artificial(inner: u64) -> Self {
        Instant { inner }
    }

    pub fn checked_duration_since(&self, earlier: Instant) -> Option<Duration> {
        self.inner.checked_sub(earlier.inner).map(|cycles| {
            let secs = cycles >> 15;
            let nanos = ((cycles % (1 << 15)) * 1953125) >> 6;

            Duration::new(secs, nanos as u32)
        })
    }
}

impl ops::Add<Duration> for Instant {
    type Output = Self;

    fn add(self, dur: Duration) -> Self {
        let cycles = (dur.as_secs() * 1953125 + u64::from(dur.subsec_nanos() >> 9)) >> 6;

        Instant {
            inner: self.inner + cycles,
        }
    }
}

impl ops::Sub<Duration> for Instant {
    type Output = Self;

    fn sub(self, dur: Duration) -> Self {
        let cycles = (dur.as_secs() * 1953125 + u64::from(dur.subsec_nanos() >> 9)) >> 6;

        Instant {
            inner: self.inner - cycles,
        }
    }
}
