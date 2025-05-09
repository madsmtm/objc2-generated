//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// An NSObject that represents a verifiable clinical record subject.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkverifiableclinicalrecordsubject?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKVerifiableClinicalRecordSubject;
);

unsafe impl Send for HKVerifiableClinicalRecordSubject {}

unsafe impl Sync for HKVerifiableClinicalRecordSubject {}

extern_conformance!(
    unsafe impl NSCoding for HKVerifiableClinicalRecordSubject {}
);

extern_conformance!(
    unsafe impl NSCopying for HKVerifiableClinicalRecordSubject {}
);

unsafe impl CopyingHelper for HKVerifiableClinicalRecordSubject {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for HKVerifiableClinicalRecordSubject {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for HKVerifiableClinicalRecordSubject {}
);

impl HKVerifiableClinicalRecordSubject {
    extern_methods!(
        /// The subject's full name.
        #[unsafe(method(fullName))]
        #[unsafe(method_family = none)]
        pub unsafe fn fullName(&self) -> Retained<NSString>;

        /// The subject's date of birth components.
        #[unsafe(method(dateOfBirthComponents))]
        #[unsafe(method_family = none)]
        pub unsafe fn dateOfBirthComponents(&self) -> Option<Retained<NSDateComponents>>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
