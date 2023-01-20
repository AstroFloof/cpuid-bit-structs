#![allow(non_snake_case)]

use bit_struct::*;
use core::arch::x86_64::__cpuid;

bit_struct! {
    pub struct KeyLockers(u32) {
        AES_KLE: bool,
        _res_a: u1,
        AES_WIDE_KL: bool,
        _res_b: u1,
        KL_MSRS: bool,
        _res_c: u27
    }
}

pub fn cpuid_key_lockers() -> KeyLockers {
    unsafe {
        let cpuid = __cpuid(0x19);
        KeyLockers::from_unchecked(cpuid.ebx)
    }
}
