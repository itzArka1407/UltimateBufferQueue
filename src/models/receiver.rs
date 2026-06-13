// Receiver side of the channel -- to receive inputs in different buffer modes.

use triomphe::Arc;

use crate::{
    helper_functions::{determine_flag_size, determine_marker_type_index},
    helper_traits::MarkerData,
    models::{
        buffer_modes::{MPMC, MPSC, SPMC, SPSC},
        channel::Channel,
        helper_models::MarkerTypeDecider,
    },
};

pub struct SpscReceiver<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    channel: Arc<Channel<T, SPSC, N>>,
}

pub struct SpmcReceiver<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    channel: Arc<Channel<T, SPMC, N>>,
}

pub struct MpscReceiver<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    channel: Arc<Channel<T, MPSC, N>>,
}

pub struct MpmcReceiver<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    channel: Arc<Channel<T, MPMC, N>>,
}
