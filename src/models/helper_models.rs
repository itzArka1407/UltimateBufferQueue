use std::sync::atomic::{AtomicU8, AtomicU16, AtomicU32, AtomicU64};

use crate::helper_traits::MarkerData;
const ATOMIC_ZERO: AtomicU8 = AtomicU8::new(0); // Used to init the bit flags

// Create the markers for the buffer queue, head/tail moves on insertion
// The ready mask is a bit-flag mask array that denotes if some value is ready for operations
pub(crate) struct BufferMarkers<MarkerType: Default, const MASK_SIZE: usize> {
    head: MarkerType,
    tail: MarkerType,
    ready_mask: [AtomicU8; MASK_SIZE],
}

impl<M: Default, const M_SIZE: usize> BufferMarkers<M, M_SIZE> {
    pub fn new() -> Self {
        Self {
            head: M::default(),
            tail: M::default(),
            ready_mask: [ATOMIC_ZERO; M_SIZE],
        }
    }
}

// This struct holds the info about the marker type to be used on the markers
// The different trait implementations produce the appropriate marker type -- check BufferQueue
pub struct MarkerType<const N: usize>;
impl MarkerData for MarkerType<0> {
    type MarkerType = AtomicU8;
}

impl MarkerData for MarkerType<1> {
    type MarkerType = AtomicU16;
}

impl MarkerData for MarkerType<2> {
    type MarkerType = AtomicU32;
}

impl MarkerData for MarkerType<3> {
    type MarkerType = AtomicU64;
}
