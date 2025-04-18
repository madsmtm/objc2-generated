//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamekit/gkviewcontroller?language=objc)
    pub unsafe trait GKViewController {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamekit/gkdialogcontroller?language=objc)
    #[unsafe(super(NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    pub struct GKDialogController;
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSCoding for GKDialogController {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSObjectProtocol for GKDialogController {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
impl GKDialogController {
    extern_methods!(
        #[unsafe(method(parentWindow))]
        #[unsafe(method_family = none)]
        pub unsafe fn parentWindow(&self) -> Option<Retained<NSWindow>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`parentWindow`][Self::parentWindow].
        #[unsafe(method(setParentWindow:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setParentWindow(&self, parent_window: Option<&NSWindow>);

        #[unsafe(method(presentViewController:))]
        #[unsafe(method_family = none)]
        pub unsafe fn presentViewController(&self, view_controller: &NSViewController) -> bool;

        #[unsafe(method(dismiss:))]
        #[unsafe(method_family = none)]
        pub unsafe fn dismiss(&self, sender: &AnyObject);
    );
}

/// Methods declared on superclass `NSResponder`.
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
impl GKDialogController {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
impl GKDialogController {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

/// SharedDialogController.
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
impl GKDialogController {
    extern_methods!(
        #[unsafe(method(sharedDialogController))]
        #[unsafe(method_family = none)]
        pub unsafe fn sharedDialogController(mtm: MainThreadMarker)
            -> Retained<GKDialogController>;
    );
}
