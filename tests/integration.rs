use cpuid_bs::*;

// placeholder tests... there isn't an easy way to test because it's dependent on the running cpu lol
#[test]
fn test_name() {
    assert_eq!(
        cpuid_version_and_manufacturer_id().1,
        String::from("GenuineIntel")
    );
}

#[test]
fn test_stuff() {
    unsafe { assert!(cpuid_basic_info_flags().2.MMX().get()) }
}
