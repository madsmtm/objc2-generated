//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// An MXMetric subclass that encapsulates memory metrics.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metrickit/mxmemorymetric?language=objc)
    #[unsafe(super(MXMetric, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MXMetric")]
    pub struct MXMemoryMetric;
);

#[cfg(feature = "MXMetric")]
extern_conformance!(
    unsafe impl NSCoding for MXMemoryMetric {}
);

#[cfg(feature = "MXMetric")]
extern_conformance!(
    unsafe impl NSObjectProtocol for MXMemoryMetric {}
);

#[cfg(feature = "MXMetric")]
extern_conformance!(
    unsafe impl NSSecureCoding for MXMemoryMetric {}
);

#[cfg(feature = "MXMetric")]
impl MXMemoryMetric {
    extern_methods!(
        /// A single value representing the peak memory consumption of the application.
        ///
        /// Dimensioned as NSUnitInformationStorage.
        #[unsafe(method(peakMemoryUsage))]
        #[unsafe(method_family = none)]
        pub unsafe fn peakMemoryUsage(&self) -> Retained<NSMeasurement<NSUnitInformationStorage>>;

        #[cfg(feature = "MXAverage")]
        /// Average memory of the application upon suspend.
        ///
        /// Dimensioned as NSUnitInformationStorage.
        #[unsafe(method(averageSuspendedMemory))]
        #[unsafe(method_family = none)]
        pub unsafe fn averageSuspendedMemory(
            &self,
        ) -> Retained<MXAverage<NSUnitInformationStorage>>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "MXMetric")]
impl MXMemoryMetric {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
