//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/intents/inplacemarkresolutionresult?language=objc)
    #[unsafe(super(INIntentResolutionResult, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "INIntentResolutionResult")]
    pub struct INPlacemarkResolutionResult;
);

#[cfg(feature = "INIntentResolutionResult")]
extern_conformance!(
    unsafe impl NSObjectProtocol for INPlacemarkResolutionResult {}
);

#[cfg(feature = "INIntentResolutionResult")]
impl INPlacemarkResolutionResult {
    extern_methods!(
        #[cfg(feature = "objc2-core-location")]
        #[unsafe(method(successWithResolvedPlacemark:))]
        #[unsafe(method_family = none)]
        pub unsafe fn successWithResolvedPlacemark(
            resolved_placemark: &CLPlacemark,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-location")]
        #[unsafe(method(disambiguationWithPlacemarksToDisambiguate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn disambiguationWithPlacemarksToDisambiguate(
            placemarks_to_disambiguate: &NSArray<CLPlacemark>,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-location")]
        #[unsafe(method(confirmationRequiredWithPlacemarkToConfirm:))]
        #[unsafe(method_family = none)]
        pub unsafe fn confirmationRequiredWithPlacemarkToConfirm(
            placemark_to_confirm: Option<&CLPlacemark>,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `INIntentResolutionResult`.
#[cfg(feature = "INIntentResolutionResult")]
impl INPlacemarkResolutionResult {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(needsValue))]
        #[unsafe(method_family = none)]
        pub unsafe fn needsValue() -> Retained<Self>;

        #[unsafe(method(notRequired))]
        #[unsafe(method_family = none)]
        pub unsafe fn notRequired() -> Retained<Self>;

        #[unsafe(method(unsupported))]
        #[unsafe(method_family = none)]
        pub unsafe fn unsupported() -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "INIntentResolutionResult")]
impl INPlacemarkResolutionResult {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
