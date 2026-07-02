// Sending future when a sender sends a msg

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

pub struct SendFuture<'a, T, M: BufferMode, const N: usize>
where
    BufferQueue<T, M, N>: BufferOperation<T>,
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub channel: &'a Arc<Channel<T, M, N>>,
    pub val: Option<T>,
}

impl<'a, T, M, const N: usize> Unpin for SendFuture<'a, T, M, N>
where
    M: BufferMode,
    BufferQueue<T, M, N>: BufferOperation<T>,
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
}

impl<'a, T, M, const N: usize> Future for SendFuture<'a, T, M, N>
where
    M: BufferMode,
    BufferQueue<T, M, N>: BufferOperation<T>,
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let val = self
            .val
            .take()
            .expect("[UBQ - SendFuture] polled after completion");
        match self.channel.buf.push(val) {
            // No return -- the push has happened
            None => {
                self.channel.waker.notify();
                Poll::Ready(())
            }
            // Value returned back -- the pushing failed
            Some(v) => {
                self.val = Some(v);
                self.channel.waker.register(cx);
                // re-check after registering -- a slot may have freed in the gap
                match self.channel.buf.push(self.val.take().unwrap()) {
                    None => {
                        self.channel.waker.notify();
                        Poll::Ready(())
                    }
                    Some(v) => {
                        self.val = Some(v);
                        Poll::Pending
                    }
                }
            }
        }
    }
}
