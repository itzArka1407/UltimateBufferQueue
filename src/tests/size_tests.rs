use atomic_waker::AtomicWaker;

#[test]
fn check_size() {
    dbg!(std::mem::size_of::<AtomicWaker>());
}
