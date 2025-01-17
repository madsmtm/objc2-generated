//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A mutable value object representing a postal address.
    ///
    ///
    /// CNMutablePostalAddress is not thread safe.
    ///
    ///
    /// Note: To remove properties when saving a mutable postal address, set string properties to empty values.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/contacts/cnmutablepostaladdress?language=objc)
    #[unsafe(super(CNPostalAddress, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CNPostalAddress")]
    pub struct CNMutablePostalAddress;
);

#[cfg(feature = "CNPostalAddress")]
unsafe impl NSCoding for CNMutablePostalAddress {}

#[cfg(feature = "CNPostalAddress")]
unsafe impl NSCopying for CNMutablePostalAddress {}

#[cfg(feature = "CNPostalAddress")]
unsafe impl CopyingHelper for CNMutablePostalAddress {
    type Result = CNPostalAddress;
}

#[cfg(feature = "CNPostalAddress")]
unsafe impl NSMutableCopying for CNMutablePostalAddress {}

#[cfg(feature = "CNPostalAddress")]
unsafe impl MutableCopyingHelper for CNMutablePostalAddress {
    type Result = Self;
}

#[cfg(feature = "CNPostalAddress")]
unsafe impl NSObjectProtocol for CNMutablePostalAddress {}

#[cfg(feature = "CNPostalAddress")]
unsafe impl NSSecureCoding for CNMutablePostalAddress {}

extern_methods!(
    #[cfg(feature = "CNPostalAddress")]
    unsafe impl CNMutablePostalAddress {
        /// multi-street address is delimited with carriage returns “
        /// \n”
        #[unsafe(method_family(none))]
        #[method_id(street)]
        pub unsafe fn street(&self) -> Retained<NSString>;

        /// Setter for [`street`][Self::street].
        #[method(setStreet:)]
        pub unsafe fn setStreet(&self, street: &NSString);

        #[unsafe(method_family(none))]
        #[method_id(subLocality)]
        pub unsafe fn subLocality(&self) -> Retained<NSString>;

        /// Setter for [`subLocality`][Self::subLocality].
        #[method(setSubLocality:)]
        pub unsafe fn setSubLocality(&self, sub_locality: &NSString);

        #[unsafe(method_family(none))]
        #[method_id(city)]
        pub unsafe fn city(&self) -> Retained<NSString>;

        /// Setter for [`city`][Self::city].
        #[method(setCity:)]
        pub unsafe fn setCity(&self, city: &NSString);

        #[unsafe(method_family(none))]
        #[method_id(subAdministrativeArea)]
        pub unsafe fn subAdministrativeArea(&self) -> Retained<NSString>;

        /// Setter for [`subAdministrativeArea`][Self::subAdministrativeArea].
        #[method(setSubAdministrativeArea:)]
        pub unsafe fn setSubAdministrativeArea(&self, sub_administrative_area: &NSString);

        #[unsafe(method_family(none))]
        #[method_id(state)]
        pub unsafe fn state(&self) -> Retained<NSString>;

        /// Setter for [`state`][Self::state].
        #[method(setState:)]
        pub unsafe fn setState(&self, state: &NSString);

        #[unsafe(method_family(none))]
        #[method_id(postalCode)]
        pub unsafe fn postalCode(&self) -> Retained<NSString>;

        /// Setter for [`postalCode`][Self::postalCode].
        #[method(setPostalCode:)]
        pub unsafe fn setPostalCode(&self, postal_code: &NSString);

        #[unsafe(method_family(none))]
        #[method_id(country)]
        pub unsafe fn country(&self) -> Retained<NSString>;

        /// Setter for [`country`][Self::country].
        #[method(setCountry:)]
        pub unsafe fn setCountry(&self, country: &NSString);

        #[unsafe(method_family(none))]
        #[method_id(ISOCountryCode)]
        pub unsafe fn ISOCountryCode(&self) -> Retained<NSString>;

        /// Setter for [`ISOCountryCode`][Self::ISOCountryCode].
        #[method(setISOCountryCode:)]
        pub unsafe fn setISOCountryCode(&self, iso_country_code: &NSString);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CNPostalAddress")]
    unsafe impl CNMutablePostalAddress {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
