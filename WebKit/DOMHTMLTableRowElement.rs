//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/domhtmltablerowelement?language=objc)
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
    pub struct DOMHTMLTableRowElement;
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
    unsafe impl DOMEventTarget for DOMHTMLTableRowElement {}
);

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
extern_conformance!(
    unsafe impl NSCopying for DOMHTMLTableRowElement {}
);

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl CopyingHelper for DOMHTMLTableRowElement {
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
    unsafe impl NSObjectProtocol for DOMHTMLTableRowElement {}
);

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
impl DOMHTMLTableRowElement {
    extern_methods!(
        #[deprecated]
        #[unsafe(method(rowIndex))]
        #[unsafe(method_family = none)]
        pub unsafe fn rowIndex(&self) -> c_int;

        #[deprecated]
        #[unsafe(method(sectionRowIndex))]
        #[unsafe(method_family = none)]
        pub unsafe fn sectionRowIndex(&self) -> c_int;

        #[cfg(feature = "DOMHTMLCollection")]
        #[deprecated]
        #[unsafe(method(cells))]
        #[unsafe(method_family = none)]
        pub unsafe fn cells(&self) -> Option<Retained<DOMHTMLCollection>>;

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
        #[unsafe(method(bgColor))]
        #[unsafe(method_family = none)]
        pub unsafe fn bgColor(&self) -> Retained<NSString>;

        /// Setter for [`bgColor`][Self::bgColor].
        #[deprecated]
        #[unsafe(method(setBgColor:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setBgColor(&self, bg_color: Option<&NSString>);

        #[deprecated]
        #[unsafe(method(ch))]
        #[unsafe(method_family = none)]
        pub unsafe fn ch(&self) -> Retained<NSString>;

        /// Setter for [`ch`][Self::ch].
        #[deprecated]
        #[unsafe(method(setCh:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCh(&self, ch: Option<&NSString>);

        #[deprecated]
        #[unsafe(method(chOff))]
        #[unsafe(method_family = none)]
        pub unsafe fn chOff(&self) -> Retained<NSString>;

        /// Setter for [`chOff`][Self::chOff].
        #[deprecated]
        #[unsafe(method(setChOff:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setChOff(&self, ch_off: Option<&NSString>);

        #[deprecated]
        #[unsafe(method(vAlign))]
        #[unsafe(method_family = none)]
        pub unsafe fn vAlign(&self) -> Retained<NSString>;

        /// Setter for [`vAlign`][Self::vAlign].
        #[deprecated]
        #[unsafe(method(setVAlign:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setVAlign(&self, v_align: Option<&NSString>);

        #[deprecated]
        #[unsafe(method(insertCell:))]
        #[unsafe(method_family = none)]
        pub unsafe fn insertCell(&self, index: c_int) -> Option<Retained<DOMHTMLElement>>;

        #[deprecated]
        #[unsafe(method(deleteCell:))]
        #[unsafe(method_family = none)]
        pub unsafe fn deleteCell(&self, index: c_int);
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
impl DOMHTMLTableRowElement {
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
impl DOMHTMLTableRowElement {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
