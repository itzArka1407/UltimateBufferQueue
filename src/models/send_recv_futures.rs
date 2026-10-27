//! Defines the futures created by sending and receiving from a sender/receiver

use triomphe::Arc;

use crate::{
    helper_functions::{determine_flag_size, determine_marker_type_index},
    helper_traits::MarkerData,
    models::{
        channels::{MpmcChannel, MpscChannel, SpmcChannel, SpscChannel},
        helper_models::MarkerTypeDecider,
    },
};

/// Future returned by `send` on the Spsc sender. Resolves to `()` once the value is pushed.
pub struct SpscSendFuture<'a, T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: &'a Arc<SpscChannel<T, N>>,
    pub(crate) val: Option<T>,
}

/// Future returned by `send` on the Spmc sender. Resolves to `()` once the value is pushed.
pub struct SpmcSendFuture<'a, T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: &'a Arc<SpmcChannel<T, N>>,
    pub(crate) val: Option<T>,
}

/// Future returned by `send` on the Mpsc sender. Resolves to `()` once the value is pushed.
pub struct MpscSendFuture<'a, T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: &'a Arc<MpscChannel<T, N>>,
    pub(crate) val: Option<T>,
}

/// Future returned by `send` on the Mpmc sender. Resolves to `()` once the value is pushed.
pub struct MpmcSendFuture<'a, T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: &'a Arc<MpmcChannel<T, N>>,
    pub(crate) val: Option<T>,
}

/// Future returned by `recv` on the Spsc receiver. Resolves to the received `T`.
pub struct SpscRecvFuture<'a, T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: &'a Arc<SpscChannel<T, N>>,
}

/// Future returned by `recv` on the Spmc receiver. Resolves to the received `T`.
pub struct SpmcRecvFuture<'a, T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: &'a Arc<SpmcChannel<T, N>>,
}

/// Future returned by `recv` on the Mpsc receiver. Resolves to the received `T`.
pub struct MpscRecvFuture<'a, T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: &'a Arc<MpscChannel<T, N>>,
}

/// Future returned by `recv` on the Mpmc receiver. Resolves to the received `T`.
pub struct MpmcRecvFuture<'a, T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: &'a Arc<MpmcChannel<T, N>>,
}
