//! # buffer-queue
//!
//! A fixed-capacity, lock-free, async-aware channel library supporting all four
//! producer/consumer topologies: **SPSC**, **SPMC**, **MPSC** and **MPMC**.
//!
//! The queue is a ring buffer of `MaybeUninit<T>` slots whose capacity `N` is a
//! const generic. Synchronisation on the data path uses atomics only (no locks).
//! Async wakeups are layered on top: `AtomicWaker` for single-waiter sides and a
//! mutex-guarded `VecDeque<Waker>` for multi-waiter sides. The mutex is touched
//! only when registering or waking, never when pushing or popping.
//!
//! ## Channels
//!
//! | Constructor        | Senders | Receivers | Cloneable handles      |
//! |--------------------|---------|-----------|------------------------|
//! | [`spsc_channel`]   | 1       | 1         | none                   |
//! | [`spmc_channel`]   | 1       | many      | receiver               |
//! | [`mpsc_channel`]   | many    | 1         | sender                 |
//! | [`mpmc_channel`]   | many    | many      | both                   |
//!
//! ## Example
//!
//! ```
//! use buffer_queue::mpsc_channel;
//!
//! let (tx, rx) = mpsc_channel::<u32, 8>();
//!
//! tx.try_send(1).unwrap();
//! tx.try_send(2).unwrap();
//!
//! assert_eq!(rx.try_recv(), Some(1));
//! assert_eq!(rx.try_recv(), Some(2));
//! assert_eq!(rx.try_recv(), None);
//! ```
//!
//! Async, with backpressure (works on any executor):
//!
//! ```ignore
//! let (tx, rx) = buffer_queue::spsc_channel::<u64, 1024>();
//!
//! tokio::spawn(async move {
//!     for i in 0..10_000 {
//!         tx.send(i).await; // suspends while the buffer is full
//!     }
//! });
//!
//! let v = rx.recv().await; // suspends while the buffer is empty
//! ```
//!
//! ## Notes on capacity
//!
//! One slot is kept free to distinguish "full" from "empty", so a channel with
//! `N` slots holds at most `N - 1` items.
//!
//! ## `no_std`
//!
//! The crate is `#![no_std]` and needs `alloc` plus a global allocator.
//!
//! - `std` (default): uses `parking_lot` for the multi-waiter lock and
//!   `std::thread::yield_now` when backing off from long spins.
//! - without `std`: uses `spin::Mutex` and a pure spin-loop backoff.
//!
//! ```toml
//! buffer-queue = { version = "0.1", default-features = false }
//! ```
//!
//! ## Requirements
//!
//! **Nightly Rust is required.** The API shape depends on
//! `#![feature(generic_const_exprs)]`, which is not available on stable.
//!
//! ## Status and limitations
//!
//! This is an early (0.1) release. Please read before using in production:
//!
//! - The lock-free core, including its `unsafe impl Send/Sync`, has **not** been
//!   verified with `loom` or Miri, and has not been heavily stress-tested.
//! - There is no disconnect detection: if all senders (or receivers) are dropped,
//!   the other side's `recv()`/`send()` stays pending forever.
//! - Cancellation safety of `send()` is not yet documented or tested: dropping a
//!   pending send future drops the value it holds.
//! - The sync path (`try_send`/`try_recv`) is currently slower than
//!   `crossbeam_queue::ArrayQueue`, because of the per-slot read/write masks.
//! - Benchmarks in the repository are measured on a single, non-isolated
//!   machine; treat them as directional.
//!
//! ## License
//!
//! Licensed under either of MIT or Apache-2.0, at your option.
#![no_std]
#![feature(generic_const_exprs)]
#![allow(incomplete_features, unused, private_bounds)]

extern crate alloc;

#[cfg(any(test, feature = "std"))]
#[macro_use]
extern crate std;

mod helper_functions;
pub(crate) mod helper_models;
/// Marker-type plumbing used by [`BufferQueue`](models::BufferQueue)'s const-generic bounds.
/// Not intended to be implemented by users.
pub mod helper_traits;
/// Buffer, channel handles, modes and send/receive operations.
pub mod models;
pub(crate) mod traits;

#[cfg(test)]
mod tests;

pub use models::{mpmc_channel, mpsc_channel, spmc_channel, spsc_channel};
