#![allow(non_snake_case)]

use bit_struct::*;
use core::arch::x86_64::__cpuid_count;

bit_struct! {
    pub struct MoreFeatureFlags1(u32) {
        FSGSBASE: bool,
        IA32_TSC_ADJUST_MSR: bool,
        SGX: bool,
        BMI1: bool,
        HLE: bool,
        AVX2: bool,
        FDP_EXCPIN_ONLY: bool,
        SMEP: bool,
        BMI2: bool,
        ERMS: bool,
        INVPCID: bool,
        RTM: bool,
        RDTM_PQM: bool,
        FPU_CS_DS_DEPRECATED: bool,
        MPX: bool,
        RDTA_PQE: bool,
        AVX512_F: bool,
        AVX512_DQ: bool,
        RDSEED: bool,
        ADX: bool,
        SMAP: bool,
        AVX512_IFMA: bool,
        _r22: u1,
        CLFLUSHOPT: bool,
        CLWB: bool,
        PT: bool,
        AVX512_PF: bool,
        AVX512_CD: bool,
        SHA: bool,
        AVX512_BW: bool,
        AVX512_VL: bool,
    }

    pub struct MoreFeatureFlags2(u32) {
        PREFETCTHQT1: bool,
        AVX512_VBM1: bool,
        UMIP: bool,
        PKU: bool,
        USPKE: bool,
        WAITPKG: bool,
        AVX512_VBMI2: bool,
        CET_SS: bool,
        GFNI: bool,
        VAES: bool,
        VCLMUL: bool,
        AVX512_VNNI: bool,
        AVX512_BITALG: bool,
        TME: bool,
        AVX512_VPOPCNTDQ: bool,
        _r15: u1,
        LA57: bool,
        MAWAU: u5,
        RDPID: bool,
        KL: bool,
        BUS_LOCK_DETECT: bool,
        CLDEMOTE: bool,
        _r26: u1,
        MOVDIRI: bool,
        MIVDIR64B: bool,
        SGX_LC: bool,
        PKS: bool,
    }

    pub struct MoreFeatureFlags3(u32) {
        SGX_KEYS: bool,
        _r1: u1,
        AVX512_4VNNIW: bool,
        AVX512_4FMAPS: bool,
        FSRM: bool,
        UINTR: bool,
        _r5_6: u2,
        AVX512_VP2INTERSECT: bool,
        SRDBS_CTRL: bool,
        MC_CLEAR: bool,
        RTM_ALWAYS_ABORT: bool,
        _r12: u1,
        PCONFIG: bool,
        LBR: bool,
        CET_IBT: bool,
        _r21: u1,
        AMX_BF16: bool,
        AVX512_FP16: bool,
        AMX_TILE: bool,
        AMX_INT8: bool,
        SPEC_CTRL: bool,
        STIBP: bool,
        L1D_FLUSH: bool,
        IA32_ARCH_CAPABILITIES: bool,
        IA32_CORE_CAPABILITIES: bool,
        SSBD: bool,
    }
}

/// Returns extended feature flags.
/// # Safety
/// This function can give invalid information if the CPUID instruction for this information is not implemented on the host processor.
/// It is necessary to check to make sure the CPU supports CPUID parameter 0x07 leaf 0 or greater.
pub unsafe fn cpuid_extended_feature_flags_0(
) -> (MoreFeatureFlags1, MoreFeatureFlags2, MoreFeatureFlags3) {
    let cpuid = __cpuid_count(7, 0);
    return (
        MoreFeatureFlags1::from_unchecked(cpuid.ebx),
        MoreFeatureFlags2::from_unchecked(cpuid.ecx),
        MoreFeatureFlags3::from_unchecked(cpuid.edx),
    );
}
