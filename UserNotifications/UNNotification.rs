//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/usernotifications/unnotification?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNNotification;
);

unsafe impl NSCoding for UNNotification {}

unsafe impl NSCopying for UNNotification {}

unsafe impl CopyingHelper for UNNotification {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UNNotification {}

unsafe impl NSSecureCoding for UNNotification {}

extern_methods!(
    unsafe impl UNNotification {
        #[unsafe(method_family(none))]
        #[method_id(date)]
        pub unsafe fn date(&self) -> Retained<NSDate>;

        #[cfg(feature = "UNNotificationRequest")]
        #[unsafe(method_family(none))]
        #[method_id(request)]
        pub unsafe fn request(&self) -> Retained<UNNotificationRequest>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UNNotification {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
