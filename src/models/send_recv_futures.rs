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

/// Future from sending in a spsc channel
pub struct SpscSendFuture<'a, T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: &'a Arc<SpscChannel<T, N>>,
    pub(crate) val: Option<T>,
}

/// Future from sending in a spmc channel
pub struct SpmcSendFuture<'a, T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: &'a Arc<SpmcChannel<T, N>>,
    pub(crate) val: Option<T>,
}

/// Future from sending in a mpsc channel
pub struct MpscSendFuture<'a, T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: &'a Arc<MpscChannel<T, N>>,
    pub(crate) val: Option<T>,
}

/// Future from sending in a mpmc channel
pub struct MpmcSendFuture<'a, T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: &'a Arc<MpmcChannel<T, N>>,
    pub(crate) val: Option<T>,
}

/// Future from receiving in a spsc channel
pub struct SpscRecvFuture<'a, T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: &'a Arc<SpscChannel<T, N>>,
}

/// Future from receiving in a spmc channel
pub struct SpmcRecvFuture<'a, T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: &'a Arc<SpmcChannel<T, N>>,
}

/// Future from receiving in a mpsc channel
pub struct MpscRecvFuture<'a, T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: &'a Arc<MpscChannel<T, N>>,
}

/// Future from receiving in a mpmc channel
pub struct MpmcRecvFuture<'a, T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: &'a Arc<MpmcChannel<T, N>>,
}
