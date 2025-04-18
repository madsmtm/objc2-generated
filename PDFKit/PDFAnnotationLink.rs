//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/pdfkit/pdfannotationlink?language=objc)
    #[unsafe(super(PDFAnnotation, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PDFAnnotation")]
    #[deprecated]
    pub struct PDFAnnotationLink;
);

#[cfg(feature = "PDFAnnotation")]
extern_conformance!(
    unsafe impl NSCoding for PDFAnnotationLink {}
);

#[cfg(feature = "PDFAnnotation")]
extern_conformance!(
    unsafe impl NSCopying for PDFAnnotationLink {}
);

#[cfg(feature = "PDFAnnotation")]
unsafe impl CopyingHelper for PDFAnnotationLink {
    type Result = Self;
}

#[cfg(feature = "PDFAnnotation")]
extern_conformance!(
    unsafe impl NSObjectProtocol for PDFAnnotationLink {}
);

#[cfg(feature = "PDFAnnotation")]
impl PDFAnnotationLink {
    extern_methods!(
        #[cfg(feature = "PDFDestination")]
        #[deprecated]
        #[unsafe(method(destination))]
        #[unsafe(method_family = none)]
        pub unsafe fn destination(&self) -> Option<Retained<PDFDestination>>;

        #[cfg(feature = "PDFDestination")]
        #[deprecated]
        #[unsafe(method(setDestination:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDestination(&self, destination: Option<&PDFDestination>);

        #[deprecated]
        #[unsafe(method(URL))]
        #[unsafe(method_family = none)]
        pub unsafe fn URL(&self) -> Option<Retained<NSURL>>;

        #[deprecated]
        #[unsafe(method(setURL:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setURL(&self, url: Option<&NSURL>);
    );
}

/// Methods declared on superclass `PDFAnnotation`.
#[cfg(feature = "PDFAnnotation")]
impl PDFAnnotationLink {
    extern_methods!(
        #[unsafe(method(initWithBounds:forType:withProperties:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithBounds_forType_withProperties(
            this: Allocated<Self>,
            bounds: NSRect,
            annotation_type: &PDFAnnotationSubtype,
            properties: Option<&NSDictionary>,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "PDFAnnotation")]
impl PDFAnnotationLink {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
