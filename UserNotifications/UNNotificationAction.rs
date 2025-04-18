//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/usernotifications/unnotificationactionoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UNNotificationActionOptions(pub NSUInteger);
bitflags::bitflags! {
    impl UNNotificationActionOptions: NSUInteger {
        #[doc(alias = "UNNotificationActionOptionAuthenticationRequired")]
        const AuthenticationRequired = 1<<0;
        #[doc(alias = "UNNotificationActionOptionDestructive")]
        const Destructive = 1<<1;
        #[doc(alias = "UNNotificationActionOptionForeground")]
        const Foreground = 1<<2;
    }
}

unsafe impl Encode for UNNotificationActionOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UNNotificationActionOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/usernotifications/unnotificationactionoptionnone?language=objc)
pub static UNNotificationActionOptionNone: UNNotificationActionOptions =
    UNNotificationActionOptions(0);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/usernotifications/unnotificationaction?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNNotificationAction;
);

extern_conformance!(
    unsafe impl NSCoding for UNNotificationAction {}
);

extern_conformance!(
    unsafe impl NSCopying for UNNotificationAction {}
);

unsafe impl CopyingHelper for UNNotificationAction {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for UNNotificationAction {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for UNNotificationAction {}
);

impl UNNotificationAction {
    extern_methods!(
        #[unsafe(method(identifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[unsafe(method(title))]
        #[unsafe(method_family = none)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        #[unsafe(method(options))]
        #[unsafe(method_family = none)]
        pub unsafe fn options(&self) -> UNNotificationActionOptions;

        #[cfg(feature = "UNNotificationActionIcon")]
        #[unsafe(method(icon))]
        #[unsafe(method_family = none)]
        pub unsafe fn icon(&self) -> Option<Retained<UNNotificationActionIcon>>;

        #[unsafe(method(actionWithIdentifier:title:options:))]
        #[unsafe(method_family = none)]
        pub unsafe fn actionWithIdentifier_title_options(
            identifier: &NSString,
            title: &NSString,
            options: UNNotificationActionOptions,
        ) -> Retained<Self>;

        #[cfg(feature = "UNNotificationActionIcon")]
        #[unsafe(method(actionWithIdentifier:title:options:icon:))]
        #[unsafe(method_family = none)]
        pub unsafe fn actionWithIdentifier_title_options_icon(
            identifier: &NSString,
            title: &NSString,
            options: UNNotificationActionOptions,
            icon: Option<&UNNotificationActionIcon>,
        ) -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl UNNotificationAction {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/usernotifications/untextinputnotificationaction?language=objc)
    #[unsafe(super(UNNotificationAction, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNTextInputNotificationAction;
);

extern_conformance!(
    unsafe impl NSCoding for UNTextInputNotificationAction {}
);

extern_conformance!(
    unsafe impl NSCopying for UNTextInputNotificationAction {}
);

unsafe impl CopyingHelper for UNTextInputNotificationAction {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for UNTextInputNotificationAction {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for UNTextInputNotificationAction {}
);

impl UNTextInputNotificationAction {
    extern_methods!(
        #[unsafe(method(actionWithIdentifier:title:options:textInputButtonTitle:textInputPlaceholder:))]
        #[unsafe(method_family = none)]
        pub unsafe fn actionWithIdentifier_title_options_textInputButtonTitle_textInputPlaceholder(
            identifier: &NSString,
            title: &NSString,
            options: UNNotificationActionOptions,
            text_input_button_title: &NSString,
            text_input_placeholder: &NSString,
        ) -> Retained<Self>;

        #[cfg(feature = "UNNotificationActionIcon")]
        #[unsafe(method(actionWithIdentifier:title:options:icon:textInputButtonTitle:textInputPlaceholder:))]
        #[unsafe(method_family = none)]
        pub unsafe fn actionWithIdentifier_title_options_icon_textInputButtonTitle_textInputPlaceholder(
            identifier: &NSString,
            title: &NSString,
            options: UNNotificationActionOptions,
            icon: Option<&UNNotificationActionIcon>,
            text_input_button_title: &NSString,
            text_input_placeholder: &NSString,
        ) -> Retained<Self>;

        #[unsafe(method(textInputButtonTitle))]
        #[unsafe(method_family = none)]
        pub unsafe fn textInputButtonTitle(&self) -> Retained<NSString>;

        #[unsafe(method(textInputPlaceholder))]
        #[unsafe(method_family = none)]
        pub unsafe fn textInputPlaceholder(&self) -> Retained<NSString>;
    );
}

/// Methods declared on superclass `UNNotificationAction`.
impl UNTextInputNotificationAction {
    extern_methods!(
        #[unsafe(method(actionWithIdentifier:title:options:))]
        #[unsafe(method_family = none)]
        pub unsafe fn actionWithIdentifier_title_options(
            identifier: &NSString,
            title: &NSString,
            options: UNNotificationActionOptions,
        ) -> Retained<Self>;

        #[cfg(feature = "UNNotificationActionIcon")]
        #[unsafe(method(actionWithIdentifier:title:options:icon:))]
        #[unsafe(method_family = none)]
        pub unsafe fn actionWithIdentifier_title_options_icon(
            identifier: &NSString,
            title: &NSString,
            options: UNNotificationActionOptions,
            icon: Option<&UNNotificationActionIcon>,
        ) -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl UNTextInputNotificationAction {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
