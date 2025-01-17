//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/domrgbcolor?language=objc)
    #[unsafe(super(DOMObject, WebScriptObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    #[deprecated]
    pub struct DOMRGBColor;
);

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSCopying for DOMRGBColor {}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl CopyingHelper for DOMRGBColor {
    type Result = Self;
}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSObjectProtocol for DOMRGBColor {}

extern_methods!(
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMRGBColor {
        #[cfg(all(feature = "DOMCSSPrimitiveValue", feature = "DOMCSSValue"))]
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(red)]
        pub unsafe fn red(&self) -> Option<Retained<DOMCSSPrimitiveValue>>;

        #[cfg(all(feature = "DOMCSSPrimitiveValue", feature = "DOMCSSValue"))]
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(green)]
        pub unsafe fn green(&self) -> Option<Retained<DOMCSSPrimitiveValue>>;

        #[cfg(all(feature = "DOMCSSPrimitiveValue", feature = "DOMCSSValue"))]
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(blue)]
        pub unsafe fn blue(&self) -> Option<Retained<DOMCSSPrimitiveValue>>;

        #[cfg(all(feature = "DOMCSSPrimitiveValue", feature = "DOMCSSValue"))]
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(alpha)]
        pub unsafe fn alpha(&self) -> Option<Retained<DOMCSSPrimitiveValue>>;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[unsafe(method_family(none))]
        #[method_id(color)]
        pub unsafe fn color(&self) -> Retained<NSColor>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMRGBColor {
        #[deprecated]
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMRGBColor {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
