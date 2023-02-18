#![allow(non_snake_case)]

use bit_struct::*;
use core::arch::x86_64::__cpuid;

bit_struct! {

    pub struct ThermalFeatureFlags(u32) {
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

    pub struct PowerFeatureFlags(u32) {
        HCF: bool,
        ACNT2: bool,
        _r2: u1,
        PEB: bool
    }
}
/// Returns flags and values for various power and thermal features.
/// # Safety
/// This function can give invalid information if the CPUID instruction for this information is not implemented on the host processor.
/// It is necessary to check to make sure the CPU supports function parameter 0x06 or greater.
pub unsafe fn cpuid_thermal_and_power_features_flags(
) -> (ThermalFeatureFlags, InterruptThresholds, PowerFeatureFlags) {
    let cpuid = __cpuid(6);
    return (
        ThermalFeatureFlags::from_unchecked(cpuid.eax),
        InterruptThresholds::from_unchecked(cpuid.ebx),
        PowerFeatureFlags::from_unchecked(cpuid.ecx),
    );
}
