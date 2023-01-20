#![allow(non_snake_case)]

use core::arch::x86_64::__cpuid_count;
use bit_struct::*;

bit_struct! {
    pub struct ExtraProcessorExtendedStates(u32) {
        XSAVEOPT: bool,
        XSAVEC: bool,
        XGETBV_ECX1: bool,
        XSS: bool,
        _res: u28
    }
}

pub fn cpuid_extra_processor_extended_states() -> ExtraProcessorExtendedStates {
    unsafe {
        let cpuid = __cpuid_count(0xD, 1);
        ExtraProcessorExtendedStates::from_unchecked(cpuid.eax)
    }
}