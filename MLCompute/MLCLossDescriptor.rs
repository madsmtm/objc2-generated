//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// The MLCLossDescriptor specifies a loss filter descriptor.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlclossdescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct MLCLossDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MLCLossDescriptor {}
);

unsafe impl CopyingHelper for MLCLossDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MLCLossDescriptor {}
);

impl MLCLossDescriptor {
    extern_methods!(
        #[cfg(feature = "MLCTypes")]
        /// Specifies the loss function.
        #[deprecated]
        #[unsafe(method(lossType))]
        #[unsafe(method_family = none)]
        pub unsafe fn lossType(&self) -> MLCLossType;

        #[cfg(feature = "MLCTypes")]
        /// The reduction operation performed by the loss function.
        #[deprecated]
        #[unsafe(method(reductionType))]
        #[unsafe(method_family = none)]
        pub unsafe fn reductionType(&self) -> MLCReductionType;

        /// The scale factor to apply to each element of a result.  The default value is 1.0.
        #[deprecated]
        #[unsafe(method(weight))]
        #[unsafe(method_family = none)]
        pub unsafe fn weight(&self) -> c_float;

        /// The label smoothing parameter. The default value is 0.0.
        ///
        /// This parameter is valid only for the loss functions of the following type(s):
        /// MLCLossTypeSoftmaxCrossEntropy and MLCLossTypeSigmoidCrossEntropy.
        #[deprecated]
        #[unsafe(method(labelSmoothing))]
        #[unsafe(method_family = none)]
        pub unsafe fn labelSmoothing(&self) -> c_float;

        /// The number of classes parameter. The default value is 1.
        ///
        /// This parameter is valid only for the loss function MLCLossTypeSoftmaxCrossEntropy.
        #[deprecated]
        #[unsafe(method(classCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn classCount(&self) -> NSUInteger;

        /// The epsilon parameter. The default value is 1e-7.
        ///
        /// This parameter is valid only for the loss function MLCLossTypeLog.
        #[deprecated]
        #[unsafe(method(epsilon))]
        #[unsafe(method_family = none)]
        pub unsafe fn epsilon(&self) -> c_float;

        /// The delta parameter. The default value is 1.0f.
        ///
        /// This parameter is valid only for the loss function MLCLossTypeHuber.
        #[deprecated]
        #[unsafe(method(delta))]
        #[unsafe(method_family = none)]
        pub unsafe fn delta(&self) -> c_float;

        #[deprecated]
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[deprecated]
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        /// Create a loss descriptor object
        ///
        /// Parameter `lossType`: The loss function.
        ///
        /// Parameter `reductionType`: The reduction operation
        ///
        /// Returns: A new MLCLossDescriptor object
        #[deprecated]
        #[unsafe(method(descriptorWithType:reductionType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn descriptorWithType_reductionType(
            loss_type: MLCLossType,
            reduction_type: MLCReductionType,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        /// Create a loss descriptor object
        ///
        /// Parameter `lossType`: The loss function.
        ///
        /// Parameter `reductionType`: The reduction operation
        ///
        /// Parameter `weight`: The scale factor to apply to each element of a result.
        ///
        /// Returns: A new MLCLossDescriptor object
        #[deprecated]
        #[unsafe(method(descriptorWithType:reductionType:weight:))]
        #[unsafe(method_family = none)]
        pub unsafe fn descriptorWithType_reductionType_weight(
            loss_type: MLCLossType,
            reduction_type: MLCReductionType,
            weight: c_float,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        /// Create a loss descriptor object
        ///
        /// Parameter `lossType`: The loss function.
        ///
        /// Parameter `reductionType`: The reduction operation
        ///
        /// Parameter `weight`: The scale factor to apply to each element of a result.
        ///
        /// Parameter `labelSmoothing`: The label smoothing parameter.
        ///
        /// Parameter `classCount`: The number of classes parameter.
        ///
        /// Returns: A new MLCLossDescriptor object
        #[deprecated]
        #[unsafe(method(descriptorWithType:reductionType:weight:labelSmoothing:classCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn descriptorWithType_reductionType_weight_labelSmoothing_classCount(
            loss_type: MLCLossType,
            reduction_type: MLCReductionType,
            weight: c_float,
            label_smoothing: c_float,
            class_count: NSUInteger,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        /// Create a loss descriptor object
        ///
        /// Parameter `lossType`: The loss function.
        ///
        /// Parameter `reductionType`: The reduction operation
        ///
        /// Parameter `weight`: The scale factor to apply to each element of a result.
        ///
        /// Parameter `labelSmoothing`: The label smoothing parameter.
        ///
        /// Parameter `classCount`: The number of classes parameter.
        ///
        /// Parameter `epsilon`: The epsilon used by LogLoss
        ///
        /// Parameter `delta`: The delta parameter used by Huber loss
        ///
        /// Returns: A new MLCLossDescriptor object
        #[deprecated]
        #[unsafe(method(descriptorWithType:reductionType:weight:labelSmoothing:classCount:epsilon:delta:))]
        #[unsafe(method_family = none)]
        pub unsafe fn descriptorWithType_reductionType_weight_labelSmoothing_classCount_epsilon_delta(
            loss_type: MLCLossType,
            reduction_type: MLCReductionType,
            weight: c_float,
            label_smoothing: c_float,
            class_count: NSUInteger,
            epsilon: c_float,
            delta: c_float,
        ) -> Retained<Self>;
    );
}
