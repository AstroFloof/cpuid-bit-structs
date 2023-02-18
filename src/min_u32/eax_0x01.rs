#![allow(non_snake_case)]

use bit_struct::*;
use core::arch::x86_64::__cpuid;

enums! {
    pub ProcessorType { OEM, Overdrive, Dual, Reserved }
}
bit_struct! {

    pub struct ProcessorVersionInfo(u32) {
        SteppingID: u4,
        Model: u4,
        FamilyID: u4,
        Type: ProcessorType,
        _reserved0: u2,
        ExtModelID: u4,
        ExtFamilyID: u8,
        _reserved1: u4
    }

    pub struct AdditionalValues(u32) {
        BrandIndex: u8,
        FlushLineSize: u8,
        MaxAddressableLPIDs: u8,
        LocalAPICID: u8
    }

    pub struct FeatureFlags1(u32) {
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
        _r10: u1,

        SEP: bool,
        MTRR: bool,
        PGE: bool,
        MCA: bool,
        CMOV: bool,
        PAT: bool,
        PSE_36: bool,
        PSN: bool,
        CLFSH: bool,
        _r20: u1,

        DS: bool,
        ACPI: bool,
        MMX: bool,
        FXSR: bool,
        SSE: bool,
        SSE2: bool,
        SS: bool,
        HTT: bool,
        TM: bool,
        IA64: bool,
        PBE: bool
    }

    pub struct FeatureFlags2(u32) {
        SSE3: bool,
        CLMUL: bool,
        DTES64: bool,
        MONITOR: bool,
        DS_CPL: bool,
        VMX: bool,
        SMX: bool,
        EST: bool,
        TM2: bool,
        SSSE3: bool,
        CNXT_ID: bool,
        SDBG: bool,
        FMA: bool,
        CX16: bool,
        XTPR: bool,
        PDCM: bool,
        _r16: u1,

        PCID: bool,
        DCA: bool,
        SSE4_1: bool,
        SSE4_2: bool,
        X2APIC: bool,
        MOVBE: bool,
        POPCNT: bool,
        TSC_DEADLINE: bool,
        AES: bool,
        XSAVE: bool,
        OSXSAVE: bool,
        AVX: bool,
        F16C: bool,
        RDRND: bool,
        HYPERVISOR: bool
    }
}
/// This returns the CPU's stepping, model, and family information
/// (also called the signature of a CPU), feature flags, and additional feature info.
/// # Safety
/// This function can give invalid information if the CPUID instruction for this information is not implemented on the host processor.
/// It is necessary to check to make sure the CPU supports function parameter 0x01 or greater.
pub unsafe fn cpuid_basic_info_flags() -> (
    ProcessorVersionInfo,
    AdditionalValues,
    FeatureFlags1,
    FeatureFlags2,
) {
    let cpuid = __cpuid(1);
    return (
        ProcessorVersionInfo::from_unchecked(cpuid.eax),
        AdditionalValues::from_unchecked(cpuid.ebx),
        FeatureFlags1::from_unchecked(cpuid.ecx),
        FeatureFlags2::from_unchecked(cpuid.edx),
    );
}
