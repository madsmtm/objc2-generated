//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A description of a model containing input, output, and state feature descriptions, optionally outputted features
    /// with special meaning and metadata.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coreml/mlmodeldescription?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLModelDescription;
);

extern_conformance!(
    unsafe impl NSCoding for MLModelDescription {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MLModelDescription {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for MLModelDescription {}
);

impl MLModelDescription {
    extern_methods!(
        #[cfg(feature = "MLFeatureDescription")]
        /// Description of the inputs to the model
        #[unsafe(method(inputDescriptionsByName))]
        #[unsafe(method_family = none)]
        pub unsafe fn inputDescriptionsByName(
            &self,
        ) -> Retained<NSDictionary<NSString, MLFeatureDescription>>;

        #[cfg(feature = "MLFeatureDescription")]
        /// Description of the outputs from the model
        #[unsafe(method(outputDescriptionsByName))]
        #[unsafe(method_family = none)]
        pub unsafe fn outputDescriptionsByName(
            &self,
        ) -> Retained<NSDictionary<NSString, MLFeatureDescription>>;

        #[cfg(feature = "MLFeatureDescription")]
        /// Description of the state features.
        #[unsafe(method(stateDescriptionsByName))]
        #[unsafe(method_family = none)]
        pub unsafe fn stateDescriptionsByName(
            &self,
        ) -> Retained<NSDictionary<NSString, MLFeatureDescription>>;

        /// Name of the primary target / predicted output feature in the output descriptions
        #[unsafe(method(predictedFeatureName))]
        #[unsafe(method_family = none)]
        pub unsafe fn predictedFeatureName(&self) -> Option<Retained<NSString>>;

        /// Key for all predicted probabilities stored as a MLFeatureTypeDictionary in the output descriptions
        #[unsafe(method(predictedProbabilitiesName))]
        #[unsafe(method_family = none)]
        pub unsafe fn predictedProbabilitiesName(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "MLModelMetadataKeys")]
        /// Optional metadata describing the model
        #[unsafe(method(metadata))]
        #[unsafe(method_family = none)]
        pub unsafe fn metadata(&self) -> Retained<NSDictionary<MLModelMetadataKey, AnyObject>>;

        /// Array to map a class index to the corresponding label, which is either Number or String.
        ///
        /// The property is populated from the classLabels entry specified in the model's protobuf message. When the model is a pipeline, which contains one or more sub models, the property value is calculated as follows.
        ///
        /// 1. If the pipeline model's proto message specifies predictedFeatureName parameter, use classLabels property value of the sub model with the output feature with the name.
        ///
        /// 2. Otherwise, if the pipeline model has only one sub model with non-nil classLabels property, use the property value.
        ///
        /// 3. Otherwise, the property is nil.
        #[unsafe(method(classLabels))]
        #[unsafe(method_family = none)]
        pub unsafe fn classLabels(&self) -> Option<Retained<NSArray<AnyObject>>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl MLModelDescription {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

/// MLUpdateAdditions.
/// Additions to model descriptions related to model update API.
impl MLModelDescription {
    extern_methods!(
        #[unsafe(method(isUpdatable))]
        #[unsafe(method_family = none)]
        pub unsafe fn isUpdatable(&self) -> bool;

        #[cfg(feature = "MLFeatureDescription")]
        #[unsafe(method(trainingInputDescriptionsByName))]
        #[unsafe(method_family = none)]
        pub unsafe fn trainingInputDescriptionsByName(
            &self,
        ) -> Retained<NSDictionary<NSString, MLFeatureDescription>>;
    );
}

/// MLParameters.
/// Additions to model descriptions related to model parameters
impl MLModelDescription {
    extern_methods!(
        #[cfg(all(
            feature = "MLKey",
            feature = "MLParameterDescription",
            feature = "MLParameterKey"
        ))]
        #[unsafe(method(parameterDescriptionsByKey))]
        #[unsafe(method_family = none)]
        pub unsafe fn parameterDescriptionsByKey(
            &self,
        ) -> Retained<NSDictionary<MLParameterKey, MLParameterDescription>>;
    );
}
