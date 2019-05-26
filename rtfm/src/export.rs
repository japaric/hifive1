use core::cell::Cell;

pub use crate::tq::{NotReady, TimerQueue};
use heapless::spsc::SingleCore;
pub use heapless::{consts, i::Queue as iQueue, spsc::Queue};
pub use heapless::{i::BinaryHeap as iBinaryHeap, BinaryHeap};
pub use hifive1::{msip, plic, Interrupt};
pub use riscv::{
    asm,
    csr::{mie, mstatus, mtimecmp, mtimel},
};

pub type SCFQ<N> = Queue<u8, N, u8, SingleCore>;
pub type SCRQ<T, N> = Queue<(T, u8), N, u8, SingleCore>;

pub struct Priority {
    inner: Cell<u8>,
}

impl Priority {
    #[inline(always)]
    pub unsafe fn new(value: u8) -> Self {
        Priority {
            inner: Cell::new(value),
        }
    }

    // these two methods are used by `lock` (see below) but can't be used from the RTFM application
    #[inline(always)]
    fn set(&self, value: u8) {
        self.inner.set(value)
    }

    #[inline(always)]
    fn get(&self) -> u8 {
        self.inner.get()
    }
}

pub unsafe fn lock<T, R>(
    msi: bool, // whether MSI is used at all
    mti: bool, // whether MTI is used at all
    ptr: *mut T,
    priority: &Priority,
    ceiling: u8,
    f: impl FnOnce(&mut T) -> R,
) -> R {
    let current = priority.get();

    if current < ceiling {
        if !msi && !mti {
            priority.set(ceiling);

            plic::set_threshold(ceiling);

            let r = f(&mut *ptr);

            plic::set_threshold(current);

            priority.set(current);

            r
        } else {
            let mask = mie::MSIE | if mti { mie::MTIE } else { 0 };

            if ceiling == 1 {
                priority.set(ceiling);

                mie::clear(mask);

                let r = f(&mut *ptr);

                mie::set(mask);

                priority.set(current);

                r
            } else if current == 0 {
                priority.set(ceiling);

                // ceiling is changed in two steps
                mie::clear(mask); // ceiling = 1
                plic::set_threshold(ceiling - 1);

                let r = f(&mut *ptr);

                plic::set_threshold(0); // ceiling = 1
                mie::set(mask);

                priority.set(current);

                r
            } else {
                priority.set(ceiling);

                plic::set_threshold(ceiling - 1);

                let r = f(&mut *ptr);

                plic::set_threshold(current - 1);

                priority.set(current);

                r
            }
        }
    } else {
        f(&mut *ptr)
    }
}

#[inline(always)]
pub fn assert_send<T>()
where
    T: Send,
{
}

#[inline(always)]
pub fn assert_sync<T>()
where
    T: Sync,
{
}
