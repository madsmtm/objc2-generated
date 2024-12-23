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
    /// kCTFontSymbolicTrait
    ///
    /// Dictionary key to access the symbolic traits value.
    ///
    /// Use this key to access the symbolic traits value from the font traits dictionary. The value is returned as a CFNumberRef.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctfontsymbolictrait?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTFontSymbolicTrait: CFStringRef;
}

extern "C" {
    /// kCTFontWeightTrait
    ///
    /// Dictionary key to access the weight trait value.
    ///
    /// Use this key to access the normalized weight trait from the font traits dictionary. The value returned is a CFNumberRef representing a float value between -1.0 and 1.0 for normalized weight. The value of 0.0 corresponds to the regular or medium font weight.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctfontweighttrait?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTFontWeightTrait: CFStringRef;
}

extern "C" {
    /// kCTFontWidthTrait
    ///
    /// Dictionary key to access the width (condense/expand) trait value.
    ///
    /// Use this key to access the normalized proportion trait from the font traits dictionary. This value corresponds to the relative inter-glyph spacing for a given font. The value returned is a CFNumberRef representing a float between -1.0 and 1.0. The value of 0.0 corresponds to regular glyph spacing while negative values represent condensed glyph spacing.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctfontwidthtrait?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTFontWidthTrait: CFStringRef;
}

extern "C" {
    /// kCTFontSlantTrait
    ///
    /// Dictionary key to access the slant trait value.
    ///
    /// Use this key to access the normalized slant angle from the font traits dictionary. The value returned is a CFNumberRef representing a float value between -1.0 and 1.0 for normalized slant angle. The value or 0.0 corresponds to 0 degree clockwise rotation from the vertical and 1.0 corresponds to 30 degrees clockwise rotation.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctfontslanttrait?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTFontSlantTrait: CFStringRef;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctfontclassmaskshift?language=objc)
pub const kCTFontClassMaskShift: c_uint = 28;

/// Symbolic representation of stylistic font attributes.
///
/// CTFontSymbolicTraits symbolically describes stylistic aspects of a font. The top 4 bits is used to describe appearance of the font while the lower 28 bits for typeface. The font appearance information represented by the upper 4 bits can be used for stylistic font matching.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/ctfontsymbolictraits?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CTFontSymbolicTraits(pub u32);
bitflags::bitflags! {
    impl CTFontSymbolicTraits: u32 {
        const kCTFontTraitItalic = 1<<0;
        const kCTFontTraitBold = 1<<1;
        const kCTFontTraitExpanded = 1<<5;
        const kCTFontTraitCondensed = 1<<6;
        const kCTFontTraitMonoSpace = 1<<10;
        const kCTFontTraitVertical = 1<<11;
        const kCTFontTraitUIOptimized = 1<<12;
        const kCTFontTraitColorGlyphs = 1<<13;
        const kCTFontTraitComposite = 1<<14;
        const kCTFontTraitClassMask = 15<<kCTFontClassMaskShift;
        const kCTFontItalicTrait = CTFontSymbolicTraits::kCTFontTraitItalic.0;
        const kCTFontBoldTrait = CTFontSymbolicTraits::kCTFontTraitBold.0;
        const kCTFontExpandedTrait = CTFontSymbolicTraits::kCTFontTraitExpanded.0;
        const kCTFontCondensedTrait = CTFontSymbolicTraits::kCTFontTraitCondensed.0;
        const kCTFontMonoSpaceTrait = CTFontSymbolicTraits::kCTFontTraitMonoSpace.0;
        const kCTFontVerticalTrait = CTFontSymbolicTraits::kCTFontTraitVertical.0;
        const kCTFontUIOptimizedTrait = CTFontSymbolicTraits::kCTFontTraitUIOptimized.0;
        const kCTFontColorGlyphsTrait = CTFontSymbolicTraits::kCTFontTraitColorGlyphs.0;
        const kCTFontCompositeTrait = CTFontSymbolicTraits::kCTFontTraitComposite.0;
        const kCTFontClassMaskTrait = CTFontSymbolicTraits::kCTFontTraitClassMask.0;
    }
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CTFontSymbolicTraits {
    const ENCODING: Encoding = u32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CTFontSymbolicTraits {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Stylistic class values.
///
/// CTFontStylisticClass classifies certain stylistic qualities of the font. These values correspond closely to the font class values in the OpenType 'OS/2' table. The class values are bundled in the upper four bits of the CTFontSymbolicTraits and can be obtained via the kCTFontTraitClassMask.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/ctfontstylisticclass?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CTFontStylisticClass(pub u32);
bitflags::bitflags! {
    impl CTFontStylisticClass: u32 {
        const kCTFontClassUnknown = 0<<kCTFontClassMaskShift;
        const kCTFontClassOldStyleSerifs = 1<<kCTFontClassMaskShift;
        const kCTFontClassTransitionalSerifs = 2<<kCTFontClassMaskShift;
        const kCTFontClassModernSerifs = 3<<kCTFontClassMaskShift;
        const kCTFontClassClarendonSerifs = 4<<kCTFontClassMaskShift;
        const kCTFontClassSlabSerifs = 5<<kCTFontClassMaskShift;
        const kCTFontClassFreeformSerifs = 7<<kCTFontClassMaskShift;
        const kCTFontClassSansSerif = 8<<kCTFontClassMaskShift;
        const kCTFontClassOrnamentals = 9<<kCTFontClassMaskShift;
        const kCTFontClassScripts = 10<<kCTFontClassMaskShift;
        const kCTFontClassSymbolic = 12<<kCTFontClassMaskShift;
        const kCTFontUnknownClass = CTFontStylisticClass::kCTFontClassUnknown.0;
        const kCTFontOldStyleSerifsClass = CTFontStylisticClass::kCTFontClassOldStyleSerifs.0;
        const kCTFontTransitionalSerifsClass = CTFontStylisticClass::kCTFontClassTransitionalSerifs.0;
        const kCTFontModernSerifsClass = CTFontStylisticClass::kCTFontClassModernSerifs.0;
        const kCTFontClarendonSerifsClass = CTFontStylisticClass::kCTFontClassClarendonSerifs.0;
        const kCTFontSlabSerifsClass = CTFontStylisticClass::kCTFontClassSlabSerifs.0;
        const kCTFontFreeformSerifsClass = CTFontStylisticClass::kCTFontClassFreeformSerifs.0;
        const kCTFontSansSerifClass = CTFontStylisticClass::kCTFontClassSansSerif.0;
        const kCTFontOrnamentalsClass = CTFontStylisticClass::kCTFontClassOrnamentals.0;
        const kCTFontScriptsClass = CTFontStylisticClass::kCTFontClassScripts.0;
        const kCTFontSymbolicClass = CTFontStylisticClass::kCTFontClassSymbolic.0;
    }
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CTFontStylisticClass {
    const ENCODING: Encoding = u32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CTFontStylisticClass {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
