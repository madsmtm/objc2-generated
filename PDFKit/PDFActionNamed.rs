//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/pdfkit/pdfactionnamedname?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PDFActionNamedName(pub NSInteger);
impl PDFActionNamedName {
    #[doc(alias = "kPDFActionNamedNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "kPDFActionNamedNextPage")]
    pub const NextPage: Self = Self(1);
    #[doc(alias = "kPDFActionNamedPreviousPage")]
    pub const PreviousPage: Self = Self(2);
    #[doc(alias = "kPDFActionNamedFirstPage")]
    pub const FirstPage: Self = Self(3);
    #[doc(alias = "kPDFActionNamedLastPage")]
    pub const LastPage: Self = Self(4);
    #[doc(alias = "kPDFActionNamedGoBack")]
    pub const GoBack: Self = Self(5);
    #[doc(alias = "kPDFActionNamedGoForward")]
    pub const GoForward: Self = Self(6);
    #[doc(alias = "kPDFActionNamedGoToPage")]
    pub const GoToPage: Self = Self(7);
    #[doc(alias = "kPDFActionNamedFind")]
    pub const Find: Self = Self(8);
    #[doc(alias = "kPDFActionNamedPrint")]
    pub const Print: Self = Self(9);
    #[doc(alias = "kPDFActionNamedZoomIn")]
    pub const ZoomIn: Self = Self(10);
    #[doc(alias = "kPDFActionNamedZoomOut")]
    pub const ZoomOut: Self = Self(11);
}

unsafe impl Encode for PDFActionNamedName {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for PDFActionNamedName {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/pdfkit/pdfactionnamed?language=objc)
    #[unsafe(super(PDFAction, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PDFAction")]
    pub struct PDFActionNamed;
);

#[cfg(feature = "PDFAction")]
extern_conformance!(
    unsafe impl NSCopying for PDFActionNamed {}
);

#[cfg(feature = "PDFAction")]
unsafe impl CopyingHelper for PDFActionNamed {
    type Result = Self;
}

#[cfg(feature = "PDFAction")]
extern_conformance!(
    unsafe impl NSObjectProtocol for PDFActionNamed {}
);

#[cfg(feature = "PDFAction")]
impl PDFActionNamed {
    extern_methods!(
        #[unsafe(method(initWithName:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithName(
            this: Allocated<Self>,
            name: PDFActionNamedName,
        ) -> Retained<Self>;

        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        pub unsafe fn name(&self) -> PDFActionNamedName;

        /// Setter for [`name`][Self::name].
        #[unsafe(method(setName:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setName(&self, name: PDFActionNamedName);
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "PDFAction")]
impl PDFActionNamed {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
