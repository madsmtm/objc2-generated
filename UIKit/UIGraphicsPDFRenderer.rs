//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[cfg(all(feature = "UIGraphicsRenderer", feature = "block2"))]
pub type UIGraphicsPDFDrawingActions =
    *mut block2::Block<dyn Fn(NonNull<UIGraphicsPDFRendererContext>)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIGraphicsRenderer")]
    pub struct UIGraphicsPDFRendererFormat;

    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl ClassType for UIGraphicsPDFRendererFormat {
        #[inherits(NSObject)]
        type Super = UIGraphicsRendererFormat;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "UIGraphicsRenderer")]
unsafe impl NSCopying for UIGraphicsPDFRendererFormat {}

#[cfg(feature = "UIGraphicsRenderer")]
unsafe impl NSObjectProtocol for UIGraphicsPDFRendererFormat {}

extern_methods!(
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsPDFRendererFormat {
        #[method_id(@__retain_semantics Other documentInfo)]
        pub unsafe fn documentInfo(&self) -> Id<NSDictionary<NSString, AnyObject>>;

        #[method(setDocumentInfo:)]
        pub unsafe fn setDocumentInfo(&self, document_info: &NSDictionary<NSString, AnyObject>);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIGraphicsRendererFormat`
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsPDFRendererFormat {
        #[deprecated]
        #[method_id(@__retain_semantics Other defaultFormat)]
        pub unsafe fn defaultFormat() -> Id<Self>;

        #[method_id(@__retain_semantics Other preferredFormat)]
        pub unsafe fn preferredFormat() -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsPDFRendererFormat {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIGraphicsRenderer")]
    pub struct UIGraphicsPDFRendererContext;

    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl ClassType for UIGraphicsPDFRendererContext {
        #[inherits(NSObject)]
        type Super = UIGraphicsRendererContext;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "UIGraphicsRenderer")]
unsafe impl NSObjectProtocol for UIGraphicsPDFRendererContext {}

extern_methods!(
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsPDFRendererContext {
        #[method(pdfContextBounds)]
        pub unsafe fn pdfContextBounds(&self) -> CGRect;

        #[method(beginPage)]
        pub unsafe fn beginPage(&self);

        #[method(beginPageWithBounds:pageInfo:)]
        pub unsafe fn beginPageWithBounds_pageInfo(
            &self,
            bounds: CGRect,
            page_info: &NSDictionary<NSString, AnyObject>,
        );

        #[method(setURL:forRect:)]
        pub unsafe fn setURL_forRect(&self, url: &NSURL, rect: CGRect);

        #[method(addDestinationWithName:atPoint:)]
        pub unsafe fn addDestinationWithName_atPoint(&self, name: &NSString, point: CGPoint);

        #[method(setDestinationWithName:forRect:)]
        pub unsafe fn setDestinationWithName_forRect(&self, name: &NSString, rect: CGRect);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsPDFRendererContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIGraphicsRenderer")]
    pub struct UIGraphicsPDFRenderer;

    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl ClassType for UIGraphicsPDFRenderer {
        #[inherits(NSObject)]
        type Super = UIGraphicsRenderer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "UIGraphicsRenderer")]
unsafe impl NSObjectProtocol for UIGraphicsPDFRenderer {}

extern_methods!(
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsPDFRenderer {
        #[method_id(@__retain_semantics Init initWithBounds:format:)]
        pub unsafe fn initWithBounds_format(
            this: Allocated<Self>,
            bounds: CGRect,
            format: &UIGraphicsPDFRendererFormat,
        ) -> Id<Self>;

        #[cfg(feature = "block2")]
        #[method(writePDFToURL:withActions:error:_)]
        pub unsafe fn writePDFToURL_withActions_error(
            &self,
            url: &NSURL,
            actions: UIGraphicsPDFDrawingActions,
        ) -> Result<(), Id<NSError>>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Other PDFDataWithActions:)]
        pub unsafe fn PDFDataWithActions(&self, actions: UIGraphicsPDFDrawingActions)
            -> Id<NSData>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIGraphicsRenderer`
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsPDFRenderer {
        #[method_id(@__retain_semantics Init initWithBounds:)]
        pub unsafe fn initWithBounds(this: Allocated<Self>, bounds: CGRect) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIGraphicsRenderer")]
    unsafe impl UIGraphicsPDFRenderer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);