//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsuseractivityrestoring?language=objc)
    pub unsafe trait NSUserActivityRestoring: NSObjectProtocol + MainThreadOnly {
        #[method(restoreUserActivityState:)]
        unsafe fn restoreUserActivityState(&self, user_activity: &NSUserActivity);
    }
);

extern_methods!(
    /// NSUserActivity
    #[cfg(feature = "NSResponder")]
    unsafe impl NSResponder {
        #[unsafe(method_family(none))]
        #[method_id(userActivity)]
        pub unsafe fn userActivity(&self) -> Option<Retained<NSUserActivity>>;

        /// Setter for [`userActivity`][Self::userActivity].
        #[method(setUserActivity:)]
        pub unsafe fn setUserActivity(&self, user_activity: Option<&NSUserActivity>);

        #[method(updateUserActivityState:)]
        pub unsafe fn updateUserActivityState(&self, user_activity: &NSUserActivity);
    }
);

#[cfg(feature = "NSResponder")]
unsafe impl NSUserActivityRestoring for NSResponder {}

extern_methods!(
    /// NSUserActivity
    #[cfg(feature = "NSDocument")]
    unsafe impl NSDocument {
        #[unsafe(method_family(none))]
        #[method_id(userActivity)]
        pub unsafe fn userActivity(&self) -> Option<Retained<NSUserActivity>>;

        /// Setter for [`userActivity`][Self::userActivity].
        #[method(setUserActivity:)]
        pub unsafe fn setUserActivity(&self, user_activity: Option<&NSUserActivity>);

        #[method(updateUserActivityState:)]
        pub unsafe fn updateUserActivityState(&self, activity: &NSUserActivity);
    }
);

#[cfg(feature = "NSDocument")]
unsafe impl NSUserActivityRestoring for NSDocument {}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsuseractivitydocumenturlkey?language=objc)
    pub static NSUserActivityDocumentURLKey: &'static NSString;
}
