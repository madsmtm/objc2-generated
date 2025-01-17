//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// A WKNavigation object can be used for tracking the loading progress of a webpage.
    ///
    /// A navigation is returned from the web view load methods, and is
    /// also passed to the navigation delegate methods, to uniquely identify a webpage
    /// load from start to finish.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/webkit/wknavigation?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKNavigation;
);

unsafe impl NSObjectProtocol for WKNavigation {}

extern_methods!(
    unsafe impl WKNavigation {
        #[cfg(feature = "WKWebpagePreferences")]
        /// The content mode used when loading this webpage.
        ///
        /// The value is either WKContentModeMobile or WKContentModeDesktop.
        #[method(effectiveContentMode)]
        pub unsafe fn effectiveContentMode(&self) -> WKContentMode;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKNavigation {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
