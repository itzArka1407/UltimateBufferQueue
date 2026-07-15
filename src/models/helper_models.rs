use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU16, AtomicU32, AtomicU64, Ordering};

use crate::helper_models::cache_aligned_type::CacheAligned;
use crate::helper_traits::{MarkerAtomicOperations, MarkerData};
const ATOMIC_ZERO: AtomicU8 = AtomicU8::new(0); // Used to init the bit flags

// Marking the state of a bit in the mask -- 0 for free state, 1 for busy
pub(crate) enum BitFlip {
    Register,   // Bit -> 1 -- register busy state
    Unregister, // Bit -> 0 -- unregister busy state
}

// Create the markers for the buffer queue, head/tail moves on insertion
// The ready mask is a bit-flag mask array that denotes if some value is ready for operations
pub(crate) struct BufferMarkers<
    MarkerType: Default + MarkerAtomicOperations,
    const MASK_SIZE: usize,
> {
    pub head: CacheAligned<MarkerType>,    // The buffer's head index
    pub tail: CacheAligned<MarkerType>,    // The buffer's tail index
    pub invalidated: AtomicBool,           // If the buffer is invalidated
    pub write_mask: [AtomicU8; MASK_SIZE], // The mask to represent write-state
    pub read_mask: [AtomicU8; MASK_SIZE],  // The mask to represent write-state
}

impl<M: Default + MarkerAtomicOperations, const M_SIZE: usize> BufferMarkers<M, M_SIZE> {
    #[inline]
    pub fn new() -> Self {
        Self {
            head: CacheAligned::default(),
            tail: CacheAligned::default(),
            invalidated: AtomicBool::default(),
            write_mask: [ATOMIC_ZERO; M_SIZE],
            read_mask: [ATOMIC_ZERO; M_SIZE],
        }
    }

    // Update the read mask for an active read operation
    #[inline(always)]
    pub fn update_read_mask(&self, idx: usize, order: Ordering, operation: BitFlip) {
        let el_idx = idx / 8; // Where the input in bit mask is gonna happen
        let mask = 1 << (7 - (idx % 8)); // Mark only a bit as 1, else all 0

        let el = &self.read_mask[el_idx];
        match operation {
            BitFlip::Register => el.fetch_or(mask, order), // Mark the nth bit from start - 1
            BitFlip::Unregister => el.fetch_and(!mask, order), // Mark the nth bit from start - 0
        };
    }

    // Update the write operation for an active write mask
    #[inline(always)]
    pub fn update_write_mask(&self, idx: usize, order: Ordering, operation: BitFlip) {
        let el_idx = idx / 8; // Where the input in bit mask is gonna happen
        let mask = 1 << (7 - (idx % 8)); // Mark only a bit as 1, else all 0

        let el = &self.write_mask[el_idx];
        match operation {
            BitFlip::Register => el.fetch_or(mask, order), // Mark the nth bit from start - 1
            BitFlip::Unregister => el.fetch_and(!mask, order), // Mark the nth bit from start - 0
        };
    }

    // Checks if the bit at a given idx is 0 -- means no reading going on
    #[inline(always)]
    pub fn is_not_being_read(&self, idx: usize) -> bool {
        let el_idx = idx / 8;
        let mask = 1 << (7 - (idx % 8));

        self.read_mask[el_idx].load(Ordering::Acquire) & mask == 0
    }

    // Checks if the bit at a given idx is 0 -- means no writing going on
    #[inline(always)]
    pub fn is_not_being_written(&self, idx: usize) -> bool {
        let el_idx = idx / 8;
        let mask = 1 << (7 - (idx % 8));

        self.write_mask[el_idx].load(Ordering::Acquire) & mask == 0
    }
}

// This struct holds the info about the marker type to be used on the markers
// The different trait implementations produce the appropriate marker type -- check BufferQueue
pub struct MarkerTypeDecider<const N: usize>;
impl MarkerData for MarkerTypeDecider<0> {
    type MarkerType = AtomicU8;
    const DENOMINATOR: usize = 8;
}

impl MarkerData for MarkerTypeDecider<1> {
    type MarkerType = AtomicU16;
    const DENOMINATOR: usize = 16;
}

impl MarkerData for MarkerTypeDecider<2> {
    type MarkerType = AtomicU32;
    const DENOMINATOR: usize = 32;
}

impl MarkerData for MarkerTypeDecider<3> {
    type MarkerType = AtomicU64;
    const DENOMINATOR: usize = 64;
}

// Trait impls for marker operations
impl MarkerAtomicOperations for AtomicU8 {
    type OutputItem = u8;

    fn load(&self, order: Ordering) -> Self::OutputItem {
        self.load(order)
    }

    fn store(&self, val: Self::OutputItem, order: Ordering) {
        self.store(val, order);
    }

    fn fetch_add(&self, val: usize, order: Ordering) -> usize {
        self.fetch_add(val as u8, order) as usize
    }

    fn try_update(
        &self,
        set_order: Ordering,
        fetch_order: Ordering,
        f: impl FnMut(Self::OutputItem) -> Option<Self::OutputItem>,
    ) -> Result<u8, u8> {
        self.try_update(set_order, fetch_order, f)
    }
}

impl MarkerAtomicOperations for AtomicU16 {
    type OutputItem = u16;

    fn load(&self, order: Ordering) -> Self::OutputItem {
        self.load(order)
    }

    fn store(&self, val: Self::OutputItem, order: Ordering) {
        self.store(val, order);
    }

    fn fetch_add(&self, val: usize, order: Ordering) -> usize {
        self.fetch_add(val as u16, order) as usize
    }

    fn try_update(
        &self,
        set_order: Ordering,
        fetch_order: Ordering,
        f: impl FnMut(Self::OutputItem) -> Option<Self::OutputItem>,
    ) -> Result<Self::OutputItem, Self::OutputItem> {
        self.try_update(set_order, fetch_order, f)
    }
}

impl MarkerAtomicOperations for AtomicU32 {
    type OutputItem = u32;

    fn load(&self, order: Ordering) -> Self::OutputItem {
        self.load(order)
    }

    fn store(&self, val: Self::OutputItem, order: Ordering) {
        self.store(val, order);
    }

    fn fetch_add(&self, val: usize, order: Ordering) -> usize {
        self.fetch_add(val as u32, order) as usize
    }

    fn try_update(
        &self,
        set_order: Ordering,
        fetch_order: Ordering,
        f: impl FnMut(Self::OutputItem) -> Option<Self::OutputItem>,
    ) -> Result<Self::OutputItem, Self::OutputItem> {
        self.try_update(set_order, fetch_order, f)
    }
}

impl MarkerAtomicOperations for AtomicU64 {
    type OutputItem = u64;

    fn load(&self, order: Ordering) -> Self::OutputItem {
        self.load(order)
    }

    fn store(&self, val: Self::OutputItem, order: Ordering) {
        self.store(val, order);
    }

    fn fetch_add(&self, val: usize, order: Ordering) -> usize {
        self.fetch_add(val as u64, order) as usize
    }

    fn try_update(
        &self,
        set_order: Ordering,
        fetch_order: Ordering,
        f: impl FnMut(Self::OutputItem) -> Option<Self::OutputItem>,
    ) -> Result<Self::OutputItem, Self::OutputItem> {
        self.try_update(set_order, fetch_order, f)
    }
}
