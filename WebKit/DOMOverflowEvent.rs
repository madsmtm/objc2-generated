//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/webkit/dom_horizontal?language=objc)
#[deprecated]
pub const DOM_HORIZONTAL: c_uint = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/webkit/dom_vertical?language=objc)
#[deprecated]
pub const DOM_VERTICAL: c_uint = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/webkit/dom_both?language=objc)
#[deprecated]
pub const DOM_BOTH: c_uint = 2;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/domoverflowevent?language=objc)
    #[unsafe(super(DOMEvent, DOMObject, WebScriptObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "DOMEvent",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMOverflowEvent;
);

#[cfg(all(
    feature = "DOMEvent",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSCopying for DOMOverflowEvent {}

#[cfg(all(
    feature = "DOMEvent",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl CopyingHelper for DOMOverflowEvent {
    type Result = Self;
}

#[cfg(all(
    feature = "DOMEvent",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMOverflowEvent {}

extern_methods!(
    #[cfg(all(
        feature = "DOMEvent",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMOverflowEvent {
        #[deprecated]
        #[method(orient)]
        pub unsafe fn orient(&self) -> c_ushort;

        #[deprecated]
        #[method(horizontalOverflow)]
        pub unsafe fn horizontalOverflow(&self) -> bool;

        #[deprecated]
        #[method(verticalOverflow)]
        pub unsafe fn verticalOverflow(&self) -> bool;

        #[deprecated]
        #[method(initOverflowEvent:horizontalOverflow:verticalOverflow:)]
        pub unsafe fn initOverflowEvent_horizontalOverflow_verticalOverflow(
            &self,
            orient: c_ushort,
            horizontal_overflow: bool,
            vertical_overflow: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(
        feature = "DOMEvent",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMOverflowEvent {
        #[deprecated]
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "DOMEvent",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMOverflowEvent {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
