//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/intents/ingetavailablerestaurantreservationbookingdefaultsintent?language=objc)
    #[unsafe(super(INIntent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "INIntent")]
    pub struct INGetAvailableRestaurantReservationBookingDefaultsIntent;
);

#[cfg(feature = "INIntent")]
extern_conformance!(
    unsafe impl NSCoding for INGetAvailableRestaurantReservationBookingDefaultsIntent {}
);

#[cfg(feature = "INIntent")]
extern_conformance!(
    unsafe impl NSCopying for INGetAvailableRestaurantReservationBookingDefaultsIntent {}
);

#[cfg(feature = "INIntent")]
unsafe impl CopyingHelper for INGetAvailableRestaurantReservationBookingDefaultsIntent {
    type Result = Self;
}

#[cfg(feature = "INIntent")]
extern_conformance!(
    unsafe impl NSObjectProtocol for INGetAvailableRestaurantReservationBookingDefaultsIntent {}
);

#[cfg(feature = "INIntent")]
extern_conformance!(
    unsafe impl NSSecureCoding for INGetAvailableRestaurantReservationBookingDefaultsIntent {}
);

#[cfg(feature = "INIntent")]
impl INGetAvailableRestaurantReservationBookingDefaultsIntent {
    extern_methods!(
        #[cfg(feature = "INRestaurant")]
        #[unsafe(method(initWithRestaurant:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithRestaurant(
            this: Allocated<Self>,
            restaurant: Option<&INRestaurant>,
        ) -> Retained<Self>;

        #[cfg(feature = "INRestaurant")]
        #[unsafe(method(restaurant))]
        #[unsafe(method_family = none)]
        pub unsafe fn restaurant(&self) -> Option<Retained<INRestaurant>>;

        #[cfg(feature = "INRestaurant")]
        /// Setter for [`restaurant`][Self::restaurant].
        #[unsafe(method(setRestaurant:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRestaurant(&self, restaurant: Option<&INRestaurant>);
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "INIntent")]
impl INGetAvailableRestaurantReservationBookingDefaultsIntent {
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
    /// [Apple's documentation](https://developer.apple.com/documentation/intents/ingetavailablerestaurantreservationbookingdefaultsintenthandling?language=objc)
    pub unsafe trait INGetAvailableRestaurantReservationBookingDefaultsIntentHandling:
        NSObjectProtocol
    {
        #[cfg(all(
            feature = "INGetAvailableRestaurantReservationBookingDefaultsIntentResponse",
            feature = "INIntent",
            feature = "INIntentResponse",
            feature = "block2"
        ))]
        /// Handling method - Execute the task represented by the INGetAvailableRestaurantReservationBookingDefaultsIntent that's passed in
        ///
        /// This method is called to actually execute the intent, the app must return a response for this intent and an NSUserActivity capturing the state that the app must be restored to at the end of handling this intent
        ///
        ///
        /// Parameter `intent`: The input intent
        ///
        /// Parameter `completion`: The response handling block to invoke with the response to handling the intent.
        ///
        ///
        /// See: INGetAvailableRestaurantReservationBookingDefaultsIntentResponse
        #[unsafe(method(handleGetAvailableRestaurantReservationBookingDefaults:completion:))]
        #[unsafe(method_family = none)]
        unsafe fn handleGetAvailableRestaurantReservationBookingDefaults_completion(
            &self,
            intent: &INGetAvailableRestaurantReservationBookingDefaultsIntent,
            completion: &block2::DynBlock<
                dyn Fn(NonNull<INGetAvailableRestaurantReservationBookingDefaultsIntentResponse>),
            >,
        );

        #[cfg(all(
            feature = "INGetAvailableRestaurantReservationBookingDefaultsIntentResponse",
            feature = "INIntent",
            feature = "INIntentResponse",
            feature = "block2"
        ))]
        /// Confirmation method - Validate that this intent is ready for the next step (i.e. handling)
        ///
        /// These methods are called prior to asking the app to handle the intent. The app should return a response object that contains additional information about the intent, which may be relevant for the system to show the user prior to handling. If unimplemented, the system will assume the intent is valid following resolution, and will assume there is no additional information relevant to this intent.
        ///
        ///
        /// Parameter `intent`: The input intent
        ///
        /// Parameter `completion`: The response block contains an INGetAvailableRestaurantReservationBookingDefaultsIntentResponse containing additional details about the intent that may be relevant for the system to show the user prior to handling.
        ///
        ///
        /// See: INGetAvailableRestaurantReservationBookingDefaultsIntentResponse
        #[optional]
        #[unsafe(method(confirmGetAvailableRestaurantReservationBookingDefaults:completion:))]
        #[unsafe(method_family = none)]
        unsafe fn confirmGetAvailableRestaurantReservationBookingDefaults_completion(
            &self,
            intent: &INGetAvailableRestaurantReservationBookingDefaultsIntent,
            completion: &block2::DynBlock<
                dyn Fn(NonNull<INGetAvailableRestaurantReservationBookingDefaultsIntentResponse>),
            >,
        );

        #[cfg(all(
            feature = "INIntent",
            feature = "INIntentResolutionResult",
            feature = "INRestaurantResolutionResult",
            feature = "block2"
        ))]
        /// Resolution methods - Determine if this intent is ready for the next step (confirmation)
        ///
        /// These methods are called to make sure the app extension is capable of handling this intent in its current form. This method is for validating if the intent needs any further fleshing out.
        ///
        ///
        /// Parameter `intent`: The input intent
        ///
        /// Parameter `completion`: The response block contains an INIntentResolutionResult for the parameter being resolved
        ///
        ///
        /// See: INGetAvailableRestaurantReservationBookingDefaultsIntentResponse
        #[optional]
        #[unsafe(method(resolveRestaurantForGetAvailableRestaurantReservationBookingDefaults:withCompletion:))]
        #[unsafe(method_family = none)]
        unsafe fn resolveRestaurantForGetAvailableRestaurantReservationBookingDefaults_withCompletion(
            &self,
            intent: &INGetAvailableRestaurantReservationBookingDefaultsIntent,
            completion: &block2::DynBlock<dyn Fn(NonNull<INRestaurantResolutionResult>)>,
        );
    }
);
