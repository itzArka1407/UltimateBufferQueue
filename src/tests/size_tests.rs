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
    dbg!(core::mem::size_of::<AtomicWaker>());
    dbg!(core::mem::size_of::<Notify>());
    dbg!(core::mem::size_of::<MultiWaker>());
    #[cfg(feature = "std")]
    dbg!(core::mem::size_of::<parking_lot::Mutex<u8>>());
    dbg!(core::mem::size_of::<SpscChannel<(), 0>>());

    dbg!(core::mem::size_of::<BufferQueue<u8, SPSC, 100>>());
    dbg!(core::mem::size_of::<BufferQueue<u8, MPMC, 100>>());

    dbg!(core::mem::size_of::<SpscChannel<u8, 100>>());
    dbg!(core::mem::size_of::<MpmcChannel<u8, 100>>());
}
