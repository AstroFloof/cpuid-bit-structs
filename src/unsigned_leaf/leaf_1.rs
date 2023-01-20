#![allow(non_snake_case)]

use core::arch::x86_64::__cpuid;
use bit_struct::*;

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

    pub struct Leaf1AdditionalInfo(u32) {
        BrandIndex: u8,
        FlushLineSize: u8,
        MaxAddressableLPIDs: u8,
        LocalAPICID: u8
    }

    pub struct FeatureInfo1(u32) {
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

    pub struct FeatureInfo2(u32) {
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

pub fn cpuid_basic_info() -> (ProcessorVersionInfo, Leaf1AdditionalInfo, FeatureInfo1, FeatureInfo2) {
    unsafe {
        let cpuid = __cpuid(1);
        (
            ProcessorVersionInfo::from_unchecked(cpuid.eax),
            Leaf1AdditionalInfo::from_unchecked(cpuid.ebx),
            FeatureInfo1::from_unchecked(cpuid.ecx),
            FeatureInfo2::from_unchecked(cpuid.edx)
        )
    }
}