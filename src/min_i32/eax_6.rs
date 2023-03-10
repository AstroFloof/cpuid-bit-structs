use core::arch::x86_64::__cpuid;
use bit_struct::*;
bit_struct! {
    pub struct L2CacheDetails(u32) {
        line_size: u8,
        _res_0: u4,
        associativity: u4,
        cache_size: u16
    }
}

pub unsafe fn cpuid_l2_cache_details() -> L2CacheDetails {
    return L2CacheDetails::from_unchecked(__cpuid((i32::MIN + 6) as u32).ecx);
}