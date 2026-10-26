//! The operation modes for the buffer
use atomic_waker::AtomicWaker;

use crate::traits::buffer_mode_traits::BufferMode;

/// Single producer, single consumer
pub struct SPSC;
/// Single producer, multiple consumer
pub struct SPMC;
/// Multiple producer, single consumer
pub struct MPSC;
/// Multiple producer, multiple consumer
pub struct MPMC;

// Implement the buffer modes
impl BufferMode for SPSC {}
impl BufferMode for SPMC {}
impl BufferMode for MPSC {}
impl BufferMode for MPMC {}
