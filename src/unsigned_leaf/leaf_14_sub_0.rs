#![allow(non_snake_case)]

use core::arch::x86_64::__cpuid_count;
use bit_struct::*;

bit_struct! {
    pub struct PTWrite(u32) {
        _res_a: u4,
        PTWRITE: bool,
        _res_b: u27
    }
}

pub fn cpuid_ptwrite() -> PTWrite {
    unsafe {
        let cpuid = __cpuid_count(0x14, 0);
        PTWrite::from_unchecked(cpuid.eax)
    }
}