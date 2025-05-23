//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// A batch normalizaion layer
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcbatchnormalizationlayer?language=objc)
    #[unsafe(super(MLCLayer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCLayer")]
    #[deprecated]
    pub struct MLCBatchNormalizationLayer;
);

#[cfg(feature = "MLCLayer")]
extern_conformance!(
    unsafe impl NSObjectProtocol for MLCBatchNormalizationLayer {}
);

#[cfg(feature = "MLCLayer")]
impl MLCBatchNormalizationLayer {
    extern_methods!(
        /// The number of feature channels
        #[deprecated]
        #[unsafe(method(featureChannelCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn featureChannelCount(&self) -> NSUInteger;

        #[cfg(feature = "MLCTensor")]
        /// The mean tensor
        #[deprecated]
        #[unsafe(method(mean))]
        #[unsafe(method_family = none)]
        pub unsafe fn mean(&self) -> Retained<MLCTensor>;

        #[cfg(feature = "MLCTensor")]
        /// The variance tensor
        #[deprecated]
        #[unsafe(method(variance))]
        #[unsafe(method_family = none)]
        pub unsafe fn variance(&self) -> Retained<MLCTensor>;

        #[cfg(feature = "MLCTensor")]
        /// The beta tensor
        #[deprecated]
        #[unsafe(method(beta))]
        #[unsafe(method_family = none)]
        pub unsafe fn beta(&self) -> Option<Retained<MLCTensor>>;

        #[cfg(feature = "MLCTensor")]
        /// The gamma tensor
        #[deprecated]
        #[unsafe(method(gamma))]
        #[unsafe(method_family = none)]
        pub unsafe fn gamma(&self) -> Option<Retained<MLCTensor>>;

        #[cfg(feature = "MLCTensorParameter")]
        /// The beta tensor parameter used for optimizer update
        #[deprecated]
        #[unsafe(method(betaParameter))]
        #[unsafe(method_family = none)]
        pub unsafe fn betaParameter(&self) -> Option<Retained<MLCTensorParameter>>;

        #[cfg(feature = "MLCTensorParameter")]
        /// The gamma tensor parameter used for optimizer update
        #[deprecated]
        #[unsafe(method(gammaParameter))]
        #[unsafe(method_family = none)]
        pub unsafe fn gammaParameter(&self) -> Option<Retained<MLCTensorParameter>>;

        /// A value used for numerical stability
        #[deprecated]
        #[unsafe(method(varianceEpsilon))]
        #[unsafe(method_family = none)]
        pub unsafe fn varianceEpsilon(&self) -> c_float;

        /// The value used for the running mean and variance computation
        ///
        /// The default is 0.99f.
        #[deprecated]
        #[unsafe(method(momentum))]
        #[unsafe(method_family = none)]
        pub unsafe fn momentum(&self) -> c_float;

        #[cfg(feature = "MLCTensor")]
        /// Create a batch normalization layer
        ///
        /// Parameter `featureChannelCount`: The number of feature channels
        ///
        /// Parameter `mean`: The mean tensor
        ///
        /// Parameter `variance`: The variance tensor
        ///
        /// Parameter `beta`: The beta tensor
        ///
        /// Parameter `gamma`: The gamma tensor
        ///
        /// Parameter `varianceEpsilon`: The  epslion value
        ///
        /// Returns: A new batch normalization layer.
        #[deprecated]
        #[unsafe(method(layerWithFeatureChannelCount:mean:variance:beta:gamma:varianceEpsilon:))]
        #[unsafe(method_family = none)]
        pub unsafe fn layerWithFeatureChannelCount_mean_variance_beta_gamma_varianceEpsilon(
            feature_channel_count: NSUInteger,
            mean: &MLCTensor,
            variance: &MLCTensor,
            beta: Option<&MLCTensor>,
            gamma: Option<&MLCTensor>,
            variance_epsilon: c_float,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "MLCTensor")]
        /// Create a batch normalization layer
        ///
        /// Parameter `featureChannelCount`: The number of feature channels
        ///
        /// Parameter `mean`: The mean tensor
        ///
        /// Parameter `variance`: The variance tensor
        ///
        /// Parameter `beta`: The beta tensor
        ///
        /// Parameter `gamma`: The gamma tensor
        ///
        /// Parameter `varianceEpsilon`: The  epslion value
        ///
        /// Parameter `momentum`: The  momentum value for the running mean and variance computation
        ///
        /// Returns: A new batch normalization layer.
        #[deprecated]
        #[unsafe(method(layerWithFeatureChannelCount:mean:variance:beta:gamma:varianceEpsilon:momentum:))]
        #[unsafe(method_family = none)]
        pub unsafe fn layerWithFeatureChannelCount_mean_variance_beta_gamma_varianceEpsilon_momentum(
            feature_channel_count: NSUInteger,
            mean: &MLCTensor,
            variance: &MLCTensor,
            beta: Option<&MLCTensor>,
            gamma: Option<&MLCTensor>,
            variance_epsilon: c_float,
            momentum: c_float,
        ) -> Option<Retained<Self>>;
    );
}

/// Methods declared on superclass `MLCLayer`.
#[cfg(feature = "MLCLayer")]
impl MLCBatchNormalizationLayer {
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
