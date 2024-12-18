//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfcharactersetref?language=objc)
pub type CFCharacterSetRef = *mut c_void;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfmutablecharactersetref?language=objc)
pub type CFMutableCharacterSetRef = *mut c_void;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfcharactersetpredefinedset?language=objc)
// NS_ENUM
#[cfg(feature = "CFBase")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CFCharacterSetPredefinedSet(pub CFIndex);
#[cfg(feature = "CFBase")]
impl CFCharacterSetPredefinedSet {
    pub const kCFCharacterSetControl: Self = Self(1);
    pub const kCFCharacterSetWhitespace: Self = Self(2);
    pub const kCFCharacterSetWhitespaceAndNewline: Self = Self(3);
    pub const kCFCharacterSetDecimalDigit: Self = Self(4);
    pub const kCFCharacterSetLetter: Self = Self(5);
    pub const kCFCharacterSetLowercaseLetter: Self = Self(6);
    pub const kCFCharacterSetUppercaseLetter: Self = Self(7);
    pub const kCFCharacterSetNonBase: Self = Self(8);
    pub const kCFCharacterSetDecomposable: Self = Self(9);
    pub const kCFCharacterSetAlphaNumeric: Self = Self(10);
    pub const kCFCharacterSetPunctuation: Self = Self(11);
    pub const kCFCharacterSetCapitalizedLetter: Self = Self(13);
    pub const kCFCharacterSetSymbol: Self = Self(14);
    pub const kCFCharacterSetNewline: Self = Self(15);
    pub const kCFCharacterSetIllegal: Self = Self(12);
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl Encode for CFCharacterSetPredefinedSet {
    const ENCODING: Encoding = CFIndex::ENCODING;
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl RefEncode for CFCharacterSetPredefinedSet {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFCharacterSetGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFCharacterSetGetPredefined(
        the_set_identifier: CFCharacterSetPredefinedSet,
    ) -> CFCharacterSetRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFCharacterSetCreateWithCharactersInRange(
        alloc: CFAllocatorRef,
        the_range: CFRange,
    ) -> CFCharacterSetRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFCharacterSetCreateWithCharactersInString(
        alloc: CFAllocatorRef,
        the_string: CFStringRef,
    ) -> CFCharacterSetRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFData"))]
    pub fn CFCharacterSetCreateWithBitmapRepresentation(
        alloc: CFAllocatorRef,
        the_data: CFDataRef,
    ) -> CFCharacterSetRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFCharacterSetCreateInvertedSet(
        alloc: CFAllocatorRef,
        the_set: CFCharacterSetRef,
    ) -> CFCharacterSetRef;
}

extern "C-unwind" {
    pub fn CFCharacterSetIsSupersetOfSet(
        the_set: CFCharacterSetRef,
        the_otherset: CFCharacterSetRef,
    ) -> Boolean;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFCharacterSetHasMemberInPlane(
        the_set: CFCharacterSetRef,
        the_plane: CFIndex,
    ) -> Boolean;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFCharacterSetCreateMutable(alloc: CFAllocatorRef) -> CFMutableCharacterSetRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFCharacterSetCreateCopy(
        alloc: CFAllocatorRef,
        the_set: CFCharacterSetRef,
    ) -> CFCharacterSetRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFCharacterSetCreateMutableCopy(
        alloc: CFAllocatorRef,
        the_set: CFCharacterSetRef,
    ) -> CFMutableCharacterSetRef;
}

extern "C-unwind" {
    pub fn CFCharacterSetIsCharacterMember(
        the_set: CFCharacterSetRef,
        the_char: UniChar,
    ) -> Boolean;
}

extern "C-unwind" {
    pub fn CFCharacterSetIsLongCharacterMember(
        the_set: CFCharacterSetRef,
        the_char: UTF32Char,
    ) -> Boolean;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFData"))]
    pub fn CFCharacterSetCreateBitmapRepresentation(
        alloc: CFAllocatorRef,
        the_set: CFCharacterSetRef,
    ) -> CFDataRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFCharacterSetAddCharactersInRange(
        the_set: CFMutableCharacterSetRef,
        the_range: CFRange,
    );
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFCharacterSetRemoveCharactersInRange(
        the_set: CFMutableCharacterSetRef,
        the_range: CFRange,
    );
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFCharacterSetAddCharactersInString(
        the_set: CFMutableCharacterSetRef,
        the_string: CFStringRef,
    );
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFCharacterSetRemoveCharactersInString(
        the_set: CFMutableCharacterSetRef,
        the_string: CFStringRef,
    );
}

extern "C-unwind" {
    pub fn CFCharacterSetUnion(the_set: CFMutableCharacterSetRef, the_other_set: CFCharacterSetRef);
}

extern "C-unwind" {
    pub fn CFCharacterSetIntersect(
        the_set: CFMutableCharacterSetRef,
        the_other_set: CFCharacterSetRef,
    );
}

extern "C-unwind" {
    pub fn CFCharacterSetInvert(the_set: CFMutableCharacterSetRef);
}
