//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/intents/inmessageattributeresolutionresult?language=objc)
    #[unsafe(super(INIntentResolutionResult, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "INIntentResolutionResult")]
    pub struct INMessageAttributeResolutionResult;
);

#[cfg(feature = "INIntentResolutionResult")]
extern_conformance!(
    unsafe impl NSObjectProtocol for INMessageAttributeResolutionResult {}
);

#[cfg(feature = "INIntentResolutionResult")]
impl INMessageAttributeResolutionResult {
    extern_methods!(
        #[cfg(feature = "INMessageAttribute")]
        #[unsafe(method(successWithResolvedMessageAttribute:))]
        #[unsafe(method_family = none)]
        pub unsafe fn successWithResolvedMessageAttribute(
            resolved_message_attribute: INMessageAttribute,
        ) -> Retained<Self>;

        #[cfg(feature = "INMessageAttribute")]
        #[deprecated]
        #[unsafe(method(successWithResolvedValue:))]
        #[unsafe(method_family = none)]
        pub unsafe fn successWithResolvedValue(
            resolved_value: INMessageAttribute,
        ) -> Retained<Self>;

        #[cfg(feature = "INMessageAttribute")]
        #[unsafe(method(confirmationRequiredWithMessageAttributeToConfirm:))]
        #[unsafe(method_family = none)]
        pub unsafe fn confirmationRequiredWithMessageAttributeToConfirm(
            message_attribute_to_confirm: INMessageAttribute,
        ) -> Retained<Self>;

        #[cfg(feature = "INMessageAttribute")]
        #[deprecated]
        #[unsafe(method(confirmationRequiredWithValueToConfirm:))]
        #[unsafe(method_family = none)]
        pub unsafe fn confirmationRequiredWithValueToConfirm(
            value_to_confirm: INMessageAttribute,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `INIntentResolutionResult`.
#[cfg(feature = "INIntentResolutionResult")]
impl INMessageAttributeResolutionResult {
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
impl INMessageAttributeResolutionResult {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
