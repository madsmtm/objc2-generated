//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-graphics")]
#[cfg(target_vendor = "apple")]
use objc2_core_graphics::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsglyphinfo?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSGlyphInfo;
);

unsafe impl NSCoding for NSGlyphInfo {}

unsafe impl NSCopying for NSGlyphInfo {}

unsafe impl CopyingHelper for NSGlyphInfo {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSGlyphInfo {}

unsafe impl NSSecureCoding for NSGlyphInfo {}

extern_methods!(
    unsafe impl NSGlyphInfo {
        #[cfg(all(feature = "NSFont", feature = "objc2-core-graphics"))]
        #[cfg(target_vendor = "apple")]
        #[unsafe(method_family(none))]
        #[method_id(glyphInfoWithCGGlyph:forFont:baseString:)]
        pub unsafe fn glyphInfoWithCGGlyph_forFont_baseString(
            glyph: CGGlyph,
            font: &NSFont,
            string: &NSString,
        ) -> Option<Retained<NSGlyphInfo>>;

        #[cfg(feature = "objc2-core-graphics")]
        #[cfg(target_vendor = "apple")]
        #[method(glyphID)]
        pub unsafe fn glyphID(&self) -> CGGlyph;

        #[unsafe(method_family(none))]
        #[method_id(baseString)]
        pub unsafe fn baseString(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSGlyphInfo {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscharactercollection?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSCharacterCollection(pub NSUInteger);
impl NSCharacterCollection {
    #[doc(alias = "NSIdentityMappingCharacterCollection")]
    pub const IdentityMappingCharacterCollection: Self = Self(0);
    #[doc(alias = "NSAdobeCNS1CharacterCollection")]
    pub const AdobeCNS1CharacterCollection: Self = Self(1);
    #[doc(alias = "NSAdobeGB1CharacterCollection")]
    pub const AdobeGB1CharacterCollection: Self = Self(2);
    #[doc(alias = "NSAdobeJapan1CharacterCollection")]
    pub const AdobeJapan1CharacterCollection: Self = Self(3);
    #[doc(alias = "NSAdobeJapan2CharacterCollection")]
    pub const AdobeJapan2CharacterCollection: Self = Self(4);
    #[doc(alias = "NSAdobeKorea1CharacterCollection")]
    pub const AdobeKorea1CharacterCollection: Self = Self(5);
}

unsafe impl Encode for NSCharacterCollection {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSCharacterCollection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// NSGlyphInfo_Deprecated
    unsafe impl NSGlyphInfo {
        #[cfg(feature = "NSFont")]
        #[unsafe(method_family(none))]
        #[method_id(glyphInfoWithGlyphName:forFont:baseString:)]
        pub unsafe fn glyphInfoWithGlyphName_forFont_baseString(
            glyph_name: &NSString,
            font: &NSFont,
            string: &NSString,
        ) -> Option<Retained<NSGlyphInfo>>;

        #[cfg(feature = "NSFont")]
        #[unsafe(method_family(none))]
        #[method_id(glyphInfoWithGlyph:forFont:baseString:)]
        pub unsafe fn glyphInfoWithGlyph_forFont_baseString(
            glyph: NSGlyph,
            font: &NSFont,
            string: &NSString,
        ) -> Option<Retained<NSGlyphInfo>>;

        #[unsafe(method_family(none))]
        #[method_id(glyphInfoWithCharacterIdentifier:collection:baseString:)]
        pub unsafe fn glyphInfoWithCharacterIdentifier_collection_baseString(
            cid: NSUInteger,
            character_collection: NSCharacterCollection,
            string: &NSString,
        ) -> Option<Retained<NSGlyphInfo>>;

        #[unsafe(method_family(none))]
        #[method_id(glyphName)]
        pub unsafe fn glyphName(&self) -> Option<Retained<NSString>>;

        #[method(characterIdentifier)]
        pub unsafe fn characterIdentifier(&self) -> NSUInteger;

        #[method(characterCollection)]
        pub unsafe fn characterCollection(&self) -> NSCharacterCollection;
    }
);
