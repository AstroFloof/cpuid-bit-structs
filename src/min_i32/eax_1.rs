#![allow(non_snake_case)]

use bit_struct::*;
use core::arch::x86_64::__cpuid;

bit_struct! {

    pub struct AMDFeatureFlags1(u32) {
        FPU: bool,
        VME: bool,
        DE: bool,
        PSE: bool,
        TSC: bool,
        MSR: bool,
        PAE: bool,
        MCE: bool,
        CX8: bool,
        APIC: bool,
        _res_a: u1,
        SYSCALL: bool,
        MTRR: bool,
        PGE: bool,
        MCA: bool,
        CMOV: bool,
        PAT: bool,
        PSE36: bool,
        _res_b: u1,
        MMEXT: bool,
        MMX: bool,
        FXSR: bool,
        FXSR_OPT: bool,
        PDPE1GB: bool,
        RDTSCP: bool,
        _res_c: u1,
        LM: bool,
        AMD3DNOWEXT: bool,
        AMD3DNOW: bool,
    }

    pub struct AMDFeatureFlags2(u32) {
        LAHF_LM: bool,
        CMP_LEGACY: bool,
        SVM: bool,
        EXTAPIC: bool,
        CR8_LEGACY: bool,
        ABM: bool,
        SSE4A: bool,
        MISALIGNSSE: bool,
        AMD3DNOWPREFETCH: bool,
        OSVW: bool,
        IBS: bool,
        XOP: bool,
        SKINIT: bool,
        WDT: bool,
        _res_a: u1,
        LWP: bool,
        FMA4: bool,
        TCE: bool,
        _res_b: u1,
        NODEID_MSR: bool,
        _res_c: u1,
        TBM: bool,
        TOPOEXT: bool,
        PERFCTR_CORE: bool,
        PERFCTR_NB: bool,
        _res_d: u1,
        DBX: bool,
        PERFTSC: bool,
        PCX_L2I: bool,
        MONITORX: bool,
        ADDR_MASK_EXT: bool,
        _res_e: u1
    }
}
/// This returns feature flags specific to AMD processors.
/// # Safety
/// This function can give invalid information if the CPUID instruction for this information is not implemented on the host processor.
/// It is necessary to check to make sure the CPU supports function parameter 0x01 or greater.
pub unsafe fn cpuid_amd_specific_feature_flags() -> (AMDFeatureFlags1, AMDFeatureFlags2) {
    unsafe {
        let cpuid = __cpuid((i32::MIN + 1) as u32);
        return (
            AMDFeatureFlags1::from_unchecked(cpuid.edx),
            AMDFeatureFlags2::from_unchecked(cpuid.ecx),
        );
    }
}
