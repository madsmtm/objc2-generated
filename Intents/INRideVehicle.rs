//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/intents/inridevehicle?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct INRideVehicle;
);

extern_conformance!(
    unsafe impl NSCoding for INRideVehicle {}
);

extern_conformance!(
    unsafe impl NSCopying for INRideVehicle {}
);

unsafe impl CopyingHelper for INRideVehicle {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for INRideVehicle {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for INRideVehicle {}
);

impl INRideVehicle {
    extern_methods!(
        #[cfg(feature = "objc2-core-location")]
        #[unsafe(method(location))]
        #[unsafe(method_family = none)]
        pub unsafe fn location(&self) -> Option<Retained<CLLocation>>;

        #[cfg(feature = "objc2-core-location")]
        /// Setter for [`location`][Self::location].
        #[unsafe(method(setLocation:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLocation(&self, location: Option<&CLLocation>);

        #[unsafe(method(registrationPlate))]
        #[unsafe(method_family = none)]
        pub unsafe fn registrationPlate(&self) -> Option<Retained<NSString>>;

        /// Setter for [`registrationPlate`][Self::registrationPlate].
        #[unsafe(method(setRegistrationPlate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRegistrationPlate(&self, registration_plate: Option<&NSString>);

        #[unsafe(method(manufacturer))]
        #[unsafe(method_family = none)]
        pub unsafe fn manufacturer(&self) -> Option<Retained<NSString>>;

        /// Setter for [`manufacturer`][Self::manufacturer].
        #[unsafe(method(setManufacturer:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setManufacturer(&self, manufacturer: Option<&NSString>);

        #[unsafe(method(model))]
        #[unsafe(method_family = none)]
        pub unsafe fn model(&self) -> Option<Retained<NSString>>;

        /// Setter for [`model`][Self::model].
        #[unsafe(method(setModel:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setModel(&self, model: Option<&NSString>);

        #[cfg(feature = "INImage")]
        #[unsafe(method(mapAnnotationImage))]
        #[unsafe(method_family = none)]
        pub unsafe fn mapAnnotationImage(&self) -> Option<Retained<INImage>>;

        #[cfg(feature = "INImage")]
        /// Setter for [`mapAnnotationImage`][Self::mapAnnotationImage].
        #[unsafe(method(setMapAnnotationImage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMapAnnotationImage(&self, map_annotation_image: Option<&INImage>);
    );
}

/// Methods declared on superclass `NSObject`.
impl INRideVehicle {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
