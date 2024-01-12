#![no_std]
#![no_main]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
mod board;
pub use board::*;

pub mod accelerometer;
pub mod display;
pub mod mic;
pub mod speaker;
