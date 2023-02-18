#![allow(non_snake_case)]

use bit_struct::*;
use core::arch::x86_64::__cpuid_count;

bit_struct! {
    pub struct MoreFeatureFlags4(u32) {
        _res_a: u3,
        RAO_INT: bool,
        AVX_VNNI: bool,
        AVX512_BF16: bool,
        LASS: bool,
        CMPCCXADD: bool,
        ARCHPERFMONEXT: bool,
        _res_b: u1,
        FAST_ZERO_REP_MOVSB: bool,
        FAST_SHORT_REP_STOSB: bool,
        FAST_SHORT_REP_CMPSB_SCASB: bool,
        _res_c: u4,
        FRED: bool,
        LKGS: bool,
        WRMSRNS: bool,
        _res_d: u1,
        AMX_FP16: bool,
        HRESET: bool,
        AVX_IFMA: bool,
        _res_e: u2,
        LAM: bool,
        MSRLIST: bool,
        _res_f: u4
    }

    pub struct MoreFeatureFlags5(u32) {
        IA32_PPIN: bool,
        _res: u31
    }

    pub struct MoreFeatureFlags6(u32) {
        _res_a: u4,
        AVX_VNNI_INT8: bool,
        AVX_NE_CONVERT: bool,
        _res_b: u8,
        PREFETCHITI: bool,
        _res_c: u3,
        CET_SSS: bool,
        _res_d: u13
    }
}

/// Returns extended feature flags.
/// # Safety
/// This function can give invalid information if the CPUID instruction for this information is not implemented on the host processor.
/// It is necessary to check to make sure the CPU supports CPUID parameter 0x07 leaf 1 or greater.
pub unsafe fn cpuid_extended_feature_flags_1(
) -> (MoreFeatureFlags4, MoreFeatureFlags5, MoreFeatureFlags6) {
    let cpuid = __cpuid_count(7, 1);
    return (
        MoreFeatureFlags4::from_unchecked(cpuid.eax),
        MoreFeatureFlags5::from_unchecked(cpuid.ebx),
        MoreFeatureFlags6::from_unchecked(cpuid.edx),
    );
}
