//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/intents/inbillpayeeresolutionresult?language=objc)
    #[unsafe(super(INIntentResolutionResult, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "INIntentResolutionResult")]
    #[deprecated]
    pub struct INBillPayeeResolutionResult;
);

#[cfg(feature = "INIntentResolutionResult")]
extern_conformance!(
    unsafe impl NSObjectProtocol for INBillPayeeResolutionResult {}
);

#[cfg(feature = "INIntentResolutionResult")]
impl INBillPayeeResolutionResult {
    extern_methods!(
        #[cfg(feature = "INBillPayee")]
        #[deprecated]
        #[unsafe(method(successWithResolvedBillPayee:))]
        #[unsafe(method_family = none)]
        pub unsafe fn successWithResolvedBillPayee(
            resolved_bill_payee: &INBillPayee,
        ) -> Retained<Self>;

        #[cfg(feature = "INBillPayee")]
        #[deprecated]
        #[unsafe(method(disambiguationWithBillPayeesToDisambiguate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn disambiguationWithBillPayeesToDisambiguate(
            bill_payees_to_disambiguate: &NSArray<INBillPayee>,
        ) -> Retained<Self>;

        #[cfg(feature = "INBillPayee")]
        #[deprecated]
        #[unsafe(method(confirmationRequiredWithBillPayeeToConfirm:))]
        #[unsafe(method_family = none)]
        pub unsafe fn confirmationRequiredWithBillPayeeToConfirm(
            bill_payee_to_confirm: Option<&INBillPayee>,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `INIntentResolutionResult`.
#[cfg(feature = "INIntentResolutionResult")]
impl INBillPayeeResolutionResult {
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
impl INBillPayeeResolutionResult {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
