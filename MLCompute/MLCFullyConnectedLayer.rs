//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// A fully connected layer a.k.a a dense layer
    ///
    /// For C:input feature channel, C':output feature channel, the layer maps (*,C) --> (*,C') where * can be 1, 2 or 3 dimesnion.
    /// There is an exception for the case of (N,C,1,1) which gets mapped to (N,C',1,1).
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcfullyconnectedlayer?language=objc)
    #[unsafe(super(MLCLayer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCLayer")]
    #[deprecated]
    pub struct MLCFullyConnectedLayer;
);

#[cfg(feature = "MLCLayer")]
extern_conformance!(
    unsafe impl NSObjectProtocol for MLCFullyConnectedLayer {}
);

#[cfg(feature = "MLCLayer")]
impl MLCFullyConnectedLayer {
    extern_methods!(
        #[cfg(feature = "MLCConvolutionDescriptor")]
        /// The convolution descriptor
        #[deprecated]
        #[unsafe(method(descriptor))]
        #[unsafe(method_family = none)]
        pub unsafe fn descriptor(&self) -> Retained<MLCConvolutionDescriptor>;

        #[cfg(feature = "MLCTensor")]
        /// The weights tensor used by the convolution layer
        #[deprecated]
        #[unsafe(method(weights))]
        #[unsafe(method_family = none)]
        pub unsafe fn weights(&self) -> Retained<MLCTensor>;

        #[cfg(feature = "MLCTensor")]
        /// The bias tensor used by the convolution layer
        #[deprecated]
        #[unsafe(method(biases))]
        #[unsafe(method_family = none)]
        pub unsafe fn biases(&self) -> Option<Retained<MLCTensor>>;

        #[cfg(feature = "MLCTensorParameter")]
        /// The weights tensor parameter used for optimizer update
        #[deprecated]
        #[unsafe(method(weightsParameter))]
        #[unsafe(method_family = none)]
        pub unsafe fn weightsParameter(&self) -> Retained<MLCTensorParameter>;

        #[cfg(feature = "MLCTensorParameter")]
        /// The bias tensor parameter used for optimizer update
        #[deprecated]
        #[unsafe(method(biasesParameter))]
        #[unsafe(method_family = none)]
        pub unsafe fn biasesParameter(&self) -> Option<Retained<MLCTensorParameter>>;

        #[cfg(all(feature = "MLCConvolutionDescriptor", feature = "MLCTensor"))]
        /// Create a fully connected layer
        ///
        /// Parameter `weights`: The weights tensor
        ///
        /// Parameter `biases`: The bias tensor
        ///
        /// Parameter `descriptor`: The convolution descriptor
        ///
        /// Returns: A new fully connected layer
        #[deprecated]
        #[unsafe(method(layerWithWeights:biases:descriptor:))]
        #[unsafe(method_family = none)]
        pub unsafe fn layerWithWeights_biases_descriptor(
            weights: &MLCTensor,
            biases: Option<&MLCTensor>,
            descriptor: &MLCConvolutionDescriptor,
        ) -> Option<Retained<Self>>;
    );
}

/// Methods declared on superclass `MLCLayer`.
#[cfg(feature = "MLCLayer")]
impl MLCFullyConnectedLayer {
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
