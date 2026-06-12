// Output trait implementations -- common ground for returning data to the caller
use crate::helper_traits::OutputTrait;

impl OutputTrait for u8 {
    #[inline(always)]
    fn to_usize(self) -> usize {
        self as usize
    }

    #[inline(always)]
    fn from_usize(val: usize) -> Self {
        val as Self
    }

    #[inline(always)]
    fn wrapping_increment(self, boundary: usize) -> Self {
        (self + 1) % boundary as Self
    }
}

impl OutputTrait for u16 {
    #[inline(always)]
    fn to_usize(self) -> usize {
        self as usize
    }

    #[inline(always)]
    fn from_usize(val: usize) -> Self {
        val as Self
    }

    #[inline(always)]
    fn wrapping_increment(self, boundary: usize) -> Self {
        (self + 1) % boundary as Self
    }
}

impl OutputTrait for u32 {
    #[inline(always)]
    fn to_usize(self) -> usize {
        self as usize
    }

    #[inline(always)]
    fn from_usize(val: usize) -> Self {
        val as Self
    }

    #[inline(always)]
    fn wrapping_increment(self, boundary: usize) -> Self {
        (self + 1) % boundary as Self
    }
}

impl OutputTrait for u64 {
    #[inline(always)]
    fn to_usize(self) -> usize {
        self as usize
    }

    #[inline(always)]
    fn from_usize(val: usize) -> Self {
        val as Self
    }

    #[inline(always)]
    fn wrapping_increment(self, boundary: usize) -> Self {
        (self + 1) % boundary as Self
    }
}
