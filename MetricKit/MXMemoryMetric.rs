//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MXMetric")]
    pub struct MXMemoryMetric;

    #[cfg(feature = "MXMetric")]
    unsafe impl ClassType for MXMemoryMetric {
        #[inherits(NSObject)]
        type Super = MXMetric;
    }
);

#[cfg(feature = "MXMetric")]
unsafe impl NSCoding for MXMemoryMetric {}

#[cfg(feature = "MXMetric")]
unsafe impl NSObjectProtocol for MXMemoryMetric {}

#[cfg(feature = "MXMetric")]
unsafe impl NSSecureCoding for MXMemoryMetric {}

extern_methods!(
    #[cfg(feature = "MXMetric")]
    unsafe impl MXMemoryMetric {
        #[method_id(@__retain_semantics Other peakMemoryUsage)]
        pub unsafe fn peakMemoryUsage(&self) -> Retained<NSMeasurement<NSUnitInformationStorage>>;

        #[cfg(feature = "MXAverage")]
        #[method_id(@__retain_semantics Other averageSuspendedMemory)]
        pub unsafe fn averageSuspendedMemory(
            &self,
        ) -> Retained<MXAverage<NSUnitInformationStorage>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MXMetric")]
    unsafe impl MXMemoryMetric {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
