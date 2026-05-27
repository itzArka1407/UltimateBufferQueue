// Function to determine the size of flag buffer based on length of buffer(n) and flags size(d)
pub(crate) const fn determine_flag_size(n: usize, d: usize) -> usize {
    (n + d - 1) / d
}

// Function to determine the index(const N) of MarkerType data type that will be used to create the marker type of BufferQueue
// Based on length n, display the index
pub(crate) const fn determine_marker_type_index(n: usize) -> usize {
    if n < u8::MAX as usize {
        0
    } else if n < u16::MAX as usize {
        1
    } else if n < u32::MAX as usize {
        2
    } else {
        3
    }
}
