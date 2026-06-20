use crate::traits::notifier::Notifier;

// Trait to decide the type of buffer: e.g: SPSC, MPSC
pub trait BufferMode {
    type Notify: Notifier;
}
