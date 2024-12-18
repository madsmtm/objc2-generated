//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coretext/ctglyphinforef?language=objc)
pub type CTGlyphInfoRef = *mut c_void;

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTGlyphInfoGetTypeID() -> CFTypeID;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coretext/ctcharactercollection?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CTCharacterCollection(pub u16);
impl CTCharacterCollection {
    pub const kCTCharacterCollectionIdentityMapping: Self = Self(0);
    pub const kCTCharacterCollectionAdobeCNS1: Self = Self(1);
    pub const kCTCharacterCollectionAdobeGB1: Self = Self(2);
    pub const kCTCharacterCollectionAdobeJapan1: Self = Self(3);
    pub const kCTCharacterCollectionAdobeJapan2: Self = Self(4);
    pub const kCTCharacterCollectionAdobeKorea1: Self = Self(5);
    #[deprecated = "Deprecated"]
    pub const kCTIdentityMappingCharacterCollection: Self =
        Self(CTCharacterCollection::kCTCharacterCollectionIdentityMapping.0);
    #[deprecated = "Deprecated"]
    pub const kCTAdobeCNS1CharacterCollection: Self =
        Self(CTCharacterCollection::kCTCharacterCollectionAdobeCNS1.0);
    #[deprecated = "Deprecated"]
    pub const kCTAdobeGB1CharacterCollection: Self =
        Self(CTCharacterCollection::kCTCharacterCollectionAdobeGB1.0);
    #[deprecated = "Deprecated"]
    pub const kCTAdobeJapan1CharacterCollection: Self =
        Self(CTCharacterCollection::kCTCharacterCollectionAdobeJapan1.0);
    #[deprecated = "Deprecated"]
    pub const kCTAdobeJapan2CharacterCollection: Self =
        Self(CTCharacterCollection::kCTCharacterCollectionAdobeJapan2.0);
    #[deprecated = "Deprecated"]
    pub const kCTAdobeKorea1CharacterCollection: Self =
        Self(CTCharacterCollection::kCTCharacterCollectionAdobeKorea1.0);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CTCharacterCollection {
    const ENCODING: Encoding = u16::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CTCharacterCollection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(all(feature = "CTFont", feature = "objc2-core-foundation"))]
    pub fn CTGlyphInfoCreateWithGlyphName(
        glyph_name: CFStringRef,
        font: CTFontRef,
        base_string: CFStringRef,
    ) -> CTGlyphInfoRef;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CTFont",
        feature = "objc2-core-foundation",
        feature = "objc2-core-graphics"
    ))]
    pub fn CTGlyphInfoCreateWithGlyph(
        glyph: CGGlyph,
        font: CTFontRef,
        base_string: CFStringRef,
    ) -> CTGlyphInfoRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-core-graphics"))]
    pub fn CTGlyphInfoCreateWithCharacterIdentifier(
        cid: CGFontIndex,
        collection: CTCharacterCollection,
        base_string: CFStringRef,
    ) -> CTGlyphInfoRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTGlyphInfoGetGlyphName(glyph_info: CTGlyphInfoRef) -> CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-graphics")]
    pub fn CTGlyphInfoGetGlyph(glyph_info: CTGlyphInfoRef) -> CGGlyph;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-graphics")]
    pub fn CTGlyphInfoGetCharacterIdentifier(glyph_info: CTGlyphInfoRef) -> CGFontIndex;
}

extern "C-unwind" {
    pub fn CTGlyphInfoGetCharacterCollection(glyph_info: CTGlyphInfoRef) -> CTCharacterCollection;
}
