use crate::{
    helper_functions::{determine_flag_size, determine_marker_type_index},
    helper_traits::{MarkerAtomicOperations, MarkerData, MarkerTypeBound},
    models::helper_models::{BufferMarkers, MarkerTypeDecider},
};
use std::{cell::UnsafeCell, mem::MaybeUninit};

// SAFETY: Uses nightly features, stable rust as of May 2026 doesn't support generic const
// evaluations, so this is not possible to do with stable rust
pub struct BufferQueue<T, const N: usize>
where
    [(); determine_marker_type_index(N)]: Sized,
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerTypeBound,
{
    buf: UnsafeCell<[MaybeUninit<T>; N]>,
    markers: BufferMarkers<
        <MarkerTypeDecider<{ determine_marker_type_index(N) }> as MarkerData>::MarkerType,
        { determine_flag_size(N, 8) },
    >,
}

impl<T, const N: usize> BufferQueue<T, N>
where
    [(); determine_marker_type_index(N)]: Sized,
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerTypeBound,
{
    pub fn new() -> Self {
        Self {
            buf: unsafe { UnsafeCell::new(MaybeUninit::uninit().assume_init()) },
            markers: BufferMarkers::new(),
        }
    }

    // The following functions are methods of altering the buffer's internals, not accessible by a
    // user crate -- these functions are used in other places

    #[inline(always)]
    fn raw_spsc_push(&self) -> bool {
        let head = self.markers.head;
    }
}
