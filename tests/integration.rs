use cpuid_bit_structs::*;


// placeholder tests... there isn't an easy way to test because it's dependent on the running cpu lol
#[test]
fn test_name() {
    assert_eq!(cpuid_highest_function_parameter_and_manufacturer_id().1, String::from("GenuineIntel"));
}

#[test]
fn test_stuff() {
    assert!(cpuid_basic_info().2.MMX().get())
}