use atomic_waker::AtomicWaker;
use tokio::sync::Notify;

use crate::models::{
    BufferQueue,
    buffer_modes::{MPMC, SPSC},
    channels::{MpmcChannel, SpscChannel},
    senders_receivers::{MpmcReceiver, MpmcSender, SpscReceiver, SpscSender},
    wakers::MultiWaker,
};

#[test]
fn check_size() {
    dbg!(std::mem::size_of::<AtomicWaker>());
    dbg!(std::mem::size_of::<Notify>());
    dbg!(std::mem::size_of::<MultiWaker>());
    dbg!(std::mem::size_of::<parking_lot::Mutex<u8>>());
    dbg!(std::mem::size_of::<SpscChannel<(), 0>>());

    dbg!(std::mem::size_of::<BufferQueue<u8, SPSC, 100>>());
    dbg!(std::mem::size_of::<BufferQueue<u8, MPMC, 100>>());

    dbg!(std::mem::size_of::<SpscChannel<u8, 100>>());
    dbg!(std::mem::size_of::<MpmcChannel<u8, 100>>());
}
