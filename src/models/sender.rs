// Senders of various kinds for different Buffer modes.

use triomphe::Arc;

use crate::{
    helper_functions::{determine_flag_size, determine_marker_type_index},
    helper_traits::MarkerData,
    models::{
        BufferQueue,
        buffer_modes::{MPMC, MPSC, SPMC, SPSC},
        channel::Channel,
        helper_models::MarkerTypeDecider,
        send_future::SendFuture,
    },
    traits::{
        buffer_mode_traits::BufferMode, buffer_operations_traits::BufferOperation,
        notifier::Notifier,
    },
};

#[repr(transparent)]
pub struct SpscSender<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    channel: Arc<Channel<T, SPSC, N>>,
}

#[repr(transparent)]
pub struct SpmcSender<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    channel: Arc<Channel<T, SPMC, N>>,
}

// Only senders for multiple producers implement clone
#[repr(transparent)]
#[derive(Clone)]
pub struct MpscSender<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    channel: Arc<Channel<T, MPSC, N>>,
}

#[repr(transparent)]
#[derive(Clone)]
pub struct MpmcSender<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    channel: Arc<Channel<T, MPMC, N>>,
}

impl<T, const N: usize> SpscSender<T, N>
where
    BufferQueue<T, SPSC, N>: BufferOperation<T>,
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    #[inline(always)]
    pub fn try_send(&self, val: T) -> Result<(), T> {
        match self.channel.buf.push(val) {
            None => {
                self.channel.waker.notify_one();
                Ok(())
            }
            Some(v) => Err(v),
        }
    }

    pub fn send(&self, val: T) -> SendFuture<'_, T, SPSC, N> {
        SendFuture {
            channel: &self.channel,
            val: Some(val),
        }
    }
}

impl<T, const N: usize> SpmcSender<T, N>
where
    BufferQueue<T, SPMC, N>: BufferOperation<T>,
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    #[inline(always)]
    pub fn try_send(&self, val: T) -> Result<(), T> {
        match self.channel.buf.push(val) {
            None => {
                self.channel.waker.notify_one();
                Ok(())
            }
            Some(v) => Err(v),
        }
    }

    pub fn send(&self, val: T) -> SendFuture<'_, T, SPMC, N> {
        SendFuture {
            channel: &self.channel,
            val: Some(val),
        }
    }
}

impl<T, const N: usize> MpscSender<T, N>
where
    BufferQueue<T, MPSC, N>: BufferOperation<T>,
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    #[inline(always)]
    pub fn try_send(&self, val: T) -> Result<(), T> {
        match self.channel.buf.push(val) {
            None => {
                self.channel.waker.notify_one();
                Ok(())
            }
            Some(v) => Err(v),
        }
    }

    pub fn send(&self, val: T) -> SendFuture<'_, T, MPSC, N> {
        SendFuture {
            channel: &self.channel,
            val: Some(val),
        }
    }
}

impl<T, const N: usize> MpmcSender<T, N>
where
    BufferQueue<T, MPMC, N>: BufferOperation<T>,
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    #[inline(always)]
    pub fn try_send(&self, val: T) -> Result<(), T> {
        match self.channel.buf.push(val) {
            None => {
                self.channel.waker.notify_one();
                Ok(())
            }
            Some(v) => Err(v),
        }
    }

    pub fn send(&self, val: T) -> SendFuture<'_, T, MPMC, N> {
        SendFuture {
            channel: &self.channel,
            val: Some(val),
        }
    }
}
