#![allow(non_snake_case)]

use core::arch::x86_64::__cpuid;

/// Obtain the
pub fn cpuid_highest_extended_function_implemented() -> u32 {
    unsafe {
        let cpuid = __cpuid(i32::MIN as u32);
        return cpuid.eax;
    }
}
