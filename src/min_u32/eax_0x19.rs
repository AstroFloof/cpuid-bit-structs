#![allow(non_snake_case)]

use bit_struct::*;
use core::arch::x86_64::__cpuid;

bit_struct! {
    pub struct KeyLockerFlags(u32) {
        AES_KLE: bool,
        _res_a: u1,
        AES_WIDE_KL: bool,
        _res_b: u1,
        KL_MSRS: bool,
        _res_c: u27
    }
}
/// Returns feature flags for AES KL functions.
/// # Safety
/// This function can give invalid information if the CPUID instruction for this information is not implemented on the host processor.
/// It is necessary to check to make sure the CPU supports function parameter 0x19 or greater.
pub unsafe fn cpuid_key_locker_flags() -> KeyLockerFlags {
    let cpuid = __cpuid(0x19);
    return KeyLockerFlags::from_unchecked(cpuid.ebx);
}
