use crate::{
    helper_functions::{determine_flag_size, determine_marker_type_index},
    helper_traits::{MarkerAtomicOperations, MarkerData, OutputTrait},
    models::helper_models::{BitFlip, BufferMarkers, MarkerTypeDecider},
    traits::buffer_mode_traits::BufferMode,
};
use std::{cell::UnsafeCell, marker::PhantomData, mem::MaybeUninit, sync::atomic::Ordering};

// SAFETY: Uses nightly features, stable rust as of May 2026 doesn't support generic const
// evaluations, so this is not possible to do with stable rust
pub struct BufferQueue<T, Mode: BufferMode, const N: usize>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    pub(crate) buf: UnsafeCell<[MaybeUninit<T>; N]>,
    pub(crate) markers: BufferMarkers<
        <MarkerTypeDecider<{ determine_marker_type_index(N) }> as MarkerData>::MarkerType,
        { determine_flag_size(N, 8) },
    >,
    _mode: PhantomData<Mode>,
}

impl<T, M: BufferMode, const N: usize> BufferQueue<T, M, N>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
    #[inline]
    pub fn new() -> Self {
        Self {
            buf: unsafe { UnsafeCell::new(MaybeUninit::uninit().assume_init()) },
            markers: BufferMarkers::new(),
            _mode: PhantomData,
        }
    }

    // The following functions are methods of altering the buffer's internals, not accessible by a
    // user crate -- these functions are used in other places
    #[inline(always)]
    pub(crate) fn _sp_push(&self, val: T) -> bool {
        // TODO: Update the buffer
        self.markers.head.fetch_add(1, Ordering::Release);
        false
    }

    #[inline(always)]
    pub(crate) fn _mp_push(&self, val: T) -> bool {
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
    pub(crate) fn _sc_pop(&self) -> Option<T> {
        let read_slot = self.markers.tail.load(Ordering::Relaxed).to_usize();
        // Empty buffer OR invalidated OR data is being written
        if read_slot == self.markers.head.load(Ordering::Acquire).to_usize()
            || self.markers.invalidated.load(Ordering::Relaxed)
            || !self.markers.is_not_being_written(read_slot)
        {
            return None;
        }
        unsafe {
            let read_ptr = (self.buf.get() as *mut MaybeUninit<T>).add(read_slot);
            let val = std::ptr::read(read_ptr);
            *read_ptr = MaybeUninit::uninit();
            self.markers.tail.store(
                OutputTrait::from_usize((read_slot + 1) % N),
                Ordering::Release,
            );
            Some(val.assume_init())
        }
    }

    #[inline(always)]
    pub(crate) fn _mc_pop(&self) -> Option<T> {
        let old_tail = self
            .markers
            .tail
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |read_slot| {
                // Check if empty buffer OR invalidated already
                if read_slot.to_usize() == self.markers.head.load(Ordering::Acquire).to_usize()
                    || self.markers.invalidated.load(Ordering::Relaxed)
                {
                    return None;
                }
                // Try to increment the tail idx
                Some(read_slot.wrapping_increment(N))
            })
            .ok()?;
        let read_idx = old_tail.to_usize();

        // Until the writing in the slot completes, spin
        while !self.markers.is_not_being_written(read_idx) {
            if self.markers.invalidated.load(Ordering::Relaxed) {
                return None;
            }
            std::hint::spin_loop();
        }

        // Read from the read index and init the value
        // FIXME: RACE Condition -- if a new thread writes onto read_idx(which it can because tail
        // has incremented, and the register flag hasn't been implemeneted yet) before the following lines
        // are executed, there can be a read & write on the same exact memory slot at the same time
        // TOTAL MEMORY CORRUPTION -- Fix the architecture. Might be a complete change.
        unsafe {
            let ptr = (self.buf.get() as *mut MaybeUninit<T>).add(read_idx);
            self.markers
                .update_read_mask(read_idx, Ordering::Relaxed, BitFlip::Register);
            let val = std::ptr::read(ptr);
            *ptr = MaybeUninit::uninit();
            self.markers
                .update_read_mask(read_idx, Ordering::Release, BitFlip::Unregister);
            Some(val.assume_init())
        }
    }

    #[inline(always)]
    pub fn invalidate(&self) {
        self.markers.invalidated.store(true, Ordering::Release);
    }
}
