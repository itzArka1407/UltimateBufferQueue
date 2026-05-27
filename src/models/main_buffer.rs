use crate::{
    helper_functions::{determine_flag_size, determine_marker_type_index},
    helper_traits::MarkerData,
    models::helper_models::{BufferMarkers, MarkerType},
};
use std::{cell::UnsafeCell, mem::MaybeUninit};

pub struct BufferQueue<T, const N: usize>
where
    [(); determine_marker_type_index(N)]: Sized,
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerType<{ determine_marker_type_index(N) }>: MarkerData,
{
    buf: UnsafeCell<[MaybeUninit<T>; N]>,
    markers: BufferMarkers<
        <MarkerType<{ determine_marker_type_index(N) }> as MarkerData>::MarkerType,
        { determine_flag_size(N, 8) },
    >,
}

impl<T, const N: usize> BufferQueue<T, N>
where
    [(); determine_marker_type_index(N)]: Sized,
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerType<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub fn new() -> Self {
        Self {
            buf: unsafe { UnsafeCell::new(MaybeUninit::uninit().assume_init()) },
            markers: BufferMarkers::new(),
        }
    }
}
