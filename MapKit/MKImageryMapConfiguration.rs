//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkimagerymapconfiguration?language=objc)
    #[unsafe(super(MKMapConfiguration, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MKMapConfiguration")]
    pub struct MKImageryMapConfiguration;
);

#[cfg(feature = "MKMapConfiguration")]
unsafe impl NSCoding for MKImageryMapConfiguration {}

#[cfg(feature = "MKMapConfiguration")]
unsafe impl NSCopying for MKImageryMapConfiguration {}

#[cfg(feature = "MKMapConfiguration")]
unsafe impl CopyingHelper for MKImageryMapConfiguration {
    type Result = Self;
}

#[cfg(feature = "MKMapConfiguration")]
unsafe impl NSObjectProtocol for MKImageryMapConfiguration {}

#[cfg(feature = "MKMapConfiguration")]
unsafe impl NSSecureCoding for MKImageryMapConfiguration {}

extern_methods!(
    #[cfg(feature = "MKMapConfiguration")]
    unsafe impl MKImageryMapConfiguration {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithElevationStyle:)]
        pub unsafe fn initWithElevationStyle(
            this: Allocated<Self>,
            elevation_style: MKMapElevationStyle,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MKMapConfiguration`
    #[cfg(feature = "MKMapConfiguration")]
    unsafe impl MKImageryMapConfiguration {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
