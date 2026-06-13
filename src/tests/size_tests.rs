use atomic_waker::AtomicWaker;
use tokio::sync::Notify;

#[test]
fn check_size() {
    dbg!(std::mem::size_of::<AtomicWaker>());
    dbg!(std::mem::size_of::<Notify>());
}
