use alloc::string::String;
use alloc::vec::Vec;

use core::arch::x86_64::__cpuid;

/// This returns the highest basic calling parameter (largest value that EAX can be set to before calling CPUID)
/// as well as the CPU's manufacturer ID string – a twelve-character ASCII string.
/// This function is safe because any CPUID-capable processor will give an output.
pub fn cpuid_version_and_manufacturer_id() -> (u32, String) {
    let cpuid = unsafe { __cpuid(0) };
    let mut bytes: Vec<u8> = Vec::with_capacity(12);
    bytes.extend_from_slice(&cpuid.ebx.to_le_bytes());
    bytes.extend_from_slice(&cpuid.edx.to_le_bytes());
    bytes.extend_from_slice(&cpuid.ecx.to_le_bytes());

    return (cpuid.eax, unsafe { String::from_utf8_unchecked(bytes) });
}
