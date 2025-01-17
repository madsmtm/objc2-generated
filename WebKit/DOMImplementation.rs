//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/domimplementation?language=objc)
    #[unsafe(super(DOMObject, WebScriptObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    #[deprecated]
    pub struct DOMImplementation;
);

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSCopying for DOMImplementation {}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl CopyingHelper for DOMImplementation {
    type Result = Self;
}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl NSObjectProtocol for DOMImplementation {}

extern_methods!(
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMImplementation {
        #[method(hasFeature:version:)]
        pub unsafe fn hasFeature_version(
            &self,
            feature: Option<&NSString>,
            version: Option<&NSString>,
        ) -> bool;

        #[cfg(all(feature = "DOMDocumentType", feature = "DOMNode"))]
        #[unsafe(method_family(none))]
        #[method_id(createDocumentType:publicId:systemId:)]
        pub unsafe fn createDocumentType_publicId_systemId(
            &self,
            qualified_name: Option<&NSString>,
            public_id: Option<&NSString>,
            system_id: Option<&NSString>,
        ) -> Option<Retained<DOMDocumentType>>;

        #[cfg(all(
            feature = "DOMDocument",
            feature = "DOMDocumentType",
            feature = "DOMNode"
        ))]
        #[unsafe(method_family(none))]
        #[method_id(createDocument:qualifiedName:doctype:)]
        pub unsafe fn createDocument_qualifiedName_doctype(
            &self,
            namespace_uri: Option<&NSString>,
            qualified_name: Option<&NSString>,
            doctype: Option<&DOMDocumentType>,
        ) -> Option<Retained<DOMDocument>>;

        #[cfg(all(feature = "DOMCSSStyleSheet", feature = "DOMStyleSheet"))]
        #[unsafe(method_family(none))]
        #[method_id(createCSSStyleSheet:media:)]
        pub unsafe fn createCSSStyleSheet_media(
            &self,
            title: Option<&NSString>,
            media: Option<&NSString>,
        ) -> Option<Retained<DOMCSSStyleSheet>>;

        #[cfg(all(
            feature = "DOMDocument",
            feature = "DOMHTMLDocument",
            feature = "DOMNode"
        ))]
        #[unsafe(method_family(none))]
        #[method_id(createHTMLDocument:)]
        pub unsafe fn createHTMLDocument(
            &self,
            title: Option<&NSString>,
        ) -> Option<Retained<DOMHTMLDocument>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMImplementation {
        #[deprecated]
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMImplementation {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// DOMImplementationDeprecated
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    unsafe impl DOMImplementation {
        #[deprecated]
        #[method(hasFeature::)]
        pub unsafe fn hasFeature(
            &self,
            feature: Option<&NSString>,
            version: Option<&NSString>,
        ) -> bool;

        #[cfg(all(feature = "DOMDocumentType", feature = "DOMNode"))]
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(createDocumentType:::)]
        pub unsafe fn createDocumentType(
            &self,
            qualified_name: Option<&NSString>,
            public_id: Option<&NSString>,
            system_id: Option<&NSString>,
        ) -> Option<Retained<DOMDocumentType>>;

        #[cfg(all(
            feature = "DOMDocument",
            feature = "DOMDocumentType",
            feature = "DOMNode"
        ))]
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(createDocument:::)]
        pub unsafe fn createDocument(
            &self,
            namespace_uri: Option<&NSString>,
            qualified_name: Option<&NSString>,
            doctype: Option<&DOMDocumentType>,
        ) -> Option<Retained<DOMDocument>>;

        #[cfg(all(feature = "DOMCSSStyleSheet", feature = "DOMStyleSheet"))]
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(createCSSStyleSheet::)]
        pub unsafe fn createCSSStyleSheet(
            &self,
            title: Option<&NSString>,
            media: Option<&NSString>,
        ) -> Option<Retained<DOMCSSStyleSheet>>;
    }
);
