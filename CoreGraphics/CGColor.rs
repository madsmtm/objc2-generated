//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgcolorref?language=objc)
pub type CGColorRef = *mut c_void;

extern "C-unwind" {
    #[cfg(all(feature = "CGColorSpace", feature = "objc2-core-foundation"))]
    pub fn CGColorCreate(space: CGColorSpaceRef, components: *mut CGFloat) -> CGColorRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGColorCreateGenericGray(gray: CGFloat, alpha: CGFloat) -> CGColorRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGColorCreateGenericRGB(
        red: CGFloat,
        green: CGFloat,
        blue: CGFloat,
        alpha: CGFloat,
    ) -> CGColorRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGColorCreateGenericCMYK(
        cyan: CGFloat,
        magenta: CGFloat,
        yellow: CGFloat,
        black: CGFloat,
        alpha: CGFloat,
    ) -> CGColorRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGColorCreateGenericGrayGamma2_2(gray: CGFloat, alpha: CGFloat) -> CGColorRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGColorCreateSRGB(
        red: CGFloat,
        green: CGFloat,
        blue: CGFloat,
        alpha: CGFloat,
    ) -> CGColorRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGColorGetConstantColor(color_name: CFStringRef) -> CGColorRef;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CGColorSpace",
        feature = "CGPattern",
        feature = "objc2-core-foundation"
    ))]
    pub fn CGColorCreateWithPattern(
        space: CGColorSpaceRef,
        pattern: CGPatternRef,
        components: *mut CGFloat,
    ) -> CGColorRef;
}

extern "C-unwind" {
    pub fn CGColorCreateCopy(color: CGColorRef) -> CGColorRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGColorCreateCopyWithAlpha(color: CGColorRef, alpha: CGFloat) -> CGColorRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGColorSpace", feature = "objc2-core-foundation"))]
    pub fn CGColorCreateCopyByMatchingToColorSpace(
        _: CGColorSpaceRef,
        intent: CGColorRenderingIntent,
        color: CGColorRef,
        options: CFDictionaryRef,
    ) -> CGColorRef;
}

extern "C-unwind" {
    pub fn CGColorRetain(color: CGColorRef) -> CGColorRef;
}

extern "C-unwind" {
    pub fn CGColorRelease(color: CGColorRef);
}

extern "C-unwind" {
    pub fn CGColorEqualToColor(color1: CGColorRef, color2: CGColorRef) -> bool;
}

extern "C-unwind" {
    pub fn CGColorGetNumberOfComponents(color: CGColorRef) -> usize;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGColorGetComponents(color: CGColorRef) -> *mut CGFloat;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGColorGetAlpha(color: CGColorRef) -> CGFloat;
}

extern "C-unwind" {
    #[cfg(feature = "CGColorSpace")]
    pub fn CGColorGetColorSpace(color: CGColorRef) -> CGColorSpaceRef;
}

extern "C-unwind" {
    #[cfg(feature = "CGPattern")]
    pub fn CGColorGetPattern(color: CGColorRef) -> CGPatternRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGColorGetTypeID() -> CFTypeID;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorwhite?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorWhite: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorblack?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorBlack: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorclear?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorClear: CFStringRef;
}