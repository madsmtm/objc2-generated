//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_methods!(
    /// MLState
    #[cfg(feature = "MLModel")]
    unsafe impl MLModel {
        #[cfg(feature = "MLState")]
        #[method_id(@__retain_semantics New newState)]
        pub unsafe fn newState(&self) -> Retained<MLState>;

        #[cfg(all(feature = "MLFeatureProvider", feature = "MLState"))]
        #[method_id(@__retain_semantics Other predictionFromFeatures:usingState:error:_)]
        pub unsafe fn predictionFromFeatures_usingState_error(
            &self,
            input_features: &ProtocolObject<dyn MLFeatureProvider>,
            state: &MLState,
        ) -> Result<Retained<ProtocolObject<dyn MLFeatureProvider>>, Retained<NSError>>;

        #[cfg(all(
            feature = "MLFeatureProvider",
            feature = "MLPredictionOptions",
            feature = "MLState"
        ))]
        #[method_id(@__retain_semantics Other predictionFromFeatures:usingState:options:error:_)]
        pub unsafe fn predictionFromFeatures_usingState_options_error(
            &self,
            input_features: &ProtocolObject<dyn MLFeatureProvider>,
            state: &MLState,
            options: &MLPredictionOptions,
        ) -> Result<Retained<ProtocolObject<dyn MLFeatureProvider>>, Retained<NSError>>;

        #[cfg(all(
            feature = "MLFeatureProvider",
            feature = "MLPredictionOptions",
            feature = "MLState",
            feature = "block2"
        ))]
        #[method(predictionFromFeatures:usingState:options:completionHandler:)]
        pub unsafe fn predictionFromFeatures_usingState_options_completionHandler(
            &self,
            input_features: &ProtocolObject<dyn MLFeatureProvider>,
            state: &MLState,
            options: &MLPredictionOptions,
            completion_handler: &block2::Block<
                dyn Fn(*mut ProtocolObject<dyn MLFeatureProvider>, *mut NSError),
            >,
        );
    }
);
