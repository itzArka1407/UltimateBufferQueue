use atomic_waker::AtomicWaker;
use parking_lot::Mutex;
use std::{
    collections::VecDeque,
    sync::atomic::{AtomicBool, Ordering},
    task::{Context, Waker},
};

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

pub struct MultiWaker {
    pub any_parked: AtomicBool, // If any waker in the list of wakers is parked
    pub wakers: Mutex<VecDeque<Waker>>,
}

impl MultiWaker {
    pub fn register(&self, cx: &mut Context<'_>) {
        self.wakers.lock().push_back(cx.waker().clone());
        self.any_parked.store(true, Ordering::Relaxed); // Mark the waker as parked
    }

    pub fn notify(&self) {
        if !self.any_parked.load(Ordering::Relaxed) {
            return; // No waker parked, so can't notify on any end
        }

        let mut wakers = self.wakers.lock();
        if let Some(w) = wakers.pop_front() {
            // No more wakers are waiting, the notifications operation are closed now
            if wakers.is_empty() {
                self.any_parked.store(false, Ordering::Relaxed);
            }
            // Wake the extracted waker
            w.wake();
        }
    }
}
