#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use bit_struct::*;
use core::arch::x86_64::__cpuid_count;

bit_struct! {
    pub struct SGXLeaves_EnclaveFunctions(u32) {
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

pub fn cpuid_sgx_leaves_enclave_functions() -> SGXLeaves_EnclaveFunctions {
    unsafe {
        let cpuid = __cpuid_count(0x12, 0);
        SGXLeaves_EnclaveFunctions::from_unchecked(cpuid.eax)
    }
}
