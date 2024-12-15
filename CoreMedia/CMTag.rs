//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmtagerror?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CMTagError(pub OSStatus);
impl CMTagError {
    pub const kCMTagError_ParamErr: Self = Self(-15730);
    pub const kCMTagError_AllocationFailed: Self = Self(-15731);
}

unsafe impl Encode for CMTagError {
    const ENCODING: Encoding = OSStatus::ENCODING;
}

unsafe impl RefEncode for CMTagError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmtagcategory?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CMTagCategory(pub FourCharCode);
impl CMTagCategory {
    pub const kCMTagCategory_Undefined: Self = Self(0);
    pub const kCMTagCategory_MediaType: Self = Self(0x6d646961);
    pub const kCMTagCategory_MediaSubType: Self = Self(0x6d737562);
    pub const kCMTagCategory_TrackID: Self = Self(0x7472616b);
    pub const kCMTagCategory_ChannelID: Self = Self(0x7663686e);
    pub const kCMTagCategory_VideoLayerID: Self = Self(0x766c6179);
    pub const kCMTagCategory_PixelFormat: Self = Self(0x70697866);
    pub const kCMTagCategory_PackingType: Self = Self(0x7061636b);
    pub const kCMTagCategory_ProjectionType: Self = Self(0x70726f6a);
    pub const kCMTagCategory_StereoView: Self = Self(0x65796573);
    pub const kCMTagCategory_StereoViewInterpretation: Self = Self(0x65796970);
}

unsafe impl Encode for CMTagCategory {
    const ENCODING: Encoding = FourCharCode::ENCODING;
}

unsafe impl RefEncode for CMTagCategory {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmtagdatatype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CMTagDataType(pub u32);
impl CMTagDataType {
    pub const kCMTagDataType_Invalid: Self = Self(0);
    pub const kCMTagDataType_SInt64: Self = Self(2);
    pub const kCMTagDataType_Float64: Self = Self(3);
    pub const kCMTagDataType_OSType: Self = Self(5);
    pub const kCMTagDataType_Flags: Self = Self(7);
}

unsafe impl Encode for CMTagDataType {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for CMTagDataType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmtagvalue?language=objc)
pub type CMTagValue = u64;

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmtag?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CMTag {
    pub category: CMTagCategory,
    pub dataType: CMTagDataType,
    pub value: CMTagValue,
}

unsafe impl Encode for CMTag {
    const ENCODING: Encoding = Encoding::Struct(
        "CMTag",
        &[
            <CMTagCategory>::ENCODING,
            <CMTagDataType>::ENCODING,
            <CMTagValue>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for CMTag {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// TODO: pub fn CMTagIsValid(tag: CMTag,) -> Boolean;

extern "C-unwind" {
    pub fn CMTagGetValueDataType(tag: CMTag) -> CMTagDataType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtaginvalid?language=objc)
    pub static kCMTagInvalid: CMTag;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtagmediatypevideo?language=objc)
    pub static kCMTagMediaTypeVideo: CMTag;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtagmediasubtypemebx?language=objc)
    pub static kCMTagMediaSubTypeMebx: CMTag;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtagmediatypeaudio?language=objc)
    pub static kCMTagMediaTypeAudio: CMTag;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtagmediatypemetadata?language=objc)
    pub static kCMTagMediaTypeMetadata: CMTag;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmstereoviewcomponents?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CMStereoViewComponents(pub u64);
bitflags::bitflags! {
    impl CMStereoViewComponents: u64 {
        const kCMStereoView_None = 0;
        const kCMStereoView_LeftEye = 1<<0;
        const kCMStereoView_RightEye = 1<<1;
    }
}

unsafe impl Encode for CMStereoViewComponents {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for CMStereoViewComponents {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtagstereolefteye?language=objc)
    pub static kCMTagStereoLeftEye: CMTag;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtagstereorighteye?language=objc)
    pub static kCMTagStereoRightEye: CMTag;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtagstereoleftandrighteye?language=objc)
    pub static kCMTagStereoLeftAndRightEye: CMTag;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtagstereonone?language=objc)
    pub static kCMTagStereoNone: CMTag;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmstereoviewinterpretationoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CMStereoViewInterpretationOptions(pub u64);
bitflags::bitflags! {
    impl CMStereoViewInterpretationOptions: u64 {
        const kCMStereoViewInterpretation_Default = 0;
        const kCMStereoViewInterpretation_StereoOrderReversed = 1<<0;
        const kCMStereoViewInterpretation_AdditionalViews = 1<<1;
    }
}

unsafe impl Encode for CMStereoViewInterpretationOptions {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for CMStereoViewInterpretationOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtagstereointerpretationorderreversed?language=objc)
    pub static kCMTagStereoInterpretationOrderReversed: CMTag;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmprojectiontype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CMProjectionType(pub u64);
impl CMProjectionType {
    pub const kCMProjectionType_Rectangular: Self = Self(0x72656374);
    pub const kCMProjectionType_Equirectangular: Self = Self(0x65717569);
    pub const kCMProjectionType_HalfEquirectangular: Self = Self(0x68657175);
    pub const kCMProjectionType_Fisheye: Self = Self(0x66697368);
}

