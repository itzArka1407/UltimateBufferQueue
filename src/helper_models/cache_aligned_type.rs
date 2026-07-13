// Make a cached type based on the system where the program is compiled

#[cfg_attr(any(target_arch = "x86_64", target_arch = "x86"), repr(align(64)))]
#[cfg_attr(any(target_arch = "aarch64", target_arch = "arm"), repr(align(128)))]
#[cfg_attr(
    not(any(
        target_arch = "aarch64",
        target_arch = "arm",
        target_arch = "x86_64",
        target_arch = "x86"
    )),
    repr(align(64)) // Safe fallback for cache
)]
pub struct CacheAligned<T>(pub T);
