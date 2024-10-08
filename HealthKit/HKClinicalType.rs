//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_ENUM
pub type HKClinicalTypeIdentifier = NSString;

extern "C" {
    pub static HKClinicalTypeIdentifierAllergyRecord: &'static HKClinicalTypeIdentifier;
}

extern "C" {
    pub static HKClinicalTypeIdentifierClinicalNoteRecord: &'static HKClinicalTypeIdentifier;
}

extern "C" {
    pub static HKClinicalTypeIdentifierConditionRecord: &'static HKClinicalTypeIdentifier;
}

extern "C" {
    pub static HKClinicalTypeIdentifierImmunizationRecord: &'static HKClinicalTypeIdentifier;
}

extern "C" {
    pub static HKClinicalTypeIdentifierLabResultRecord: &'static HKClinicalTypeIdentifier;
}

extern "C" {
    pub static HKClinicalTypeIdentifierMedicationRecord: &'static HKClinicalTypeIdentifier;
}

extern "C" {
    pub static HKClinicalTypeIdentifierProcedureRecord: &'static HKClinicalTypeIdentifier;
}

extern "C" {
    pub static HKClinicalTypeIdentifierVitalSignRecord: &'static HKClinicalTypeIdentifier;
}

extern "C" {
    pub static HKClinicalTypeIdentifierCoverageRecord: &'static HKClinicalTypeIdentifier;
}

extern_methods!(
    /// ClinicalType
    #[cfg(feature = "HKObjectType")]
    unsafe impl HKObjectType {
        #[method_id(@__retain_semantics Other clinicalTypeForIdentifier:)]
        pub unsafe fn clinicalTypeForIdentifier(
            identifier: &HKClinicalTypeIdentifier,
        ) -> Option<Retained<HKClinicalType>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HKObjectType")]
    pub struct HKClinicalType;

    #[cfg(feature = "HKObjectType")]
    unsafe impl ClassType for HKClinicalType {
        #[inherits(HKObjectType, NSObject)]
        type Super = HKSampleType;
    }
);

#[cfg(feature = "HKObjectType")]
unsafe impl NSCoding for HKClinicalType {}

#[cfg(feature = "HKObjectType")]
unsafe impl NSCopying for HKClinicalType {}

#[cfg(feature = "HKObjectType")]
unsafe impl CopyingHelper for HKClinicalType {
    type Result = Self;
}

#[cfg(feature = "HKObjectType")]
unsafe impl NSObjectProtocol for HKClinicalType {}

#[cfg(feature = "HKObjectType")]
unsafe impl NSSecureCoding for HKClinicalType {}

extern_methods!(
    #[cfg(feature = "HKObjectType")]
    unsafe impl HKClinicalType {}
);

extern_methods!(
    /// Methods declared on superclass `HKObjectType`
    #[cfg(feature = "HKObjectType")]
    unsafe impl HKClinicalType {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HKObjectType")]
    unsafe impl HKClinicalType {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
