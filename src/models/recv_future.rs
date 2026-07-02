use std::{
    pin::Pin,
    task::{Context, Poll},
};

use triomphe::Arc;

use crate::{
    helper_functions::{determine_flag_size, determine_marker_type_index},
    helper_traits::MarkerData,
    models::{BufferQueue, channel::Channel, helper_models::MarkerTypeDecider},
    traits::{
        buffer_mode_traits::BufferMode, buffer_operations_traits::BufferOperation,
        notifier::Notifier,
    },
};

pub struct RecvFuture<'a, T, M: BufferMode, const N: usize>
where
    BufferQueue<T, M, N>: BufferOperation<T>,
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub channel: &'a Arc<Channel<T, M, N>>,
}

impl<'a, T, M, const N: usize> Future for RecvFuture<'a, T, M, N>
where
    M: BufferMode,
    BufferQueue<T, M, N>: BufferOperation<T>,
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        if let Some(val) = self.channel.buf.pop() {
            self.channel.waker.notify();
            return Poll::Ready(val);
        }
        // Reattempt to prevent TOCTOU race: Time of checking of pop & Time of register(cx)
        self.channel.waker.register(cx);
        if let Some(val) = self.channel.buf.pop() {
            self.channel.waker.notify();
            return Poll::Ready(val);
        }
        Poll::Pending
    }
}
