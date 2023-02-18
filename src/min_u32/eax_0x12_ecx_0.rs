#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use bit_struct::*;
use core::arch::x86_64::__cpuid_count;

bit_struct! {
    pub struct SoftwareGuardExtensionsFlags(u32) {
        SGX1: bool,
        SGX2: bool,
        _res_a: u3,
        ENCLV: bool,
        ENCLS: bool,
        _res_b: u4,
        ENCLU: bool,
        _res_c: u20
    }
}
/// Returns feature flags for SGX functions.
/// # Safety
/// This function can give invalid information if the CPUID instruction for this information is not implemented on the host processor.
/// It is necessary to check to make sure the CPU supports function parameter 0x12 or greater.
pub unsafe fn cpuid_software_guard_extensions_flags() -> SoftwareGuardExtensionsFlags {
    let cpuid = __cpuid_count(0x12, 0);
    return SoftwareGuardExtensionsFlags::from_unchecked(cpuid.eax);
}
