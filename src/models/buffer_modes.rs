use crate::traits::buffer_mode_traits::BufferMode;

pub struct SPSC; // Single producer, single consumer
pub struct SPMC; // Single producer, multiple consumer
pub struct MPSC; // Multiple producer, single consumer
pub struct MPMC; // Multiple producer, multiple consumer

impl BufferMode for SPSC {}
impl BufferMode for SPMC {}
impl BufferMode for MPSC {}
impl BufferMode for MPMC {}
