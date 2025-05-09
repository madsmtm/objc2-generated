//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/domhtmliframeelement?language=objc)
    #[unsafe(super(
        DOMHTMLElement,
        DOMElement,
        DOMNode,
        DOMObject,
        WebScriptObject,
        NSObject
    ))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMHTMLIFrameElement;
);

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMEventTarget",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
extern_conformance!(
    unsafe impl DOMEventTarget for DOMHTMLIFrameElement {}
);

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
extern_conformance!(
    unsafe impl NSCopying for DOMHTMLIFrameElement {}
);

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl CopyingHelper for DOMHTMLIFrameElement {
    type Result = Self;
}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
extern_conformance!(
    unsafe impl NSObjectProtocol for DOMHTMLIFrameElement {}
);

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
impl DOMHTMLIFrameElement {
    extern_methods!(
        #[deprecated]
        #[unsafe(method(align))]
        #[unsafe(method_family = none)]
        pub unsafe fn align(&self) -> Retained<NSString>;

        /// Setter for [`align`][Self::align].
        #[deprecated]
        #[unsafe(method(setAlign:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAlign(&self, align: Option<&NSString>);

        #[deprecated]
        #[unsafe(method(frameBorder))]
        #[unsafe(method_family = none)]
        pub unsafe fn frameBorder(&self) -> Retained<NSString>;

        /// Setter for [`frameBorder`][Self::frameBorder].
        #[deprecated]
        #[unsafe(method(setFrameBorder:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFrameBorder(&self, frame_border: Option<&NSString>);

        #[deprecated]
        #[unsafe(method(height))]
        #[unsafe(method_family = none)]
        pub unsafe fn height(&self) -> Retained<NSString>;

        /// Setter for [`height`][Self::height].
        #[deprecated]
        #[unsafe(method(setHeight:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setHeight(&self, height: Option<&NSString>);

        #[deprecated]
        #[unsafe(method(longDesc))]
        #[unsafe(method_family = none)]
        pub unsafe fn longDesc(&self) -> Retained<NSString>;

        /// Setter for [`longDesc`][Self::longDesc].
        #[deprecated]
        #[unsafe(method(setLongDesc:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLongDesc(&self, long_desc: Option<&NSString>);

        #[deprecated]
        #[unsafe(method(marginHeight))]
        #[unsafe(method_family = none)]
        pub unsafe fn marginHeight(&self) -> Retained<NSString>;

        /// Setter for [`marginHeight`][Self::marginHeight].
        #[deprecated]
        #[unsafe(method(setMarginHeight:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMarginHeight(&self, margin_height: Option<&NSString>);

        #[deprecated]
        #[unsafe(method(marginWidth))]
        #[unsafe(method_family = none)]
        pub unsafe fn marginWidth(&self) -> Retained<NSString>;

        /// Setter for [`marginWidth`][Self::marginWidth].
        #[deprecated]
        #[unsafe(method(setMarginWidth:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMarginWidth(&self, margin_width: Option<&NSString>);

        #[deprecated]
        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        /// Setter for [`name`][Self::name].
        #[deprecated]
        #[unsafe(method(setName:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[deprecated]
        #[unsafe(method(scrolling))]
        #[unsafe(method_family = none)]
        pub unsafe fn scrolling(&self) -> Retained<NSString>;

        /// Setter for [`scrolling`][Self::scrolling].
        #[deprecated]
        #[unsafe(method(setScrolling:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setScrolling(&self, scrolling: Option<&NSString>);

        #[deprecated]
        #[unsafe(method(src))]
        #[unsafe(method_family = none)]
        pub unsafe fn src(&self) -> Retained<NSString>;

        /// Setter for [`src`][Self::src].
        #[deprecated]
        #[unsafe(method(setSrc:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSrc(&self, src: Option<&NSString>);

        #[deprecated]
        #[unsafe(method(width))]
        #[unsafe(method_family = none)]
        pub unsafe fn width(&self) -> Retained<NSString>;

        /// Setter for [`width`][Self::width].
        #[deprecated]
        #[unsafe(method(setWidth:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setWidth(&self, width: Option<&NSString>);

        #[cfg(feature = "DOMDocument")]
        #[deprecated]
        #[unsafe(method(contentDocument))]
        #[unsafe(method_family = none)]
        pub unsafe fn contentDocument(&self) -> Option<Retained<DOMDocument>>;

        #[cfg(feature = "DOMAbstractView")]
        #[unsafe(method(contentWindow))]
        #[unsafe(method_family = none)]
        pub unsafe fn contentWindow(&self) -> Option<Retained<DOMAbstractView>>;
    );
}

/// Methods declared on superclass `DOMObject`.
#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
impl DOMHTMLIFrameElement {
    extern_methods!(
        #[deprecated]
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
impl DOMHTMLIFrameElement {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
