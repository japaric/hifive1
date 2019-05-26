use core::cmp::{self, Ordering};

use heapless::{binary_heap::Min, ArrayLength, BinaryHeap};
use riscv::csr::mtimecmp;

use crate::Instant;

pub struct TimerQueue<T, N>(pub BinaryHeap<NotReady<T>, N, Min>)
where
    N: ArrayLength<NotReady<T>>,
    T: Copy;

impl<T, N> TimerQueue<T, N>
where
    N: ArrayLength<NotReady<T>>,
    T: Copy,
{
    #[inline]
    pub unsafe fn enqueue_unchecked(&mut self, nr: NotReady<T>) {
        if self
            .0
            .peek()
            .map(|head| nr.instant < head.instant)
            .unwrap_or(true)
        {
            // pend the MTI
            mtimecmp::write(0)
        }

        self.0.push_unchecked(nr);
    }

    #[inline]
    pub fn dequeue(&mut self) -> Option<(T, u8)> {
        if let Some(instant) = self.0.peek().map(|p| p.instant) {
            let now = Instant::now();

            if instant < now {
                // task became ready
                let nr = unsafe { self.0.pop_unchecked() };

                Some((nr.task, nr.index))
            } else {
                // set a new timeout
                mtimecmp::write(instant.inner);

                None
            }
        } else {
            // the queue is empty -- prevent new interrupts
            mtimecmp::write_max();

            None
        }
    }
}

pub struct NotReady<T>
where
    T: Copy,
{
    pub index: u8,
    pub instant: Instant,
    pub task: T,
}

impl<T> Eq for NotReady<T> where T: Copy {}

impl<T> Ord for NotReady<T>
where
    T: Copy,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.instant.cmp(&other.instant)
    }
}

impl<T> PartialEq for NotReady<T>
where
    T: Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.instant == other.instant
    }
}

impl<T> PartialOrd for NotReady<T>
where
    T: Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
