//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-foundation")]
use objc2_foundation::*;

use crate::*;

#[cfg(feature = "objc2")]
extern_class!(
    /// A light estimate representing the light in the scene.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/arkit/arlightestimate?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2")]
    pub struct ARLightEstimate;
);

#[cfg(feature = "objc2")]
unsafe impl Send for ARLightEstimate {}

#[cfg(feature = "objc2")]
unsafe impl Sync for ARLightEstimate {}

#[cfg(feature = "objc2")]
unsafe impl NSObjectProtocol for ARLightEstimate {}

#[cfg(feature = "objc2")]
extern_methods!(
    #[cfg(feature = "objc2")]
    unsafe impl ARLightEstimate {
        #[cfg(feature = "objc2-core-foundation")]
        /// Ambient intensity of the lighting.
        ///
        ///
        /// In a well lit environment, this value is close to 1000. It typically ranges from 0 (very dark) to around 2000 (very bright).
        #[method(ambientIntensity)]
        pub unsafe fn ambientIntensity(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// The ambient color temperature of the lighting.
        ///
        ///
        /// This specifies the ambient color temperature of the lighting in Kelvin (6500 corresponds to pure white).
        #[method(ambientColorTemperature)]
        pub unsafe fn ambientColorTemperature(&self) -> CGFloat;

        /// Unavailable
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

#[cfg(feature = "objc2")]
extern_class!(
    /// A directional light estimate representing the light intensity and direction in the scene.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/arkit/ardirectionallightestimate?language=objc)
    #[unsafe(super(ARLightEstimate, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2")]
    pub struct ARDirectionalLightEstimate;
);

#[cfg(feature = "objc2")]
unsafe impl Send for ARDirectionalLightEstimate {}

#[cfg(feature = "objc2")]
unsafe impl Sync for ARDirectionalLightEstimate {}

#[cfg(feature = "objc2")]
unsafe impl NSObjectProtocol for ARDirectionalLightEstimate {}

#[cfg(feature = "objc2")]
extern_methods!(
    #[cfg(feature = "objc2")]
    unsafe impl ARDirectionalLightEstimate {
        #[cfg(feature = "objc2-foundation")]
        /// Second degree spherical harmonics coefficients representing the intensity of light.
        ///
        ///
        /// The data is an array of 27 32-bit floating-point values, containing three non-interleaved data sets
        /// corresponding to the red, green, and blue sets of coefficients.
        #[method_id(@__retain_semantics Other sphericalHarmonicsCoefficients)]
        pub unsafe fn sphericalHarmonicsCoefficients(&self) -> Retained<NSData>;

        #[cfg(feature = "objc2-core-foundation")]
        /// The intensity of light in the primary direction.
        #[method(primaryLightIntensity)]
        pub unsafe fn primaryLightIntensity(&self) -> CGFloat;
    }
);

#[cfg(feature = "objc2")]
extern_methods!(
    /// Methods declared on superclass `ARLightEstimate`
    #[cfg(feature = "objc2")]
    unsafe impl ARDirectionalLightEstimate {
        /// Unavailable
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);