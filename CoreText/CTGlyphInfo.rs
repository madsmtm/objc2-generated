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
pub type CTGlyphInfoRef = *const c_void;

extern "C-unwind" {
    /// Returns the CFType of the glyph info object
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTGlyphInfoGetTypeID() -> CFTypeID;
}

/// These constants specify character collections.
///
///
/// Indicates that the character identifier is equal to the CGGlyph
/// glyph index.
///
///
/// Indicates the Adobe-CNS1 mapping.
///
///
/// Indicates the Adobe-GB1 mapping.
///
///
/// Indicates the Adobe-Japan1 mapping.
///
///
/// Indicates the Adobe-Japan2 mapping.
///
///
/// Indicates the Adobe-Korea1 mapping.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/ctcharactercollection?language=objc)
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
    /// Creates an immutable glyph info object.
    ///
    ///
    /// This function creates an immutable glyph info object for a glyph
    /// name such as "copyright" and a specified font.
    ///
    ///
    /// Parameter `glyphName`: The name of the glyph.
    ///
    ///
    /// Parameter `font`: The font to be associated with the returned CTGlyphInfo object.
    ///
    ///
    /// Parameter `baseString`: The part of the string the returned object is intended
    /// to override.
    ///
    ///
    /// Returns: This function will return a reference to a CTGlyphInfo object.
    #[cfg(all(feature = "CTFont", feature = "objc2-core-foundation"))]
    pub fn CTGlyphInfoCreateWithGlyphName(
        glyph_name: CFStringRef,
        font: CTFontRef,
        base_string: CFStringRef,
    ) -> CTGlyphInfoRef;
}

extern "C-unwind" {
    /// Creates an immutable glyph info object.
    ///
    ///
    /// This function creates an immutable glyph info object for a glyph
    /// index and a specified font.
    ///
    ///
    /// Parameter `glyph`: The glyph identifier.
    ///
    ///
    /// Parameter `font`: The font to be associated with the returned CTGlyphInfo object.
    ///
    ///
    /// Parameter `baseString`: The part of the string the returned object is intended
    /// to override.
    ///
    ///
    /// Returns: This function will return a reference to a CTGlyphInfo object.
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
    /// Creates an immutable glyph info object.
    ///
    ///
    /// This function creates an immutable glyph info object for a
    /// character identifier and a character collection.
    ///
    ///
    /// Parameter `cid`: A character identifier.
    ///
    ///
    /// Parameter `collection`: A character collection identifier.
    ///
    ///
    /// Parameter `baseString`: The part of the string the returned object is intended
    /// to override.
    ///
    ///
    /// Returns: This function will return a reference to a CTGlyphInfo object.
    #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-core-graphics"))]
    pub fn CTGlyphInfoCreateWithCharacterIdentifier(
        cid: CGFontIndex,
        collection: CTCharacterCollection,
        base_string: CFStringRef,
    ) -> CTGlyphInfoRef;
}

extern "C-unwind" {
    /// Gets the glyph name for a glyph info, if applicable.
    ///
    ///
    /// This function will return the glyph name.
    ///
    ///
    /// Parameter `glyphInfo`: The glyph info for which you would like the glyph name.
    ///
    ///
    /// Returns: If the glyph info object was created with a glyph name, it will
    /// be returned. Otherwise, this function will return NULL.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTGlyphInfoGetGlyphName(glyph_info: CTGlyphInfoRef) -> CFStringRef;
}

extern "C-unwind" {
    /// Gets the glyph for a glyph info, if applicable.
    ///
    ///
    /// This function will return the glyph.
    ///
    ///
    /// Parameter `glyphInfo`: The glyph info from which you would like the glyph.
    ///
    ///
    /// Returns: If the glyph info object was created with a font, it will be
    /// returned. Otherwise, this function will return 0.
    #[cfg(feature = "objc2-core-graphics")]
    pub fn CTGlyphInfoGetGlyph(glyph_info: CTGlyphInfoRef) -> CGGlyph;
}

extern "C-unwind" {
    /// Gets the character identifier for a glyph info.
    ///
    ///
    /// This function will return the character identifier.
    ///
    ///
    /// Parameter `glyphInfo`: The glyph info for which you would like the character identifier.
    ///
    ///
    /// Returns: If the glyph info object was created with a character identifier,
    /// it will be returned. Otherwise, this function will return 0.
    #[cfg(feature = "objc2-core-graphics")]
    pub fn CTGlyphInfoGetCharacterIdentifier(glyph_info: CTGlyphInfoRef) -> CGFontIndex;
}

extern "C-unwind" {
    /// Gets the character collection for a glyph info.
    ///
    ///
    /// This function will return the character collection. If the glyph
    /// info object was created with a glyph name or a glyph index, its
    /// character collection will be
    /// kCTIdentityMappingCharacterCollection.
    ///
    ///
    /// Parameter `glyphInfo`: The glyph info for which you would like the character collection.
    ///
    ///
    /// Returns: This function will return the character collection of the given
    /// glyph info.
    pub fn CTGlyphInfoGetCharacterCollection(glyph_info: CTGlyphInfoRef) -> CTCharacterCollection;
}
