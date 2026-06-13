// Senders of various kinds for different Buffer modes.

use triomphe::Arc;

use crate::{
    helper_functions::{determine_flag_size, determine_marker_type_index},
    helper_traits::MarkerData,
    models::{
        buffer_modes::{MPMC, MPSC, SPMC, SPSC},
        channel::Channel,
        helper_models::MarkerTypeDecider,
    },
    traits::buffer_mode_traits::BufferMode,
};

pub struct SpscSender<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    channel: Arc<Channel<T, SPSC, N>>,
}

pub struct SpmcSender<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    channel: Arc<Channel<T, SPMC, N>>,
}

pub struct MpscSender<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    channel: Arc<Channel<T, MPSC, N>>,
}

pub struct MpmcSender<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    channel: Arc<Channel<T, MPMC, N>>,
}
