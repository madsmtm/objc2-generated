//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKNavigation;

    unsafe impl ClassType for WKNavigation {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for WKNavigation {}

extern_methods!(
    unsafe impl WKNavigation {
        #[cfg(feature = "WKWebpagePreferences")]
        #[method(effectiveContentMode)]
        pub unsafe fn effectiveContentMode(&self) -> WKContentMode;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKNavigation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
