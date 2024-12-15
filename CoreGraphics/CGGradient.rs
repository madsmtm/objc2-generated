//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cggradientref?language=objc)
pub type CGGradientRef = *mut c_void;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cggradientdrawingoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CGGradientDrawingOptions(pub u32);
bitflags::bitflags! {
    impl CGGradientDrawingOptions: u32 {
        const kCGGradientDrawsBeforeStartLocation = 1<<0;
        const kCGGradientDrawsAfterEndLocation = 1<<1;
    }
}

unsafe impl Encode for CGGradientDrawingOptions {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for CGGradientDrawingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGGradientGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGColorSpace", feature = "objc2-core-foundation"))]
    pub fn CGGradientCreateWithColorComponents(
        space: CGColorSpaceRef,
        components: *mut CGFloat,
        locations: *mut CGFloat,
        count: usize,
    ) -> CGGradientRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGColorSpace", feature = "objc2-core-foundation"))]
    pub fn CGGradientCreateWithColors(
        space: CGColorSpaceRef,
        colors: CFArrayRef,
        locations: *mut CGFloat,
    ) -> CGGradientRef;
}

extern "C-unwind" {
    pub fn CGGradientRetain(gradient: CGGradientRef) -> CGGradientRef;
}

extern "C-unwind" {
    pub fn CGGradientRelease(gradient: CGGradientRef);
}