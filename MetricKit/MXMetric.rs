//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// An abstract class that describes a specific metric vended by MetricKit.
    ///
    /// All supported metrics are subclasses of MXMetric.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metrickit/mxmetric?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MXMetric;
);

unsafe impl NSCoding for MXMetric {}

unsafe impl NSObjectProtocol for MXMetric {}

unsafe impl NSSecureCoding for MXMetric {}

extern_methods!(
    unsafe impl MXMetric {
        /// Convenience method to return a JSON representation of this metric.
        ///
        /// Returns: An NSData object containing the JSON representation
        #[method_id(@__retain_semantics Other JSONRepresentation)]
        pub unsafe fn JSONRepresentation(&self) -> Retained<NSData>;

        /// Convenience method to return a NSDictionary representation of this metric.
        ///
        /// Returns: An NSDictionary object containing the dictionary representation
        #[deprecated]
        #[method_id(@__retain_semantics Other DictionaryRepresentation)]
        pub unsafe fn DictionaryRepresentation(&self) -> Retained<NSDictionary>;

        /// Convenience method to return a NSDictionary representation of this metric.
        ///
        /// Returns: An NSDictionary object containing the dictionary representation
        #[method_id(@__retain_semantics Other dictionaryRepresentation)]
        pub unsafe fn dictionaryRepresentation(&self) -> Retained<NSDictionary>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MXMetric {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
