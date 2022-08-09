#![no_std]
#![no_main]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]
mod board;
pub use board::*;

mod display;
pub use display::*;
