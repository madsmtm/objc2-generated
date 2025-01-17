//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// The MLCOptimizerDescriptor specifies an optimizer descriptor.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcoptimizerdescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct MLCOptimizerDescriptor;
);

unsafe impl NSCopying for MLCOptimizerDescriptor {}

unsafe impl CopyingHelper for MLCOptimizerDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MLCOptimizerDescriptor {}

extern_methods!(
    unsafe impl MLCOptimizerDescriptor {
        /// The learning rate
        #[deprecated]
        #[method(learningRate)]
        pub unsafe fn learningRate(&self) -> c_float;

        /// The rescale value applied to gradients during optimizer update
        #[deprecated]
        #[method(gradientRescale)]
        pub unsafe fn gradientRescale(&self) -> c_float;

        /// Whether gradient clipping should be applied or not.
        ///
        /// The default is false
        #[deprecated]
        #[method(appliesGradientClipping)]
        pub unsafe fn appliesGradientClipping(&self) -> bool;

        /// The maximum gradient value if gradient clipping is enabled before gradient is rescaled.
        #[deprecated]
        #[method(gradientClipMax)]
        pub unsafe fn gradientClipMax(&self) -> c_float;

        /// The minimum gradient value if gradient clipping is enabled before gradient is rescaled.
        #[deprecated]
        #[method(gradientClipMin)]
        pub unsafe fn gradientClipMin(&self) -> c_float;

        /// The regularization scale.
        #[deprecated]
        #[method(regularizationScale)]
        pub unsafe fn regularizationScale(&self) -> c_float;

        #[cfg(feature = "MLCTypes")]
        /// The regularization type.
        #[deprecated]
        #[method(regularizationType)]
        pub unsafe fn regularizationType(&self) -> MLCRegularizationType;

        #[cfg(feature = "MLCTypes")]
        /// The type of clipping applied to gradient
        #[method(gradientClippingType)]
        pub unsafe fn gradientClippingType(&self) -> MLCGradientClippingType;

        /// The maximum clipping value
        #[method(maximumClippingNorm)]
        pub unsafe fn maximumClippingNorm(&self) -> c_float;

        /// Used only with MLCGradientClippingTypeByGlobalNorm. If non zero, this norm will be used in place of global norm.
        #[method(customGlobalNorm)]
        pub unsafe fn customGlobalNorm(&self) -> c_float;

        #[cfg(feature = "MLCTypes")]
        /// Create a MLCOptimizerDescriptor object
        ///
        /// Parameter `learningRate`: The learning rate
        ///
        /// Parameter `gradientRescale`: The gradient rescale value
        ///
        /// Parameter `regularizationType`: The regularization type
        ///
        /// Parameter `regularizationScale`: The regularization scale
        ///
        /// Returns: A new MLCOptimizerDescriptor object.
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(descriptorWithLearningRate:gradientRescale:regularizationType:regularizationScale:)]
        pub unsafe fn descriptorWithLearningRate_gradientRescale_regularizationType_regularizationScale(
            learning_rate: c_float,
            gradient_rescale: c_float,
            regularization_type: MLCRegularizationType,
            regularization_scale: c_float,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        /// Create a MLCOptimizerDescriptor object
        ///
        /// Parameter `learningRate`: The learning rate
        ///
        /// Parameter `gradientRescale`: The gradient rescale value
        ///
        /// Parameter `appliesGradientClipping`: Whether to apply gradient clipping
        ///
        /// Parameter `gradientClipMax`: The maximum gradient value to be used with gradient clipping
        ///
        /// Parameter `gradientClipMin`: The minimum gradient value to be used with gradient clipping
        ///
        /// Parameter `regularizationType`: The regularization type
        ///
        /// Parameter `regularizationScale`: The regularization scale
        ///
        /// Returns: A new MLCOptimizerDescriptor object.
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(descriptorWithLearningRate:gradientRescale:appliesGradientClipping:gradientClipMax:gradientClipMin:regularizationType:regularizationScale:)]
        pub unsafe fn descriptorWithLearningRate_gradientRescale_appliesGradientClipping_gradientClipMax_gradientClipMin_regularizationType_regularizationScale(
            learning_rate: c_float,
            gradient_rescale: c_float,
            applies_gradient_clipping: bool,
            gradient_clip_max: c_float,
            gradient_clip_min: c_float,
            regularization_type: MLCRegularizationType,
            regularization_scale: c_float,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        /// Create an MLCOptimizerDescriptor object
        ///
        /// Parameter `learningRate`: The learning rate
        ///
        /// Parameter `gradientRescale`: The gradient rescale value
        ///
        /// Parameter `appliesGradientClipping`: Whether to apply gradient clipping
        ///
        /// Parameter `gradientClippingType`: The type of clipping applied to gradients
        ///
        /// Parameter `gradientClipMax`: The maximum gradient value to be used with gradient clipping
        ///
        /// Parameter `gradientClipMin`: The minimum gradient value to be used with gradient clipping
        ///
        /// Parameter `maximumClippingNorm`: The maximum norm to be used with gradient clipping
        ///
        /// Parameter `customGlobalNorm`: If non-zero, the norm to be used instead of calculating the global norm
        ///
        /// Parameter `regularizationType`: The regularization type
        ///
        /// Parameter `regularizationScale`: The regularization scale
        ///
        /// Returns: A new MLCOptimizerDescriptor object.
        #[unsafe(method_family(none))]
        #[method_id(descriptorWithLearningRate:gradientRescale:appliesGradientClipping:gradientClippingType:gradientClipMax:gradientClipMin:maximumClippingNorm:customGlobalNorm:regularizationType:regularizationScale:)]
        pub unsafe fn descriptorWithLearningRate_gradientRescale_appliesGradientClipping_gradientClippingType_gradientClipMax_gradientClipMin_maximumClippingNorm_customGlobalNorm_regularizationType_regularizationScale(
            learning_rate: c_float,
            gradient_rescale: c_float,
            applies_gradient_clipping: bool,
            gradient_clipping_type: MLCGradientClippingType,
            gradient_clip_max: c_float,
            gradient_clip_min: c_float,
            maximum_clipping_norm: c_float,
            custom_global_norm: c_float,
            regularization_type: MLCRegularizationType,
            regularization_scale: c_float,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MLCOptimizerDescriptor {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
