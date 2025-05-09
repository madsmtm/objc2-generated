//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// A loss layer
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlclosslayer?language=objc)
    #[unsafe(super(MLCLayer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCLayer")]
    #[deprecated]
    pub struct MLCLossLayer;
);

#[cfg(feature = "MLCLayer")]
extern_conformance!(
    unsafe impl NSObjectProtocol for MLCLossLayer {}
);

#[cfg(feature = "MLCLayer")]
impl MLCLossLayer {
    extern_methods!(
        #[cfg(feature = "MLCLossDescriptor")]
        /// The loss descriptor
        #[deprecated]
        #[unsafe(method(descriptor))]
        #[unsafe(method_family = none)]
        pub unsafe fn descriptor(&self) -> Retained<MLCLossDescriptor>;

        #[cfg(feature = "MLCTensor")]
        /// The loss label weights tensor
        #[deprecated]
        #[unsafe(method(weights))]
        #[unsafe(method_family = none)]
        pub unsafe fn weights(&self) -> Option<Retained<MLCTensor>>;

        #[cfg(feature = "MLCLossDescriptor")]
        /// Create a loss layer
        ///
        /// Parameter `lossDescriptor`: The loss descriptor
        ///
        /// Returns: A new loss layer.
        #[deprecated]
        #[unsafe(method(layerWithDescriptor:))]
        #[unsafe(method_family = none)]
        pub unsafe fn layerWithDescriptor(loss_descriptor: &MLCLossDescriptor) -> Retained<Self>;

        #[cfg(all(feature = "MLCLossDescriptor", feature = "MLCTensor"))]
        /// Create a MLComputeLoss layer
        ///
        /// Parameter `lossDescriptor`: The loss descriptor
        ///
        /// Parameter `weights`: The loss label weights tensor
        ///
        /// Returns: A new loss layer.
        #[deprecated]
        #[unsafe(method(layerWithDescriptor:weights:))]
        #[unsafe(method_family = none)]
        pub unsafe fn layerWithDescriptor_weights(
            loss_descriptor: &MLCLossDescriptor,
            weights: &MLCTensor,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        /// Create a loss layer
        ///
        /// Parameter `reductionType`: The reduction type to use
        ///
        /// Parameter `labelSmoothing`: Label smoothing value
        ///
        /// Parameter `classCount`: Number of classes
        ///
        /// Parameter `weight`: A scalar floating point value
        ///
        /// Returns: A new softmax cross entropy loss layer.
        #[deprecated]
        #[unsafe(method(softmaxCrossEntropyLossWithReductionType:labelSmoothing:classCount:weight:))]
        #[unsafe(method_family = none)]
        pub unsafe fn softmaxCrossEntropyLossWithReductionType_labelSmoothing_classCount_weight(
            reduction_type: MLCReductionType,
            label_smoothing: c_float,
            class_count: NSUInteger,
            weight: c_float,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        /// Create a loss layer
        ///
        /// Parameter `reductionType`: The reduction type to use
        ///
        /// Parameter `labelSmoothing`: Label smoothing value
        ///
        /// Parameter `classCount`: Number of classes
        ///
        /// Parameter `weights`: The loss label weights tensor
        ///
        /// Returns: A new softmax cross entropy loss layer.
        #[deprecated]
        #[unsafe(method(softmaxCrossEntropyLossWithReductionType:labelSmoothing:classCount:weights:))]
        #[unsafe(method_family = none)]
        pub unsafe fn softmaxCrossEntropyLossWithReductionType_labelSmoothing_classCount_weights(
            reduction_type: MLCReductionType,
            label_smoothing: c_float,
            class_count: NSUInteger,
            weights: Option<&MLCTensor>,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        /// Create a loss layer
        ///
        /// Parameter `reductionType`: The reduction type to use
        ///
        /// Parameter `labelSmoothing`: Label smoothing value
        ///
        /// Parameter `classCount`: Number of classes
        ///
        /// Parameter `weight`: A scalar floating point value
        ///
        /// Returns: A new categorical cross entropy loss layer.
        #[deprecated]
        #[unsafe(method(categoricalCrossEntropyLossWithReductionType:labelSmoothing:classCount:weight:))]
        #[unsafe(method_family = none)]
        pub unsafe fn categoricalCrossEntropyLossWithReductionType_labelSmoothing_classCount_weight(
            reduction_type: MLCReductionType,
            label_smoothing: c_float,
            class_count: NSUInteger,
            weight: c_float,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        /// Create a loss layer
        ///
        /// Parameter `reductionType`: The reduction type to use
        ///
        /// Parameter `labelSmoothing`: Label smoothing value
        ///
        /// Parameter `classCount`: Number of classes
        ///
        /// Parameter `weights`: The loss label weights tensor
        ///
        /// Returns: A new categorical cross entropy loss layer.
        #[deprecated]
        #[unsafe(method(categoricalCrossEntropyLossWithReductionType:labelSmoothing:classCount:weights:))]
        #[unsafe(method_family = none)]
        pub unsafe fn categoricalCrossEntropyLossWithReductionType_labelSmoothing_classCount_weights(
            reduction_type: MLCReductionType,
            label_smoothing: c_float,
            class_count: NSUInteger,
            weights: Option<&MLCTensor>,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        /// Create a loss layer
        ///
        /// Parameter `reductionType`: The reduction type to use
        ///
        /// Parameter `labelSmoothing`: Label smoothing value
        ///
        /// Parameter `weight`: A scalar floating-point value
        ///
        /// Returns: A new sigmoid cross entropy loss layer.
        #[deprecated]
        #[unsafe(method(sigmoidCrossEntropyLossWithReductionType:labelSmoothing:weight:))]
        #[unsafe(method_family = none)]
        pub unsafe fn sigmoidCrossEntropyLossWithReductionType_labelSmoothing_weight(
            reduction_type: MLCReductionType,
            label_smoothing: c_float,
            weight: c_float,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        /// Create a loss layer
        ///
        /// Parameter `reductionType`: The reduction type to use
        ///
        /// Parameter `labelSmoothing`: Label smoothing value
        ///
        /// Parameter `weights`: The loss label weights tensor
        ///
        /// Returns: A new sigmoid cross entropy loss layer.
        #[deprecated]
        #[unsafe(method(sigmoidCrossEntropyLossWithReductionType:labelSmoothing:weights:))]
        #[unsafe(method_family = none)]
        pub unsafe fn sigmoidCrossEntropyLossWithReductionType_labelSmoothing_weights(
            reduction_type: MLCReductionType,
            label_smoothing: c_float,
            weights: Option<&MLCTensor>,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        /// Create a loss layer
        ///
        /// Parameter `reductionType`: The reduction type to use
        ///
        /// Parameter `epsilon`: The epsilon parameter
        ///
        /// Parameter `weight`: A scalar floating-point value
        ///
        /// Returns: A new log loss layer.
        #[deprecated]
        #[unsafe(method(logLossWithReductionType:epsilon:weight:))]
        #[unsafe(method_family = none)]
        pub unsafe fn logLossWithReductionType_epsilon_weight(
            reduction_type: MLCReductionType,
            epsilon: c_float,
            weight: c_float,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        /// Create a loss layer
        ///
        /// Parameter `reductionType`: The reduction type to use
        ///
        /// Parameter `epsilon`: The epsilon parameter
        ///
        /// Parameter `weights`: The loss label weights tensor
        ///
        /// Returns: A new log loss layer.
        #[deprecated]
        #[unsafe(method(logLossWithReductionType:epsilon:weights:))]
        #[unsafe(method_family = none)]
        pub unsafe fn logLossWithReductionType_epsilon_weights(
            reduction_type: MLCReductionType,
            epsilon: c_float,
            weights: Option<&MLCTensor>,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        /// Create a loss layer
        ///
        /// Parameter `reductionType`: The reduction type to use
        ///
        /// Parameter `delta`: The delta parameter
        ///
        /// Parameter `weight`: A scalar floating-point value
        ///
        /// Returns: A new huber loss layer.
        #[deprecated]
        #[unsafe(method(huberLossWithReductionType:delta:weight:))]
        #[unsafe(method_family = none)]
        pub unsafe fn huberLossWithReductionType_delta_weight(
            reduction_type: MLCReductionType,
            delta: c_float,
            weight: c_float,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        /// Create a loss layer
        ///
        /// Parameter `reductionType`: The reduction type to use
        ///
        /// Parameter `delta`: The delta parameter
        ///
        /// Parameter `weights`: The loss label weights tensor
        ///
        /// Returns: A new huber loss layer.
        #[deprecated]
        #[unsafe(method(huberLossWithReductionType:delta:weights:))]
        #[unsafe(method_family = none)]
        pub unsafe fn huberLossWithReductionType_delta_weights(
            reduction_type: MLCReductionType,
            delta: c_float,
            weights: Option<&MLCTensor>,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        /// Create a loss layer
        ///
        /// Parameter `reductionType`: The reduction type to use
        ///
        /// Parameter `weight`: A scalar floating-point value
        ///
        /// Returns: A new L1 i.e. mean absolute error loss layer.
        #[deprecated]
        #[unsafe(method(meanAbsoluteErrorLossWithReductionType:weight:))]
        #[unsafe(method_family = none)]
        pub unsafe fn meanAbsoluteErrorLossWithReductionType_weight(
            reduction_type: MLCReductionType,
            weight: c_float,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        /// Create a loss layer
        ///
        /// Parameter `reductionType`: The reduction type to use
        ///
        /// Parameter `weights`: The loss label weights tensor
        ///
        /// Returns: A new L1 i.e. mean absolute error loss layer.
        #[deprecated]
        #[unsafe(method(meanAbsoluteErrorLossWithReductionType:weights:))]
        #[unsafe(method_family = none)]
        pub unsafe fn meanAbsoluteErrorLossWithReductionType_weights(
            reduction_type: MLCReductionType,
            weights: Option<&MLCTensor>,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        /// Create a loss layer
        ///
        /// Parameter `reductionType`: The reduction type to use
        ///
        /// Parameter `weight`: A scalar floating-point value
        ///
        /// Returns: A new L2 i.e. mean squared error loss layer.
        #[deprecated]
        #[unsafe(method(meanSquaredErrorLossWithReductionType:weight:))]
        #[unsafe(method_family = none)]
        pub unsafe fn meanSquaredErrorLossWithReductionType_weight(
            reduction_type: MLCReductionType,
            weight: c_float,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        /// Create a loss layer
        ///
        /// Parameter `reductionType`: The reduction type to use
        ///
        /// Parameter `weights`: The loss label weights tensor
        ///
        /// Returns: A new L2 i.e. mean squared error loss layer.
        #[deprecated]
        #[unsafe(method(meanSquaredErrorLossWithReductionType:weights:))]
        #[unsafe(method_family = none)]
        pub unsafe fn meanSquaredErrorLossWithReductionType_weights(
            reduction_type: MLCReductionType,
            weights: Option<&MLCTensor>,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        /// Create a loss layer
        ///
        /// Parameter `reductionType`: The reduction type to use
        ///
        /// Parameter `weight`: A scalar floating-point value
        ///
        /// Returns: A new hinge loss layer.
        #[deprecated]
        #[unsafe(method(hingeLossWithReductionType:weight:))]
        #[unsafe(method_family = none)]
        pub unsafe fn hingeLossWithReductionType_weight(
            reduction_type: MLCReductionType,
            weight: c_float,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        /// Create a loss layer
        ///
        /// Parameter `reductionType`: The reduction type to use
        ///
        /// Parameter `weights`: The loss label weights tensor
        ///
        /// Returns: A new hinge loss layer.
        #[deprecated]
        #[unsafe(method(hingeLossWithReductionType:weights:))]
        #[unsafe(method_family = none)]
        pub unsafe fn hingeLossWithReductionType_weights(
            reduction_type: MLCReductionType,
            weights: Option<&MLCTensor>,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        /// Create a loss layer
        ///
        /// Parameter `reductionType`: The reduction type to use
        ///
        /// Parameter `weight`: A scalar floating-point value
        ///
        /// Returns: A new cosine distance loss layer.
        #[deprecated]
        #[unsafe(method(cosineDistanceLossWithReductionType:weight:))]
        #[unsafe(method_family = none)]
        pub unsafe fn cosineDistanceLossWithReductionType_weight(
            reduction_type: MLCReductionType,
            weight: c_float,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MLCTensor", feature = "MLCTypes"))]
        /// Create a loss layer
        ///
        /// Parameter `reductionType`: The reduction type to use
        ///
        /// Parameter `weights`: The loss label weights tensor
        ///
        /// Returns: A new cosine distance loss layer.
        #[deprecated]
        #[unsafe(method(cosineDistanceLossWithReductionType:weights:))]
        #[unsafe(method_family = none)]
        pub unsafe fn cosineDistanceLossWithReductionType_weights(
            reduction_type: MLCReductionType,
            weights: Option<&MLCTensor>,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `MLCLayer`.
#[cfg(feature = "MLCLayer")]
impl MLCLossLayer {
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
