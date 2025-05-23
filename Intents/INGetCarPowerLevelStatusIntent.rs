//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/intents/ingetcarpowerlevelstatusintent?language=objc)
    #[unsafe(super(INIntent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "INIntent")]
    pub struct INGetCarPowerLevelStatusIntent;
);

#[cfg(feature = "INIntent")]
extern_conformance!(
    unsafe impl NSCoding for INGetCarPowerLevelStatusIntent {}
);

#[cfg(feature = "INIntent")]
extern_conformance!(
    unsafe impl NSCopying for INGetCarPowerLevelStatusIntent {}
);

#[cfg(feature = "INIntent")]
unsafe impl CopyingHelper for INGetCarPowerLevelStatusIntent {
    type Result = Self;
}

#[cfg(feature = "INIntent")]
extern_conformance!(
    unsafe impl NSObjectProtocol for INGetCarPowerLevelStatusIntent {}
);

#[cfg(feature = "INIntent")]
extern_conformance!(
    unsafe impl NSSecureCoding for INGetCarPowerLevelStatusIntent {}
);

#[cfg(feature = "INIntent")]
impl INGetCarPowerLevelStatusIntent {
    extern_methods!(
        #[cfg(feature = "INSpeakableString")]
        #[unsafe(method(initWithCarName:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCarName(
            this: Allocated<Self>,
            car_name: Option<&INSpeakableString>,
        ) -> Retained<Self>;

        #[cfg(feature = "INSpeakableString")]
        #[unsafe(method(carName))]
        #[unsafe(method_family = none)]
        pub unsafe fn carName(&self) -> Option<Retained<INSpeakableString>>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "INIntent")]
impl INGetCarPowerLevelStatusIntent {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_protocol!(
    /// Protocol to declare support for handling an INGetCarPowerLevelStatusIntent. By implementing this protocol, a class can provide logic for resolving, confirming and handling the intent.
    ///
    /// The minimum requirement for an implementing class is that it should be able to handle the intent. The resolution and confirmation methods are optional. The handling method is always called last, after resolving and confirming the intent.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/intents/ingetcarpowerlevelstatusintenthandling?language=objc)
    pub unsafe trait INGetCarPowerLevelStatusIntentHandling: NSObjectProtocol {
        #[cfg(all(
            feature = "INGetCarPowerLevelStatusIntentResponse",
            feature = "INIntent",
            feature = "INIntentResponse",
            feature = "block2"
        ))]
        /// Handling method - Execute the task represented by the INGetCarPowerLevelStatusIntent that's passed in
        ///
        /// Called to actually execute the intent. The app must return a response for this intent.
        ///
        ///
        /// Parameter `intent`: The input intent
        ///
        /// Parameter `completion`: The response handling block takes a INGetCarPowerLevelStatusIntentResponse containing the details of the result of having executed the intent
        ///
        ///
        /// See: INGetCarPowerLevelStatusIntentResponse
        #[unsafe(method(handleGetCarPowerLevelStatus:completion:))]
        #[unsafe(method_family = none)]
        unsafe fn handleGetCarPowerLevelStatus_completion(
            &self,
            intent: &INGetCarPowerLevelStatusIntent,
            completion: &block2::DynBlock<dyn Fn(NonNull<INGetCarPowerLevelStatusIntentResponse>)>,
        );

        #[cfg(feature = "INIntent")]
        #[optional]
        #[unsafe(method(startSendingUpdatesForGetCarPowerLevelStatus:toObserver:))]
        #[unsafe(method_family = none)]
        unsafe fn startSendingUpdatesForGetCarPowerLevelStatus_toObserver(
            &self,
            intent: &INGetCarPowerLevelStatusIntent,
            observer: &ProtocolObject<dyn INGetCarPowerLevelStatusIntentResponseObserver>,
        );

        #[cfg(feature = "INIntent")]
        #[optional]
        #[unsafe(method(stopSendingUpdatesForGetCarPowerLevelStatus:))]
        #[unsafe(method_family = none)]
        unsafe fn stopSendingUpdatesForGetCarPowerLevelStatus(
            &self,
            intent: &INGetCarPowerLevelStatusIntent,
        );

        #[cfg(all(
            feature = "INGetCarPowerLevelStatusIntentResponse",
            feature = "INIntent",
            feature = "INIntentResponse",
            feature = "block2"
        ))]
        /// Confirmation method - Validate that this intent is ready for the next step (i.e. handling)
        ///
        /// Called prior to asking the app to handle the intent. The app should return a response object that contains additional information about the intent, which may be relevant for the system to show the user prior to handling. If unimplemented, the system will assume the intent is valid following resolution, and will assume there is no additional information relevant to this intent.
        ///
        ///
        /// Parameter `intent`: The input intent
        ///
        /// Parameter `completion`: The response block contains an INGetCarPowerLevelStatusIntentResponse containing additional details about the intent that may be relevant for the system to show the user prior to handling.
        ///
        ///
        /// See: INGetCarPowerLevelStatusIntentResponse
        #[optional]
        #[unsafe(method(confirmGetCarPowerLevelStatus:completion:))]
        #[unsafe(method_family = none)]
        unsafe fn confirmGetCarPowerLevelStatus_completion(
            &self,
            intent: &INGetCarPowerLevelStatusIntent,
            completion: &block2::DynBlock<dyn Fn(NonNull<INGetCarPowerLevelStatusIntentResponse>)>,
        );

        #[cfg(all(
            feature = "INIntent",
            feature = "INIntentResolutionResult",
            feature = "INSpeakableStringResolutionResult",
            feature = "block2"
        ))]
        /// Resolution methods - Determine if this intent is ready for the next step (confirmation)
        ///
        /// Called to make sure the app extension is capable of handling this intent in its current form. This method is for validating if the intent needs any further fleshing out.
        ///
        ///
        /// Parameter `intent`: The input intent
        ///
        /// Parameter `completion`: The response block contains an INIntentResolutionResult for the parameter being resolved
        ///
        ///
        /// See: INIntentResolutionResult
        #[optional]
        #[unsafe(method(resolveCarNameForGetCarPowerLevelStatus:withCompletion:))]
        #[unsafe(method_family = none)]
        unsafe fn resolveCarNameForGetCarPowerLevelStatus_withCompletion(
            &self,
            intent: &INGetCarPowerLevelStatusIntent,
            completion: &block2::DynBlock<dyn Fn(NonNull<INSpeakableStringResolutionResult>)>,
        );
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/intents/ingetcarpowerlevelstatusintentresponseobserver?language=objc)
    pub unsafe trait INGetCarPowerLevelStatusIntentResponseObserver:
        NSObjectProtocol
    {
        #[cfg(all(
            feature = "INGetCarPowerLevelStatusIntentResponse",
            feature = "INIntentResponse"
        ))]
        #[unsafe(method(getCarPowerLevelStatusResponseDidUpdate:))]
        #[unsafe(method_family = none)]
        unsafe fn getCarPowerLevelStatusResponseDidUpdate(
            &self,
            response: &INGetCarPowerLevelStatusIntentResponse,
        );
    }
);
