// The notifier is defined here. The traits associated are defined. Notifer types are different
// based on use cases, so common methods are defined by the traits

use std::task::Context;

use atomic_waker::AtomicWaker;

pub trait Notifier {
    fn new() -> Self; // Create a new notifier
    fn notify(&self); // Notify once
    fn register(&self, cx: &mut Context<'_>);
}

// AtomicWaker is used for all buffer modes apart from MPMC
impl Notifier for AtomicWaker {
    fn new() -> Self {
        Self::new()
    }

    fn notify(&self) {
        self.wake();
    }

    fn register(&self, cx: &mut Context<'_>) {
        self.register(cx.waker());
    }
}
