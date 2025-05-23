//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/intents/ineditmessageintentresponsecode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct INEditMessageIntentResponseCode(pub NSInteger);
impl INEditMessageIntentResponseCode {
    #[doc(alias = "INEditMessageIntentResponseCodeUnspecified")]
    pub const Unspecified: Self = Self(0);
    #[doc(alias = "INEditMessageIntentResponseCodeReady")]
    pub const Ready: Self = Self(1);
    #[doc(alias = "INEditMessageIntentResponseCodeInProgress")]
    pub const InProgress: Self = Self(2);
    #[doc(alias = "INEditMessageIntentResponseCodeSuccess")]
    pub const Success: Self = Self(3);
    #[doc(alias = "INEditMessageIntentResponseCodeFailure")]
    pub const Failure: Self = Self(4);
    #[doc(alias = "INEditMessageIntentResponseCodeFailureRequiringAppLaunch")]
    pub const FailureRequiringAppLaunch: Self = Self(5);
    #[doc(alias = "INEditMessageIntentResponseCodeFailureMessageNotFound")]
    pub const FailureMessageNotFound: Self = Self(6);
    #[doc(alias = "INEditMessageIntentResponseCodeFailurePastEditTimeLimit")]
    pub const FailurePastEditTimeLimit: Self = Self(7);
    #[doc(alias = "INEditMessageIntentResponseCodeFailureMessageTypeUnsupported")]
    pub const FailureMessageTypeUnsupported: Self = Self(8);
    #[doc(alias = "INEditMessageIntentResponseCodeFailureUnsupportedOnService")]
    pub const FailureUnsupportedOnService: Self = Self(9);
    #[doc(alias = "INEditMessageIntentResponseCodeFailureMessageServiceNotAvailable")]
    pub const FailureMessageServiceNotAvailable: Self = Self(10);
    #[doc(alias = "INEditMessageIntentResponseCodeFailureRequiringInAppAuthentication")]
    pub const FailureRequiringInAppAuthentication: Self = Self(11);
}

unsafe impl Encode for INEditMessageIntentResponseCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for INEditMessageIntentResponseCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/intents/ineditmessageintentresponse?language=objc)
    #[unsafe(super(INIntentResponse, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "INIntentResponse")]
    pub struct INEditMessageIntentResponse;
);

#[cfg(feature = "INIntentResponse")]
extern_conformance!(
    unsafe impl NSCoding for INEditMessageIntentResponse {}
);

#[cfg(feature = "INIntentResponse")]
extern_conformance!(
    unsafe impl NSCopying for INEditMessageIntentResponse {}
);

#[cfg(feature = "INIntentResponse")]
unsafe impl CopyingHelper for INEditMessageIntentResponse {
    type Result = Self;
}

#[cfg(feature = "INIntentResponse")]
extern_conformance!(
    unsafe impl NSObjectProtocol for INEditMessageIntentResponse {}
);

#[cfg(feature = "INIntentResponse")]
extern_conformance!(
    unsafe impl NSSecureCoding for INEditMessageIntentResponse {}
);

#[cfg(feature = "INIntentResponse")]
impl INEditMessageIntentResponse {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(initWithCode:userActivity:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCode_userActivity(
            this: Allocated<Self>,
            code: INEditMessageIntentResponseCode,
            user_activity: Option<&NSUserActivity>,
        ) -> Retained<Self>;

        #[unsafe(method(code))]
        #[unsafe(method_family = none)]
        pub unsafe fn code(&self) -> INEditMessageIntentResponseCode;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "INIntentResponse")]
impl INEditMessageIntentResponse {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
