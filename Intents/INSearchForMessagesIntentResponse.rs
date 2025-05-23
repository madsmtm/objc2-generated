//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/intents/insearchformessagesintentresponsecode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct INSearchForMessagesIntentResponseCode(pub NSInteger);
impl INSearchForMessagesIntentResponseCode {
    #[doc(alias = "INSearchForMessagesIntentResponseCodeUnspecified")]
    pub const Unspecified: Self = Self(0);
    #[doc(alias = "INSearchForMessagesIntentResponseCodeReady")]
    pub const Ready: Self = Self(1);
    #[doc(alias = "INSearchForMessagesIntentResponseCodeInProgress")]
    pub const InProgress: Self = Self(2);
    #[doc(alias = "INSearchForMessagesIntentResponseCodeSuccess")]
    pub const Success: Self = Self(3);
    #[doc(alias = "INSearchForMessagesIntentResponseCodeFailure")]
    pub const Failure: Self = Self(4);
    #[doc(alias = "INSearchForMessagesIntentResponseCodeFailureRequiringAppLaunch")]
    pub const FailureRequiringAppLaunch: Self = Self(5);
    #[doc(alias = "INSearchForMessagesIntentResponseCodeFailureMessageServiceNotAvailable")]
    pub const FailureMessageServiceNotAvailable: Self = Self(6);
    #[doc(alias = "INSearchForMessagesIntentResponseCodeFailureMessageTooManyResults")]
    pub const FailureMessageTooManyResults: Self = Self(7);
    #[doc(alias = "INSearchForMessagesIntentResponseCodeFailureRequiringInAppAuthentication")]
    pub const FailureRequiringInAppAuthentication: Self = Self(8);
}

unsafe impl Encode for INSearchForMessagesIntentResponseCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for INSearchForMessagesIntentResponseCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/intents/insearchformessagesintentresponse?language=objc)
    #[unsafe(super(INIntentResponse, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "INIntentResponse")]
    pub struct INSearchForMessagesIntentResponse;
);

#[cfg(feature = "INIntentResponse")]
extern_conformance!(
    unsafe impl NSCoding for INSearchForMessagesIntentResponse {}
);

#[cfg(feature = "INIntentResponse")]
extern_conformance!(
    unsafe impl NSCopying for INSearchForMessagesIntentResponse {}
);

#[cfg(feature = "INIntentResponse")]
unsafe impl CopyingHelper for INSearchForMessagesIntentResponse {
    type Result = Self;
}

#[cfg(feature = "INIntentResponse")]
extern_conformance!(
    unsafe impl NSObjectProtocol for INSearchForMessagesIntentResponse {}
);

#[cfg(feature = "INIntentResponse")]
extern_conformance!(
    unsafe impl NSSecureCoding for INSearchForMessagesIntentResponse {}
);

#[cfg(feature = "INIntentResponse")]
impl INSearchForMessagesIntentResponse {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(initWithCode:userActivity:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCode_userActivity(
            this: Allocated<Self>,
            code: INSearchForMessagesIntentResponseCode,
            user_activity: Option<&NSUserActivity>,
        ) -> Retained<Self>;

        #[unsafe(method(code))]
        #[unsafe(method_family = none)]
        pub unsafe fn code(&self) -> INSearchForMessagesIntentResponseCode;

        #[cfg(feature = "INMessage")]
        #[unsafe(method(messages))]
        #[unsafe(method_family = none)]
        pub unsafe fn messages(&self) -> Option<Retained<NSArray<INMessage>>>;

        #[cfg(feature = "INMessage")]
        /// Setter for [`messages`][Self::messages].
        #[unsafe(method(setMessages:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMessages(&self, messages: Option<&NSArray<INMessage>>);
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "INIntentResponse")]
impl INSearchForMessagesIntentResponse {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
