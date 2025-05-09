//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/intents/insetcarlockstatusintentresponsecode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct INSetCarLockStatusIntentResponseCode(pub NSInteger);
impl INSetCarLockStatusIntentResponseCode {
    #[doc(alias = "INSetCarLockStatusIntentResponseCodeUnspecified")]
    pub const Unspecified: Self = Self(0);
    #[doc(alias = "INSetCarLockStatusIntentResponseCodeReady")]
    pub const Ready: Self = Self(1);
    #[doc(alias = "INSetCarLockStatusIntentResponseCodeInProgress")]
    pub const InProgress: Self = Self(2);
    #[doc(alias = "INSetCarLockStatusIntentResponseCodeSuccess")]
    pub const Success: Self = Self(3);
    #[doc(alias = "INSetCarLockStatusIntentResponseCodeFailure")]
    pub const Failure: Self = Self(4);
    #[doc(alias = "INSetCarLockStatusIntentResponseCodeFailureRequiringAppLaunch")]
    pub const FailureRequiringAppLaunch: Self = Self(5);
}

unsafe impl Encode for INSetCarLockStatusIntentResponseCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for INSetCarLockStatusIntentResponseCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/intents/insetcarlockstatusintentresponse?language=objc)
    #[unsafe(super(INIntentResponse, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "INIntentResponse")]
    pub struct INSetCarLockStatusIntentResponse;
);

#[cfg(feature = "INIntentResponse")]
extern_conformance!(
    unsafe impl NSCoding for INSetCarLockStatusIntentResponse {}
);

#[cfg(feature = "INIntentResponse")]
extern_conformance!(
    unsafe impl NSCopying for INSetCarLockStatusIntentResponse {}
);

#[cfg(feature = "INIntentResponse")]
unsafe impl CopyingHelper for INSetCarLockStatusIntentResponse {
    type Result = Self;
}

#[cfg(feature = "INIntentResponse")]
extern_conformance!(
    unsafe impl NSObjectProtocol for INSetCarLockStatusIntentResponse {}
);

#[cfg(feature = "INIntentResponse")]
extern_conformance!(
    unsafe impl NSSecureCoding for INSetCarLockStatusIntentResponse {}
);

#[cfg(feature = "INIntentResponse")]
impl INSetCarLockStatusIntentResponse {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(initWithCode:userActivity:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCode_userActivity(
            this: Allocated<Self>,
            code: INSetCarLockStatusIntentResponseCode,
            user_activity: Option<&NSUserActivity>,
        ) -> Retained<Self>;

        #[unsafe(method(code))]
        #[unsafe(method_family = none)]
        pub unsafe fn code(&self) -> INSetCarLockStatusIntentResponseCode;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "INIntentResponse")]
impl INSetCarLockStatusIntentResponse {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
