use crate::{
    helper_functions::{determine_flag_size, determine_marker_type_index},
    helper_traits::MarkerData,
    models::{
        BufferQueue,
        buffer_modes::{SPMC, SPSC},
        helper_models::MarkerTypeDecider,
        send_recv_futures::{SpscRecvFuture, SpscSendFuture},
        senders_receivers::{SpmcSender, SpscReceiver, SpscSender},
    },
    traits::buffer_operations_traits::BufferOperation,
};

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
                self.channel.recv_waker.notify();
                Ok(())
            }
            Some(v) => Err(v),
        }
    }
    pub fn send(&self, val: T) -> SpscSendFuture<'_, T, N> {
        SpscSendFuture {
            channel: &self.channel,
            val: Some(val),
        }
    }
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
            self.channel.send_waker.notify();
        }
        val
    }
    pub fn recv(&self) -> SpscRecvFuture<'_, T, N> {
        SpscRecvFuture {
            channel: &self.channel,
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
                self.channel.recv_waker.notify();
                Ok(())
            }
            Some(v) => Err(v),
        }
    }
    pub fn send(&self, val: T) -> SpmcSendFuture<'_, T, N> {
        SpmcSendFuture {
            channel: &self.channel,
            val: Some(val),
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
            self.channel.send_waker.notify();
        }
        val
    }
    pub fn recv(&self) -> SpmcRecvFuture<'_, T, N> {
        SpmcRecvFuture {
            channel: &self.channel,
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
                self.channel.recv_waker.notify();
                Ok(())
            }
            Some(v) => Err(v),
        }
    }
    pub fn send(&self, val: T) -> MpscSendFuture<'_, T, N> {
        MpscSendFuture {
            channel: &self.channel,
            val: Some(val),
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
            self.channel.send_waker.notify();
        }
        val
    }
    pub fn recv(&self) -> MpscRecvFuture<'_, T, N> {
        MpscRecvFuture {
            channel: &self.channel,
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
                self.channel.recv_waker.notify();
                Ok(())
            }
            Some(v) => Err(v),
        }
    }
    pub fn send(&self, val: T) -> MpmcSendFuture<'_, T, N> {
        MpmcSendFuture {
            channel: &self.channel,
            val: Some(val),
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
            self.channel.send_waker.notify();
        }
        val
    }
    pub fn recv(&self) -> MpmcRecvFuture<'_, T, N> {
        MpmcRecvFuture {
            channel: &self.channel,
        }
    }
}
