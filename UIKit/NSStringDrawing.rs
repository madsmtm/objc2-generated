//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/nsstringdrawingcontext?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSStringDrawingContext;
);

unsafe impl NSObjectProtocol for NSStringDrawingContext {}

extern_methods!(
    unsafe impl NSStringDrawingContext {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(minimumScaleFactor)]
        pub unsafe fn minimumScaleFactor(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`minimumScaleFactor`][Self::minimumScaleFactor].
        #[method(setMinimumScaleFactor:)]
        pub unsafe fn setMinimumScaleFactor(&self, minimum_scale_factor: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(actualScaleFactor)]
        pub unsafe fn actualScaleFactor(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(totalBounds)]
        pub unsafe fn totalBounds(&self) -> CGRect;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSStringDrawingContext {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_category!(
    /// Category on [`NSString`].
    pub unsafe trait NSStringDrawing {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(sizeWithAttributes:)]
        unsafe fn sizeWithAttributes(
            &self,
            attrs: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
        ) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(drawAtPoint:withAttributes:)]
        unsafe fn drawAtPoint_withAttributes(
            &self,
            point: CGPoint,
            attrs: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[method(drawInRect:withAttributes:)]
        unsafe fn drawInRect_withAttributes(
            &self,
            rect: CGRect,
            attrs: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
        );
    }

    unsafe impl NSStringDrawing for NSString {}
);

extern_category!(
    /// Category "NSStringDrawing" on [`NSAttributedString`].
    #[doc(alias = "NSStringDrawing")]
    pub unsafe trait NSAttributedStringNSStringDrawing {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(size)]
        unsafe fn size(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(drawAtPoint:)]
        unsafe fn drawAtPoint(&self, point: CGPoint);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(drawInRect:)]
        unsafe fn drawInRect(&self, rect: CGRect);
    }

    unsafe impl NSAttributedStringNSStringDrawing for NSAttributedString {}
);

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/nsstringdrawingoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSStringDrawingOptions(pub NSInteger);
bitflags::bitflags! {
    impl NSStringDrawingOptions: NSInteger {
        #[doc(alias = "NSStringDrawingUsesLineFragmentOrigin")]
        const UsesLineFragmentOrigin = 1<<0;
        #[doc(alias = "NSStringDrawingUsesFontLeading")]
        const UsesFontLeading = 1<<1;
        #[doc(alias = "NSStringDrawingUsesDeviceMetrics")]
        const UsesDeviceMetrics = 1<<3;
        #[doc(alias = "NSStringDrawingTruncatesLastVisibleLine")]
        const TruncatesLastVisibleLine = 1<<5;
    }
}

unsafe impl Encode for NSStringDrawingOptions {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSStringDrawingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_category!(
    /// Category "NSExtendedStringDrawing" on [`NSString`].
    #[doc(alias = "NSExtendedStringDrawing")]
    pub unsafe trait NSStringNSExtendedStringDrawing {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(drawWithRect:options:attributes:context:)]
        unsafe fn drawWithRect_options_attributes_context(
            &self,
            rect: CGRect,
            options: NSStringDrawingOptions,
            attributes: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
            context: Option<&NSStringDrawingContext>,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[method(boundingRectWithSize:options:attributes:context:)]
        unsafe fn boundingRectWithSize_options_attributes_context(
            &self,
            size: CGSize,
            options: NSStringDrawingOptions,
            attributes: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
            context: Option<&NSStringDrawingContext>,
        ) -> CGRect;
    }

    unsafe impl NSStringNSExtendedStringDrawing for NSString {}
);

extern_category!(
    /// Category "NSExtendedStringDrawing" on [`NSAttributedString`].
    #[doc(alias = "NSExtendedStringDrawing")]
    pub unsafe trait NSAttributedStringNSExtendedStringDrawing {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(drawWithRect:options:context:)]
        unsafe fn drawWithRect_options_context(
            &self,
            rect: CGRect,
            options: NSStringDrawingOptions,
            context: Option<&NSStringDrawingContext>,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[method(boundingRectWithSize:options:context:)]
        unsafe fn boundingRectWithSize_options_context(
            &self,
            size: CGSize,
            options: NSStringDrawingOptions,
            context: Option<&NSStringDrawingContext>,
        ) -> CGRect;
    }

    unsafe impl NSAttributedStringNSExtendedStringDrawing for NSAttributedString {}
);

extern_methods!(
    /// NSStringDrawingContextDeprecated
    /// ********************** Deprecated ***********************
    unsafe impl NSStringDrawingContext {
        #[cfg(feature = "objc2-core-foundation")]
        #[deprecated]
        #[method(minimumTrackingAdjustment)]
        pub unsafe fn minimumTrackingAdjustment(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`minimumTrackingAdjustment`][Self::minimumTrackingAdjustment].
        #[deprecated]
        #[method(setMinimumTrackingAdjustment:)]
        pub unsafe fn setMinimumTrackingAdjustment(&self, minimum_tracking_adjustment: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[deprecated]
        #[method(actualTrackingAdjustment)]
        pub unsafe fn actualTrackingAdjustment(&self) -> CGFloat;
    }
);
