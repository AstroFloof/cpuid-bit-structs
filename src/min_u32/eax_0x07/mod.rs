mod ecx_0;
pub use ecx_0::*;
mod ecx_1;
pub use ecx_1::*;

/// Returns all extended feature flags.
/// # Safety
/// This function can give invalid information if the CPUID instruction for this information is not implemented on the host processor.
/// It is necessary to check to make sure the CPU supports CPUID parameter 0x07 leaf 1 or greater.
pub unsafe fn cpuid_all_extended_feature_flags() -> (
    MoreFeatureFlags1,
    MoreFeatureFlags2,
    MoreFeatureFlags3,
    MoreFeatureFlags4,
    MoreFeatureFlags5,
    MoreFeatureFlags6,
) {
    let (f1, f2, f3) = cpuid_extended_feature_flags_0();
    let (f4, f5, f6) = cpuid_extended_feature_flags_1();
    return (f1, f2, f3, f4, f5, f6);
}
