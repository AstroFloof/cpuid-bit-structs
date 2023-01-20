/// https://en.wikipedia.org/wiki/CPUID#EAX=0:_Highest_Function_Parameter_and_Manufacturer_ID
mod leaf_0;
pub use leaf_0::*;

/// https://en.wikipedia.org/wiki/CPUID#EAX=1:_Processor_Info_and_Feature_Bits
mod leaf_1;
pub use leaf_1::*;

/// https://en.wikipedia.org/wiki/CPUID#EAX=6:_Thermal_and_power_management
mod leaf_6;
pub use leaf_6::*;

/// https://en.wikipedia.org/wiki/CPUID#EAX=7,_ECX=0:_Extended_Features
/// https://en.wikipedia.org/wiki/CPUID#EAX=7,_ECX=1:_Extended_Features
mod leaf7;
pub use leaf7::*;

/// https://en.wikipedia.org/wiki/CPUID#EAX=0Dh,_ECX=1
mod leaf_D_sub1;
pub use leaf_D_sub1::*;

/// https://en.wikipedia.org/wiki/CPUID#EAX=12h,_ECX=0:_SGX_Leaf_Functions
mod leaf_12_sub_0;
pub use leaf_12_sub_0::*;
