#![no_std]
#![feature(generic_const_exprs)]
#![allow(incomplete_features, unused, private_bounds)]

extern crate alloc;

#[cfg(any(test, feature = "std"))]
#[macro_use]
extern crate std;

mod helper_functions;
pub(crate) mod helper_models;
pub mod helper_traits;
pub mod models;
pub(crate) mod traits;

#[cfg(test)]
mod tests;

pub use models::{mpmc_channel, mpsc_channel, spmc_channel, spsc_channel};
