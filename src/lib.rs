#![no_std]
#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

mod min_u32;
pub use min_u32::*;

mod min_i32;
pub use min_i32::*;

extern crate alloc;
