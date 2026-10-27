use core::sync::atomic::Ordering;

/// Selects the atomic marker type (`AtomicU8`..`AtomicU64`) used for head/tail
/// indices, based on the buffer capacity `N`. Implemented for `MarkerTypeDecider`.
pub trait MarkerData {
    /// Atomic type used for the head/tail markers.
    type MarkerType: MarkerAtomicOperations + Default;
    /// Bit width of the marker type.
    const DENOMINATOR: usize;
}

/// Used to define functions for markers to perform operations on them
pub(crate) trait MarkerAtomicOperations {
    type OutputItem: OutputTrait + Copy;
    fn load(&self, order: Ordering) -> Self::OutputItem;
    fn store(&self, val: Self::OutputItem, order: Ordering);
    fn fetch_add(&self, val: usize, order: Ordering) -> usize;

    fn try_update(
        &self,
        set_order: Ordering,
        fetch_order: Ordering,
        f: impl FnMut(Self::OutputItem) -> Option<Self::OutputItem>,
    ) -> Result<Self::OutputItem, Self::OutputItem>;
}

/// Lossless-by-contract conversion between a marker's integer type and `usize`,
/// plus ring-index increment. Implemented for `u8`, `u16`, `u32`, `u64`.
pub trait OutputTrait {
    /// Widens the value to `usize`.
    fn to_usize(self) -> usize;
    /// Narrows a `usize` index to this type (caller guarantees it fits).
    fn from_usize(val: usize) -> Self;
    /// Returns `(self + 1) % boundary`.
    fn wrapping_increment(self, boundary: usize) -> Self;
}
