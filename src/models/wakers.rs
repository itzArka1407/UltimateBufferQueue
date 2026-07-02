use atomic_waker::AtomicWaker;
use parking_lot::Mutex;
use std::{
    collections::VecDeque,
    task::{Context, Waker},
};

pub struct SingleWaker {
    waker: AtomicWaker,
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
    wakers: Mutex<VecDeque<Waker>>,
}

impl MultiWaker {
    pub fn register(&self, cx: &mut Context<'_>) {
        let mut wakers = self.wakers.lock();
        // avoid duplicate registration from the same task
        if !wakers.iter().any(|w| w.will_wake(cx.waker())) {
            wakers.push_back(cx.waker().clone());
        }
    }
}
