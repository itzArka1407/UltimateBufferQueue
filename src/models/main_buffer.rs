use crate::{
    helper_functions::{determine_flag_size, determine_marker_type_index},
    helper_traits::{MarkerAtomicOperations, MarkerData, OutputTrait},
    models::helper_models::{BitFlip, BufferMarkers, MarkerTypeDecider},
    traits::buffer_mode_traits::BufferMode,
};
use std::{
    cell::UnsafeCell,
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
    sync::atomic::Ordering,
};

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

unsafe impl<T: Send, Mode: BufferMode, const N: usize> Sync for BufferQueue<T, Mode, N>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
}
unsafe impl<T: Send, Mode: BufferMode, const N: usize> Send for BufferQueue<T, Mode, N>
where
    [(); determine_flag_size(N, 8)]: Sized,
    MarkerTypeDecider<{ determine_marker_type_index(N) }>: MarkerData,
{
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

    // For pushes -- Some(val) -> when push fails, the val is returned, None -> push success
    // For pops -- Some(val) -> pop success, val is returned, None -> pop failed

    #[inline(always)]
    pub(crate) fn _sp_push(&self, val: T) -> Option<T> {
        // let val = ManuallyDrop::new(val);
        let write_slot = self.markers.head.0.load(Ordering::Relaxed).to_usize();
        let next = (write_slot + 1) % N;

        // Buffer full | invalidated | read going on at that index
        if next == self.markers.tail.0.load(Ordering::Acquire).to_usize()
            || self.markers.invalidated.load(Ordering::Relaxed)
            || !self.markers.is_not_being_read(write_slot)
        {
            return Some(val);
        }

        self.markers
            .update_write_mask(write_slot, Ordering::Relaxed, BitFlip::Register);

        unsafe {
            let write_ptr = (self.buf.get() as *mut MaybeUninit<T>).add(write_slot);
            std::ptr::write(write_ptr, MaybeUninit::new(val));
            self.markers
                .update_write_mask(write_slot, Ordering::Release, BitFlip::Unregister);
            self.markers
                .head
                .0
                .store(OutputTrait::from_usize(next), Ordering::Release);
        }
        None
    }

    #[inline(always)]
    pub(crate) fn _mp_push(&self, val: T) -> Option<T> {
        // let val = ManuallyDrop::new(val);

        let old_head = match self.markers.head.0.try_update(
            Ordering::AcqRel,
            Ordering::Acquire,
            |write_slot| {
                let next = write_slot.wrapping_increment(N);
                if next.to_usize() == self.markers.tail.0.load(Ordering::Acquire).to_usize()
                    || self.markers.invalidated.load(Ordering::Relaxed)
                {
                    return None;
                }
                Some(next)
            },
        ) {
            Ok(head) => head,
            Err(_) => return Some(val),
        };

        let write_idx = old_head.to_usize();

        let mut spins = 0u8;
        loop {
            if self.markers.invalidated.load(Ordering::Relaxed) {
                return Some(val);
            }
            if self.markers.is_not_being_read(write_idx) {
                break;
            }

            // For short waits, spin the loop continuously(needed for hot-paths)
            if spins < 32 {
                std::hint::spin_loop();
                spins += 1;
            } else {
                // Waited for long, yield the thread
                std::thread::yield_now();
                spins = 0;
            }
        }

        self.markers
            .update_write_mask(write_idx, Ordering::Relaxed, BitFlip::Register);

        unsafe {
            let write_ptr = (self.buf.get() as *mut MaybeUninit<T>).add(write_idx);
            std::ptr::write(write_ptr, MaybeUninit::new(val));
            self.markers
                .update_write_mask(write_idx, Ordering::Release, BitFlip::Unregister);
        }
        None
    }

    #[inline(always)]
    pub(crate) fn _sc_pop(&self) -> Option<T> {
        let read_slot = self.markers.tail.0.load(Ordering::Relaxed).to_usize();

        if read_slot == self.markers.head.0.load(Ordering::Acquire).to_usize()
            || self.markers.invalidated.load(Ordering::Relaxed)
            || !self.markers.is_not_being_written(read_slot)
        {
            return None;
        }

        // Claim slot before pusher can wrap around into it
        self.markers
            .update_read_mask(read_slot, Ordering::Release, BitFlip::Register);

        unsafe {
            let read_ptr = (self.buf.get() as *mut MaybeUninit<T>).add(read_slot);
            let val = std::ptr::read(read_ptr);
            *read_ptr = MaybeUninit::uninit();
            self.markers
                .update_read_mask(read_slot, Ordering::Release, BitFlip::Unregister);
            self.markers.tail.0.store(
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
            .0
            .try_update(Ordering::AcqRel, Ordering::Acquire, |read_slot| {
                if read_slot.to_usize() == self.markers.head.0.load(Ordering::Acquire).to_usize()
                    || self.markers.invalidated.load(Ordering::Relaxed)
                {
                    return None;
                }
                Some(read_slot.wrapping_increment(N))
            })
            .ok()?;

        let read_idx = old_tail.to_usize();

        // Wait for the pusher to finish pushing
        let mut spins = 0u8;
        loop {
            if self.markers.invalidated.load(Ordering::Relaxed)
                || self.markers.is_not_being_written(read_idx)
            {
                break;
            }

            // For short waits, spin the loop continuously(needed for hot-paths)
            if spins < 32 {
                std::hint::spin_loop();
                spins += 1;
            } else {
                // Waited for long, yield the thread
                std::thread::yield_now();
                spins = 0;
            }
        }

        // Phase 2: claim slot for reading, then read
        self.markers
            .update_read_mask(read_idx, Ordering::Release, BitFlip::Register);

        unsafe {
            let ptr = (self.buf.get() as *mut MaybeUninit<T>).add(read_idx);
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
