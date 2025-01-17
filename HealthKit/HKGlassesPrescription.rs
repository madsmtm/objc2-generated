//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// An object subclass representing a glasses prescription
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkglassesprescription?language=objc)
    #[unsafe(super(HKVisionPrescription, HKSample, HKObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "HKObject",
        feature = "HKSample",
        feature = "HKVisionPrescription"
    ))]
    pub struct HKGlassesPrescription;
);

#[cfg(all(
    feature = "HKObject",
    feature = "HKSample",
    feature = "HKVisionPrescription"
))]
unsafe impl Send for HKGlassesPrescription {}

#[cfg(all(
    feature = "HKObject",
    feature = "HKSample",
    feature = "HKVisionPrescription"
))]
unsafe impl Sync for HKGlassesPrescription {}

#[cfg(all(
    feature = "HKObject",
    feature = "HKSample",
    feature = "HKVisionPrescription"
))]
unsafe impl NSCoding for HKGlassesPrescription {}

#[cfg(all(
    feature = "HKObject",
    feature = "HKSample",
    feature = "HKVisionPrescription"
))]
unsafe impl NSCopying for HKGlassesPrescription {}

#[cfg(all(
    feature = "HKObject",
    feature = "HKSample",
    feature = "HKVisionPrescription"
))]
unsafe impl CopyingHelper for HKGlassesPrescription {
    type Result = Self;
}

#[cfg(all(
    feature = "HKObject",
    feature = "HKSample",
    feature = "HKVisionPrescription"
))]
unsafe impl NSObjectProtocol for HKGlassesPrescription {}

#[cfg(all(
    feature = "HKObject",
    feature = "HKSample",
    feature = "HKVisionPrescription"
))]
unsafe impl NSSecureCoding for HKGlassesPrescription {}

extern_methods!(
    #[cfg(all(
        feature = "HKObject",
        feature = "HKSample",
        feature = "HKVisionPrescription"
    ))]
    unsafe impl HKGlassesPrescription {
        #[cfg(all(
            feature = "HKGlassesLensSpecification",
            feature = "HKLensSpecification"
        ))]
        /// The right eye lens specification
        #[unsafe(method_family(none))]
        #[method_id(rightEye)]
        pub unsafe fn rightEye(&self) -> Option<Retained<HKGlassesLensSpecification>>;

        #[cfg(all(
            feature = "HKGlassesLensSpecification",
            feature = "HKLensSpecification"
        ))]
        /// The left eye lens specification
        #[unsafe(method_family(none))]
        #[method_id(leftEye)]
        pub unsafe fn leftEye(&self) -> Option<Retained<HKGlassesLensSpecification>>;

        #[cfg(all(
            feature = "HKDevice",
            feature = "HKGlassesLensSpecification",
            feature = "HKLensSpecification"
        ))]
        /// Parameter `rightEyeSpecification`: The right eye specification
        ///
        /// Parameter `leftEyeSpecification`: The left eye specification
        ///
        /// Parameter `dateIssued`: The date the prescription was issued
        ///
        /// Parameter `expirationDate`: The date the prescription expires
        ///
        /// Parameter `device`: The device that generated the sample
        ///
        /// Parameter `metadata`: The metadata for the sample
        #[unsafe(method_family(none))]
        #[method_id(prescriptionWithRightEyeSpecification:leftEyeSpecification:dateIssued:expirationDate:device:metadata:)]
        pub unsafe fn prescriptionWithRightEyeSpecification_leftEyeSpecification_dateIssued_expirationDate_device_metadata(
            right_eye_specification: Option<&HKGlassesLensSpecification>,
            left_eye_specification: Option<&HKGlassesLensSpecification>,
            date_issued: &NSDate,
            expiration_date: Option<&NSDate>,
            device: Option<&HKDevice>,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "HKDevice")]
        #[unsafe(method_family(none))]
        #[method_id(prescriptionWithType:dateIssued:expirationDate:device:metadata:)]
        pub unsafe fn prescriptionWithType_dateIssued_expirationDate_device_metadata(
            r#type: HKVisionPrescriptionType,
            date_issued: &NSDate,
            expiration_date: Option<&NSDate>,
            device: Option<&HKDevice>,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;
    }
);
