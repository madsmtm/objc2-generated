//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/domprogressevent?language=objc)
    #[unsafe(super(DOMEvent, DOMObject, WebScriptObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "DOMEvent",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMProgressEvent;
);

#[cfg(all(
    feature = "DOMEvent",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
extern_conformance!(
    unsafe impl NSCopying for DOMProgressEvent {}
);

#[cfg(all(
    feature = "DOMEvent",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl CopyingHelper for DOMProgressEvent {
    type Result = Self;
}

#[cfg(all(
    feature = "DOMEvent",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
extern_conformance!(
    unsafe impl NSObjectProtocol for DOMProgressEvent {}
);

#[cfg(all(
    feature = "DOMEvent",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
impl DOMProgressEvent {
    extern_methods!(
        #[deprecated]
        #[unsafe(method(lengthComputable))]
        #[unsafe(method_family = none)]
        pub unsafe fn lengthComputable(&self) -> bool;

        #[deprecated]
        #[unsafe(method(loaded))]
        #[unsafe(method_family = none)]
        pub unsafe fn loaded(&self) -> c_ulonglong;

        #[deprecated]
        #[unsafe(method(total))]
        #[unsafe(method_family = none)]
        pub unsafe fn total(&self) -> c_ulonglong;
    );
}

/// Methods declared on superclass `DOMObject`.
#[cfg(all(
    feature = "DOMEvent",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
impl DOMProgressEvent {
    extern_methods!(
        #[deprecated]
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(
    feature = "DOMEvent",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
impl DOMProgressEvent {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
