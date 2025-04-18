//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/intents/indatesearchtyperesolutionresult?language=objc)
    #[unsafe(super(INIntentResolutionResult, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "INIntentResolutionResult")]
    pub struct INDateSearchTypeResolutionResult;
);

#[cfg(feature = "INIntentResolutionResult")]
extern_conformance!(
    unsafe impl NSObjectProtocol for INDateSearchTypeResolutionResult {}
);

#[cfg(feature = "INIntentResolutionResult")]
impl INDateSearchTypeResolutionResult {
    extern_methods!(
        #[cfg(feature = "INDateSearchType")]
        #[unsafe(method(successWithResolvedDateSearchType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn successWithResolvedDateSearchType(
            resolved_date_search_type: INDateSearchType,
        ) -> Retained<Self>;

        #[cfg(feature = "INDateSearchType")]
        #[unsafe(method(confirmationRequiredWithDateSearchTypeToConfirm:))]
        #[unsafe(method_family = none)]
        pub unsafe fn confirmationRequiredWithDateSearchTypeToConfirm(
            date_search_type_to_confirm: INDateSearchType,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `INIntentResolutionResult`.
#[cfg(feature = "INIntentResolutionResult")]
impl INDateSearchTypeResolutionResult {
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
impl INDateSearchTypeResolutionResult {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
