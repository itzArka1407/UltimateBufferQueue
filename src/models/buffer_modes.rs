use atomic_waker::AtomicWaker;
use tokio::sync::Notify;

use crate::traits::buffer_mode_traits::BufferMode;

pub struct SPSC; // Single producer, single consumer
pub struct SPMC; // Single producer, multiple consumer
pub struct MPSC; // Multiple producer, single consumer
pub struct MPMC; // Multiple producer, multiple consumer

impl BufferMode for SPSC {
    type Notify = AtomicWaker;
}
impl BufferMode for SPMC {
    type Notify = AtomicWaker;
}
impl BufferMode for MPSC {
    type Notify = AtomicWaker;
}
impl BufferMode for MPMC {
    type Notify = Notify;
}
