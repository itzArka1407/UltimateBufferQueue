/// This trait determines the type of marker to be used for the buffer queue
pub trait MarkerData {
    /// The type of marker -- implements default(this is used to set the value during
    /// initialization)
    type MarkerType: MarkerAtomicOperations + Default;
}

/// Used to define functions for markers to perform operations on them
pub(crate) trait MarkerAtomicOperations {
    type OutputItem;
    fn load_acq(&self) -> Self::OutputItem;
    fn store_rel(&self, val: Self::OutputItem);
    fn fetch_add_acq_rel(&self) {}
}

/// The trait bounds markers and executes MarkerAtomicOperations on its marker type as blanket
/// implementation -- NEEDED to remove cyclic execution
pub(crate) trait MarkerTypeBound:
    MarkerData<MarkerType: MarkerAtomicOperations + Default>
{
}

// Blanket implementation to automcatically bind the trait to all MarkerData implementors
impl<T> MarkerTypeBound for T
where
    T: MarkerData,
    <T as MarkerData>::MarkerType: MarkerAtomicOperations + Default,
{
}
