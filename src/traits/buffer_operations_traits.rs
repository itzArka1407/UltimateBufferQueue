use crate::{
    helper_functions::{determine_flag_size, determine_marker_type_index},
    helper_traits::MarkerData,
    models::{
        BufferQueue,
        buffer_modes::{MPMC, MPSC, SPMC, SPSC},
        helper_models::MarkerTypeDecider,
    },
    traits::buffer_mode_traits::BufferMode,
};

// The type of operation that the queue performs for common operations like push/pop
pub trait BufferOperation<T> {
    fn push(&mut self, val: T) -> Option<T>; // To push a val
    fn pop(&mut self) -> Option<T>; // To pop a val
}

impl<T, const N: usize> BufferOperation<T> for BufferQueue<T, SPSC, N>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    fn push(&mut self, val: T) -> Option<T> {
        self._sp_push(val)
    }

    fn pop(&mut self) -> Option<T> {
        self._sc_pop()
    }
}

impl<T, const N: usize> BufferOperation<T> for BufferQueue<T, SPMC, N>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    fn push(&mut self, val: T) -> Option<T> {
        self._sp_push(val)
    }

    fn pop(&mut self) -> Option<T> {
        self._mc_pop()
    }
}

impl<T, const N: usize> BufferOperation<T> for BufferQueue<T, MPSC, N>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    fn push(&mut self, val: T) -> Option<T> {
        self._mp_push(val)
    }

    fn pop(&mut self) -> Option<T> {
        self._sc_pop()
    }
}

impl<T, const N: usize> BufferOperation<T> for BufferQueue<T, MPMC, N>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    fn push(&mut self, val: T) -> Option<T> {
        self._mp_push(val)
    }

    fn pop(&mut self) -> Option<T> {
        self._mc_pop()
    }
}
