//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

/// The type of action triggering a navigation.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/webkit/wknavigationtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WKNavigationType(pub NSInteger);
impl WKNavigationType {
    #[doc(alias = "WKNavigationTypeLinkActivated")]
    pub const LinkActivated: Self = Self(0);
    #[doc(alias = "WKNavigationTypeFormSubmitted")]
    pub const FormSubmitted: Self = Self(1);
    #[doc(alias = "WKNavigationTypeBackForward")]
    pub const BackForward: Self = Self(2);
    #[doc(alias = "WKNavigationTypeReload")]
    pub const Reload: Self = Self(3);
    #[doc(alias = "WKNavigationTypeFormResubmitted")]
    pub const FormResubmitted: Self = Self(4);
    #[doc(alias = "WKNavigationTypeOther")]
    pub const Other: Self = Self(-1);
}

unsafe impl Encode for WKNavigationType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for WKNavigationType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// A WKNavigationAction object contains information about an action that may cause a navigation, used for making policy decisions.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/webkit/wknavigationaction?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKNavigationAction;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for WKNavigationAction {}
);

impl WKNavigationAction {
    extern_methods!(
        #[cfg(feature = "WKFrameInfo")]
        /// The target frame, or nil if this is a new window navigation.
        #[unsafe(method(targetFrame))]
        #[unsafe(method_family = none)]
        pub unsafe fn targetFrame(&self) -> Option<Retained<WKFrameInfo>>;

        /// The type of action that triggered the navigation.
        ///
        /// The value is one of the constants of the enumerated type WKNavigationType.
        #[unsafe(method(navigationType))]
        #[unsafe(method_family = none)]
        pub unsafe fn navigationType(&self) -> WKNavigationType;

        /// The navigation's request.
        #[unsafe(method(request))]
        #[unsafe(method_family = none)]
        pub unsafe fn request(&self) -> Retained<NSURLRequest>;

        /// A value indicating whether the web content used a download attribute to indicate that this should be downloaded.
        #[unsafe(method(shouldPerformDownload))]
        #[unsafe(method_family = none)]
        pub unsafe fn shouldPerformDownload(&self) -> bool;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        /// The modifier keys that were in effect when the navigation was requested.
        #[unsafe(method(modifierFlags))]
        #[unsafe(method_family = none)]
        pub unsafe fn modifierFlags(&self) -> NSEventModifierFlags;

        /// The number of the mouse button causing the navigation to be requested.
        #[unsafe(method(buttonNumber))]
        #[unsafe(method_family = none)]
        pub unsafe fn buttonNumber(&self) -> NSInteger;
    );
}

/// Methods declared on superclass `NSObject`.
impl WKNavigationAction {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
