use triomphe::Arc;

use crate::models::BufferQueue;
use crate::models::buffer_modes::{MPMC, MPSC, SPMC, SPSC};
use crate::models::send_recv_futures::{
    MpmcRecvFuture, MpmcSendFuture, MpscRecvFuture, MpscSendFuture, SpmcRecvFuture, SpmcSendFuture,
    SpscRecvFuture, SpscSendFuture,
};
use crate::traits::buffer_operations_traits::BufferOperation;
use crate::{
    helper_functions::{determine_flag_size, determine_marker_type_index},
    helper_traits::MarkerData,
    models::{
        channels::{MpmcChannel, MpscChannel, SpmcChannel, SpscChannel},
        helper_models::MarkerTypeDecider,
    },
};
use std::pin::Pin;
use std::task::{Context, Poll};

// Macros to avoid repetition -- implements the futures required
macro_rules! impl_send_future {
    ($future:ty, $channel:ty, $buf_mode:ty) => {
        impl<'a, T, const N: usize> Unpin for $future
        where
            [(); determine_flag_size(N, 8)]: Sized,
            MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
        {
        }

        impl<'a, T, const N: usize> Future for $future
        where
            BufferQueue<T, $buf_mode, N>: BufferOperation<T>,
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
                    None => {
                        self.channel.recv_waker.notify();
                        Poll::Ready(())
                    }
                    Some(v) => {
                        self.val = Some(v);
                        self.channel.send_waker.register(cx);
                        match self.channel.buf.push(self.val.take().unwrap()) {
                            None => {
                                self.channel.recv_waker.notify();
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
    };
}

macro_rules! impl_recv_future {
    ($future:ty, $channel:ty, $buf_mode:ty) => {
        impl<'a, T, const N: usize> Unpin for $future
        where
            [(); determine_flag_size(N, 8)]: Sized,
            MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
        {
        }

        impl<'a, T, const N: usize> Future for $future
        where
            BufferQueue<T, $buf_mode, N>: BufferOperation<T>,
            [(); determine_flag_size(N, 8)]: Sized,
            MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
        {
            type Output = T;
            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
                if let Some(val) = self.channel.buf.pop() {
                    self.channel.send_waker.notify();
                    return Poll::Ready(val);
                }
                self.channel.recv_waker.register(cx);
                if let Some(val) = self.channel.buf.pop() {
                    self.channel.send_waker.notify();
                    return Poll::Ready(val);
                }
                Poll::Pending
            }
        }
    };
}

impl_send_future!(SpscSendFuture<'a, T, N>, SpscChannel<T, N>, SPSC);
impl_send_future!(SpmcSendFuture<'a, T, N>, SpmcChannel<T, N>, SPMC);
impl_send_future!(MpscSendFuture<'a, T, N>, MpscChannel<T, N>, MPSC);
impl_send_future!(MpmcSendFuture<'a, T, N>, MpmcChannel<T, N>, MPMC);

impl_recv_future!(SpscRecvFuture<'a, T, N>, SpscChannel<T, N>, SPSC);
impl_recv_future!(SpmcRecvFuture<'a, T, N>, SpmcChannel<T, N>, SPMC);
impl_recv_future!(MpscRecvFuture<'a, T, N>, MpscChannel<T, N>, MPSC);
impl_recv_future!(MpmcRecvFuture<'a, T, N>, MpmcChannel<T, N>, MPMC);
