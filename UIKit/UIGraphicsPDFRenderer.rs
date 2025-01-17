//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uigraphicspdfdrawingactions?language=objc)
#[cfg(all(feature = "UIGraphicsRenderer", feature = "block2"))]
pub type UIGraphicsPDFDrawingActions =
    *mut block2::Block<dyn Fn(NonNull<UIGraphicsPDFRendererContext>)>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uigraphicspdfrendererformat?language=objc)
    #[unsafe(super(UIGraphicsRendererFormat, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIGraphicsRenderer")]
    pub struct UIGraphicsPDFRendererFormat;
);

#[cfg(feature = "UIGraphicsRenderer")]
unsafe impl NSCopying for UIGraphicsPDFRendererFormat {}

#[cfg(feature = "UIGraphicsRenderer")]
unsafe impl CopyingHelper for UIGraphicsPDFRendererFormat {
    type Result = Self;
}

#[cfg(feature = "UIGraphicsRenderer")]
unsafe impl NSObjectProtocol for UIGraphicsPDFRendererFormat {}

extern_methods!(
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsPDFRendererFormat {
        #[unsafe(method_family(none))]
        #[method_id(documentInfo)]
        pub unsafe fn documentInfo(&self) -> Retained<NSDictionary<NSString, AnyObject>>;

        /// Setter for [`documentInfo`][Self::documentInfo].
        #[method(setDocumentInfo:)]
        pub unsafe fn setDocumentInfo(&self, document_info: &NSDictionary<NSString, AnyObject>);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIGraphicsRendererFormat`
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsPDFRendererFormat {
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(defaultFormat)]
        pub unsafe fn defaultFormat() -> Retained<Self>;

        #[unsafe(method_family(none))]
        #[method_id(preferredFormat)]
        pub unsafe fn preferredFormat() -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsPDFRendererFormat {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uigraphicspdfrenderercontext?language=objc)
    #[unsafe(super(UIGraphicsRendererContext, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIGraphicsRenderer")]
    pub struct UIGraphicsPDFRendererContext;
);

#[cfg(feature = "UIGraphicsRenderer")]
unsafe impl NSObjectProtocol for UIGraphicsPDFRendererContext {}

extern_methods!(
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsPDFRendererContext {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(pdfContextBounds)]
        pub unsafe fn pdfContextBounds(&self) -> CGRect;

        #[method(beginPage)]
        pub unsafe fn beginPage(&self);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(beginPageWithBounds:pageInfo:)]
        pub unsafe fn beginPageWithBounds_pageInfo(
            &self,
            bounds: CGRect,
            page_info: &NSDictionary<NSString, AnyObject>,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setURL:forRect:)]
        pub unsafe fn setURL_forRect(&self, url: &NSURL, rect: CGRect);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(addDestinationWithName:atPoint:)]
        pub unsafe fn addDestinationWithName_atPoint(&self, name: &NSString, point: CGPoint);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setDestinationWithName:forRect:)]
        pub unsafe fn setDestinationWithName_forRect(&self, name: &NSString, rect: CGRect);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsPDFRendererContext {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uigraphicspdfrenderer?language=objc)
    #[unsafe(super(UIGraphicsRenderer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIGraphicsRenderer")]
    pub struct UIGraphicsPDFRenderer;
);

#[cfg(feature = "UIGraphicsRenderer")]
unsafe impl NSObjectProtocol for UIGraphicsPDFRenderer {}

extern_methods!(
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsPDFRenderer {
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method_family(init))]
        #[method_id(initWithBounds:format:)]
        pub unsafe fn initWithBounds_format(
            this: Allocated<Self>,
            bounds: CGRect,
            format: &UIGraphicsPDFRendererFormat,
        ) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method(writePDFToURL:withActions:error:_)]
        pub unsafe fn writePDFToURL_withActions_error(
            &self,
            url: &NSURL,
            actions: UIGraphicsPDFDrawingActions,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "block2")]
        #[unsafe(method_family(none))]
        #[method_id(PDFDataWithActions:)]
        pub unsafe fn PDFDataWithActions(
            &self,
            actions: UIGraphicsPDFDrawingActions,
        ) -> Retained<NSData>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIGraphicsRenderer`
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsPDFRenderer {
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method_family(init))]
        #[method_id(initWithBounds:)]
        pub unsafe fn initWithBounds(this: Allocated<Self>, bounds: CGRect) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsPDFRenderer {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
