//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/domhtmlobjectelement?language=objc)
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
    pub struct DOMHTMLObjectElement;
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
    unsafe impl DOMEventTarget for DOMHTMLObjectElement {}
);

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
extern_conformance!(
    unsafe impl NSCopying for DOMHTMLObjectElement {}
);

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl CopyingHelper for DOMHTMLObjectElement {
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
    unsafe impl NSObjectProtocol for DOMHTMLObjectElement {}
);

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
impl DOMHTMLObjectElement {
    extern_methods!(
        #[cfg(feature = "DOMHTMLFormElement")]
        #[deprecated]
        #[unsafe(method(form))]
        #[unsafe(method_family = none)]
        pub unsafe fn form(&self) -> Option<Retained<DOMHTMLFormElement>>;

        #[deprecated]
        #[unsafe(method(code))]
        #[unsafe(method_family = none)]
        pub unsafe fn code(&self) -> Retained<NSString>;

        /// Setter for [`code`][Self::code].
        #[deprecated]
        #[unsafe(method(setCode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCode(&self, code: Option<&NSString>);

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
        #[unsafe(method(archive))]
        #[unsafe(method_family = none)]
        pub unsafe fn archive(&self) -> Retained<NSString>;

        /// Setter for [`archive`][Self::archive].
        #[deprecated]
        #[unsafe(method(setArchive:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setArchive(&self, archive: Option<&NSString>);

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
        #[unsafe(method(codeBase))]
        #[unsafe(method_family = none)]
        pub unsafe fn codeBase(&self) -> Retained<NSString>;

        /// Setter for [`codeBase`][Self::codeBase].
        #[deprecated]
        #[unsafe(method(setCodeBase:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCodeBase(&self, code_base: Option<&NSString>);

        #[deprecated]
        #[unsafe(method(codeType))]
        #[unsafe(method_family = none)]
        pub unsafe fn codeType(&self) -> Retained<NSString>;

        /// Setter for [`codeType`][Self::codeType].
        #[deprecated]
        #[unsafe(method(setCodeType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCodeType(&self, code_type: Option<&NSString>);

        #[deprecated]
        #[unsafe(method(data))]
        #[unsafe(method_family = none)]
        pub unsafe fn data(&self) -> Retained<NSString>;

        /// Setter for [`data`][Self::data].
        #[deprecated]
        #[unsafe(method(setData:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setData(&self, data: Option<&NSString>);

        #[deprecated]
        #[unsafe(method(declare))]
        #[unsafe(method_family = none)]
        pub unsafe fn declare(&self) -> bool;

        /// Setter for [`declare`][Self::declare].
        #[deprecated]
        #[unsafe(method(setDeclare:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDeclare(&self, declare: bool);

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
        #[unsafe(method(hspace))]
        #[unsafe(method_family = none)]
        pub unsafe fn hspace(&self) -> c_int;

        /// Setter for [`hspace`][Self::hspace].
        #[deprecated]
        #[unsafe(method(setHspace:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setHspace(&self, hspace: c_int);

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
        #[unsafe(method(standby))]
        #[unsafe(method_family = none)]
        pub unsafe fn standby(&self) -> Retained<NSString>;

        /// Setter for [`standby`][Self::standby].
        #[deprecated]
        #[unsafe(method(setStandby:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setStandby(&self, standby: Option<&NSString>);

        #[deprecated]
        #[unsafe(method(type))]
        #[unsafe(method_family = none)]
        pub unsafe fn r#type(&self) -> Retained<NSString>;

        /// Setter for [`type`][Self::type].
        #[deprecated]
        #[unsafe(method(setType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setType(&self, r#type: Option<&NSString>);

        #[deprecated]
        #[unsafe(method(useMap))]
        #[unsafe(method_family = none)]
        pub unsafe fn useMap(&self) -> Retained<NSString>;

        /// Setter for [`useMap`][Self::useMap].
        #[deprecated]
        #[unsafe(method(setUseMap:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setUseMap(&self, use_map: Option<&NSString>);

        #[deprecated]
        #[unsafe(method(vspace))]
        #[unsafe(method_family = none)]
        pub unsafe fn vspace(&self) -> c_int;

        /// Setter for [`vspace`][Self::vspace].
        #[deprecated]
        #[unsafe(method(setVspace:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setVspace(&self, vspace: c_int);

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

        #[unsafe(method(absoluteImageURL))]
        #[unsafe(method_family = none)]
        pub unsafe fn absoluteImageURL(&self) -> Retained<NSURL>;
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
impl DOMHTMLObjectElement {
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
impl DOMHTMLObjectElement {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
