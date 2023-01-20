use alloc::string::String;
use alloc::vec::Vec;

use core::arch::x86_64::__cpuid;

pub fn cpuid_highest_function_implemented_and_manufacturer_id() -> (u32, String) {
    let cpuid = unsafe { __cpuid(0) };
    let mut bytes: Vec<u8> = Vec::with_capacity(12);
    bytes.extend_from_slice(&cpuid.ebx.to_le_bytes());
    bytes.extend_from_slice(&cpuid.edx.to_le_bytes());
    bytes.extend_from_slice(&cpuid.ecx.to_le_bytes());
    (cpuid.eax, String::from_utf8(bytes).unwrap())
}
