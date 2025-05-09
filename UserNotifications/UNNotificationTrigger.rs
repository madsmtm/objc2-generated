//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/usernotifications/unnotificationtrigger?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNNotificationTrigger;
);

extern_conformance!(
    unsafe impl NSCoding for UNNotificationTrigger {}
);

extern_conformance!(
    unsafe impl NSCopying for UNNotificationTrigger {}
);

unsafe impl CopyingHelper for UNNotificationTrigger {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for UNNotificationTrigger {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for UNNotificationTrigger {}
);

impl UNNotificationTrigger {
    extern_methods!(
        #[unsafe(method(repeats))]
        #[unsafe(method_family = none)]
        pub unsafe fn repeats(&self) -> bool;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl UNNotificationTrigger {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/usernotifications/unpushnotificationtrigger?language=objc)
    #[unsafe(super(UNNotificationTrigger, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNPushNotificationTrigger;
);

extern_conformance!(
    unsafe impl NSCoding for UNPushNotificationTrigger {}
);

extern_conformance!(
    unsafe impl NSCopying for UNPushNotificationTrigger {}
);

unsafe impl CopyingHelper for UNPushNotificationTrigger {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for UNPushNotificationTrigger {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for UNPushNotificationTrigger {}
);

impl UNPushNotificationTrigger {
    extern_methods!();
}

/// Methods declared on superclass `UNNotificationTrigger`.
impl UNPushNotificationTrigger {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl UNPushNotificationTrigger {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/usernotifications/untimeintervalnotificationtrigger?language=objc)
    #[unsafe(super(UNNotificationTrigger, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNTimeIntervalNotificationTrigger;
);

extern_conformance!(
    unsafe impl NSCoding for UNTimeIntervalNotificationTrigger {}
);

extern_conformance!(
    unsafe impl NSCopying for UNTimeIntervalNotificationTrigger {}
);

unsafe impl CopyingHelper for UNTimeIntervalNotificationTrigger {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for UNTimeIntervalNotificationTrigger {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for UNTimeIntervalNotificationTrigger {}
);

impl UNTimeIntervalNotificationTrigger {
    extern_methods!(
        #[unsafe(method(timeInterval))]
        #[unsafe(method_family = none)]
        pub unsafe fn timeInterval(&self) -> NSTimeInterval;

        #[unsafe(method(triggerWithTimeInterval:repeats:))]
        #[unsafe(method_family = none)]
        pub unsafe fn triggerWithTimeInterval_repeats(
            time_interval: NSTimeInterval,
            repeats: bool,
        ) -> Retained<Self>;

        #[unsafe(method(nextTriggerDate))]
        #[unsafe(method_family = none)]
        pub unsafe fn nextTriggerDate(&self) -> Option<Retained<NSDate>>;
    );
}

/// Methods declared on superclass `UNNotificationTrigger`.
impl UNTimeIntervalNotificationTrigger {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl UNTimeIntervalNotificationTrigger {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/usernotifications/uncalendarnotificationtrigger?language=objc)
    #[unsafe(super(UNNotificationTrigger, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNCalendarNotificationTrigger;
);

extern_conformance!(
    unsafe impl NSCoding for UNCalendarNotificationTrigger {}
);

extern_conformance!(
    unsafe impl NSCopying for UNCalendarNotificationTrigger {}
);

unsafe impl CopyingHelper for UNCalendarNotificationTrigger {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for UNCalendarNotificationTrigger {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for UNCalendarNotificationTrigger {}
);

impl UNCalendarNotificationTrigger {
    extern_methods!(
        #[unsafe(method(dateComponents))]
        #[unsafe(method_family = none)]
        pub unsafe fn dateComponents(&self) -> Retained<NSDateComponents>;

        #[unsafe(method(triggerWithDateMatchingComponents:repeats:))]
        #[unsafe(method_family = none)]
        pub unsafe fn triggerWithDateMatchingComponents_repeats(
            date_components: &NSDateComponents,
            repeats: bool,
        ) -> Retained<Self>;

        #[unsafe(method(nextTriggerDate))]
        #[unsafe(method_family = none)]
        pub unsafe fn nextTriggerDate(&self) -> Option<Retained<NSDate>>;
    );
}

/// Methods declared on superclass `UNNotificationTrigger`.
impl UNCalendarNotificationTrigger {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl UNCalendarNotificationTrigger {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/usernotifications/unlocationnotificationtrigger?language=objc)
    #[unsafe(super(UNNotificationTrigger, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNLocationNotificationTrigger;
);

extern_conformance!(
    unsafe impl NSCoding for UNLocationNotificationTrigger {}
);

extern_conformance!(
    unsafe impl NSCopying for UNLocationNotificationTrigger {}
);

unsafe impl CopyingHelper for UNLocationNotificationTrigger {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for UNLocationNotificationTrigger {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for UNLocationNotificationTrigger {}
);

impl UNLocationNotificationTrigger {
    extern_methods!(
        #[cfg(feature = "objc2-core-location")]
        #[unsafe(method(region))]
        #[unsafe(method_family = none)]
        pub unsafe fn region(&self) -> Retained<CLRegion>;

        #[cfg(feature = "objc2-core-location")]
        #[unsafe(method(triggerWithRegion:repeats:))]
        #[unsafe(method_family = none)]
        pub unsafe fn triggerWithRegion_repeats(region: &CLRegion, repeats: bool)
            -> Retained<Self>;
    );
}

/// Methods declared on superclass `UNNotificationTrigger`.
impl UNLocationNotificationTrigger {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl UNLocationNotificationTrigger {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
