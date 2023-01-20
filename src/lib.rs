#![no_std]
extern crate alloc;

/// https://en.wikipedia.org/wiki/CPUID#EAX=0:_Highest_Function_Parameter_and_Manufacturer_ID
mod leaf0;
pub use leaf0::*;
mod leaf1;
pub use leaf1::*;
mod leaf6;
pub use leaf6::*;
mod leaf7;
pub use leaf7::*;
