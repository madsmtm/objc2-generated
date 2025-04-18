//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// An MXMetric subclass that encapsulates CPU metrics.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metrickit/mxcpumetric?language=objc)
    #[unsafe(super(MXMetric, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MXMetric")]
    pub struct MXCPUMetric;
);

#[cfg(feature = "MXMetric")]
extern_conformance!(
    unsafe impl NSCoding for MXCPUMetric {}
);

#[cfg(feature = "MXMetric")]
extern_conformance!(
    unsafe impl NSObjectProtocol for MXCPUMetric {}
);

#[cfg(feature = "MXMetric")]
extern_conformance!(
    unsafe impl NSSecureCoding for MXCPUMetric {}
);

#[cfg(feature = "MXMetric")]
impl MXCPUMetric {
    extern_methods!(
        /// CPU time aggregated cumulatively.
        ///
        /// The data here represents the total CPU time an application consumed over the date range of the containing payload.
        ///
        /// Dimensioned as NSUnitDuration.
        #[unsafe(method(cumulativeCPUTime))]
        #[unsafe(method_family = none)]
        pub unsafe fn cumulativeCPUTime(&self) -> Retained<NSMeasurement<NSUnitDuration>>;

        /// CPU instructions retired aggregated cumulatively.
        ///
        /// The data here represents the total number of CPU instructions an application retired over the date range of the containing payload.
        ///
        /// Dimensionless.
        #[unsafe(method(cumulativeCPUInstructions))]
        #[unsafe(method_family = none)]
        pub unsafe fn cumulativeCPUInstructions(&self) -> Retained<NSMeasurement<NSUnit>>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "MXMetric")]
impl MXCPUMetric {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