unsafe impl Encode for CMProjectionType {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for CMProjectionType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtagprojectiontyperectangular?language=objc)
    pub static kCMTagProjectionTypeRectangular: CMTag;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtagprojectiontypeequirectangular?language=objc)
    pub static kCMTagProjectionTypeEquirectangular: CMTag;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtagprojectiontypehalfequirectangular?language=objc)
    pub static kCMTagProjectionTypeHalfEquirectangular: CMTag;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtagprojectiontypefisheye?language=objc)
    pub static kCMTagProjectionTypeFisheye: CMTag;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coremedia/cmpackingtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CMPackingType(pub u64);
impl CMPackingType {
    pub const kCMPackingType_None: Self = Self(0x6e6f6e65);
    pub const kCMPackingType_SideBySide: Self = Self(0x73696465);
    pub const kCMPackingType_OverUnder: Self = Self(0x6f766572);
}

unsafe impl Encode for CMPackingType {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for CMPackingType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtagpackingtypenone?language=objc)
    pub static kCMTagPackingTypeNone: CMTag;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtagpackingtypesidebyside?language=objc)
    pub static kCMTagPackingTypeSideBySide: CMTag;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtagpackingtypeoverunder?language=objc)
    pub static kCMTagPackingTypeOverUnder: CMTag;
}

// TODO: pub fn CMTagGetCategory(tag: CMTag,) -> CMTagCategory;

// TODO: pub fn CMTagCategoryEqualToTagCategory(tag1: CMTag,tag2: CMTag,) -> Boolean;

// TODO: pub fn CMTagGetValue(tag: CMTag,) -> CMTagValue;

// TODO: pub fn CMTagHasCategory(tag: CMTag,category: CMTagCategory,) -> Boolean;

extern "C-unwind" {
    pub fn CMTagHasSInt64Value(tag: CMTag) -> Boolean;
}

extern "C-unwind" {
    pub fn CMTagGetSInt64Value(tag: CMTag) -> i64;
}

extern "C-unwind" {
    pub fn CMTagHasFloat64Value(tag: CMTag) -> Boolean;
}

extern "C-unwind" {
    pub fn CMTagGetFloat64Value(tag: CMTag) -> f64;
}

extern "C-unwind" {
    pub fn CMTagHasOSTypeValue(tag: CMTag) -> Boolean;
}

extern "C-unwind" {
    pub fn CMTagGetOSTypeValue(tag: CMTag) -> OSType;
}

extern "C-unwind" {
    pub fn CMTagHasFlagsValue(tag: CMTag) -> Boolean;
}

extern "C-unwind" {
    pub fn CMTagGetFlagsValue(tag: CMTag) -> u64;
}

extern "C-unwind" {
    pub fn CMTagMakeWithSInt64Value(category: CMTagCategory, value: i64) -> CMTag;
}

extern "C-unwind" {
    pub fn CMTagMakeWithFloat64Value(category: CMTagCategory, value: f64) -> CMTag;
}

extern "C-unwind" {
    pub fn CMTagMakeWithOSTypeValue(category: CMTagCategory, value: OSType) -> CMTag;
}

extern "C-unwind" {
    pub fn CMTagMakeWithFlagsValue(category: CMTagCategory, flags_for_tag: u64) -> CMTag;
}

extern "C-unwind" {
    pub fn CMTagEqualToTag(tag1: CMTag, tag2: CMTag) -> Boolean;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CMTagCompare(tag1: CMTag, tag2: CMTag) -> CFComparisonResult;
}

// TODO: pub fn CMTagCategoryValueEqualToValue(tag1: CMTag,tag2: CMTag,) -> Boolean;

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CMTagHash(tag: CMTag) -> CFHashCode;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CMTagCopyDescription(allocator: CFAllocatorRef, tag: CMTag) -> CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CMTagCopyAsDictionary(tag: CMTag, allocator: CFAllocatorRef) -> CFDictionaryRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CMTagMakeFromDictionary(dict: CFDictionaryRef) -> CMTag;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtagvaluekey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMTagValueKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtagcategorykey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMTagCategoryKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coremedia/kcmtagdatatypekey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMTagDataTypeKey: CFStringRef;
}

// TODO: pub fn CMTagGetCategory(tag: CMTag,) -> CMTagCategory;

// TODO: pub fn CMTagGetValue(tag: CMTag,) -> CMTagValue;

// TODO: pub fn CMTagHasCategory(tag: CMTag,category: CMTagCategory,) -> Boolean;

// TODO: pub fn CMTagCategoryEqualToTagCategory(tag1: CMTag,tag2: CMTag,) -> Boolean;

// TODO: pub fn CMTagIsValid(tag: CMTag,) -> Boolean;

// TODO: pub fn CMTagCategoryValueEqualToValue(tag1: CMTag,tag2: CMTag,) -> Boolean;