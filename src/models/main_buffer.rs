use crate::{
    helper_functions::{determine_flag_size, determine_marker_type_index},
    helper_traits::{MarkerAtomicOperations, MarkerData},
    models::helper_models::{BitFlip, BufferMarkers, MarkerTypeDecider},
};
use std::{cell::UnsafeCell, mem::MaybeUninit, sync::atomic::Ordering};

// SAFETY: Uses nightly features, stable rust as of May 2026 doesn't support generic const
// evaluations, so this is not possible to do with stable rust
pub struct BufferQueue<T, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) buf: UnsafeCell<[MaybeUninit<T>; N]>,
    pub(crate) markers: BufferMarkers<
        <MarkerTypeDecider<{ determine_marker_type_index(N) }> as MarkerData>::MarkerType,
        { determine_flag_size(N, 8) },
    >,
}

impl<T, const N: usize> BufferQueue<T, N>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    #[inline]
    pub fn new() -> Self {
        Self {
            buf: unsafe { UnsafeCell::new(MaybeUninit::uninit().assume_init()) },
            markers: BufferMarkers::new(),
        }
    }

    // The following functions are methods of altering the buffer's internals, not accessible by a
    // user crate -- these functions are used in other places
    #[inline(always)]
    fn _sp_push(&self, val: T) -> bool {
        // TODO: Update the buffer
        self.markers.head.fetch_add(1, Ordering::Release);
        false
    }

    #[inline(always)]
    fn _mp_push(&self, val: T) -> bool {
        let write_slot = self.markers.head.fetch_add(1, Ordering::Acquire);
        // Register the write state
        self.markers
            .update_read_mask(write_slot, Ordering::Relaxed, BitFlip::Register);
        // TODO: Write into the buffer
        self.markers
            .update_read_mask(write_slot, Ordering::Release, BitFlip::Unregister);
        false
    }

    #[inline(always)]
    fn _sc_pop(&self) -> Option<T> {
        let read_slot = self.markers.tail.load(Ordering::Relaxed);
        // Empty buffer OR invalidated OR data is being written
        if read_slot == self.markers.head.load(Ordering::Acquire)
            || self.markers.invalidated.load(Ordering::Relaxed)
            || !self.markers.is_not_being_written(read_slot)
        {
            return None;
        }
        unsafe {
            let read_ptr = self.buf.get().add(read_slot) as *mut MaybeUninit<T>;
            let val = std::ptr::read(read_ptr);
            *read_ptr = MaybeUninit::uninit();
            self.markers
                .tail
                .store((read_slot + 1) % N, Ordering::Release);
            Some(val.assume_init())
        }
    }

    #[inline(always)]
    fn _mc_pop(&self) -> Option<T> {
        let old_tail = self
            .markers
            .tail
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |val| {
                if val == self.markers.head.load(Ordering::Acquire)
                    || self.markers.invalidated.load(Ordering::Relaxed)
                {
                    return None;
                }
                Some((val + 1) % N)
            })
            .ok()?;
        None
    }

    #[inline(always)]
    pub fn invalidate(&self) {
        self.markers.invalidated.store(true, Ordering::Release);
    }
}
