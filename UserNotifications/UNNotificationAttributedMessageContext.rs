//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/usernotifications/unnotificationattributedmessagecontext?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNNotificationAttributedMessageContext;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for UNNotificationAttributedMessageContext {}
);

#[cfg(feature = "UNNotificationContent")]
extern_conformance!(
    unsafe impl UNNotificationContentProviding for UNNotificationAttributedMessageContext {}
);

impl UNNotificationAttributedMessageContext {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl UNNotificationAttributedMessageContext {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
