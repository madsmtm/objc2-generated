//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfstringencoding?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CFStringEncoding(pub u32);
impl CFStringEncoding {
    pub const kCFStringEncodingMacRoman: Self = Self(0);
    pub const kCFStringEncodingWindowsLatin1: Self = Self(0x0500);
    pub const kCFStringEncodingISOLatin1: Self = Self(0x0201);
    pub const kCFStringEncodingNextStepLatin: Self = Self(0x0B01);
    pub const kCFStringEncodingASCII: Self = Self(0x0600);
    pub const kCFStringEncodingUnicode: Self = Self(0x0100);
    pub const kCFStringEncodingUTF8: Self = Self(0x08000100);
    pub const kCFStringEncodingNonLossyASCII: Self = Self(0x0BFF);
    pub const kCFStringEncodingUTF16: Self = Self(0x0100);
    pub const kCFStringEncodingUTF16BE: Self = Self(0x10000100);
    pub const kCFStringEncodingUTF16LE: Self = Self(0x14000100);
    pub const kCFStringEncodingUTF32: Self = Self(0x0c000100);
    pub const kCFStringEncodingUTF32BE: Self = Self(0x18000100);
    pub const kCFStringEncodingUTF32LE: Self = Self(0x1c000100);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CFStringEncoding {
    const ENCODING: Encoding = u32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CFStringEncoding {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    /// * Immutable string creation functions **
    #[cfg(feature = "CFBase")]
    pub fn CFStringCreateWithPascalString(
        alloc: CFAllocatorRef,
        p_str: ConstStr255Param,
        encoding: CFStringEncoding,
    ) -> CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringCreateWithCString(
        alloc: CFAllocatorRef,
        c_str: *const c_char,
        encoding: CFStringEncoding,
    ) -> CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringCreateWithBytes(
        alloc: CFAllocatorRef,
        bytes: *const u8,
        num_bytes: CFIndex,
        encoding: CFStringEncoding,
        is_external_representation: Boolean,
    ) -> CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringCreateWithCharacters(
        alloc: CFAllocatorRef,
        chars: *const UniChar,
        num_chars: CFIndex,
    ) -> CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringCreateWithPascalStringNoCopy(
        alloc: CFAllocatorRef,
        p_str: ConstStr255Param,
        encoding: CFStringEncoding,
        contents_deallocator: CFAllocatorRef,
    ) -> CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringCreateWithCStringNoCopy(
        alloc: CFAllocatorRef,
        c_str: *const c_char,
        encoding: CFStringEncoding,
        contents_deallocator: CFAllocatorRef,
    ) -> CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringCreateWithBytesNoCopy(
        alloc: CFAllocatorRef,
        bytes: *const u8,
        num_bytes: CFIndex,
        encoding: CFStringEncoding,
        is_external_representation: Boolean,
        contents_deallocator: CFAllocatorRef,
    ) -> CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringCreateWithCharactersNoCopy(
        alloc: CFAllocatorRef,
        chars: *const UniChar,
        num_chars: CFIndex,
        contents_deallocator: CFAllocatorRef,
    ) -> CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringCreateWithSubstring(
        alloc: CFAllocatorRef,
        str: CFStringRef,
        range: CFRange,
    ) -> CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringCreateCopy(alloc: CFAllocatorRef, the_string: CFStringRef) -> CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringCreateMutable(alloc: CFAllocatorRef, max_length: CFIndex) -> CFMutableStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringCreateMutableCopy(
        alloc: CFAllocatorRef,
        max_length: CFIndex,
        the_string: CFStringRef,
    ) -> CFMutableStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringCreateMutableWithExternalCharactersNoCopy(
        alloc: CFAllocatorRef,
        chars: *mut UniChar,
        num_chars: CFIndex,
        capacity: CFIndex,
        external_characters_allocator: CFAllocatorRef,
    ) -> CFMutableStringRef;
}

extern "C-unwind" {
    /// * Basic accessors for the contents **
    #[cfg(feature = "CFBase")]
    pub fn CFStringGetLength(the_string: CFStringRef) -> CFIndex;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringGetCharacterAtIndex(the_string: CFStringRef, idx: CFIndex) -> UniChar;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringGetCharacters(the_string: CFStringRef, range: CFRange, buffer: *mut UniChar);
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringGetPascalString(
        the_string: CFStringRef,
        buffer: StringPtr,
        buffer_size: CFIndex,
        encoding: CFStringEncoding,
    ) -> Boolean;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringGetCString(
        the_string: CFStringRef,
        buffer: *mut c_char,
        buffer_size: CFIndex,
        encoding: CFStringEncoding,
    ) -> Boolean;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringGetPascalStringPtr(
        the_string: CFStringRef,
        encoding: CFStringEncoding,
    ) -> ConstStringPtr;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringGetCStringPtr(
        the_string: CFStringRef,
        encoding: CFStringEncoding,
    ) -> *const c_char;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringGetCharactersPtr(the_string: CFStringRef) -> *const UniChar;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringGetBytes(
        the_string: CFStringRef,
        range: CFRange,
        encoding: CFStringEncoding,
        loss_byte: u8,
        is_external_representation: Boolean,
        buffer: *mut u8,
        max_buf_len: CFIndex,
        used_buf_len: *mut CFIndex,
    ) -> CFIndex;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFData"))]
    pub fn CFStringCreateFromExternalRepresentation(
        alloc: CFAllocatorRef,
        data: CFDataRef,
        encoding: CFStringEncoding,
    ) -> CFStringRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFData"))]
    pub fn CFStringCreateExternalRepresentation(
        alloc: CFAllocatorRef,
        the_string: CFStringRef,
        encoding: CFStringEncoding,
        loss_byte: u8,
    ) -> CFDataRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringGetSmallestEncoding(the_string: CFStringRef) -> CFStringEncoding;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringGetFastestEncoding(the_string: CFStringRef) -> CFStringEncoding;
}

extern "C-unwind" {
    pub fn CFStringGetSystemEncoding() -> CFStringEncoding;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringGetMaximumSizeForEncoding(
        length: CFIndex,
        encoding: CFStringEncoding,
    ) -> CFIndex;
}

extern "C-unwind" {
    /// * FileSystem path conversion functions **
    #[cfg(feature = "CFBase")]
    pub fn CFStringGetFileSystemRepresentation(
        string: CFStringRef,
        buffer: *mut c_char,
        max_buf_len: CFIndex,
    ) -> Boolean;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringGetMaximumSizeOfFileSystemRepresentation(string: CFStringRef) -> CFIndex;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringCreateWithFileSystemRepresentation(
        alloc: CFAllocatorRef,
        buffer: *const c_char,
    ) -> CFStringRef;
}

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfstringcompareflags?language=objc)
// NS_OPTIONS
#[cfg(feature = "CFBase")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CFStringCompareFlags(pub CFOptionFlags);
#[cfg(feature = "CFBase")]
bitflags::bitflags! {
    impl CFStringCompareFlags: CFOptionFlags {
        const kCFCompareCaseInsensitive = 1;
        const kCFCompareBackwards = 4;
        const kCFCompareAnchored = 8;
        const kCFCompareNonliteral = 16;
        const kCFCompareLocalized = 32;
        const kCFCompareNumerically = 64;
        const kCFCompareDiacriticInsensitive = 128;
        const kCFCompareWidthInsensitive = 256;
        const kCFCompareForcedOrdering = 512;
    }
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl Encode for CFStringCompareFlags {
    const ENCODING: Encoding = CFOptionFlags::ENCODING;
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl RefEncode for CFStringCompareFlags {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFLocale"))]
    pub fn CFStringCompareWithOptionsAndLocale(
        the_string1: CFStringRef,
        the_string2: CFStringRef,
        range_to_compare: CFRange,
        compare_options: CFStringCompareFlags,
        locale: CFLocaleRef,
    ) -> CFComparisonResult;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringCompareWithOptions(
        the_string1: CFStringRef,
        the_string2: CFStringRef,
        range_to_compare: CFRange,
        compare_options: CFStringCompareFlags,
    ) -> CFComparisonResult;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringCompare(
        the_string1: CFStringRef,
        the_string2: CFStringRef,
        compare_options: CFStringCompareFlags,
    ) -> CFComparisonResult;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFLocale"))]
    pub fn CFStringFindWithOptionsAndLocale(
        the_string: CFStringRef,
        string_to_find: CFStringRef,
        range_to_search: CFRange,
        search_options: CFStringCompareFlags,
        locale: CFLocaleRef,
        result: *mut CFRange,
    ) -> Boolean;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringFindWithOptions(
        the_string: CFStringRef,
        string_to_find: CFStringRef,
        range_to_search: CFRange,
        search_options: CFStringCompareFlags,
        result: *mut CFRange,
    ) -> Boolean;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFArray", feature = "CFBase"))]
    pub fn CFStringCreateArrayWithFindResults(
        alloc: CFAllocatorRef,
        the_string: CFStringRef,
        string_to_find: CFStringRef,
        range_to_search: CFRange,
        compare_options: CFStringCompareFlags,
    ) -> CFArrayRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringFind(
        the_string: CFStringRef,
        string_to_find: CFStringRef,
        compare_options: CFStringCompareFlags,
    ) -> CFRange;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringHasPrefix(the_string: CFStringRef, prefix: CFStringRef) -> Boolean;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringHasSuffix(the_string: CFStringRef, suffix: CFStringRef) -> Boolean;
}

extern "C-unwind" {
    /// Returns the range of the composed character sequence at the specified index.
    ///
    /// Parameter `theString`: The CFString which is to be searched.  If this
    /// parameter is not a valid CFString, the behavior is
    /// undefined.
    ///
    /// Parameter `theIndex`: The index of the character contained in the
    /// composed character sequence.  If the index is
    /// outside the index space of the string (0 to N-1 inclusive,
    /// where N is the length of the string), the behavior is
    /// undefined.
    ///
    /// Returns: The range of the composed character sequence.
    #[cfg(feature = "CFBase")]
    pub fn CFStringGetRangeOfComposedCharactersAtIndex(
        the_string: CFStringRef,
        the_index: CFIndex,
    ) -> CFRange;
}

extern "C-unwind" {
    /// Query the range of the first character contained in the specified character set.
    ///
    /// Parameter `theString`: The CFString which is to be searched.  If this
    /// parameter is not a valid CFString, the behavior is
    /// undefined.
    ///
    /// Parameter `theSet`: The CFCharacterSet against which the membership
    /// of characters is checked.  If this parameter is not a valid
    /// CFCharacterSet, the behavior is undefined.
    ///
    /// Parameter `rangeToSearch`: The range of characters within the string to search. If
    /// the range location or end point (defined by the location
    /// plus length minus 1) are outside the index space of the
    /// string (0 to N-1 inclusive, where N is the length of the
    /// string), the behavior is undefined. If the range length is
    /// negative, the behavior is undefined. The range may be empty
    /// (length 0), in which case no search is performed.
    ///
    /// Parameter `searchOptions`: The bitwise-or'ed option flags to control
    /// the search behavior.  The supported options are
    /// kCFCompareBackwards andkCFCompareAnchored.
    /// If other option flags are specified, the behavior
    /// is undefined.
    ///
    /// Parameter `result`: The pointer to a CFRange supplied by the caller in
    /// which the search result is stored.  Note that the length
    /// of this range can be more than 1, if for instance the
    /// result is a composed character. If a pointer to an invalid
    /// memory is specified, the behavior is undefined.
    ///
    /// Returns: true, if at least a character which is a member of the character
    /// set is found and result is filled, otherwise, false.
    #[cfg(all(feature = "CFBase", feature = "CFCharacterSet"))]
    pub fn CFStringFindCharacterFromSet(
        the_string: CFStringRef,
        the_set: CFCharacterSetRef,
        range_to_search: CFRange,
        search_options: CFStringCompareFlags,
        result: *mut CFRange,
    ) -> Boolean;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringGetLineBounds(
        the_string: CFStringRef,
        range: CFRange,
        line_begin_index: *mut CFIndex,
        line_end_index: *mut CFIndex,
        contents_end_index: *mut CFIndex,
    );
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringGetParagraphBounds(
        string: CFStringRef,
        range: CFRange,
        par_begin_index: *mut CFIndex,
        par_end_index: *mut CFIndex,
        contents_end_index: *mut CFIndex,
    );
}

extern "C-unwind" {
    /// Retrieve the first potential hyphenation location found before the specified location.
    ///
    /// Parameter `string`: The CFString which is to be hyphenated.  If this
    /// parameter is not a valid CFString, the behavior is
    /// undefined.
    ///
    /// Parameter `location`: An index in the string.  If a valid hyphen index is returned, it
    /// will be before this index.
    ///
    /// Parameter `limitRange`: The range of characters within the string to search. If
    /// the range location or end point (defined by the location
    /// plus length minus 1) are outside the index space of the
    /// string (0 to N-1 inclusive, where N is the length of the
    /// string), the behavior is undefined. If the range length is
    /// negative, the behavior is undefined. The range may be empty
    /// (length 0), in which case no hyphen location is generated.
    ///
    /// Parameter `options`: Reserved for future use.
    ///
    /// Parameter `locale`: Specifies which language's hyphenation conventions to use.
    /// This must be a valid locale.  Hyphenation data is not available
    /// for all locales.  You can use CFStringIsHyphenationAvailableForLocale
    /// to test for availability of hyphenation data.
    ///
    /// Parameter `character`: The suggested hyphen character to insert.  Pass NULL if you
    /// do not need this information.
    ///
    /// Returns: an index in the string where it is appropriate to insert a hyphen, if
    /// one exists; else kCFNotFound
    #[cfg(all(feature = "CFBase", feature = "CFLocale"))]
    pub fn CFStringGetHyphenationLocationBeforeIndex(
        string: CFStringRef,
        location: CFIndex,
        limit_range: CFRange,
        options: CFOptionFlags,
        locale: CFLocaleRef,
        character: *mut UTF32Char,
    ) -> CFIndex;
}

extern "C-unwind" {
    #[cfg(feature = "CFLocale")]
    pub fn CFStringIsHyphenationAvailableForLocale(locale: CFLocaleRef) -> Boolean;
}

extern "C-unwind" {
    /// * Exploding and joining strings with a separator string **
    #[cfg(all(feature = "CFArray", feature = "CFBase"))]
    pub fn CFStringCreateByCombiningStrings(
        alloc: CFAllocatorRef,
        the_array: CFArrayRef,
        separator_string: CFStringRef,
    ) -> CFStringRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFArray", feature = "CFBase"))]
    pub fn CFStringCreateArrayBySeparatingStrings(
        alloc: CFAllocatorRef,
        the_string: CFStringRef,
        separator_string: CFStringRef,
    ) -> CFArrayRef;
}

extern "C-unwind" {
    /// * Parsing non-localized numbers from strings **
    #[cfg(feature = "CFBase")]
    pub fn CFStringGetIntValue(str: CFStringRef) -> i32;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringGetDoubleValue(str: CFStringRef) -> c_double;
}

extern "C-unwind" {
    /// * MutableString functions **
    #[cfg(feature = "CFBase")]
    pub fn CFStringAppend(the_string: CFMutableStringRef, appended_string: CFStringRef);
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringAppendCharacters(
        the_string: CFMutableStringRef,
        chars: *const UniChar,
        num_chars: CFIndex,
    );
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringAppendPascalString(
        the_string: CFMutableStringRef,
        p_str: ConstStr255Param,
        encoding: CFStringEncoding,
    );
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringAppendCString(
        the_string: CFMutableStringRef,
        c_str: *const c_char,
        encoding: CFStringEncoding,
    );
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringInsert(str: CFMutableStringRef, idx: CFIndex, inserted_str: CFStringRef);
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringDelete(the_string: CFMutableStringRef, range: CFRange);
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringReplace(
        the_string: CFMutableStringRef,
        range: CFRange,
        replacement: CFStringRef,
    );
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringReplaceAll(the_string: CFMutableStringRef, replacement: CFStringRef);
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringFindAndReplace(
        the_string: CFMutableStringRef,
        string_to_find: CFStringRef,
        replacement_string: CFStringRef,
        range_to_search: CFRange,
        compare_options: CFStringCompareFlags,
    ) -> CFIndex;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringSetExternalCharactersNoCopy(
        the_string: CFMutableStringRef,
        chars: *mut UniChar,
        length: CFIndex,
        capacity: CFIndex,
    );
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringPad(
        the_string: CFMutableStringRef,
        pad_string: CFStringRef,
        length: CFIndex,
        index_into_pad: CFIndex,
    );
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringTrim(the_string: CFMutableStringRef, trim_string: CFStringRef);
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringTrimWhitespace(the_string: CFMutableStringRef);
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFLocale"))]
    pub fn CFStringLowercase(the_string: CFMutableStringRef, locale: CFLocaleRef);
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFLocale"))]
    pub fn CFStringUppercase(the_string: CFMutableStringRef, locale: CFLocaleRef);
}

extern "C-unwind" {
    #[cfg(all(feature = "CFBase", feature = "CFLocale"))]
    pub fn CFStringCapitalize(the_string: CFMutableStringRef, locale: CFLocaleRef);
}

/// This is the type of Unicode normalization forms as described in
/// Unicode Technical Report #15. To normalize for use with file
/// system calls, use CFStringGetFileSystemRepresentation().
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfstringnormalizationform?language=objc)
// NS_ENUM
#[cfg(feature = "CFBase")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CFStringNormalizationForm(pub CFIndex);
#[cfg(feature = "CFBase")]
impl CFStringNormalizationForm {
    pub const kCFStringNormalizationFormD: Self = Self(0);
    pub const kCFStringNormalizationFormKD: Self = Self(1);
    pub const kCFStringNormalizationFormC: Self = Self(2);
    pub const kCFStringNormalizationFormKC: Self = Self(3);
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl Encode for CFStringNormalizationForm {
    const ENCODING: Encoding = CFIndex::ENCODING;
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl RefEncode for CFStringNormalizationForm {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    /// Normalizes the string into the specified form as described in
    /// Unicode Technical Report #15.
    ///
    /// Parameter `theString`: The string which is to be normalized.  If this
    /// parameter is not a valid mutable CFString, the behavior is
    /// undefined.
    ///
    /// Parameter `theForm`: The form into which the string is to be normalized.
    /// If this parameter is not a valid CFStringNormalizationForm value,
    /// the behavior is undefined.
    #[cfg(feature = "CFBase")]
    pub fn CFStringNormalize(the_string: CFMutableStringRef, the_form: CFStringNormalizationForm);
}

extern "C-unwind" {
    /// Folds the string into the form specified by the flags.
    /// Character foldings are operations that convert any of a set of characters
    /// sharing similar semantics into a single representative from that set.
    /// This function can be used to preprocess strings that are to be compared,
    /// searched, or indexed.
    /// Note that folding does not include normalization, so it is necessary
    /// to use CFStringNormalize in addition to CFStringFold in order to obtain
    /// the effect of kCFCompareNonliteral.
    ///
    /// Parameter `theString`: The string which is to be folded.  If this parameter is not
    /// a valid mutable CFString, the behavior is undefined.
    ///
    /// Parameter `theFlags`: The equivalency flags which describes the character folding form.
    /// Only those flags containing the word "insensitive" are recognized here; other flags are ignored.
    /// Folding with kCFCompareCaseInsensitive removes case distinctions in accordance with the mapping
    /// specified by ftp://ftp.unicode.org/Public/UNIDATA/CaseFolding.txt.  Folding with
    /// kCFCompareDiacriticInsensitive removes distinctions of accents and other diacritics.  Folding
    /// with kCFCompareWidthInsensitive removes character width distinctions by mapping characters in
    /// the range U+FF00-U+FFEF to their ordinary equivalents.
    ///
    /// Parameter `theLocale`: The locale tailoring the character folding behavior. If NULL,
    /// it's considered to be the system locale returned from CFLocaleGetSystem().
    /// If non-NULL and not a valid CFLocale object, the behavior is undefined.
    #[cfg(all(feature = "CFBase", feature = "CFLocale"))]
    pub fn CFStringFold(
        the_string: CFMutableStringRef,
        the_flags: CFStringCompareFlags,
        the_locale: CFLocaleRef,
    );
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringTransform(
        string: CFMutableStringRef,
        range: *mut CFRange,
        transform: CFStringRef,
        reverse: Boolean,
    ) -> Boolean;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfstringtransformstripcombiningmarks?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFStringTransformStripCombiningMarks: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfstringtransformtolatin?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFStringTransformToLatin: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfstringtransformfullwidthhalfwidth?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFStringTransformFullwidthHalfwidth: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfstringtransformlatinkatakana?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFStringTransformLatinKatakana: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfstringtransformlatinhiragana?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFStringTransformLatinHiragana: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfstringtransformhiraganakatakana?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFStringTransformHiraganaKatakana: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfstringtransformmandarinlatin?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFStringTransformMandarinLatin: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfstringtransformlatinhangul?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFStringTransformLatinHangul: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfstringtransformlatinarabic?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFStringTransformLatinArabic: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfstringtransformlatinhebrew?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFStringTransformLatinHebrew: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfstringtransformlatinthai?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFStringTransformLatinThai: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfstringtransformlatincyrillic?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFStringTransformLatinCyrillic: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfstringtransformlatingreek?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFStringTransformLatinGreek: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfstringtransformtoxmlhex?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFStringTransformToXMLHex: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfstringtransformtounicodename?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFStringTransformToUnicodeName: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfstringtransformstripdiacritics?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFStringTransformStripDiacritics: CFStringRef;
}

extern "C-unwind" {
    /// * General encoding related functionality **
    pub fn CFStringIsEncodingAvailable(encoding: CFStringEncoding) -> Boolean;
}

extern "C-unwind" {
    pub fn CFStringGetListOfAvailableEncodings() -> *const CFStringEncoding;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringGetNameOfEncoding(encoding: CFStringEncoding) -> CFStringRef;
}

extern "C-unwind" {
    pub fn CFStringConvertEncodingToNSStringEncoding(encoding: CFStringEncoding) -> c_ulong;
}

extern "C-unwind" {
    pub fn CFStringConvertNSStringEncodingToEncoding(encoding: c_ulong) -> CFStringEncoding;
}

extern "C-unwind" {
    pub fn CFStringConvertEncodingToWindowsCodepage(encoding: CFStringEncoding) -> u32;
}

extern "C-unwind" {
    pub fn CFStringConvertWindowsCodepageToEncoding(codepage: u32) -> CFStringEncoding;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringConvertIANACharSetNameToEncoding(the_string: CFStringRef) -> CFStringEncoding;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFStringConvertEncodingToIANACharSetName(encoding: CFStringEncoding) -> CFStringRef;
}

extern "C-unwind" {
    pub fn CFStringGetMostCompatibleMacStringEncoding(
        encoding: CFStringEncoding,
    ) -> CFStringEncoding;
}

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfstringinlinebuffer?language=objc)
#[cfg(feature = "CFBase")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CFStringInlineBuffer {
    pub buffer: [UniChar; 64],
    pub theString: CFStringRef,
    pub directUniCharBuffer: *const UniChar,
    pub directCStringBuffer: *const c_char,
    pub rangeToBuffer: CFRange,
    pub bufferedRangeStart: CFIndex,
    pub bufferedRangeEnd: CFIndex,
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl Encode for CFStringInlineBuffer {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <[UniChar; 64]>::ENCODING,
            <CFStringRef>::ENCODING,
            <*const UniChar>::ENCODING,
            <*const c_char>::ENCODING,
            <CFRange>::ENCODING,
            <CFIndex>::ENCODING,
            <CFIndex>::ENCODING,
        ],
    );
}

#[cfg(all(feature = "CFBase", feature = "objc2"))]
unsafe impl RefEncode for CFStringInlineBuffer {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// TODO: pub fn CFStringInitInlineBuffer(str: CFStringRef,buf: *mut CFStringInlineBuffer,range: CFRange,);

// TODO: pub fn CFStringGetCharacterFromInlineBuffer(buf: *mut CFStringInlineBuffer,idx: CFIndex,) -> UniChar;

// TODO: pub fn CFStringIsSurrogateHighCharacter(character: UniChar,) -> Boolean;

// TODO: pub fn CFStringIsSurrogateLowCharacter(character: UniChar,) -> Boolean;

// TODO: pub fn CFStringGetLongCharacterForSurrogatePair(surrogate_high: UniChar,surrogate_low: UniChar,) -> UTF32Char;

// TODO: pub fn CFStringGetSurrogatePairForLongCharacter(character: UTF32Char,surrogates: *mut UniChar,) -> Boolean;

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFShow(obj: CFTypeRef);
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFShowStr(str: CFStringRef);
}
