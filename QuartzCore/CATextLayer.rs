//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/catextlayertruncationmode?language=objc)
// NS_TYPED_ENUM
pub type CATextLayerTruncationMode = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/catextlayeralignmentmode?language=objc)
// NS_TYPED_ENUM
pub type CATextLayerAlignmentMode = NSString;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/catextlayer?language=objc)
    #[unsafe(super(CALayer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CALayer")]
    pub struct CATextLayer;
);

#[cfg(all(feature = "CALayer", feature = "CAMediaTiming"))]
unsafe impl CAMediaTiming for CATextLayer {}

#[cfg(feature = "CALayer")]
unsafe impl NSCoding for CATextLayer {}

#[cfg(feature = "CALayer")]
unsafe impl NSObjectProtocol for CATextLayer {}

#[cfg(feature = "CALayer")]
unsafe impl NSSecureCoding for CATextLayer {}

extern_methods!(
    #[cfg(feature = "CALayer")]
    unsafe impl CATextLayer {
        #[method_id(@__retain_semantics Other string)]
        pub unsafe fn string(&self) -> Option<Retained<AnyObject>>;

        /// Setter for [`string`][Self::string].
        #[method(setString:)]
        pub unsafe fn setString(&self, string: Option<&AnyObject>);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(font)]
        pub unsafe fn font(&self) -> CFTypeRef;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`font`][Self::font].
        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: CFTypeRef);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(fontSize)]
        pub unsafe fn fontSize(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`fontSize`][Self::fontSize].
        #[method(setFontSize:)]
        pub unsafe fn setFontSize(&self, font_size: CGFloat);

        #[cfg(feature = "objc2-core-graphics")]
        #[method(foregroundColor)]
        pub unsafe fn foregroundColor(&self) -> CGColorRef;

        #[cfg(feature = "objc2-core-graphics")]
        /// Setter for [`foregroundColor`][Self::foregroundColor].
        #[method(setForegroundColor:)]
        pub unsafe fn setForegroundColor(&self, foreground_color: CGColorRef);

        #[method(isWrapped)]
        pub unsafe fn isWrapped(&self) -> bool;

        /// Setter for [`isWrapped`][Self::isWrapped].
        #[method(setWrapped:)]
        pub unsafe fn setWrapped(&self, wrapped: bool);

        #[method_id(@__retain_semantics Other truncationMode)]
        pub unsafe fn truncationMode(&self) -> Retained<CATextLayerTruncationMode>;

        /// Setter for [`truncationMode`][Self::truncationMode].
        #[method(setTruncationMode:)]
        pub unsafe fn setTruncationMode(&self, truncation_mode: &CATextLayerTruncationMode);

        #[method_id(@__retain_semantics Other alignmentMode)]
        pub unsafe fn alignmentMode(&self) -> Retained<CATextLayerAlignmentMode>;

        /// Setter for [`alignmentMode`][Self::alignmentMode].
        #[method(setAlignmentMode:)]
        pub unsafe fn setAlignmentMode(&self, alignment_mode: &CATextLayerAlignmentMode);

        #[method(allowsFontSubpixelQuantization)]
        pub unsafe fn allowsFontSubpixelQuantization(&self) -> bool;

        /// Setter for [`allowsFontSubpixelQuantization`][Self::allowsFontSubpixelQuantization].
        #[method(setAllowsFontSubpixelQuantization:)]
        pub unsafe fn setAllowsFontSubpixelQuantization(
            &self,
            allows_font_subpixel_quantization: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `CALayer`
    #[cfg(feature = "CALayer")]
    unsafe impl CATextLayer {
        /// Layer creation and initialization. *
        #[method_id(@__retain_semantics Other layer)]
        pub unsafe fn layer() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn initWithLayer(this: Allocated<Self>, layer: &AnyObject) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CALayer")]
    unsafe impl CATextLayer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcatruncationnone?language=objc)
    pub static kCATruncationNone: &'static CATextLayerTruncationMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcatruncationstart?language=objc)
    pub static kCATruncationStart: &'static CATextLayerTruncationMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcatruncationend?language=objc)
    pub static kCATruncationEnd: &'static CATextLayerTruncationMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcatruncationmiddle?language=objc)
    pub static kCATruncationMiddle: &'static CATextLayerTruncationMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaalignmentnatural?language=objc)
    pub static kCAAlignmentNatural: &'static CATextLayerAlignmentMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaalignmentleft?language=objc)
    pub static kCAAlignmentLeft: &'static CATextLayerAlignmentMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaalignmentright?language=objc)
    pub static kCAAlignmentRight: &'static CATextLayerAlignmentMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaalignmentcenter?language=objc)
    pub static kCAAlignmentCenter: &'static CATextLayerAlignmentMode;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcaalignmentjustified?language=objc)
    pub static kCAAlignmentJustified: &'static CATextLayerAlignmentMode;
}
