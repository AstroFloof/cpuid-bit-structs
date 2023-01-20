#![allow(non_snake_case)]

use bit_struct::*;
use core::arch::x86_64::__cpuid;

bit_struct! {

    pub struct ThermalFeatures(u32) {
        DTS: bool,
        Turbo: bool,
        ARAT: bool,
        _r3: u1,
        PLN: bool,
        ECMD: bool,
        PTM: bool
    }

    pub struct InterruptThresholds(u32) {
        thresholds: u4
    }

    pub struct PowerFeatures(u32) {
        HCF: bool,
        ACNT2: bool,
        _r2: u1,
        PEB: bool
    }
}

pub fn cpuid_thermal_and_power_features() -> (ThermalFeatures, InterruptThresholds, PowerFeatures) {
    unsafe {
        let cpuid = __cpuid(6);
        (
            ThermalFeatures::from_unchecked(cpuid.eax),
            InterruptThresholds::from_unchecked(cpuid.ebx),
            PowerFeatures::from_unchecked(cpuid.ecx),
        )
    }
}
