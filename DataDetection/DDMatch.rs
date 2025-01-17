//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A base class for common types of data that the data detection system
    /// matches.
    ///
    /// The DataDetection framework returns results in objects that are subclasses
    /// of `DDMatch`, which are specific to the type of matching data. Each object
    /// contains the matched string.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/datadetection/ddmatch?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct DDMatch;
);

unsafe impl NSObjectProtocol for DDMatch {}

extern_methods!(
    unsafe impl DDMatch {
        /// A substring that the data detection system identifies from an original
        /// string as a common type of data.
        ///
        /// Use `DDMatch` subclasses that the data detection system provides for a
        /// semantic interpretation of this string.
        #[unsafe(method_family(none))]
        #[method_id(matchedString)]
        pub unsafe fn matchedString(&self) -> Retained<NSString>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl DDMatch {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// An object that contains a web link that the data detection system matches.
    ///
    /// The DataDetection framework returns a link match in a `DDMatchLink` object, which contains a
    /// <doc
    /// ://com.apple.documentation/documentation/foundation/url> (Swift) or
    /// <doc
    /// ://com.apple.documentation/documentation/foundation/nsurl> (Objective-C).
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/datadetection/ddmatchlink?language=objc)
    #[unsafe(super(DDMatch, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct DDMatchLink;
);

unsafe impl NSObjectProtocol for DDMatchLink {}

extern_methods!(
    unsafe impl DDMatchLink {
        /// An address for a web resource, such as a webpage or image.
        #[unsafe(method_family(none))]
        #[method_id(URL)]
        pub unsafe fn URL(&self) -> Retained<NSURL>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DDMatch`
    unsafe impl DDMatchLink {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl DDMatchLink {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// An object that contains a phone number that the data detection system
    /// matches.
    ///
    /// The DataDetection framework returns a phone number match in a
    /// `DDMatchPhoneNumber` object, which contains a phone number, and optionally a
    /// label that categorizes the phone number.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/datadetection/ddmatchphonenumber?language=objc)
    #[unsafe(super(DDMatch, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct DDMatchPhoneNumber;
);

unsafe impl NSObjectProtocol for DDMatchPhoneNumber {}

extern_methods!(
    unsafe impl DDMatchPhoneNumber {
        /// A string that represents a phone number.
        #[unsafe(method_family(none))]
        #[method_id(phoneNumber)]
        pub unsafe fn phoneNumber(&self) -> Retained<NSString>;

        /// A string that categorizes a phone number, such as Home or Work.
        #[unsafe(method_family(none))]
        #[method_id(label)]
        pub unsafe fn label(&self) -> Option<Retained<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DDMatch`
    unsafe impl DDMatchPhoneNumber {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl DDMatchPhoneNumber {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// An object that contains an email address that the data detection system
    /// matches.
    ///
    /// The DataDetection framework returns an email match in a
    /// `DDMatchEmailAddress` object, which includes an email address, and
    /// optionally a label that categorizes the email address.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/datadetection/ddmatchemailaddress?language=objc)
    #[unsafe(super(DDMatch, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct DDMatchEmailAddress;
);

unsafe impl NSObjectProtocol for DDMatchEmailAddress {}

extern_methods!(
    unsafe impl DDMatchEmailAddress {
        /// A string that represents an email address.
        #[unsafe(method_family(none))]
        #[method_id(emailAddress)]
        pub unsafe fn emailAddress(&self) -> Retained<NSString>;

        /// A string that categorizes an email address, such as Home or Work.
        #[unsafe(method_family(none))]
        #[method_id(label)]
        pub unsafe fn label(&self) -> Option<Retained<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DDMatch`
    unsafe impl DDMatchEmailAddress {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl DDMatchEmailAddress {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// An object that contains a postal address that the data detection system
    /// matches.
    ///
    /// The DataDetection framework returns a postal address match in a
    /// `DDMatchPostalAddress` object, which optionally contains the matching parts
    /// of a postal address: street, city, state, postal code, and country.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/datadetection/ddmatchpostaladdress?language=objc)
    #[unsafe(super(DDMatch, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct DDMatchPostalAddress;
);

unsafe impl NSObjectProtocol for DDMatchPostalAddress {}

extern_methods!(
    unsafe impl DDMatchPostalAddress {
        /// The street name in a postal address.
        #[unsafe(method_family(none))]
        #[method_id(street)]
        pub unsafe fn street(&self) -> Option<Retained<NSString>>;

        /// The city name in a postal address.
        #[unsafe(method_family(none))]
        #[method_id(city)]
        pub unsafe fn city(&self) -> Option<Retained<NSString>>;

        /// The state name in a postal address.
        #[unsafe(method_family(none))]
        #[method_id(state)]
        pub unsafe fn state(&self) -> Option<Retained<NSString>>;

        /// The postal code in a postal address.
        #[unsafe(method_family(none))]
        #[method_id(postalCode)]
        pub unsafe fn postalCode(&self) -> Option<Retained<NSString>>;

        /// The country or region name in a postal address.
        #[unsafe(method_family(none))]
        #[method_id(country)]
        pub unsafe fn country(&self) -> Option<Retained<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DDMatch`
    unsafe impl DDMatchPostalAddress {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl DDMatchPostalAddress {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// An object that represents a calendar date or date range that the data
    /// detection system matches.
    ///
    /// The DataDetection framework returns a calendar event match in a
    /// `DDMatchCalendarEvent` object, which has only a beginning date, only an end
    /// date, or both a beginning date and an end date.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/datadetection/ddmatchcalendarevent?language=objc)
    #[unsafe(super(DDMatch, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct DDMatchCalendarEvent;
);

unsafe impl NSObjectProtocol for DDMatchCalendarEvent {}

extern_methods!(
    unsafe impl DDMatchCalendarEvent {
        /// A Boolean value that indicates whether the event is an all-day event.
        #[method(isAllDay)]
        pub unsafe fn isAllDay(&self) -> bool;

        /// A date that represents the start of the event.
        #[unsafe(method_family(none))]
        #[method_id(startDate)]
        pub unsafe fn startDate(&self) -> Option<Retained<NSDate>>;

        /// The time zone for the event’s start date.
        #[unsafe(method_family(none))]
        #[method_id(startTimeZone)]
        pub unsafe fn startTimeZone(&self) -> Option<Retained<NSTimeZone>>;

        /// A date that represents the end of the event.
        #[unsafe(method_family(none))]
        #[method_id(endDate)]
        pub unsafe fn endDate(&self) -> Option<Retained<NSDate>>;

        /// The time zone for the event’s end date.
        #[unsafe(method_family(none))]
        #[method_id(endTimeZone)]
        pub unsafe fn endTimeZone(&self) -> Option<Retained<NSTimeZone>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DDMatch`
    unsafe impl DDMatchCalendarEvent {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl DDMatchCalendarEvent {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// An object that contains parcel tracking information that the data detection
    /// system matches.
    ///
    /// The DataDetection framework returns a shipment tracking number match in a
    /// `DDMatchShipmentTrackingNumber` object, which contains a carrier name and
    /// tracking identifier.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/datadetection/ddmatchshipmenttrackingnumber?language=objc)
    #[unsafe(super(DDMatch, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct DDMatchShipmentTrackingNumber;
);

unsafe impl NSObjectProtocol for DDMatchShipmentTrackingNumber {}

extern_methods!(
    unsafe impl DDMatchShipmentTrackingNumber {
        /// The name of a parcel carrier.
        #[unsafe(method_family(none))]
        #[method_id(carrier)]
        pub unsafe fn carrier(&self) -> Retained<NSString>;

        /// A string that represents a carrier’s tracking identifier for a parcel.
        #[unsafe(method_family(none))]
        #[method_id(trackingNumber)]
        pub unsafe fn trackingNumber(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DDMatch`
    unsafe impl DDMatchShipmentTrackingNumber {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl DDMatchShipmentTrackingNumber {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// An object that contains a flight number that the data detection system
    /// matches.
    ///
    /// The DataDetection framework returns a flight number match in a
    /// `DDMatchFlightNumber` object, which contains an airline name and flight
    /// number.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/datadetection/ddmatchflightnumber?language=objc)
    #[unsafe(super(DDMatch, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct DDMatchFlightNumber;
);

unsafe impl NSObjectProtocol for DDMatchFlightNumber {}

extern_methods!(
    unsafe impl DDMatchFlightNumber {
        /// The name of an airline.
        #[unsafe(method_family(none))]
        #[method_id(airline)]
        pub unsafe fn airline(&self) -> Retained<NSString>;

        /// A string that represents a flight number.
        #[unsafe(method_family(none))]
        #[method_id(flightNumber)]
        pub unsafe fn flightNumber(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DDMatch`
    unsafe impl DDMatchFlightNumber {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl DDMatchFlightNumber {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// An object that contains an amount of money that the data detection system
    /// matches.
    ///
    /// The DataDetection framework returns a match for an amount of money in a
    /// `DDMatchMoneyAmount` object, which contains an amount of money and an ISO
    /// currency code.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/datadetection/ddmatchmoneyamount?language=objc)
    #[unsafe(super(DDMatch, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct DDMatchMoneyAmount;
);

unsafe impl NSObjectProtocol for DDMatchMoneyAmount {}

extern_methods!(
    unsafe impl DDMatchMoneyAmount {
        /// A string that contains an ISO currency code, which the data detection system
        /// identifies from the matched string and user preferences.
        #[unsafe(method_family(none))]
        #[method_id(currency)]
        pub unsafe fn currency(&self) -> Retained<NSString>;

        /// A number that represents an amount of money.
        #[method(amount)]
        pub unsafe fn amount(&self) -> c_double;
    }
);

extern_methods!(
    /// Methods declared on superclass `DDMatch`
    unsafe impl DDMatchMoneyAmount {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl DDMatchMoneyAmount {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
