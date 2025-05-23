//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkdirectionsroutepreference?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MKDirectionsRoutePreference(pub NSInteger);
impl MKDirectionsRoutePreference {
    #[doc(alias = "MKDirectionsRoutePreferenceAny")]
    pub const Any: Self = Self(0);
    #[doc(alias = "MKDirectionsRoutePreferenceAvoid")]
    pub const Avoid: Self = Self(1);
}

unsafe impl Encode for MKDirectionsRoutePreference {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MKDirectionsRoutePreference {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkdirectionsrequest?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKDirectionsRequest;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MKDirectionsRequest {}
);

impl MKDirectionsRequest {
    extern_methods!(
        #[cfg(feature = "MKMapItem")]
        #[unsafe(method(source))]
        #[unsafe(method_family = none)]
        pub unsafe fn source(&self) -> Option<Retained<MKMapItem>>;

        #[cfg(feature = "MKMapItem")]
        /// Setter for [`source`][Self::source].
        #[unsafe(method(setSource:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSource(&self, source: Option<&MKMapItem>);

        #[cfg(feature = "MKMapItem")]
        #[unsafe(method(destination))]
        #[unsafe(method_family = none)]
        pub unsafe fn destination(&self) -> Option<Retained<MKMapItem>>;

        #[cfg(feature = "MKMapItem")]
        /// Setter for [`destination`][Self::destination].
        #[unsafe(method(setDestination:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDestination(&self, destination: Option<&MKMapItem>);
    );
}

/// Methods declared on superclass `NSObject`.
impl MKDirectionsRequest {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

/// MKRequestOptions.
impl MKDirectionsRequest {
    extern_methods!(
        #[cfg(feature = "MKDirectionsTypes")]
        #[unsafe(method(transportType))]
        #[unsafe(method_family = none)]
        pub unsafe fn transportType(&self) -> MKDirectionsTransportType;

        #[cfg(feature = "MKDirectionsTypes")]
        /// Setter for [`transportType`][Self::transportType].
        #[unsafe(method(setTransportType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTransportType(&self, transport_type: MKDirectionsTransportType);

        #[unsafe(method(requestsAlternateRoutes))]
        #[unsafe(method_family = none)]
        pub unsafe fn requestsAlternateRoutes(&self) -> bool;

        /// Setter for [`requestsAlternateRoutes`][Self::requestsAlternateRoutes].
        #[unsafe(method(setRequestsAlternateRoutes:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRequestsAlternateRoutes(&self, requests_alternate_routes: bool);

        #[unsafe(method(departureDate))]
        #[unsafe(method_family = none)]
        pub unsafe fn departureDate(&self) -> Option<Retained<NSDate>>;

        /// Setter for [`departureDate`][Self::departureDate].
        #[unsafe(method(setDepartureDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDepartureDate(&self, departure_date: Option<&NSDate>);

        #[unsafe(method(arrivalDate))]
        #[unsafe(method_family = none)]
        pub unsafe fn arrivalDate(&self) -> Option<Retained<NSDate>>;

        /// Setter for [`arrivalDate`][Self::arrivalDate].
        #[unsafe(method(setArrivalDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setArrivalDate(&self, arrival_date: Option<&NSDate>);

        #[unsafe(method(tollPreference))]
        #[unsafe(method_family = none)]
        pub unsafe fn tollPreference(&self) -> MKDirectionsRoutePreference;

        /// Setter for [`tollPreference`][Self::tollPreference].
        #[unsafe(method(setTollPreference:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTollPreference(&self, toll_preference: MKDirectionsRoutePreference);

        #[unsafe(method(highwayPreference))]
        #[unsafe(method_family = none)]
        pub unsafe fn highwayPreference(&self) -> MKDirectionsRoutePreference;

        /// Setter for [`highwayPreference`][Self::highwayPreference].
        #[unsafe(method(setHighwayPreference:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setHighwayPreference(&self, highway_preference: MKDirectionsRoutePreference);
    );
}

/// MKDirectionsURL.
impl MKDirectionsRequest {
    extern_methods!(
        #[unsafe(method(initWithContentsOfURL:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithContentsOfURL(this: Allocated<Self>, url: &NSURL) -> Retained<Self>;

        #[unsafe(method(isDirectionsRequestURL:))]
        #[unsafe(method_family = none)]
        pub unsafe fn isDirectionsRequestURL(url: &NSURL) -> bool;
    );
}
