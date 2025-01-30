//! Peripheral access API for ADUCM410 microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.35.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.35.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [aducm-rs](https://github.com/rt4bc/aducm-rs)
//!
//! This crate supports all ADUCM410 devices; for the complete list please
//! see:
//! [aducm410](https://crates.io/crates/aducm410)
//!
//! Due to doc build limitations, not all devices may be shown on docs.rs;
//! a representative few have been selected instead. For a complete list of
//! available registers and fields see: [aducm-rs Device Coverage](https://github.com/rt4bc/aducm-rs)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "aducm410")]
pub mod aducm410;
