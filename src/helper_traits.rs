/// This trait determines the type of marker to be used for the buffer queue
pub trait MarkerData {
    /// The type of marker -- implements default(this is used to set the value during
    /// initialization)
    type MarkerType: Default;
}
