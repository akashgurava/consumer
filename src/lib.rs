#![warn(
    clippy::all,
    clippy::perf,
    clippy::correctness,
    clippy::style,
    // clippy::restriction,
    // clippy::pedantic,
    // clippy::nursery,
    // clippy::cargo
)]

#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate thiserror;

// #[cfg(not(any(feature = "hyper-client", feature = "isahc-client",)))]
// compile_error!("One of the features ['hyper-client', 'isahc-client'] must be enabled");

mod client;
mod error;

pub use client::*;
pub use error::ConsumerError;
