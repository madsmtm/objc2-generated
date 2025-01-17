//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/domhtmlbuttonelement?language=objc)
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
    pub struct DOMHTMLButtonElement;
);

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMEventTarget",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl DOMEventTarget for DOMHTMLButtonElement {}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSCopying for DOMHTMLButtonElement {}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl CopyingHelper for DOMHTMLButtonElement {
    type Result = Self;
}

#[cfg(all(
    feature = "DOMElement",
    feature = "DOMHTMLElement",
    feature = "DOMNode",
    feature = "DOMObject",
    feature = "WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMHTMLButtonElement {}

extern_methods!(
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLButtonElement {
        #[method(autofocus)]
        pub unsafe fn autofocus(&self) -> bool;

        /// Setter for [`autofocus`][Self::autofocus].
        #[method(setAutofocus:)]
        pub unsafe fn setAutofocus(&self, autofocus: bool);

        #[deprecated]
        #[method(disabled)]
        pub unsafe fn disabled(&self) -> bool;

        /// Setter for [`disabled`][Self::disabled].
        #[deprecated]
        #[method(setDisabled:)]
        pub unsafe fn setDisabled(&self, disabled: bool);

        #[cfg(feature = "DOMHTMLFormElement")]
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(form)]
        pub unsafe fn form(&self) -> Option<Retained<DOMHTMLFormElement>>;

        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(type)]
        pub unsafe fn r#type(&self) -> Retained<NSString>;

        /// Setter for [`type`][Self::type].
        #[deprecated]
        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: Option<&NSString>);

        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        /// Setter for [`name`][Self::name].
        #[deprecated]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(value)]
        pub unsafe fn value(&self) -> Retained<NSString>;

        /// Setter for [`value`][Self::value].
        #[deprecated]
        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: Option<&NSString>);

        #[method(willValidate)]
        pub unsafe fn willValidate(&self) -> bool;

        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(accessKey)]
        pub unsafe fn accessKey(&self) -> Retained<NSString>;

        /// Setter for [`accessKey`][Self::accessKey].
        #[deprecated]
        #[method(setAccessKey:)]
        pub unsafe fn setAccessKey(&self, access_key: Option<&NSString>);

        #[method(click)]
        pub unsafe fn click(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLButtonElement {
        #[deprecated]
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "DOMElement",
        feature = "DOMHTMLElement",
        feature = "DOMNode",
        feature = "DOMObject",
        feature = "WebScriptObject"
    ))]
    unsafe impl DOMHTMLButtonElement {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
