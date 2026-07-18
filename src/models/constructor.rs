// To be transferred under main buffer
use std::{collections::VecDeque, sync::atomic::AtomicBool};

use atomic_waker::AtomicWaker;
use parking_lot::Mutex;
use triomphe::Arc;

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
