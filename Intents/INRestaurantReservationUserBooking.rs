//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/intents/inrestaurantreservationuserbookingstatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct INRestaurantReservationUserBookingStatus(pub NSUInteger);
impl INRestaurantReservationUserBookingStatus {
    #[doc(alias = "INRestaurantReservationUserBookingStatusPending")]
    pub const Pending: Self = Self(0);
    #[doc(alias = "INRestaurantReservationUserBookingStatusConfirmed")]
    pub const Confirmed: Self = Self(1);
    #[doc(alias = "INRestaurantReservationUserBookingStatusDenied")]
    pub const Denied: Self = Self(2);
}

unsafe impl Encode for INRestaurantReservationUserBookingStatus {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for INRestaurantReservationUserBookingStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/intents/inrestaurantreservationuserbooking?language=objc)
    #[unsafe(super(INRestaurantReservationBooking, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "INRestaurantReservationBooking")]
    pub struct INRestaurantReservationUserBooking;
);

#[cfg(feature = "INRestaurantReservationBooking")]
extern_conformance!(
    unsafe impl NSCoding for INRestaurantReservationUserBooking {}
);

#[cfg(feature = "INRestaurantReservationBooking")]
extern_conformance!(
    unsafe impl NSCopying for INRestaurantReservationUserBooking {}
);

#[cfg(feature = "INRestaurantReservationBooking")]
unsafe impl CopyingHelper for INRestaurantReservationUserBooking {
    type Result = Self;
}

#[cfg(feature = "INRestaurantReservationBooking")]
extern_conformance!(
    unsafe impl NSObjectProtocol for INRestaurantReservationUserBooking {}
);

#[cfg(feature = "INRestaurantReservationBooking")]
extern_conformance!(
    unsafe impl NSSecureCoding for INRestaurantReservationUserBooking {}
);

#[cfg(feature = "INRestaurantReservationBooking")]
impl INRestaurantReservationUserBooking {
    extern_methods!(
        #[cfg(all(
            feature = "INPerson",
            feature = "INRestaurant",
            feature = "INRestaurantGuest"
        ))]
        #[unsafe(method(initWithRestaurant:bookingDate:partySize:bookingIdentifier:guest:status:dateStatusModified:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithRestaurant_bookingDate_partySize_bookingIdentifier_guest_status_dateStatusModified(
            this: Allocated<Self>,
            restaurant: &INRestaurant,
            booking_date: &NSDate,
            party_size: NSUInteger,
            booking_identifier: &NSString,
            guest: &INRestaurantGuest,
            status: INRestaurantReservationUserBookingStatus,
            date_status_modified: &NSDate,
        ) -> Retained<Self>;

        #[cfg(all(feature = "INPerson", feature = "INRestaurantGuest"))]
        #[unsafe(method(guest))]
        #[unsafe(method_family = none)]
        pub unsafe fn guest(&self) -> Retained<INRestaurantGuest>;

        #[cfg(all(feature = "INPerson", feature = "INRestaurantGuest"))]
        /// Setter for [`guest`][Self::guest].
        #[unsafe(method(setGuest:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setGuest(&self, guest: &INRestaurantGuest);

        #[unsafe(method(advisementText))]
        #[unsafe(method_family = none)]
        pub unsafe fn advisementText(&self) -> Option<Retained<NSString>>;

        /// Setter for [`advisementText`][Self::advisementText].
        #[unsafe(method(setAdvisementText:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAdvisementText(&self, advisement_text: Option<&NSString>);

        #[cfg(feature = "INRestaurantOffer")]
        #[unsafe(method(selectedOffer))]
        #[unsafe(method_family = none)]
        pub unsafe fn selectedOffer(&self) -> Option<Retained<INRestaurantOffer>>;

        #[cfg(feature = "INRestaurantOffer")]
        /// Setter for [`selectedOffer`][Self::selectedOffer].
        #[unsafe(method(setSelectedOffer:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSelectedOffer(&self, selected_offer: Option<&INRestaurantOffer>);

        #[unsafe(method(guestProvidedSpecialRequestText))]
        #[unsafe(method_family = none)]
        pub unsafe fn guestProvidedSpecialRequestText(&self) -> Option<Retained<NSString>>;

        /// Setter for [`guestProvidedSpecialRequestText`][Self::guestProvidedSpecialRequestText].
        #[unsafe(method(setGuestProvidedSpecialRequestText:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setGuestProvidedSpecialRequestText(
            &self,
            guest_provided_special_request_text: Option<&NSString>,
        );

        #[unsafe(method(status))]
        #[unsafe(method_family = none)]
        pub unsafe fn status(&self) -> INRestaurantReservationUserBookingStatus;

        /// Setter for [`status`][Self::status].
        #[unsafe(method(setStatus:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setStatus(&self, status: INRestaurantReservationUserBookingStatus);

        #[unsafe(method(dateStatusModified))]
        #[unsafe(method_family = none)]
        pub unsafe fn dateStatusModified(&self) -> Retained<NSDate>;

        /// Setter for [`dateStatusModified`][Self::dateStatusModified].
        #[unsafe(method(setDateStatusModified:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDateStatusModified(&self, date_status_modified: &NSDate);
    );
}

/// Methods declared on superclass `INRestaurantReservationBooking`.
#[cfg(feature = "INRestaurantReservationBooking")]
impl INRestaurantReservationUserBooking {
    extern_methods!(
        #[cfg(feature = "INRestaurant")]
        #[unsafe(method(initWithRestaurant:bookingDate:partySize:bookingIdentifier:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithRestaurant_bookingDate_partySize_bookingIdentifier(
            this: Allocated<Self>,
            restaurant: &INRestaurant,
            booking_date: &NSDate,
            party_size: NSUInteger,
            booking_identifier: &NSString,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "INRestaurantReservationBooking")]
impl INRestaurantReservationUserBooking {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
