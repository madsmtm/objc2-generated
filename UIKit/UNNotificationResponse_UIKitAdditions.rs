//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-user-notifications")]
use objc2_user_notifications::*;

use crate::*;

mod private_UNNotificationResponseUIKitAdditions {
    pub trait Sealed {}
}

/// Category "UIKitAdditions" on [`UNNotificationResponse`].
#[doc(alias = "UIKitAdditions")]
pub unsafe trait UNNotificationResponseUIKitAdditions:
    ClassType + Sized + private_UNNotificationResponseUIKitAdditions::Sealed
{
    extern_methods!(
        #[cfg(all(feature = "UIResponder", feature = "UIScene"))]
        #[unsafe(method(targetScene))]
        #[unsafe(method_family = none)]
        unsafe fn targetScene(&self, mtm: MainThreadMarker) -> Option<Retained<UIScene>>;
    );
}

#[cfg(feature = "objc2-user-notifications")]
impl private_UNNotificationResponseUIKitAdditions::Sealed for UNNotificationResponse {}
#[cfg(feature = "objc2-user-notifications")]
unsafe impl UNNotificationResponseUIKitAdditions for UNNotificationResponse {}
