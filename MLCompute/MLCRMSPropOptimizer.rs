//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// The MLCRMSPropOptimizer specifies the RMSProp optimizer.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcrmspropoptimizer?language=objc)
    #[unsafe(super(MLCOptimizer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCOptimizer")]
    #[deprecated = "Use Metal Performance Shaders Graph or BNNS instead."]
    pub struct MLCRMSPropOptimizer;
);

#[cfg(feature = "MLCOptimizer")]
extern_conformance!(
    unsafe impl NSCopying for MLCRMSPropOptimizer {}
);

#[cfg(feature = "MLCOptimizer")]
unsafe impl CopyingHelper for MLCRMSPropOptimizer {
    type Result = Self;
}

#[cfg(feature = "MLCOptimizer")]
extern_conformance!(
    unsafe impl NSObjectProtocol for MLCRMSPropOptimizer {}
);

#[cfg(feature = "MLCOptimizer")]
impl MLCRMSPropOptimizer {
    extern_methods!(
        /// The momentum factor.  A hyper-parameter.
        ///
        /// The default is 0.0.
        #[deprecated = "Use Metal Performance Shaders Graph or BNNS instead."]
        #[unsafe(method(momentumScale))]
        #[unsafe(method_family = none)]
        pub unsafe fn momentumScale(&self) -> c_float;

        /// The smoothing constant.
        ///
        /// The default is 0.99.
        #[deprecated = "Use Metal Performance Shaders Graph or BNNS instead."]
        #[unsafe(method(alpha))]
        #[unsafe(method_family = none)]
        pub unsafe fn alpha(&self) -> c_float;

        /// A term added to improve numerical stability.
        ///
        /// The default is 1e-8.
        #[deprecated = "Use Metal Performance Shaders Graph or BNNS instead."]
        #[unsafe(method(epsilon))]
        #[unsafe(method_family = none)]
        pub unsafe fn epsilon(&self) -> c_float;

        /// If True, compute the centered RMSProp, the gradient is normalized by an estimation of its variance.
        ///
        /// The default is false.
        #[deprecated = "Use Metal Performance Shaders Graph or BNNS instead."]
        #[unsafe(method(isCentered))]
        #[unsafe(method_family = none)]
        pub unsafe fn isCentered(&self) -> bool;

        #[cfg(feature = "MLCOptimizerDescriptor")]
        /// Create a MLCRMSPropOptimizer object with defaults
        ///
        /// Returns: A new MLCRMSPropOptimizer object.
        #[deprecated = "Use Metal Performance Shaders Graph or BNNS instead."]
        #[unsafe(method(optimizerWithDescriptor:))]
        #[unsafe(method_family = none)]
        pub unsafe fn optimizerWithDescriptor(
            optimizer_descriptor: &MLCOptimizerDescriptor,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCOptimizerDescriptor")]
        /// Create a MLCRMSPropOptimizer object
        ///
        /// Parameter `optimizerDescriptor`: The optimizer descriptor object
        ///
        /// Parameter `momentumScale`: The momentum scale
        ///
        /// Parameter `alpha`: The smoothing constant value
        ///
        /// Parameter `epsilon`: The epsilon value to use to improve numerical stability
        ///
        /// Parameter `isCentered`: A boolean to specify whether to compute the centered RMSProp or not
        ///
        /// Returns: A new MLCRMSPropOptimizer object.
        #[deprecated = "Use Metal Performance Shaders Graph or BNNS instead."]
        #[unsafe(method(optimizerWithDescriptor:momentumScale:alpha:epsilon:isCentered:))]
        #[unsafe(method_family = none)]
        pub unsafe fn optimizerWithDescriptor_momentumScale_alpha_epsilon_isCentered(
            optimizer_descriptor: &MLCOptimizerDescriptor,
            momentum_scale: c_float,
            alpha: c_float,
            epsilon: c_float,
            is_centered: bool,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `MLCOptimizer`.
#[cfg(feature = "MLCOptimizer")]
impl MLCRMSPropOptimizer {
    extern_methods!(
        #[deprecated]
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[deprecated]
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}
