//! All the wakers to notify about a sent msg from the receiving end is defined here
use atomic_waker::AtomicWaker;
use core::{
    sync::atomic::{AtomicBool, Ordering},
    task::{Context, Waker},
};

#[cfg(not(feature = "std"))]
use spin::Mutex;

#[cfg(feature = "std")]
use parking_lot::Mutex;

use alloc::collections::VecDeque;

/// A single waker for a single receiver
pub struct SingleWaker {
    pub waker: AtomicWaker,
}

impl SingleWaker {
    pub fn register(&self, cx: &mut Context<'_>) {
        self.waker.register(cx.waker());
    }
    pub fn notify(&self) {
        self.waker.wake();
    }
}

/// Multiple wakers to depict multiple receivers in a channel
pub struct MultiWaker {
    pub any_parked: AtomicBool, // If any waker in the list of wakers is parked
    pub wakers: Mutex<VecDeque<Waker>>,
}

impl MultiWaker {
    pub fn register(&self, cx: &mut Context<'_>) {
        let mut wakers = self.wakers.lock();
        // Check if some other waker is registered for the same task(no registration then)
        if !wakers.iter().any(|w| w.will_wake(cx.waker())) {
            wakers.push_back(cx.waker().clone());
            self.any_parked.store(true, Ordering::Relaxed);
        }
    }

    pub fn notify(&self) {
        if !self.any_parked.load(Ordering::Relaxed) {
            return;
        }
        let w = {
            let mut wakers = self.wakers.lock();
            let w = wakers.pop_front();
            if wakers.is_empty() {
                self.any_parked.store(false, Ordering::Relaxed);
            }
            w
        }; // guard dropped here
        if let Some(w) = w {
            w.wake();
        }
    }
}
