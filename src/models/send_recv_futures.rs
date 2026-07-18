use triomphe::Arc;

use crate::{
    helper_functions::{determine_flag_size, determine_marker_type_index},
    helper_traits::MarkerData,
    models::{
        channels::{MpmcChannel, MpscChannel, SpmcChannel, SpscChannel},
        helper_models::MarkerTypeDecider,
    },
};

pub struct SpscSendFuture<'a, T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: &'a Arc<SpscChannel<T, N>>,
    pub(crate) val: Option<T>,
}

pub struct SpmcSendFuture<'a, T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: &'a Arc<SpmcChannel<T, N>>,
    pub(crate) val: Option<T>,
}

pub struct MpscSendFuture<'a, T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: &'a Arc<MpscChannel<T, N>>,
    pub(crate) val: Option<T>,
}

pub struct MpmcSendFuture<'a, T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: &'a Arc<MpmcChannel<T, N>>,
    pub(crate) val: Option<T>,
}

pub struct SpscRecvFuture<'a, T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: &'a Arc<SpscChannel<T, N>>,
}

pub struct SpmcRecvFuture<'a, T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: &'a Arc<SpmcChannel<T, N>>,
}

pub struct MpscRecvFuture<'a, T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: &'a Arc<MpscChannel<T, N>>,
}

pub struct MpmcRecvFuture<'a, T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) channel: &'a Arc<MpmcChannel<T, N>>,
}
