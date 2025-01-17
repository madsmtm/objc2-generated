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
unsafe impl NSCoding for MXCPUMetric {}

#[cfg(feature = "MXMetric")]
unsafe impl NSObjectProtocol for MXCPUMetric {}

#[cfg(feature = "MXMetric")]
unsafe impl NSSecureCoding for MXCPUMetric {}

extern_methods!(
    #[cfg(feature = "MXMetric")]
    unsafe impl MXCPUMetric {
        /// CPU time aggregated cumulatively.
        ///
        /// The data here represents the total CPU time an application consumed over the date range of the containing payload.
        ///
        /// Dimensioned as NSUnitDuration.
        #[unsafe(method_family(none))]
        #[method_id(cumulativeCPUTime)]
        pub unsafe fn cumulativeCPUTime(&self) -> Retained<NSMeasurement<NSUnitDuration>>;

        /// CPU instructions retired aggregated cumulatively.
        ///
        /// The data here represents the total number of CPU instructions an application retired over the date range of the containing payload.
        ///
        /// Dimensionless.
        #[unsafe(method_family(none))]
        #[method_id(cumulativeCPUInstructions)]
        pub unsafe fn cumulativeCPUInstructions(&self) -> Retained<NSMeasurement<NSUnit>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MXMetric")]
    unsafe impl MXCPUMetric {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
