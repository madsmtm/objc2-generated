//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/domcssfontfacerule?language=objc)
    #[unsafe(super(DOMCSSRule, DOMObject, WebScriptObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "DOMCSSRule",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMCSSFontFaceRule;
);

#[cfg(all(
    feature = "DOMCSSRule",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
extern_conformance!(
    unsafe impl NSCopying for DOMCSSFontFaceRule {}
);

#[cfg(all(
    feature = "DOMCSSRule",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl CopyingHelper for DOMCSSFontFaceRule {
    type Result = Self;
}

#[cfg(all(
    feature = "DOMCSSRule",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
extern_conformance!(
    unsafe impl NSObjectProtocol for DOMCSSFontFaceRule {}
);

#[cfg(all(
    feature = "DOMCSSRule",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
impl DOMCSSFontFaceRule {
    extern_methods!(
        #[cfg(feature = "DOMCSSStyleDeclaration")]
        #[deprecated]
        #[unsafe(method(style))]
        #[unsafe(method_family = none)]
        pub unsafe fn style(&self) -> Option<Retained<DOMCSSStyleDeclaration>>;
    );
}

/// Methods declared on superclass `DOMObject`.
#[cfg(all(
    feature = "DOMCSSRule",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
impl DOMCSSFontFaceRule {
    extern_methods!(
        #[deprecated]
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(
    feature = "DOMCSSRule",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
impl DOMCSSFontFaceRule {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
