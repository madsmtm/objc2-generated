//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctfontattributename?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTFontAttributeName: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctforegroundcolorfromcontextattributename?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTForegroundColorFromContextAttributeName: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctkernattributename?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTKernAttributeName: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kcttrackingattributename?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTTrackingAttributeName: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctligatureattributename?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTLigatureAttributeName: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctforegroundcolorattributename?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTForegroundColorAttributeName: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctbackgroundcolorattributename?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTBackgroundColorAttributeName: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctparagraphstyleattributename?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTParagraphStyleAttributeName: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctstrokewidthattributename?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTStrokeWidthAttributeName: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctstrokecolorattributename?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTStrokeColorAttributeName: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctunderlinestyleattributename?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTUnderlineStyleAttributeName: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctsuperscriptattributename?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTSuperscriptAttributeName: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctunderlinecolorattributename?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTUnderlineColorAttributeName: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctverticalformsattributename?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTVerticalFormsAttributeName: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kcthorizontalinverticalformsattributename?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTHorizontalInVerticalFormsAttributeName: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctglyphinfoattributename?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTGlyphInfoAttributeName: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctcharactershapeattributename?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTCharacterShapeAttributeName: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctlanguageattributename?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTLanguageAttributeName: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctrundelegateattributename?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTRunDelegateAttributeName: CFStringRef;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coretext/ctunderlinestyle?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CTUnderlineStyle(pub i32);
bitflags::bitflags! {
    impl CTUnderlineStyle: i32 {
        const kCTUnderlineStyleNone = 0x00;
        const kCTUnderlineStyleSingle = 0x01;
        const kCTUnderlineStyleThick = 0x02;
        const kCTUnderlineStyleDouble = 0x09;
    }
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CTUnderlineStyle {
    const ENCODING: Encoding = i32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CTUnderlineStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coretext/ctunderlinestylemodifiers?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CTUnderlineStyleModifiers(pub i32);
bitflags::bitflags! {
    impl CTUnderlineStyleModifiers: i32 {
        const kCTUnderlinePatternSolid = 0x0000;
        const kCTUnderlinePatternDot = 0x0100;
        const kCTUnderlinePatternDash = 0x0200;
        const kCTUnderlinePatternDashDot = 0x0300;
        const kCTUnderlinePatternDashDotDot = 0x0400;
    }
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CTUnderlineStyleModifiers {
    const ENCODING: Encoding = i32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CTUnderlineStyleModifiers {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctbaselineclassattributename?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTBaselineClassAttributeName: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctbaselineinfoattributename?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTBaselineInfoAttributeName: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctbaselinereferenceinfoattributename?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTBaselineReferenceInfoAttributeName: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctbaselineoffsetattributename?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTBaselineOffsetAttributeName: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctwritingdirectionattributename?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTWritingDirectionAttributeName: CFStringRef;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctwritingdirectionembedding?language=objc)
pub const kCTWritingDirectionEmbedding: c_uint = 0 << 1;
/// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctwritingdirectionoverride?language=objc)
pub const kCTWritingDirectionOverride: c_uint = 1 << 1;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctrubyannotationattributename?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTRubyAnnotationAttributeName: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctadaptiveimageproviderattributename?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTAdaptiveImageProviderAttributeName: CFStringRef;
}
