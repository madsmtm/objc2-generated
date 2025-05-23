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
extern_conformance!(
    unsafe impl NSCopying for DOMObject {}
);

#[cfg(feature = "WebScriptObject")]
unsafe impl CopyingHelper for DOMObject {
    type Result = Self;
}

#[cfg(feature = "WebScriptObject")]
extern_conformance!(
    unsafe impl NSObjectProtocol for DOMObject {}
);

#[cfg(feature = "WebScriptObject")]
impl DOMObject {
    extern_methods!(
        #[deprecated]
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "WebScriptObject")]
impl DOMObject {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

/// DOMLinkStyle.
#[cfg(feature = "WebScriptObject")]
impl DOMObject {
    extern_methods!(
        #[cfg(feature = "DOMStyleSheet")]
        #[unsafe(method(sheet))]
        #[unsafe(method_family = none)]
        pub unsafe fn sheet(&self) -> Option<Retained<DOMStyleSheet>>;
    );
}
