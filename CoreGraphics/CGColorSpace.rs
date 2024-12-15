//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgcolorspaceref?language=objc)
pub type CGColorSpaceRef = *mut c_void;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgcolorrenderingintent?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CGColorRenderingIntent(pub i32);
impl CGColorRenderingIntent {
    pub const kCGRenderingIntentDefault: Self = Self(0);
    pub const kCGRenderingIntentAbsoluteColorimetric: Self = Self(1);
    pub const kCGRenderingIntentRelativeColorimetric: Self = Self(2);
    pub const kCGRenderingIntentPerceptual: Self = Self(3);
    pub const kCGRenderingIntentSaturation: Self = Self(4);
}

unsafe impl Encode for CGColorRenderingIntent {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for CGColorRenderingIntent {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgcolorspacemodel?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CGColorSpaceModel(pub i32);
impl CGColorSpaceModel {
    pub const kCGColorSpaceModelUnknown: Self = Self(-1);
    pub const kCGColorSpaceModelMonochrome: Self = Self(0);
    pub const kCGColorSpaceModelRGB: Self = Self(1);
    pub const kCGColorSpaceModelCMYK: Self = Self(2);
    pub const kCGColorSpaceModelLab: Self = Self(3);
    pub const kCGColorSpaceModelDeviceN: Self = Self(4);
    pub const kCGColorSpaceModelIndexed: Self = Self(5);
    pub const kCGColorSpaceModelPattern: Self = Self(6);
    pub const kCGColorSpaceModelXYZ: Self = Self(7);
}

unsafe impl Encode for CGColorSpaceModel {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for CGColorSpaceModel {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspacegenericgray?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceGenericGray: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspacegenericrgb?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceGenericRGB: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspacegenericcmyk?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceGenericCMYK: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspacedisplayp3?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceDisplayP3: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspacegenericrgblinear?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceGenericRGBLinear: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspaceadobergb1998?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceAdobeRGB1998: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspacesrgb?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceSRGB: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspacegenericgraygamma2_2?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceGenericGrayGamma2_2: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspacegenericxyz?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceGenericXYZ: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspacegenericlab?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceGenericLab: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspaceacescglinear?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceACESCGLinear: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspaceitur_709?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceITUR_709: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspaceitur_709_pq?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceITUR_709_PQ: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspaceitur_709_hlg?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceITUR_709_HLG: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspaceitur_2020?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceITUR_2020: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspaceitur_2020_srgbgamma?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceITUR_2020_sRGBGamma: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspacerommrgb?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceROMMRGB: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspacedcip3?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceDCIP3: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspacelinearitur_2020?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceLinearITUR_2020: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspaceextendeditur_2020?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceExtendedITUR_2020: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspaceextendedlinearitur_2020?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceExtendedLinearITUR_2020: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspacelineardisplayp3?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceLinearDisplayP3: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspaceextendeddisplayp3?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceExtendedDisplayP3: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspaceextendedlineardisplayp3?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceExtendedLinearDisplayP3: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspaceitur_2100_pq?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceITUR_2100_PQ: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspaceitur_2100_hlg?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceITUR_2100_HLG: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspacedisplayp3_pq?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceDisplayP3_PQ: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspacedisplayp3_hlg?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceDisplayP3_HLG: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspaceitur_2020_pq?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceITUR_2020_PQ: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspaceitur_2020_hlg?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceITUR_2020_HLG: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspacedisplayp3_pq_eotf?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceDisplayP3_PQ_EOTF: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspaceitur_2020_pq_eotf?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceITUR_2020_PQ_EOTF: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspaceextendedsrgb?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceExtendedSRGB: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspacelinearsrgb?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceLinearSRGB: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspaceextendedlinearsrgb?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceExtendedLinearSRGB: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspaceextendedgray?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceExtendedGray: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspacelineargray?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceLinearGray: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspaceextendedlineargray?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceExtendedLinearGray: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspacecoremedia709?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceCoreMedia709: CFStringRef;
}

extern "C-unwind" {
    pub fn CGColorSpaceCreateDeviceGray() -> CGColorSpaceRef;
}

extern "C-unwind" {
    pub fn CGColorSpaceCreateDeviceRGB() -> CGColorSpaceRef;
}

extern "C-unwind" {
    pub fn CGColorSpaceCreateDeviceCMYK() -> CGColorSpaceRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGColorSpaceCreateWithICCData(data: CFTypeRef) -> CGColorSpaceRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGDataProvider", feature = "objc2-core-foundation"))]
    pub fn CGColorSpaceCreateICCBased(
        n_components: usize,
        range: *mut CGFloat,
        profile: CGDataProviderRef,
        alternate: CGColorSpaceRef,
    ) -> CGColorSpaceRef;
}

extern "C-unwind" {
    pub fn CGColorSpaceCreateIndexed(
        base_space: CGColorSpaceRef,
        last_index: usize,
        color_table: *mut c_uchar,
    ) -> CGColorSpaceRef;
}

extern "C-unwind" {
    pub fn CGColorSpaceCreatePattern(base_space: CGColorSpaceRef) -> CGColorSpaceRef;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/colorsyncprofileref?language=objc)
pub type ColorSyncProfileRef = *mut c_void;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/kcgcolorspaceextendedrange?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCGColorSpaceExtendedRange: CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGColorSpaceCreateWithColorSyncProfile(
        _: ColorSyncProfileRef,
        options: CFDictionaryRef,
    ) -> CGColorSpaceRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGColorSpaceCreateWithName(name: CFStringRef) -> CGColorSpaceRef;
}

extern "C-unwind" {
    pub fn CGColorSpaceRetain(space: CGColorSpaceRef) -> CGColorSpaceRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGColorSpaceGetName(space: CGColorSpaceRef) -> CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGColorSpaceCopyName(space: CGColorSpaceRef) -> CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGColorSpaceGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    pub fn CGColorSpaceGetNumberOfComponents(space: CGColorSpaceRef) -> usize;
}

extern "C-unwind" {
    pub fn CGColorSpaceGetModel(space: CGColorSpaceRef) -> CGColorSpaceModel;
}

extern "C-unwind" {
    pub fn CGColorSpaceGetBaseColorSpace(space: CGColorSpaceRef) -> CGColorSpaceRef;
}

extern "C-unwind" {
    pub fn CGColorSpaceCopyBaseColorSpace(space: CGColorSpaceRef) -> CGColorSpaceRef;
}

extern "C-unwind" {
    pub fn CGColorSpaceGetColorTableCount(space: CGColorSpaceRef) -> usize;
}

extern "C-unwind" {
    pub fn CGColorSpaceGetColorTable(space: CGColorSpaceRef, table: *mut u8);
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGColorSpaceCopyICCData(space: CGColorSpaceRef) -> CFDataRef;
}

extern "C-unwind" {
    pub fn CGColorSpaceIsWideGamutRGB(_: CGColorSpaceRef) -> bool;
}

extern "C-unwind" {
    pub fn CGColorSpaceIsHDR(_: CGColorSpaceRef) -> bool;
}

extern "C-unwind" {
    pub fn CGColorSpaceUsesITUR_2100TF(_: CGColorSpaceRef) -> bool;
}

extern "C-unwind" {
    pub fn CGColorSpaceIsPQBased(s: CGColorSpaceRef) -> bool;
}

extern "C-unwind" {
    pub fn CGColorSpaceIsHLGBased(s: CGColorSpaceRef) -> bool;
}

extern "C-unwind" {
    pub fn CGColorSpaceSupportsOutput(space: CGColorSpaceRef) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGColorSpaceCopyPropertyList(space: CGColorSpaceRef) -> CFPropertyListRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGColorSpaceCreateWithPropertyList(plist: CFPropertyListRef) -> CGColorSpaceRef;
}

extern "C-unwind" {
    pub fn CGColorSpaceUsesExtendedRange(space: CGColorSpaceRef) -> bool;
}

extern "C-unwind" {
    pub fn CGColorSpaceCreateLinearized(space: CGColorSpaceRef) -> CGColorSpaceRef;
}

extern "C-unwind" {
    pub fn CGColorSpaceCreateExtended(space: CGColorSpaceRef) -> CGColorSpaceRef;
}

extern "C-unwind" {
    pub fn CGColorSpaceCreateExtendedLinearized(space: CGColorSpaceRef) -> CGColorSpaceRef;
}

extern "C-unwind" {
    pub fn CGColorSpaceCreateCopyWithStandardRange(s: CGColorSpaceRef) -> CGColorSpaceRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated = "No longer supported"]
    pub fn CGColorSpaceCreateWithICCProfile(data: CFDataRef) -> CGColorSpaceRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated = "No longer supported"]
    pub fn CGColorSpaceCopyICCProfile(space: CGColorSpaceRef) -> CFDataRef;
}

extern "C-unwind" {
    #[deprecated = "No longer supported"]
    pub fn CGColorSpaceCreateWithPlatformColorSpace(r#ref: *mut c_void) -> CGColorSpaceRef;
}