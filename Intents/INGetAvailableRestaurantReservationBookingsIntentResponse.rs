//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/intents/ingetavailablerestaurantreservationbookingsintentcode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct INGetAvailableRestaurantReservationBookingsIntentCode(pub NSInteger);
impl INGetAvailableRestaurantReservationBookingsIntentCode {
    #[doc(alias = "INGetAvailableRestaurantReservationBookingsIntentCodeSuccess")]
    pub const Success: Self = Self(0);
    #[doc(alias = "INGetAvailableRestaurantReservationBookingsIntentCodeFailure")]
    pub const Failure: Self = Self(1);
    #[doc(
        alias = "INGetAvailableRestaurantReservationBookingsIntentCodeFailureRequestUnsatisfiable"
    )]
    pub const FailureRequestUnsatisfiable: Self = Self(2);
    #[doc(alias = "INGetAvailableRestaurantReservationBookingsIntentCodeFailureRequestUnspecified")]
    pub const FailureRequestUnspecified: Self = Self(3);
}

unsafe impl Encode for INGetAvailableRestaurantReservationBookingsIntentCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for INGetAvailableRestaurantReservationBookingsIntentCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/intents/ingetavailablerestaurantreservationbookingsintentresponse?language=objc)
    #[unsafe(super(INIntentResponse, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "INIntentResponse")]
    pub struct INGetAvailableRestaurantReservationBookingsIntentResponse;
);

#[cfg(feature = "INIntentResponse")]
extern_conformance!(
    unsafe impl NSCoding for INGetAvailableRestaurantReservationBookingsIntentResponse {}
);

#[cfg(feature = "INIntentResponse")]
extern_conformance!(
    unsafe impl NSCopying for INGetAvailableRestaurantReservationBookingsIntentResponse {}
);

#[cfg(feature = "INIntentResponse")]
unsafe impl CopyingHelper for INGetAvailableRestaurantReservationBookingsIntentResponse {
    type Result = Self;
}

#[cfg(feature = "INIntentResponse")]
extern_conformance!(
    unsafe impl NSObjectProtocol for INGetAvailableRestaurantReservationBookingsIntentResponse {}
);

#[cfg(feature = "INIntentResponse")]
extern_conformance!(
    unsafe impl NSSecureCoding for INGetAvailableRestaurantReservationBookingsIntentResponse {}
);

#[cfg(feature = "INIntentResponse")]
impl INGetAvailableRestaurantReservationBookingsIntentResponse {
    extern_methods!(
        #[cfg(feature = "INRestaurantReservationBooking")]
        #[unsafe(method(initWithAvailableBookings:code:userActivity:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithAvailableBookings_code_userActivity(
            this: Allocated<Self>,
            available_bookings: &NSArray<INRestaurantReservationBooking>,
            code: INGetAvailableRestaurantReservationBookingsIntentCode,
            user_activity: Option<&NSUserActivity>,
        ) -> Retained<Self>;

        #[unsafe(method(code))]
        #[unsafe(method_family = none)]
        pub unsafe fn code(&self) -> INGetAvailableRestaurantReservationBookingsIntentCode;

        #[unsafe(method(localizedRestaurantDescriptionText))]
        #[unsafe(method_family = none)]
        pub unsafe fn localizedRestaurantDescriptionText(&self) -> Option<Retained<NSString>>;

        /// Setter for [`localizedRestaurantDescriptionText`][Self::localizedRestaurantDescriptionText].
        #[unsafe(method(setLocalizedRestaurantDescriptionText:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLocalizedRestaurantDescriptionText(
            &self,
            localized_restaurant_description_text: Option<&NSString>,
        );

        #[unsafe(method(localizedBookingAdvisementText))]
        #[unsafe(method_family = none)]
        pub unsafe fn localizedBookingAdvisementText(&self) -> Option<Retained<NSString>>;

        /// Setter for [`localizedBookingAdvisementText`][Self::localizedBookingAdvisementText].
        #[unsafe(method(setLocalizedBookingAdvisementText:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLocalizedBookingAdvisementText(
            &self,
            localized_booking_advisement_text: Option<&NSString>,
        );

        #[cfg(feature = "INTermsAndConditions")]
        #[unsafe(method(termsAndConditions))]
        #[unsafe(method_family = none)]
        pub unsafe fn termsAndConditions(&self) -> Option<Retained<INTermsAndConditions>>;

        #[cfg(feature = "INTermsAndConditions")]
        /// Setter for [`termsAndConditions`][Self::termsAndConditions].
        #[unsafe(method(setTermsAndConditions:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTermsAndConditions(
            &self,
            terms_and_conditions: Option<&INTermsAndConditions>,
        );

        #[cfg(feature = "INRestaurantReservationBooking")]
        #[unsafe(method(availableBookings))]
        #[unsafe(method_family = none)]
        pub unsafe fn availableBookings(&self)
            -> Retained<NSArray<INRestaurantReservationBooking>>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "INIntentResponse")]
impl INGetAvailableRestaurantReservationBookingsIntentResponse {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
