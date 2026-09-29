//! # libc2
//!
//! libc2 is a library that provides various definitions necessary to interoperate with
//! Super Smash Bros. Ultimate.

pub mod app;
pub mod cpp;
mod lib_impl;
pub mod phx;

pub use lib_impl::lib;
