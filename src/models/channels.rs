//! The different form of channels which are responsible for sending/receiving of messages
use triomphe::Arc;

use crate::{
    helper_functions::{determine_flag_size, determine_marker_type_index},
    helper_traits::MarkerData,
    models::{
        BufferQueue,
        buffer_modes::{MPMC, MPSC, SPMC, SPSC},
        helper_models::MarkerTypeDecider,
        wakers::{MultiWaker, SingleWaker},
    },
};

/// A channel to handle a SPSC buffer
pub struct SpscChannel<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) buf: BufferQueue<T, SPSC, N>,
    pub(crate) recv_waker: SingleWaker,
    pub(crate) send_waker: SingleWaker,
}

/// A channel to handle a SPMC buffer
pub struct SpmcChannel<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) buf: BufferQueue<T, SPMC, N>,
    pub(crate) recv_waker: MultiWaker,
    pub(crate) send_waker: SingleWaker,
}

/// A channel to handle a MPSC buffer
pub struct MpscChannel<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) buf: BufferQueue<T, MPSC, N>,
    pub(crate) recv_waker: SingleWaker,
    pub(crate) send_waker: MultiWaker,
}

/// A channel to handle a MPMC buffer
pub struct MpmcChannel<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) buf: BufferQueue<T, MPMC, N>,
    pub(crate) recv_waker: MultiWaker,
    pub(crate) send_waker: MultiWaker,
}
