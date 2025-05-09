//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/domfile?language=objc)
    #[unsafe(super(DOMBlob, DOMObject, WebScriptObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "DOMBlob",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMFile;
);

#[cfg(all(
    feature = "DOMBlob",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
extern_conformance!(
    unsafe impl NSCopying for DOMFile {}
);

#[cfg(all(
    feature = "DOMBlob",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl CopyingHelper for DOMFile {
    type Result = Self;
}

#[cfg(all(
    feature = "DOMBlob",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
extern_conformance!(
    unsafe impl NSObjectProtocol for DOMFile {}
);

#[cfg(all(
    feature = "DOMBlob",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
impl DOMFile {
    extern_methods!(
        #[deprecated]
        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        pub unsafe fn name(&self) -> Retained<NSString>;
    );
}

/// Methods declared on superclass `DOMObject`.
#[cfg(all(
    feature = "DOMBlob",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
impl DOMFile {
    extern_methods!(
        #[deprecated]
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(
    feature = "DOMBlob",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
impl DOMFile {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
