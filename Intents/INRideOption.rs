//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/intents/inrideoption?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct INRideOption;
);

extern_conformance!(
    unsafe impl NSCoding for INRideOption {}
);

extern_conformance!(
    unsafe impl NSCopying for INRideOption {}
);

unsafe impl CopyingHelper for INRideOption {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for INRideOption {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for INRideOption {}
);

impl INRideOption {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(initWithName:estimatedPickupDate:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithName_estimatedPickupDate(
            this: Allocated<Self>,
            name: &NSString,
            estimated_pickup_date: &NSDate,
        ) -> Retained<Self>;

        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            decoder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        /// Setter for [`name`][Self::name].
        #[unsafe(method(setName:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setName(&self, name: &NSString);

        #[unsafe(method(estimatedPickupDate))]
        #[unsafe(method_family = none)]
        pub unsafe fn estimatedPickupDate(&self) -> Retained<NSDate>;

        /// Setter for [`estimatedPickupDate`][Self::estimatedPickupDate].
        #[unsafe(method(setEstimatedPickupDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEstimatedPickupDate(&self, estimated_pickup_date: &NSDate);

        #[cfg(feature = "INPriceRange")]
        #[unsafe(method(priceRange))]
        #[unsafe(method_family = none)]
        pub unsafe fn priceRange(&self) -> Option<Retained<INPriceRange>>;

        #[cfg(feature = "INPriceRange")]
        /// Setter for [`priceRange`][Self::priceRange].
        #[unsafe(method(setPriceRange:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPriceRange(&self, price_range: Option<&INPriceRange>);

        #[unsafe(method(usesMeteredFare))]
        #[unsafe(method_family = none)]
        pub unsafe fn usesMeteredFare(&self) -> Option<Retained<NSNumber>>;

        /// Setter for [`usesMeteredFare`][Self::usesMeteredFare].
        #[unsafe(method(setUsesMeteredFare:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setUsesMeteredFare(&self, uses_metered_fare: Option<&NSNumber>);

        #[unsafe(method(disclaimerMessage))]
        #[unsafe(method_family = none)]
        pub unsafe fn disclaimerMessage(&self) -> Option<Retained<NSString>>;

        /// Setter for [`disclaimerMessage`][Self::disclaimerMessage].
        #[unsafe(method(setDisclaimerMessage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDisclaimerMessage(&self, disclaimer_message: Option<&NSString>);

        #[cfg(feature = "INRidePartySizeOption")]
        #[unsafe(method(availablePartySizeOptions))]
        #[unsafe(method_family = none)]
        pub unsafe fn availablePartySizeOptions(
            &self,
        ) -> Option<Retained<NSArray<INRidePartySizeOption>>>;

        #[cfg(feature = "INRidePartySizeOption")]
        /// Setter for [`availablePartySizeOptions`][Self::availablePartySizeOptions].
        #[unsafe(method(setAvailablePartySizeOptions:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAvailablePartySizeOptions(
            &self,
            available_party_size_options: Option<&NSArray<INRidePartySizeOption>>,
        );

        #[unsafe(method(availablePartySizeOptionsSelectionPrompt))]
        #[unsafe(method_family = none)]
        pub unsafe fn availablePartySizeOptionsSelectionPrompt(&self)
            -> Option<Retained<NSString>>;

        /// Setter for [`availablePartySizeOptionsSelectionPrompt`][Self::availablePartySizeOptionsSelectionPrompt].
        #[unsafe(method(setAvailablePartySizeOptionsSelectionPrompt:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAvailablePartySizeOptionsSelectionPrompt(
            &self,
            available_party_size_options_selection_prompt: Option<&NSString>,
        );

        #[unsafe(method(specialPricing))]
        #[unsafe(method_family = none)]
        pub unsafe fn specialPricing(&self) -> Option<Retained<NSString>>;

        /// Setter for [`specialPricing`][Self::specialPricing].
        #[unsafe(method(setSpecialPricing:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSpecialPricing(&self, special_pricing: Option<&NSString>);

        #[cfg(feature = "INImage")]
        #[unsafe(method(specialPricingBadgeImage))]
        #[unsafe(method_family = none)]
        pub unsafe fn specialPricingBadgeImage(&self) -> Option<Retained<INImage>>;

        #[cfg(feature = "INImage")]
        /// Setter for [`specialPricingBadgeImage`][Self::specialPricingBadgeImage].
        #[unsafe(method(setSpecialPricingBadgeImage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSpecialPricingBadgeImage(
            &self,
            special_pricing_badge_image: Option<&INImage>,
        );

        #[cfg(feature = "INRideFareLineItem")]
        #[unsafe(method(fareLineItems))]
        #[unsafe(method_family = none)]
        pub unsafe fn fareLineItems(&self) -> Option<Retained<NSArray<INRideFareLineItem>>>;

        #[cfg(feature = "INRideFareLineItem")]
        /// Setter for [`fareLineItems`][Self::fareLineItems].
        #[unsafe(method(setFareLineItems:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFareLineItems(
            &self,
            fare_line_items: Option<&NSArray<INRideFareLineItem>>,
        );

        #[unsafe(method(userActivityForBookingInApplication))]
        #[unsafe(method_family = none)]
        pub unsafe fn userActivityForBookingInApplication(
            &self,
        ) -> Option<Retained<NSUserActivity>>;

        /// Setter for [`userActivityForBookingInApplication`][Self::userActivityForBookingInApplication].
        #[unsafe(method(setUserActivityForBookingInApplication:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setUserActivityForBookingInApplication(
            &self,
            user_activity_for_booking_in_application: Option<&NSUserActivity>,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl INRideOption {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

/// Deprecated.
impl INRideOption {
    extern_methods!(
        #[unsafe(method(identifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn identifier(&self) -> Option<Retained<NSString>>;

        /// Setter for [`identifier`][Self::identifier].
        #[unsafe(method(setIdentifier:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setIdentifier(&self, identifier: Option<&NSString>);
    );
}
