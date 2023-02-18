#![allow(non_snake_case)]

use bit_struct::*;
use core::arch::x86_64::__cpuid_count;

bit_struct! {
    pub struct PTWriteFlag(u32) {
        _res_a: u4,
        PTWRITE: bool,
        _res_b: u27
    }
}

/// Returns a feature flag for whether the PTWRITE instruction is implemented.
/// # Safety
/// This function can give invalid information if the CPUID instruction for this information is not implemented on the host processor.
/// It is necessary to check to make sure the CPU supports function parameter 0x14 or greater.
pub unsafe fn cpuid_ptwrite() -> PTWriteFlag {
    let cpuid = __cpuid_count(0x14, 0);
    return PTWriteFlag::from_unchecked(cpuid.eax);
}
