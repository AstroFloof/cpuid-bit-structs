#![allow(non_snake_case)]

use bit_struct::*;
use core::arch::x86_64::__cpuid_count;

bit_struct! {
    pub struct XSaveFeatures(u32) {
        XSAVEOPT: bool,
        XSAVEC: bool,
        XGETBV_ECX1: bool,
        XSS: bool,
        _res: u28
    }
}

/// This returns XSAVE capability flags
/// # Safety
/// This function can give invalid information if the CPUID instruction for this information is not implemented on the host processor.
/// It is necessary to check to make sure the CPU supports function parameter 0x0d leaf 1 or greater.
pub unsafe fn cpuid_xsave_flags() -> XSaveFeatures {
    let cpuid = __cpuid_count(0xD, 1);
    return XSaveFeatures::from_unchecked(cpuid.eax);
}
