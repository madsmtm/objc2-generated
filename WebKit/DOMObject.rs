//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/webkit/domtimestamp?language=objc)
pub type DOMTimeStamp = c_ulonglong;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/domobject?language=objc)
    #[unsafe(super(WebScriptObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebScriptObject")]
    #[deprecated]
    pub struct DOMObject;
);

#[cfg(feature = "WebScriptObject")]
unsafe impl NSCopying for DOMObject {}

#[cfg(feature = "WebScriptObject")]
unsafe impl CopyingHelper for DOMObject {
    type Result = Self;
}

#[cfg(feature = "WebScriptObject")]
unsafe impl NSObjectProtocol for DOMObject {}

extern_methods!(
    #[cfg(feature = "WebScriptObject")]
    unsafe impl DOMObject {
        #[deprecated]
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebScriptObject")]
    unsafe impl DOMObject {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// DOMLinkStyle
    #[cfg(feature = "WebScriptObject")]
    unsafe impl DOMObject {
        #[cfg(feature = "DOMStyleSheet")]
        #[unsafe(method_family(none))]
        #[method_id(sheet)]
        pub unsafe fn sheet(&self) -> Option<Retained<DOMStyleSheet>>;
    }
);
