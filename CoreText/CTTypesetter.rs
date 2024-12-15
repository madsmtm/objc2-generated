//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coretext/cttypesetterref?language=objc)
pub type CTTypesetterRef = *mut c_void;

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTTypesetterGetTypeID() -> CFTypeID;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kcttypesetteroptionallowunboundedlayout?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTTypesetterOptionAllowUnboundedLayout: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kcttypesetteroptiondisablebidiprocessing?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTTypesetterOptionDisableBidiProcessing: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/kcttypesetteroptionforcedembeddinglevel?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTTypesetterOptionForcedEmbeddingLevel: CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTTypesetterCreateWithAttributedString(string: CFAttributedStringRef)
        -> CTTypesetterRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTTypesetterCreateWithAttributedStringAndOptions(
        string: CFAttributedStringRef,
        options: CFDictionaryRef,
    ) -> CTTypesetterRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CTLine", feature = "objc2-core-foundation"))]
    pub fn CTTypesetterCreateLineWithOffset(
        typesetter: CTTypesetterRef,
        string_range: CFRange,
        offset: c_double,
    ) -> CTLineRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CTLine", feature = "objc2-core-foundation"))]
    pub fn CTTypesetterCreateLine(typesetter: CTTypesetterRef, string_range: CFRange) -> CTLineRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTTypesetterSuggestLineBreakWithOffset(
        typesetter: CTTypesetterRef,
        start_index: CFIndex,
        width: c_double,
        offset: c_double,
    ) -> CFIndex;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTTypesetterSuggestLineBreak(
        typesetter: CTTypesetterRef,
        start_index: CFIndex,
        width: c_double,
    ) -> CFIndex;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTTypesetterSuggestClusterBreakWithOffset(
        typesetter: CTTypesetterRef,
        start_index: CFIndex,
        width: c_double,
        offset: c_double,
    ) -> CFIndex;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTTypesetterSuggestClusterBreak(
        typesetter: CTTypesetterRef,
        start_index: CFIndex,
        width: c_double,
    ) -> CFIndex;
}