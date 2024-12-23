//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfnotificationname?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "CFBase")]
pub type CFNotificationName = CFStringRef;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfnotificationcenterref?language=objc)
pub type CFNotificationCenterRef = *mut c_void;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfnotificationcallback?language=objc)
#[cfg(all(feature = "CFBase", feature = "CFDictionary"))]
pub type CFNotificationCallback = Option<
    unsafe extern "C-unwind" fn(
        CFNotificationCenterRef,
        *mut c_void,
        CFNotificationName,
        *const c_void,
        CFDictionaryRef,
    ),
>;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfnotificationsuspensionbehavior?language=objc)
// NS_ENUM
#[cfg(feature = "CFBase")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CFNotificationSuspensionBehavior(pub CFIndex);
#[cfg(feature = "CFBase")]
impl CFNotificationSuspensionBehavior {
    #[doc(alias = "CFNotificationSuspensionBehaviorDrop")]
    pub const Drop: Self = Self(1);
    #[doc(alias = "CFNotificationSuspensionBehaviorCoalesce")]
    pub const Coalesce: Self = Self(2);
    #[doc(alias = "CFNotificationSuspensionBehaviorHold")]
    pub const Hold: Self = Self(3);
    #[doc(alias = "CFNotificationSuspensionBehaviorDeliverImmediately")]
    pub const DeliverImmediately: Self = Self(4);
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl Encode for CFNotificationSuspensionBehavior {
    const ENCODING: Encoding = CFIndex::ENCODING;
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl RefEncode for CFNotificationSuspensionBehavior {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFNotificationCenterGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    pub fn CFNotificationCenterGetLocalCenter() -> CFNotificationCenterRef;
}

extern "C-unwind" {
    pub fn CFNotificationCenterGetDistributedCenter() -> CFNotificationCenterRef;
}

extern "C-unwind" {
    pub fn CFNotificationCenterGetDarwinNotifyCenter() -> CFNotificationCenterRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFDictionary"))]
    pub fn CFNotificationCenterAddObserver(
        center: CFNotificationCenterRef,
        observer: *const c_void,
        call_back: CFNotificationCallback,
        name: CFStringRef,
        object: *const c_void,
        suspension_behavior: CFNotificationSuspensionBehavior,
    );
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFNotificationCenterRemoveObserver(
        center: CFNotificationCenterRef,
        observer: *const c_void,
        name: CFNotificationName,
        object: *const c_void,
    );
}

extern "C-unwind" {
    pub fn CFNotificationCenterRemoveEveryObserver(
        center: CFNotificationCenterRef,
        observer: *const c_void,
    );
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFDictionary"))]
    pub fn CFNotificationCenterPostNotification(
        center: CFNotificationCenterRef,
        name: CFNotificationName,
        object: *const c_void,
        user_info: CFDictionaryRef,
        deliver_immediately: Boolean,
    );
}

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnotificationdeliverimmediately?language=objc)
#[cfg(feature = "CFBase")]
pub const kCFNotificationDeliverImmediately: CFOptionFlags = 1 << 0;
/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfnotificationposttoallsessions?language=objc)
#[cfg(feature = "CFBase")]
pub const kCFNotificationPostToAllSessions: CFOptionFlags = 1 << 1;

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFDictionary"))]
    pub fn CFNotificationCenterPostNotificationWithOptions(
        center: CFNotificationCenterRef,
        name: CFNotificationName,
        object: *const c_void,
        user_info: CFDictionaryRef,
        options: CFOptionFlags,
    );
}
