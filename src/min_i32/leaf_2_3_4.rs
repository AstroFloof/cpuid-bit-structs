use alloc::string::String;
use alloc::vec::Vec;

use core::arch::x86_64::__cpuid;

/// Returns a 48-byte null-terminated ASCII processor brand string.
/// # Safety
/// It is necessary to check whether the feature is present in the CPU by issuing CPUID with
/// EAX = 80000000h first and checking if the returned value is not less than 80000004h.
pub unsafe fn cpuid_processor_brand_string() -> String {
    let mut bytes: Vec<u8> = Vec::with_capacity(48);

    load_into_proc_string(&mut bytes, i32::MIN + 2);
    load_into_proc_string(&mut bytes, i32::MIN + 3);
    load_into_proc_string(&mut bytes, i32::MIN + 4);

    let mut proc_brand_str = String::from_utf8_unchecked(bytes);
    if let Some(first_null) = proc_brand_str.find('\0') {
        proc_brand_str.truncate(first_null)
    }
    return proc_brand_str;
}

#[inline(always)]
unsafe fn load_into_proc_string(string: &mut Vec<u8>, n: i32) {
    let cpuid = __cpuid(n as u32);

    string.extend_from_slice(&cpuid.eax.to_le_bytes());
    string.extend_from_slice(&cpuid.ebx.to_le_bytes());
    string.extend_from_slice(&cpuid.ecx.to_le_bytes());
    string.extend_from_slice(&cpuid.edx.to_le_bytes());
}
