//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// The MLCSGDOptimizer specifies a stochastic gradient descent optimizer.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcsgdoptimizer?language=objc)
    #[unsafe(super(MLCOptimizer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCOptimizer")]
    #[deprecated]
    pub struct MLCSGDOptimizer;
);

#[cfg(feature = "MLCOptimizer")]
extern_conformance!(
    unsafe impl NSCopying for MLCSGDOptimizer {}
);

#[cfg(feature = "MLCOptimizer")]
unsafe impl CopyingHelper for MLCSGDOptimizer {
    type Result = Self;
}

#[cfg(feature = "MLCOptimizer")]
extern_conformance!(
    unsafe impl NSObjectProtocol for MLCSGDOptimizer {}
);

#[cfg(feature = "MLCOptimizer")]
impl MLCSGDOptimizer {
    extern_methods!(
        /// The momentum factor.  A hyper-parameter.
        ///
        /// The default is 0.0.
        #[deprecated]
        #[unsafe(method(momentumScale))]
        #[unsafe(method_family = none)]
        pub unsafe fn momentumScale(&self) -> c_float;

        /// A boolean that specifies whether to apply nesterov momentum or not.
        ///
        /// The default is false.
        #[deprecated]
        #[unsafe(method(usesNesterovMomentum))]
        #[unsafe(method_family = none)]
        pub unsafe fn usesNesterovMomentum(&self) -> bool;

        #[cfg(feature = "MLCOptimizerDescriptor")]
        /// Create an MLCSGDOptimizer object with defaults
        ///
        /// Returns: A new MLCSGDOptimizer object.
        #[deprecated]
        #[unsafe(method(optimizerWithDescriptor:))]
        #[unsafe(method_family = none)]
        pub unsafe fn optimizerWithDescriptor(
            optimizer_descriptor: &MLCOptimizerDescriptor,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCOptimizerDescriptor")]
        /// Create an MLCSGDOptimizer object
        ///
        /// Parameter `optimizerDescriptor`: The optimizer descriptor object
        ///
        /// Parameter `momentumScale`: The momentum scale
        ///
        /// Parameter `usesNesterovMomentum`: A boolean to enable / disable nesterov momentum
        ///
        /// Returns: A new MLCSGDOptimizer object.
        #[deprecated]
        #[unsafe(method(optimizerWithDescriptor:momentumScale:usesNesterovMomentum:))]
        #[unsafe(method_family = none)]
        pub unsafe fn optimizerWithDescriptor_momentumScale_usesNesterovMomentum(
            optimizer_descriptor: &MLCOptimizerDescriptor,
            momentum_scale: c_float,
            uses_nesterov_momentum: bool,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `MLCOptimizer`.
#[cfg(feature = "MLCOptimizer")]
impl MLCSGDOptimizer {
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
