use atomic_waker::AtomicWaker;

use crate::{
    helper_functions::{determine_flag_size, determine_marker_type_index},
    helper_traits::MarkerData,
    models::{BufferQueue, helper_models::MarkerTypeDecider},
    traits::buffer_mode_traits::BufferMode,
};

// Channel to perform operations on both ends
pub struct Channel<T, M: BufferMode, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) buf: BufferQueue<T, M, N>,
    pub(crate) waker: AtomicWaker,
}
