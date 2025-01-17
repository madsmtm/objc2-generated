//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// Represents a vision prescription type
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkvisionprescriptiontype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKVisionPrescriptionType(pub NSUInteger);
impl HKVisionPrescriptionType {
    #[doc(alias = "HKVisionPrescriptionTypeGlasses")]
    pub const Glasses: Self = Self(1);
    #[doc(alias = "HKVisionPrescriptionTypeContacts")]
    pub const Contacts: Self = Self(2);
}

unsafe impl Encode for HKVisionPrescriptionType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for HKVisionPrescriptionType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// HKSample subclass representing a vision prescription
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkvisionprescription?language=objc)
    #[unsafe(super(HKSample, HKObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    pub struct HKVisionPrescription;
);

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl Send for HKVisionPrescription {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl Sync for HKVisionPrescription {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl NSCoding for HKVisionPrescription {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl NSCopying for HKVisionPrescription {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl CopyingHelper for HKVisionPrescription {
    type Result = Self;
}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl NSObjectProtocol for HKVisionPrescription {}

#[cfg(all(feature = "HKObject", feature = "HKSample"))]
unsafe impl NSSecureCoding for HKVisionPrescription {}

extern_methods!(
    #[cfg(all(feature = "HKObject", feature = "HKSample"))]
    unsafe impl HKVisionPrescription {
        /// A vision prescription type (glasses or contacts)
        #[method(prescriptionType)]
        pub unsafe fn prescriptionType(&self) -> HKVisionPrescriptionType;

        /// The date the prescription was issued
        #[unsafe(method_family(none))]
        #[method_id(dateIssued)]
        pub unsafe fn dateIssued(&self) -> Retained<NSDate>;

        /// The date the prescription will expire
        #[unsafe(method_family(none))]
        #[method_id(expirationDate)]
        pub unsafe fn expirationDate(&self) -> Option<Retained<NSDate>>;

        #[cfg(feature = "HKDevice")]
        /// Parameter `type`: The prescription type
        ///
        /// Parameter `dateIssued`: The date the prescription was issued
        ///
        /// Parameter `expirationDate`: The date the prescription expires
        ///
        /// Parameter `device`: The device that generated the sample
        ///
        /// Parameter `metadata`: The metadata for the sample
        #[unsafe(method_family(none))]
        #[method_id(prescriptionWithType:dateIssued:expirationDate:device:metadata:)]
        pub unsafe fn prescriptionWithType_dateIssued_expirationDate_device_metadata(
            r#type: HKVisionPrescriptionType,
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
    }
);
