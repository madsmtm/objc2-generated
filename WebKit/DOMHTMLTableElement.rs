//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/domhtmltableelement?language=objc)
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
    pub struct DOMHTMLTableElement;
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
    unsafe impl DOMEventTarget for DOMHTMLTableElement {}
);

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
extern_conformance!(
    unsafe impl NSCopying for DOMHTMLTableElement {}
);

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl CopyingHelper for DOMHTMLTableElement {
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
    unsafe impl NSObjectProtocol for DOMHTMLTableElement {}
);

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
impl DOMHTMLTableElement {
    extern_methods!(
        #[cfg(feature = "DOMHTMLTableCaptionElement")]
        #[deprecated]
        #[unsafe(method(caption))]
        #[unsafe(method_family = none)]
        pub unsafe fn caption(&self) -> Option<Retained<DOMHTMLTableCaptionElement>>;

        #[cfg(feature = "DOMHTMLTableCaptionElement")]
        /// Setter for [`caption`][Self::caption].
        #[deprecated]
        #[unsafe(method(setCaption:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCaption(&self, caption: Option<&DOMHTMLTableCaptionElement>);

        #[cfg(feature = "DOMHTMLTableSectionElement")]
        #[deprecated]
        #[unsafe(method(tHead))]
        #[unsafe(method_family = none)]
        pub unsafe fn tHead(&self) -> Option<Retained<DOMHTMLTableSectionElement>>;

        #[cfg(feature = "DOMHTMLTableSectionElement")]
        /// Setter for [`tHead`][Self::tHead].
        #[deprecated]
        #[unsafe(method(setTHead:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTHead(&self, t_head: Option<&DOMHTMLTableSectionElement>);

        #[cfg(feature = "DOMHTMLTableSectionElement")]
        #[deprecated]
        #[unsafe(method(tFoot))]
        #[unsafe(method_family = none)]
        pub unsafe fn tFoot(&self) -> Option<Retained<DOMHTMLTableSectionElement>>;

        #[cfg(feature = "DOMHTMLTableSectionElement")]
        /// Setter for [`tFoot`][Self::tFoot].
        #[deprecated]
        #[unsafe(method(setTFoot:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTFoot(&self, t_foot: Option<&DOMHTMLTableSectionElement>);

        #[cfg(feature = "DOMHTMLCollection")]
        #[deprecated]
        #[unsafe(method(rows))]
        #[unsafe(method_family = none)]
        pub unsafe fn rows(&self) -> Option<Retained<DOMHTMLCollection>>;

        #[cfg(feature = "DOMHTMLCollection")]
        #[deprecated]
        #[unsafe(method(tBodies))]
        #[unsafe(method_family = none)]
        pub unsafe fn tBodies(&self) -> Option<Retained<DOMHTMLCollection>>;

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
        #[unsafe(method(border))]
        #[unsafe(method_family = none)]
        pub unsafe fn border(&self) -> Retained<NSString>;

        /// Setter for [`border`][Self::border].
        #[deprecated]
        #[unsafe(method(setBorder:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setBorder(&self, border: Option<&NSString>);

        #[deprecated]
        #[unsafe(method(cellPadding))]
        #[unsafe(method_family = none)]
        pub unsafe fn cellPadding(&self) -> Retained<NSString>;

        /// Setter for [`cellPadding`][Self::cellPadding].
        #[deprecated]
        #[unsafe(method(setCellPadding:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCellPadding(&self, cell_padding: Option<&NSString>);

        #[deprecated]
        #[unsafe(method(cellSpacing))]
        #[unsafe(method_family = none)]
        pub unsafe fn cellSpacing(&self) -> Retained<NSString>;

        /// Setter for [`cellSpacing`][Self::cellSpacing].
        #[deprecated]
        #[unsafe(method(setCellSpacing:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCellSpacing(&self, cell_spacing: Option<&NSString>);

        #[deprecated]
        #[unsafe(method(frameBorders))]
        #[unsafe(method_family = none)]
        pub unsafe fn frameBorders(&self) -> Retained<NSString>;

        /// Setter for [`frameBorders`][Self::frameBorders].
        #[deprecated]
        #[unsafe(method(setFrameBorders:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFrameBorders(&self, frame_borders: Option<&NSString>);

        #[deprecated]
        #[unsafe(method(rules))]
        #[unsafe(method_family = none)]
        pub unsafe fn rules(&self) -> Retained<NSString>;

        /// Setter for [`rules`][Self::rules].
        #[deprecated]
        #[unsafe(method(setRules:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRules(&self, rules: Option<&NSString>);

        #[deprecated]
        #[unsafe(method(summary))]
        #[unsafe(method_family = none)]
        pub unsafe fn summary(&self) -> Retained<NSString>;

        /// Setter for [`summary`][Self::summary].
        #[deprecated]
        #[unsafe(method(setSummary:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSummary(&self, summary: Option<&NSString>);

        #[deprecated]
        #[unsafe(method(width))]
        #[unsafe(method_family = none)]
        pub unsafe fn width(&self) -> Retained<NSString>;

        /// Setter for [`width`][Self::width].
        #[deprecated]
        #[unsafe(method(setWidth:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setWidth(&self, width: Option<&NSString>);

        #[deprecated]
        #[unsafe(method(createTHead))]
        #[unsafe(method_family = none)]
        pub unsafe fn createTHead(&self) -> Option<Retained<DOMHTMLElement>>;

        #[deprecated]
        #[unsafe(method(deleteTHead))]
        #[unsafe(method_family = none)]
        pub unsafe fn deleteTHead(&self);

        #[deprecated]
        #[unsafe(method(createTFoot))]
        #[unsafe(method_family = none)]
        pub unsafe fn createTFoot(&self) -> Option<Retained<DOMHTMLElement>>;

        #[deprecated]
        #[unsafe(method(deleteTFoot))]
        #[unsafe(method_family = none)]
        pub unsafe fn deleteTFoot(&self);

        #[deprecated]
        #[unsafe(method(createCaption))]
        #[unsafe(method_family = none)]
        pub unsafe fn createCaption(&self) -> Option<Retained<DOMHTMLElement>>;

        #[deprecated]
        #[unsafe(method(deleteCaption))]
        #[unsafe(method_family = none)]
        pub unsafe fn deleteCaption(&self);

        #[deprecated]
        #[unsafe(method(insertRow:))]
        #[unsafe(method_family = none)]
        pub unsafe fn insertRow(&self, index: c_int) -> Option<Retained<DOMHTMLElement>>;

        #[deprecated]
        #[unsafe(method(deleteRow:))]
        #[unsafe(method_family = none)]
        pub unsafe fn deleteRow(&self, index: c_int);
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
impl DOMHTMLTableElement {
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
impl DOMHTMLTableElement {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
