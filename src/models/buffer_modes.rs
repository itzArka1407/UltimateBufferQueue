//! Type-level tags that choose the producer/consumer topology of a buffer.
use atomic_waker::AtomicWaker;

use crate::traits::buffer_mode_traits::BufferMode;

/// Single producer, single consumer. (SPSC)
pub struct SPSC;
/// Single producer, multiple consumer.  (SPMC)
pub struct SPMC;
/// Multiple producer, single consumer.  (MPSC)
pub struct MPSC;
/// Multiple producer, multiple consumer. (MPMC)/!
pub struct MPMC;

// Implement the buffer modes
impl BufferMode for SPSC {}
impl BufferMode for SPMC {}
impl BufferMode for MPSC {}
impl BufferMode for MPMC {}
