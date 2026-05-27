use std::sync::atomic::{AtomicU8, AtomicU16, AtomicU32, AtomicU64, Ordering};

use crate::helper_traits::{MarkerAtomicOperations, MarkerData};
const ATOMIC_ZERO: AtomicU8 = AtomicU8::new(0); // Used to init the bit flags

// Create the markers for the buffer queue, head/tail moves on insertion
// The ready mask is a bit-flag mask array that denotes if some value is ready for operations
pub(crate) struct BufferMarkers<
    MarkerType: Default + MarkerAtomicOperations,
    const MASK_SIZE: usize,
> {
    pub head: MarkerType,
    pub tail: MarkerType,
    pub ready_mask: [AtomicU8; MASK_SIZE],
}

impl<M: Default + MarkerAtomicOperations, const M_SIZE: usize> BufferMarkers<M, M_SIZE> {
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
pub struct MarkerTypeDecider<const N: usize>;
impl MarkerData for MarkerTypeDecider<0> {
    type MarkerType = AtomicU8;
}

impl MarkerData for MarkerTypeDecider<1> {
    type MarkerType = AtomicU16;
}

impl MarkerData for MarkerTypeDecider<2> {
    type MarkerType = AtomicU32;
}

impl MarkerData for MarkerTypeDecider<3> {
    type MarkerType = AtomicU64;
}

// Trait impls for marker operations
impl MarkerAtomicOperations for AtomicU8 {
    type OutputItem = u8;
    fn load_acq(&self) -> u8 {
        self.load(Ordering::Acquire)
    }

    fn store_rel(&self, val: u8) {
        self.store(val, Ordering::Release);
    }
}

impl MarkerAtomicOperations for AtomicU16 {
    type OutputItem = u16;
    fn load_acq(&self) -> u16 {
        self.load(Ordering::Acquire)
    }

    fn store_rel(&self, val: u16) {
        self.store(val, Ordering::Release);
    }
}

impl MarkerAtomicOperations for AtomicU32 {
    type OutputItem = u32;
    fn load_acq(&self) -> u32 {
        self.load(Ordering::Acquire)
    }

    fn store_rel(&self, val: u32) {
        self.store(val, Ordering::Release);
    }
}

impl MarkerAtomicOperations for AtomicU64 {
    type OutputItem = u64;
    fn load_acq(&self) -> u64 {
        self.load(Ordering::Acquire)
    }

    fn store_rel(&self, val: u64) {
        self.store(val, Ordering::Release);
    }
}
