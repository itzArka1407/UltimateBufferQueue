// Receiver side of the channel -- to receive inputs in different buffer modes.

use triomphe::Arc;

use crate::{
    helper_functions::{determine_flag_size, determine_marker_type_index},
    helper_traits::MarkerData,
    models::{
        BufferQueue,
        buffer_modes::{MPMC, MPSC, SPMC, SPSC},
        channel::Channel,
        helper_models::MarkerTypeDecider,
        recv_future::RecvFuture,
    },
    traits::{buffer_operations_traits::BufferOperation, notifier::Notifier},
};

#[repr(transparent)]
pub struct SpscReceiver<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    channel: Arc<Channel<T, SPSC, N>>,
}

// Only receivers for multiple consumers implement clone
#[repr(transparent)]
#[derive(Clone)]
pub struct SpmcReceiver<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    channel: Arc<Channel<T, SPMC, N>>,
}

#[repr(transparent)]
pub struct MpscReceiver<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    channel: Arc<Channel<T, MPSC, N>>,
}

#[repr(transparent)]
#[derive(Clone)]
pub struct MpmcReceiver<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    channel: Arc<Channel<T, MPMC, N>>,
}

impl<T, const N: usize> SpscReceiver<T, N>
where
    BufferQueue<T, SPSC, N>: BufferOperation<T>,
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    #[inline(always)]
    pub fn try_recv(&self) -> Option<T> {
        let val = self.channel.buf.pop();
        if val.is_some() {
            self.channel.waker.notify();
        }
        val
    }

    pub fn recv(&self) -> RecvFuture<'_, T, SPSC, N> {
        RecvFuture {
            channel: &self.channel,
        }
    }
}

impl<T, const N: usize> SpmcReceiver<T, N>
where
    BufferQueue<T, SPMC, N>: BufferOperation<T>,
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    #[inline(always)]
    pub fn try_recv(&self) -> Option<T> {
        let val = self.channel.buf.pop();
        if val.is_some() {
            self.channel.waker.notify();
        }
        val
    }

    pub fn recv(&self) -> RecvFuture<'_, T, SPMC, N> {
        RecvFuture {
            channel: &self.channel,
        }
    }
}

impl<T, const N: usize> MpscReceiver<T, N>
where
    BufferQueue<T, MPSC, N>: BufferOperation<T>,
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    #[inline(always)]
    pub fn try_recv(&self) -> Option<T> {
        let val = self.channel.buf.pop();
        if val.is_some() {
            self.channel.waker.notify();
        }
        val
    }

    pub fn recv(&self) -> RecvFuture<'_, T, MPSC, N> {
        RecvFuture {
            channel: &self.channel,
        }
    }
}

impl<T, const N: usize> MpmcReceiver<T, N>
where
    BufferQueue<T, MPMC, N>: BufferOperation<T>,
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    #[inline(always)]
    pub fn try_recv(&self) -> Option<T> {
        let val = self.channel.buf.pop();
        if val.is_some() {
            self.channel.waker.notify();
        }
        val
    }

    pub fn recv(&self) -> RecvFuture<'_, T, MPMC, N> {
        RecvFuture {
            channel: &self.channel,
        }
    }
}
