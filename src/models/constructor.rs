//! The constructors of all the different channels are defined here. The user directly constructs
//! these channels. These are the first things to be created to be used by the user
use alloc::collections::VecDeque;
use core::sync::atomic::AtomicBool;

use crate::{
    helper_functions::{determine_flag_size, determine_marker_type_index},
    helper_traits::MarkerData,
    models::{
        BufferQueue,
        channels::{MpmcChannel, MpscChannel, SpmcChannel, SpscChannel},
        helper_models::MarkerTypeDecider,
        senders_receivers::{
            MpmcReceiver, MpmcSender, MpscReceiver, MpscSender, SpmcReceiver, SpmcSender,
            SpscReceiver, SpscSender,
        },
        wakers::{MultiWaker, SingleWaker},
    },
};
use atomic_waker::AtomicWaker;
#[cfg(feature = "std")]
use parking_lot::Mutex;
#[cfg(not(feature = "std"))]
use spin::Mutex;
use triomphe::Arc;

/// Creates a bounded SPSC channel with `N` slots (capacity `N - 1`).
///
/// # Example
/// ```
/// let (tx, rx) = buffer_queue::spsc_channel::<u8, 4>();
/// tx.try_send(1).unwrap();
/// assert_eq!(rx.try_recv(), Some(1));
/// ```
pub fn spsc_channel<T, const N: usize>() -> (SpscSender<T, N>, SpscReceiver<T, N>)
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    let channel = Arc::new(SpscChannel {
        buf: BufferQueue::new(),
        recv_waker: SingleWaker {
            waker: AtomicWaker::new(),
        },
        send_waker: SingleWaker {
            waker: AtomicWaker::new(),
        },
    });
    (
        SpscSender {
            channel: channel.clone(),
        },
        SpscReceiver { channel },
    )
}

/// Creates a bounded SPMC channel with `N` slots (capacity `N - 1`). The receiver is `Clone`.
///
/// # Example
/// ```
/// let (tx, rx) = buffer_queue::spmc_channel::<u8, 4>();
/// tx.try_send(1).unwrap();
/// assert_eq!(rx.try_recv(), Some(1));
/// ```
pub fn spmc_channel<T, const N: usize>() -> (SpmcSender<T, N>, SpmcReceiver<T, N>)
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    let channel = Arc::new(SpmcChannel {
        buf: BufferQueue::new(),
        recv_waker: MultiWaker {
            any_parked: AtomicBool::new(false),
            wakers: Mutex::new(VecDeque::new()),
        },
        send_waker: SingleWaker {
            waker: AtomicWaker::new(),
        },
    });
    (
        SpmcSender {
            channel: channel.clone(),
        },
        SpmcReceiver { channel },
    )
}

/// Creates a bounded MPSC channel with `N` slots (capacity `N - 1`). The sender is `Clone`.
///
/// # Example
/// ```
/// let (tx, rx) = buffer_queue::mpsc_channel::<u8, 4>();
/// tx.try_send(1).unwrap();
/// assert_eq!(rx.try_recv(), Some(1));
/// ```
pub fn mpsc_channel<T, const N: usize>() -> (MpscSender<T, N>, MpscReceiver<T, N>)
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    let channel = Arc::new(MpscChannel {
        buf: BufferQueue::new(),
        recv_waker: SingleWaker {
            waker: AtomicWaker::new(),
        },
        send_waker: MultiWaker {
            any_parked: AtomicBool::new(false),
            wakers: Mutex::new(VecDeque::new()),
        },
    });
    (
        MpscSender {
            channel: channel.clone(),
        },
        MpscReceiver { channel },
    )
}

/// Creates a bounded MPMC channel with `N` slots (capacity `N - 1`). Both halves are `Clone`.
///
/// # Example
/// ```
/// let (tx, rx) = buffer_queue::mpmc_channel::<u8, 4>();
/// tx.try_send(1).unwrap();
/// assert_eq!(rx.try_recv(), Some(1));
/// ```
pub fn mpmc_channel<T, const N: usize>() -> (MpmcSender<T, N>, MpmcReceiver<T, N>)
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    let channel = Arc::new(MpmcChannel {
        buf: BufferQueue::new(),
        recv_waker: MultiWaker {
            any_parked: AtomicBool::new(false),
            wakers: Mutex::new(VecDeque::new()),
        },
        send_waker: MultiWaker {
            any_parked: AtomicBool::new(false),
            wakers: Mutex::new(VecDeque::new()),
        },
    });
    (
        MpmcSender {
            channel: channel.clone(),
        },
        MpmcReceiver { channel },
    )
}
