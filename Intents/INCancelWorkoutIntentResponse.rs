//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/intents/incancelworkoutintentresponsecode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct INCancelWorkoutIntentResponseCode(pub NSInteger);
impl INCancelWorkoutIntentResponseCode {
    #[doc(alias = "INCancelWorkoutIntentResponseCodeUnspecified")]
    pub const Unspecified: Self = Self(0);
    #[doc(alias = "INCancelWorkoutIntentResponseCodeReady")]
    pub const Ready: Self = Self(1);
    #[doc(alias = "INCancelWorkoutIntentResponseCodeContinueInApp")]
    pub const ContinueInApp: Self = Self(2);
    #[doc(alias = "INCancelWorkoutIntentResponseCodeFailure")]
    pub const Failure: Self = Self(3);
    #[doc(alias = "INCancelWorkoutIntentResponseCodeFailureRequiringAppLaunch")]
    pub const FailureRequiringAppLaunch: Self = Self(4);
    #[doc(alias = "INCancelWorkoutIntentResponseCodeFailureNoMatchingWorkout")]
    pub const FailureNoMatchingWorkout: Self = Self(5);
    #[doc(alias = "INCancelWorkoutIntentResponseCodeHandleInApp")]
    pub const HandleInApp: Self = Self(6);
    #[doc(alias = "INCancelWorkoutIntentResponseCodeSuccess")]
    pub const Success: Self = Self(7);
}

unsafe impl Encode for INCancelWorkoutIntentResponseCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for INCancelWorkoutIntentResponseCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/intents/incancelworkoutintentresponse?language=objc)
    #[unsafe(super(INIntentResponse, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "INIntentResponse")]
    pub struct INCancelWorkoutIntentResponse;
);

#[cfg(feature = "INIntentResponse")]
extern_conformance!(
    unsafe impl NSCoding for INCancelWorkoutIntentResponse {}
);

#[cfg(feature = "INIntentResponse")]
extern_conformance!(
    unsafe impl NSCopying for INCancelWorkoutIntentResponse {}
);

#[cfg(feature = "INIntentResponse")]
unsafe impl CopyingHelper for INCancelWorkoutIntentResponse {
    type Result = Self;
}

#[cfg(feature = "INIntentResponse")]
extern_conformance!(
    unsafe impl NSObjectProtocol for INCancelWorkoutIntentResponse {}
);

#[cfg(feature = "INIntentResponse")]
extern_conformance!(
    unsafe impl NSSecureCoding for INCancelWorkoutIntentResponse {}
);

#[cfg(feature = "INIntentResponse")]
impl INCancelWorkoutIntentResponse {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(initWithCode:userActivity:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCode_userActivity(
            this: Allocated<Self>,
            code: INCancelWorkoutIntentResponseCode,
            user_activity: Option<&NSUserActivity>,
        ) -> Retained<Self>;

        #[unsafe(method(code))]
        #[unsafe(method_family = none)]
        pub unsafe fn code(&self) -> INCancelWorkoutIntentResponseCode;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "INIntentResponse")]
impl INCancelWorkoutIntentResponse {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
