//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Description of a feature
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coreml/mlfeaturedescription?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLFeatureDescription;
);

extern_conformance!(
    unsafe impl NSCoding for MLFeatureDescription {}
);

extern_conformance!(
    unsafe impl NSCopying for MLFeatureDescription {}
);

unsafe impl CopyingHelper for MLFeatureDescription {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MLFeatureDescription {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for MLFeatureDescription {}
);

impl MLFeatureDescription {
    extern_methods!(
        /// Name of feature
        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[cfg(feature = "MLFeatureType")]
        /// Type of data
        #[unsafe(method(type))]
        #[unsafe(method_family = none)]
        pub unsafe fn r#type(&self) -> MLFeatureType;

        /// Whether this feature can take an undefined value or not
        #[unsafe(method(isOptional))]
        #[unsafe(method_family = none)]
        pub unsafe fn isOptional(&self) -> bool;

        #[cfg(feature = "MLFeatureValue")]
        /// Check if MLFeatureValue is valid based on this description
        #[unsafe(method(isAllowedValue:))]
        #[unsafe(method_family = none)]
        pub unsafe fn isAllowedValue(&self, value: &MLFeatureValue) -> bool;
    );
}

/// Methods declared on superclass `NSObject`.
impl MLFeatureDescription {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

/// MLFeatureValueConstraints.
/// Feature value constraints for specific types
impl MLFeatureDescription {
    extern_methods!(
        #[cfg(feature = "MLMultiArrayConstraint")]
        /// Constraint when type == MLFeatureTypeMultiArray, nil otherwise
        #[unsafe(method(multiArrayConstraint))]
        #[unsafe(method_family = none)]
        pub unsafe fn multiArrayConstraint(&self) -> Option<Retained<MLMultiArrayConstraint>>;

        #[cfg(feature = "MLImageConstraint")]
        /// Constraint when type == MLFeatureTypeImage, nil otherwise
        #[unsafe(method(imageConstraint))]
        #[unsafe(method_family = none)]
        pub unsafe fn imageConstraint(&self) -> Option<Retained<MLImageConstraint>>;

        #[cfg(feature = "MLDictionaryConstraint")]
        /// Constraint when type == MLFeatureTypeDictionary, nil otherwise
        #[unsafe(method(dictionaryConstraint))]
        #[unsafe(method_family = none)]
        pub unsafe fn dictionaryConstraint(&self) -> Option<Retained<MLDictionaryConstraint>>;

        #[cfg(feature = "MLSequenceConstraint")]
        /// Constraint when type == MLFeatureTypeSequence, nil otherwise
        #[unsafe(method(sequenceConstraint))]
        #[unsafe(method_family = none)]
        pub unsafe fn sequenceConstraint(&self) -> Option<Retained<MLSequenceConstraint>>;

        #[cfg(feature = "MLStateConstraint")]
        /// The state feature value constraint.
        ///
        /// The property has a value when `.type == MLFeatureTypeState`.
        #[unsafe(method(stateConstraint))]
        #[unsafe(method_family = none)]
        pub unsafe fn stateConstraint(&self) -> Option<Retained<MLStateConstraint>>;
    );
}
