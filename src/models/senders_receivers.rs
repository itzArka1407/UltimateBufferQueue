//! Defines all the senders and receivers for the different channels to be used
use triomphe::Arc;

use crate::{
    helper_functions::{determine_flag_size, determine_marker_type_index},
    helper_traits::MarkerData,
    models::{
        channels::{MpmcChannel, MpscChannel, SpmcChannel, SpscChannel},
        helper_models::MarkerTypeDecider,
    },
};

/// Sending half of an SPSC channel. Not `Clone`.      (SpscSender)
#[repr(transparent)]
pub struct SpscSender<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: Arc<SpscChannel<T, N>>,
}

/// Receiving half of an SPSC channel. Not `Clone`.    (SpscReceiver)
#[repr(transparent)]
pub struct SpscReceiver<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: Arc<SpscChannel<T, N>>,
}

/// Sending half of an SPMC channel. Not `Clone`.      (SpmcSender)
#[repr(transparent)]
pub struct SpmcSender<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: Arc<SpmcChannel<T, N>>,
}

/// Receiving half of an SPMC channel. `Clone` to add consumers; each item is delivered to one receiver. (SpmcReceiver)
#[repr(transparent)]
#[derive(Clone)]
pub struct SpmcReceiver<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: Arc<SpmcChannel<T, N>>,
}

/// Sending half of an MPSC channel. `Clone` to add producers. (MpscSender)
#[repr(transparent)]
#[derive(Clone)]
pub struct MpscSender<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: Arc<MpscChannel<T, N>>,
}

/// Receiving half of an MPSC channel. Not `Clone`.    (MpscReceiver)
#[repr(transparent)]
pub struct MpscReceiver<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: Arc<MpscChannel<T, N>>,
}

/// Sending half of an MPMC channel. `Clone` to add producers. (MpmcSender)
#[repr(transparent)]
#[derive(Clone)]
pub struct MpmcSender<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: Arc<MpmcChannel<T, N>>,
}

/// Receiving half of an MPMC channel. `Clone` to add consumers; each item is delivered to one receiver. (MpmcReceiver)use triomphe::Arc;
#[repr(transparent)]
#[derive(Clone)]
pub struct MpmcReceiver<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: Arc<MpmcChannel<T, N>>,
}
