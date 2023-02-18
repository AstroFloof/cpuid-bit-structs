/// https://en.wikipedia.org/wiki/CPUID#EAX=0:_Highest_Function_Parameter_and_Manufacturer_ID
mod eax_0x00;
pub use eax_0x00::*;

/// https://en.wikipedia.org/wiki/CPUID#EAX=1:_Processor_Info_and_Feature_Bits
mod eax_0x01;
pub use eax_0x01::*;

/// https://en.wikipedia.org/wiki/CPUID#EAX=6:_Thermal_and_power_management
mod eax_0x06;
pub use eax_0x06::*;

/// https://en.wikipedia.org/wiki/CPUID#EAX=7,_ECX=0:_Extended_Features
/// https://en.wikipedia.org/wiki/CPUID#EAX=7,_ECX=1:_Extended_Features
mod eax_0x07;
pub use eax_0x07::*;

/// https://en.wikipedia.org/wiki/CPUID#EAX=0Dh,_ECX=1
mod eax_0x0d_ecx_1;
pub use eax_0x0d_ecx_1::*;

/// https://en.wikipedia.org/wiki/CPUID#EAX=12h,_ECX=0:_SGX_Leaf_Functions
mod eax_0x12_ecx_0;
pub use eax_0x12_ecx_0::*;

/// https://en.wikipedia.org/wiki/CPUID#EAX=14h,_ECX=0
mod eax_0x14_ecx_0;
pub use eax_0x14_ecx_0::*;

/// https://en.wikipedia.org/wiki/CPUID#EAX=19h
mod eax_0x19;
pub use eax_0x19::*;
