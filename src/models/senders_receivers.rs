use triomphe::Arc;

use crate::{
    helper_functions::{determine_flag_size, determine_marker_type_index},
    helper_traits::MarkerData,
    models::{
        channels::{MpmcChannel, MpscChannel, SpmcChannel, SpscChannel},
        helper_models::MarkerTypeDecider,
    },
};

#[repr(transparent)]
pub struct SpscSender<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: Arc<SpscChannel<T, N>>,
}

#[repr(transparent)]
pub struct SpscReceiver<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: Arc<SpscChannel<T, N>>,
}

#[repr(transparent)]
pub struct SpmcSender<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: Arc<SpmcChannel<T, N>>,
}

#[repr(transparent)]
#[derive(Clone)]
pub struct SpmcReceiver<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: Arc<SpmcChannel<T, N>>,
}

#[repr(transparent)]
#[derive(Clone)]
pub struct MpscSender<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: Arc<MpscChannel<T, N>>,
}

#[repr(transparent)]
pub struct MpscReceiver<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: Arc<MpscChannel<T, N>>,
}

#[repr(transparent)]
#[derive(Clone)]
pub struct MpmcSender<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: Arc<MpmcChannel<T, N>>,
}

#[repr(transparent)]
#[derive(Clone)]
pub struct MpmcReceiver<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: Arc<MpmcChannel<T, N>>,
}
