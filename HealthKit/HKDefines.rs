//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkerrordomain?language=objc)
    pub static HKErrorDomain: &'static NSString;
}

/// perform the requested operation.
///
///
///
///
///
/// required data types.
///
/// query's result could not be meaningfully computed.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkerrorcode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKErrorCode(pub NSInteger);
impl HKErrorCode {
    #[doc(alias = "HKUnknownError")]
    pub const UnknownError: Self = Self(0);
    #[doc(alias = "HKNoError")]
    #[deprecated]
    pub const NoError: Self = Self(HKErrorCode::UnknownError.0);
    #[doc(alias = "HKErrorHealthDataUnavailable")]
    pub const ErrorHealthDataUnavailable: Self = Self(1);
    #[doc(alias = "HKErrorHealthDataRestricted")]
    pub const ErrorHealthDataRestricted: Self = Self(2);
    #[doc(alias = "HKErrorInvalidArgument")]
    pub const ErrorInvalidArgument: Self = Self(3);
    #[doc(alias = "HKErrorAuthorizationDenied")]
    pub const ErrorAuthorizationDenied: Self = Self(4);
    #[doc(alias = "HKErrorAuthorizationNotDetermined")]
    pub const ErrorAuthorizationNotDetermined: Self = Self(5);
    #[doc(alias = "HKErrorDatabaseInaccessible")]
    pub const ErrorDatabaseInaccessible: Self = Self(6);
    #[doc(alias = "HKErrorUserCanceled")]
    pub const ErrorUserCanceled: Self = Self(7);
    #[doc(alias = "HKErrorAnotherWorkoutSessionStarted")]
    pub const ErrorAnotherWorkoutSessionStarted: Self = Self(8);
    #[doc(alias = "HKErrorUserExitedWorkoutSession")]
    pub const ErrorUserExitedWorkoutSession: Self = Self(9);
    #[doc(alias = "HKErrorRequiredAuthorizationDenied")]
    pub const ErrorRequiredAuthorizationDenied: Self = Self(10);
    #[doc(alias = "HKErrorNoData")]
    pub const ErrorNoData: Self = Self(11);
    #[doc(alias = "HKErrorWorkoutActivityNotAllowed")]
    pub const ErrorWorkoutActivityNotAllowed: Self = Self(12);
    #[doc(alias = "HKErrorDataSizeExceeded")]
    pub const ErrorDataSizeExceeded: Self = Self(13);
    #[doc(alias = "HKErrorBackgroundWorkoutSessionNotAllowed")]
    pub const ErrorBackgroundWorkoutSessionNotAllowed: Self = Self(14);
    #[doc(alias = "HKErrorNotPermissibleForGuestUserMode")]
    pub const ErrorNotPermissibleForGuestUserMode: Self = Self(15);
}

unsafe impl Encode for HKErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkupdatefrequency?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKUpdateFrequency(pub NSInteger);
impl HKUpdateFrequency {
    #[doc(alias = "HKUpdateFrequencyImmediate")]
    pub const Immediate: Self = Self(1);
    #[doc(alias = "HKUpdateFrequencyHourly")]
    pub const Hourly: Self = Self(2);
    #[doc(alias = "HKUpdateFrequencyDaily")]
    pub const Daily: Self = Self(3);
    #[doc(alias = "HKUpdateFrequencyWeekly")]
    pub const Weekly: Self = Self(4);
}

unsafe impl Encode for HKUpdateFrequency {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKUpdateFrequency {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// This enumerated type is used to indicate the currently granted authorization status for a specific
/// HKObjectType.
///
///
/// application may save objects of the specified type.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkauthorizationstatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKAuthorizationStatus(pub NSInteger);
impl HKAuthorizationStatus {
    #[doc(alias = "HKAuthorizationStatusNotDetermined")]
    pub const NotDetermined: Self = Self(0);
    #[doc(alias = "HKAuthorizationStatusSharingDenied")]
    pub const SharingDenied: Self = Self(1);
    #[doc(alias = "HKAuthorizationStatusSharingAuthorized")]
    pub const SharingAuthorized: Self = Self(2);
}

unsafe impl Encode for HKAuthorizationStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKAuthorizationStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// This enumerated type is used to indicate whether it is necessary to request authorization from the user.
///
///
/// an error occurred.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkauthorizationrequeststatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKAuthorizationRequestStatus(pub NSInteger);
impl HKAuthorizationRequestStatus {
    #[doc(alias = "HKAuthorizationRequestStatusUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "HKAuthorizationRequestStatusShouldRequest")]
    pub const ShouldRequest: Self = Self(1);
    #[doc(alias = "HKAuthorizationRequestStatusUnnecessary")]
    pub const Unnecessary: Self = Self(2);
}

unsafe impl Encode for HKAuthorizationRequestStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKAuthorizationRequestStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Returns the set of `HKCategoryValueSleepAnalysis` values that are considered to be asleep.
#[inline]
pub unsafe extern "C-unwind" fn HKCategoryValueSleepAnalysisAsleepValues(
) -> Retained<NSSet<NSNumber>> {
    extern "C-unwind" {
        fn HKCategoryValueSleepAnalysisAsleepValues() -> *mut NSSet<NSNumber>;
    }
    let ret = unsafe { HKCategoryValueSleepAnalysisAsleepValues() };
    unsafe { Retained::retain_autoreleased(ret) }
        .expect("function was marked as returning non-null, but actually returned NULL")
}
