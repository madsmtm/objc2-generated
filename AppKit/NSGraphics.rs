//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositingoperation?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSCompositingOperation(pub NSUInteger);
impl NSCompositingOperation {
    #[doc(alias = "NSCompositingOperationClear")]
    pub const Clear: Self = Self(0);
    #[doc(alias = "NSCompositingOperationCopy")]
    pub const Copy: Self = Self(1);
    #[doc(alias = "NSCompositingOperationSourceOver")]
    pub const SourceOver: Self = Self(2);
    #[doc(alias = "NSCompositingOperationSourceIn")]
    pub const SourceIn: Self = Self(3);
    #[doc(alias = "NSCompositingOperationSourceOut")]
    pub const SourceOut: Self = Self(4);
    #[doc(alias = "NSCompositingOperationSourceAtop")]
    pub const SourceAtop: Self = Self(5);
    #[doc(alias = "NSCompositingOperationDestinationOver")]
    pub const DestinationOver: Self = Self(6);
    #[doc(alias = "NSCompositingOperationDestinationIn")]
    pub const DestinationIn: Self = Self(7);
    #[doc(alias = "NSCompositingOperationDestinationOut")]
    pub const DestinationOut: Self = Self(8);
    #[doc(alias = "NSCompositingOperationDestinationAtop")]
    pub const DestinationAtop: Self = Self(9);
    #[doc(alias = "NSCompositingOperationXOR")]
    pub const XOR: Self = Self(10);
    #[doc(alias = "NSCompositingOperationPlusDarker")]
    pub const PlusDarker: Self = Self(11);
    #[doc(alias = "NSCompositingOperationHighlight")]
    #[deprecated = "Use NSCompositingOperationSourceOver instead"]
    pub const Highlight: Self = Self(12);
    #[doc(alias = "NSCompositingOperationPlusLighter")]
    pub const PlusLighter: Self = Self(13);
    #[doc(alias = "NSCompositingOperationMultiply")]
    pub const Multiply: Self = Self(14);
    #[doc(alias = "NSCompositingOperationScreen")]
    pub const Screen: Self = Self(15);
    #[doc(alias = "NSCompositingOperationOverlay")]
    pub const Overlay: Self = Self(16);
    #[doc(alias = "NSCompositingOperationDarken")]
    pub const Darken: Self = Self(17);
    #[doc(alias = "NSCompositingOperationLighten")]
    pub const Lighten: Self = Self(18);
    #[doc(alias = "NSCompositingOperationColorDodge")]
    pub const ColorDodge: Self = Self(19);
    #[doc(alias = "NSCompositingOperationColorBurn")]
    pub const ColorBurn: Self = Self(20);
    #[doc(alias = "NSCompositingOperationSoftLight")]
    pub const SoftLight: Self = Self(21);
    #[doc(alias = "NSCompositingOperationHardLight")]
    pub const HardLight: Self = Self(22);
    #[doc(alias = "NSCompositingOperationDifference")]
    pub const Difference: Self = Self(23);
    #[doc(alias = "NSCompositingOperationExclusion")]
    pub const Exclusion: Self = Self(24);
    #[doc(alias = "NSCompositingOperationHue")]
    pub const Hue: Self = Self(25);
    #[doc(alias = "NSCompositingOperationSaturation")]
    pub const Saturation: Self = Self(26);
    #[doc(alias = "NSCompositingOperationColor")]
    pub const Color: Self = Self(27);
    #[doc(alias = "NSCompositingOperationLuminosity")]
    pub const Luminosity: Self = Self(28);
}

unsafe impl Encode for NSCompositingOperation {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSCompositingOperation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositeclear?language=objc)
pub static NSCompositeClear: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Clear.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositecopy?language=objc)
pub static NSCompositeCopy: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Copy.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositesourceover?language=objc)
pub static NSCompositeSourceOver: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::SourceOver.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositesourcein?language=objc)
pub static NSCompositeSourceIn: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::SourceIn.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositesourceout?language=objc)
pub static NSCompositeSourceOut: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::SourceOut.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositesourceatop?language=objc)
pub static NSCompositeSourceAtop: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::SourceAtop.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositedestinationover?language=objc)
pub static NSCompositeDestinationOver: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::DestinationOver.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositedestinationin?language=objc)
pub static NSCompositeDestinationIn: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::DestinationIn.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositedestinationout?language=objc)
pub static NSCompositeDestinationOut: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::DestinationOut.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositedestinationatop?language=objc)
pub static NSCompositeDestinationAtop: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::DestinationAtop.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositexor?language=objc)
pub static NSCompositeXOR: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::XOR.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositeplusdarker?language=objc)
pub static NSCompositePlusDarker: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::PlusDarker.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositehighlight?language=objc)
pub static NSCompositeHighlight: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Highlight.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositepluslighter?language=objc)
pub static NSCompositePlusLighter: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::PlusLighter.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositemultiply?language=objc)
pub static NSCompositeMultiply: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Multiply.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositescreen?language=objc)
pub static NSCompositeScreen: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Screen.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositeoverlay?language=objc)
pub static NSCompositeOverlay: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Overlay.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositedarken?language=objc)
pub static NSCompositeDarken: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Darken.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositelighten?language=objc)
pub static NSCompositeLighten: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Lighten.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositecolordodge?language=objc)
pub static NSCompositeColorDodge: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::ColorDodge.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositecolorburn?language=objc)
pub static NSCompositeColorBurn: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::ColorBurn.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositesoftlight?language=objc)
pub static NSCompositeSoftLight: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::SoftLight.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositehardlight?language=objc)
pub static NSCompositeHardLight: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::HardLight.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositedifference?language=objc)
pub static NSCompositeDifference: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Difference.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositeexclusion?language=objc)
pub static NSCompositeExclusion: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Exclusion.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositehue?language=objc)
pub static NSCompositeHue: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Hue.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositesaturation?language=objc)
pub static NSCompositeSaturation: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Saturation.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositecolor?language=objc)
pub static NSCompositeColor: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Color.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscompositeluminosity?language=objc)
pub static NSCompositeLuminosity: NSCompositingOperation =
    NSCompositingOperation(NSCompositingOperation::Luminosity.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsbackingstoretype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSBackingStoreType(pub NSUInteger);
impl NSBackingStoreType {
    #[doc(alias = "NSBackingStoreRetained")]
    #[deprecated]
    pub const Retained: Self = Self(0);
    #[doc(alias = "NSBackingStoreNonretained")]
    #[deprecated]
    pub const Nonretained: Self = Self(1);
    #[doc(alias = "NSBackingStoreBuffered")]
    pub const Buffered: Self = Self(2);
}

unsafe impl Encode for NSBackingStoreType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSBackingStoreType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nswindoworderingmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSWindowOrderingMode(pub NSInteger);
impl NSWindowOrderingMode {
    #[doc(alias = "NSWindowAbove")]
    pub const Above: Self = Self(1);
    #[doc(alias = "NSWindowBelow")]
    pub const Below: Self = Self(-1);
    #[doc(alias = "NSWindowOut")]
    pub const Out: Self = Self(0);
}

unsafe impl Encode for NSWindowOrderingMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSWindowOrderingMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfocusringplacement?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFocusRingPlacement(pub NSUInteger);
impl NSFocusRingPlacement {
    #[doc(alias = "NSFocusRingOnly")]
    pub const Only: Self = Self(0);
    #[doc(alias = "NSFocusRingBelow")]
    pub const Below: Self = Self(1);
    #[doc(alias = "NSFocusRingAbove")]
    pub const Above: Self = Self(2);
}

unsafe impl Encode for NSFocusRingPlacement {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSFocusRingPlacement {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfocusringtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFocusRingType(pub NSUInteger);
impl NSFocusRingType {
    #[doc(alias = "NSFocusRingTypeDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "NSFocusRingTypeNone")]
    pub const None: Self = Self(1);
    #[doc(alias = "NSFocusRingTypeExterior")]
    pub const Exterior: Self = Self(2);
}

unsafe impl Encode for NSFocusRingType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSFocusRingType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscolorrenderingintent?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSColorRenderingIntent(pub NSInteger);
impl NSColorRenderingIntent {
    #[doc(alias = "NSColorRenderingIntentDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "NSColorRenderingIntentAbsoluteColorimetric")]
    pub const AbsoluteColorimetric: Self = Self(1);
    #[doc(alias = "NSColorRenderingIntentRelativeColorimetric")]
    pub const RelativeColorimetric: Self = Self(2);
    #[doc(alias = "NSColorRenderingIntentPerceptual")]
    pub const Perceptual: Self = Self(3);
    #[doc(alias = "NSColorRenderingIntentSaturation")]
    pub const Saturation: Self = Self(4);
}

unsafe impl Encode for NSColorRenderingIntent {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSColorRenderingIntent {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscolorspacename?language=objc)
// NS_TYPED_ENUM
pub type NSColorSpaceName = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscalibratedwhitecolorspace?language=objc)
    pub static NSCalibratedWhiteColorSpace: &'static NSColorSpaceName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscalibratedrgbcolorspace?language=objc)
    pub static NSCalibratedRGBColorSpace: &'static NSColorSpaceName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdevicewhitecolorspace?language=objc)
    pub static NSDeviceWhiteColorSpace: &'static NSColorSpaceName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdevicergbcolorspace?language=objc)
    pub static NSDeviceRGBColorSpace: &'static NSColorSpaceName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdevicecmykcolorspace?language=objc)
    pub static NSDeviceCMYKColorSpace: &'static NSColorSpaceName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsnamedcolorspace?language=objc)
    pub static NSNamedColorSpace: &'static NSColorSpaceName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspatterncolorspace?language=objc)
    pub static NSPatternColorSpace: &'static NSColorSpaceName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscustomcolorspace?language=objc)
    pub static NSCustomColorSpace: &'static NSColorSpaceName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscalibratedblackcolorspace?language=objc)
    pub static NSCalibratedBlackColorSpace: &'static NSColorSpaceName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdeviceblackcolorspace?language=objc)
    pub static NSDeviceBlackColorSpace: &'static NSColorSpaceName;
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nswindowdepth?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSWindowDepth(pub i32);
impl NSWindowDepth {
    #[doc(alias = "NSWindowDepthTwentyfourBitRGB")]
    pub const TwentyfourBitRGB: Self = Self(0x208);
    #[doc(alias = "NSWindowDepthSixtyfourBitRGB")]
    pub const SixtyfourBitRGB: Self = Self(0x210);
    #[doc(alias = "NSWindowDepthOnehundredtwentyeightBitRGB")]
    pub const OnehundredtwentyeightBitRGB: Self = Self(0x220);
}

unsafe impl Encode for NSWindowDepth {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for NSWindowDepth {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[inline]
pub unsafe extern "C-unwind" fn NSBestDepth(
    color_space: &NSColorSpaceName,
    bps: NSInteger,
    bpp: NSInteger,
    planar: bool,
    exact_match: *mut Bool,
) -> NSWindowDepth {
    extern "C-unwind" {
        fn NSBestDepth(
            color_space: &NSColorSpaceName,
            bps: NSInteger,
            bpp: NSInteger,
            planar: Bool,
            exact_match: *mut Bool,
        ) -> NSWindowDepth;
    }
    unsafe { NSBestDepth(color_space, bps, bpp, Bool::new(planar), exact_match) }
}

#[inline]
pub unsafe extern "C-unwind" fn NSPlanarFromDepth(depth: NSWindowDepth) -> bool {
    extern "C-unwind" {
        fn NSPlanarFromDepth(depth: NSWindowDepth) -> Bool;
    }
    unsafe { NSPlanarFromDepth(depth) }.as_bool()
}

#[inline]
pub unsafe extern "C-unwind" fn NSColorSpaceFromDepth(
    depth: NSWindowDepth,
) -> Option<Retained<NSColorSpaceName>> {
    extern "C-unwind" {
        fn NSColorSpaceFromDepth(depth: NSWindowDepth) -> *mut NSColorSpaceName;
    }
    let ret = unsafe { NSColorSpaceFromDepth(depth) };
    unsafe { Retained::retain_autoreleased(ret) }
}

extern "C-unwind" {
    pub fn NSBitsPerSampleFromDepth(depth: NSWindowDepth) -> NSInteger;
}

extern "C-unwind" {
    pub fn NSBitsPerPixelFromDepth(depth: NSWindowDepth) -> NSInteger;
}

extern "C-unwind" {
    pub fn NSNumberOfColorComponents(color_space_name: &NSColorSpaceName) -> NSInteger;
}

#[inline]
pub unsafe extern "C-unwind" fn NSAvailableWindowDepths() -> NonNull<NSWindowDepth> {
    extern "C-unwind" {
        fn NSAvailableWindowDepths() -> Option<NonNull<NSWindowDepth>>;
    }
    let ret = unsafe { NSAvailableWindowDepths() };
    ret.expect("function was marked as returning non-null, but actually returned NULL")
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nswhite?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static NSWhite: CGFloat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nslightgray?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static NSLightGray: CGFloat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdarkgray?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static NSDarkGray: CGFloat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsblack?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static NSBlack: CGFloat;
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdisplaygamut?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDisplayGamut(pub NSInteger);
impl NSDisplayGamut {
    #[doc(alias = "NSDisplayGamutSRGB")]
    pub const SRGB: Self = Self(1);
    #[doc(alias = "NSDisplayGamutP3")]
    pub const P3: Self = Self(2);
}

unsafe impl Encode for NSDisplayGamut {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSDisplayGamut {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdevicedescriptionkey?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type NSDeviceDescriptionKey = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdeviceresolution?language=objc)
    pub static NSDeviceResolution: &'static NSDeviceDescriptionKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdevicecolorspacename?language=objc)
    pub static NSDeviceColorSpaceName: &'static NSDeviceDescriptionKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdevicebitspersample?language=objc)
    pub static NSDeviceBitsPerSample: &'static NSDeviceDescriptionKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdeviceisscreen?language=objc)
    pub static NSDeviceIsScreen: &'static NSDeviceDescriptionKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdeviceisprinter?language=objc)
    pub static NSDeviceIsPrinter: &'static NSDeviceDescriptionKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsdevicesize?language=objc)
    pub static NSDeviceSize: &'static NSDeviceDescriptionKey;
}

extern "C-unwind" {
    pub fn NSRectFill(rect: NSRect);
}

extern "C-unwind" {
    pub fn NSRectFillList(rects: NonNull<NSRect>, count: NSInteger);
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn NSRectFillListWithGrays(rects: NonNull<NSRect>, grays: NonNull<CGFloat>, num: NSInteger);
}

extern "C-unwind" {
    #[cfg(feature = "NSColor")]
    pub fn NSRectFillListWithColors(
        rects: NonNull<NSRect>,
        colors: NonNull<NonNull<NSColor>>,
        num: NSInteger,
    );
}

extern "C-unwind" {
    pub fn NSRectFillUsingOperation(rect: NSRect, op: NSCompositingOperation);
}

extern "C-unwind" {
    pub fn NSRectFillListUsingOperation(
        rects: NonNull<NSRect>,
        count: NSInteger,
        op: NSCompositingOperation,
    );
}

extern "C-unwind" {
    #[cfg(feature = "NSColor")]
    pub fn NSRectFillListWithColorsUsingOperation(
        rects: NonNull<NSRect>,
        colors: NonNull<NonNull<NSColor>>,
        num: NSInteger,
        op: NSCompositingOperation,
    );
}

extern "C-unwind" {
    pub fn NSFrameRect(rect: NSRect);
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn NSFrameRectWithWidth(rect: NSRect, frame_width: CGFloat);
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn NSFrameRectWithWidthUsingOperation(
        rect: NSRect,
        frame_width: CGFloat,
        op: NSCompositingOperation,
    );
}

extern "C-unwind" {
    pub fn NSRectClip(rect: NSRect);
}

extern "C-unwind" {
    pub fn NSRectClipList(rects: NonNull<NSRect>, count: NSInteger);
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn NSDrawTiledRects(
        bounds_rect: NSRect,
        clip_rect: NSRect,
        sides: NonNull<NSRectEdge>,
        grays: NonNull<CGFloat>,
        count: NSInteger,
    ) -> NSRect;
}

extern "C-unwind" {
    pub fn NSDrawGrayBezel(rect: NSRect, clip_rect: NSRect);
}

extern "C-unwind" {
    pub fn NSDrawGroove(rect: NSRect, clip_rect: NSRect);
}

extern "C-unwind" {
    pub fn NSDrawWhiteBezel(rect: NSRect, clip_rect: NSRect);
}

extern "C-unwind" {
    pub fn NSDrawButton(rect: NSRect, clip_rect: NSRect);
}

extern "C-unwind" {
    pub fn NSEraseRect(rect: NSRect);
}

#[cfg(feature = "NSColor")]
#[deprecated = "Use -[NSBitmapImageRep colorAtX:y:] to interrogate pixel values.  If necessary, use -[NSView cacheDisplayInRect:toBitmapImageRep:] to snapshot a view hierarchy into an NSBitmapImageRep."]
#[inline]
pub unsafe extern "C-unwind" fn NSReadPixel(passed_point: NSPoint) -> Option<Retained<NSColor>> {
    extern "C-unwind" {
        fn NSReadPixel(passed_point: NSPoint) -> *mut NSColor;
    }
    let ret = unsafe { NSReadPixel(passed_point) };
    unsafe { Retained::retain_autoreleased(ret) }
}

extern "C-unwind" {
    #[deprecated]
    pub fn NSHighlightRect(rect: NSRect);
}

extern "C-unwind" {
    pub fn NSBeep();
}

extern "C-unwind" {
    #[deprecated = "Doesn't return anything useful since 10.0"]
    pub fn NSGetWindowServerMemory(
        context: NSInteger,
        virtual_memory: NonNull<NSInteger>,
        window_backing_memory: NonNull<NSInteger>,
        window_dump_string: NonNull<NonNull<NSString>>,
    ) -> NSInteger;
}

extern "C-unwind" {
    #[cfg(feature = "NSColor")]
    pub fn NSDrawColorTiledRects(
        bounds_rect: NSRect,
        clip_rect: NSRect,
        sides: NonNull<NSRectEdge>,
        colors: NonNull<NonNull<NSColor>>,
        count: NSInteger,
    ) -> NSRect;
}

extern "C-unwind" {
    pub fn NSDrawDarkBezel(rect: NSRect, clip_rect: NSRect);
}

extern "C-unwind" {
    pub fn NSDrawLightBezel(rect: NSRect, clip_rect: NSRect);
}

extern "C-unwind" {
    pub fn NSDottedFrameRect(rect: NSRect);
}

extern "C-unwind" {
    pub fn NSDrawWindowBackground(rect: NSRect);
}

extern "C-unwind" {
    pub fn NSSetFocusRingStyle(placement: NSFocusRingPlacement);
}

extern "C-unwind" {
    #[deprecated = "As of 10.11 it is not generally necessary to take explicit action to achieve visual atomicity. +[NSAnimationContext runAnimationGroup:] and other similar methods can be used when a stronger than normal need for visual atomicity is required. The NSAnimationContext methods do not suffer from the same performance problems as NSDisableScreenUpdates."]
    pub fn NSDisableScreenUpdates();
}

extern "C-unwind" {
    #[deprecated = "As of 10.11 it is not generally necessary to take explicit action to achieve visual atomicity. +[NSAnimationContext runAnimationGroup:] and other similar methods can be used when a stronger than normal need for visual atomicity is required. The NSAnimationContext methods do not suffer from the same performance problems as NSEnableScreenUpdates."]
    pub fn NSEnableScreenUpdates();
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsanimationeffect?language=objc)
// NS_ENUM
#[deprecated = "Use +[NSCursor disappearingItemCursor] instead"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSAnimationEffect(pub NSUInteger);
impl NSAnimationEffect {
    #[doc(alias = "NSAnimationEffectDisappearingItemDefault")]
    pub const DisappearingItemDefault: Self = Self(0);
    #[doc(alias = "NSAnimationEffectPoof")]
    pub const Poof: Self = Self(10);
}

unsafe impl Encode for NSAnimationEffect {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSAnimationEffect {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[deprecated = "Use +[NSCursor disappearingItemCursor] instead"]
    pub fn NSShowAnimationEffect(
        animation_effect: NSAnimationEffect,
        center_location: NSPoint,
        size: NSSize,
        animation_delegate: Option<&AnyObject>,
        did_end_selector: Option<Sel>,
        context_info: *mut c_void,
    );
}

extern "C-unwind" {
    #[deprecated = "Use +[NSWindow windowNumbersWithOptions:] instead"]
    pub fn NSCountWindows(count: NonNull<NSInteger>);
}

extern "C-unwind" {
    #[deprecated = "Use +[NSWindow windowNumbersWithOptions:] instead"]
    pub fn NSWindowList(size: NSInteger, list: NonNull<NSInteger>);
}

extern "C-unwind" {
    #[deprecated = "Use +[NSWindow windowNumbersWithOptions:] instead"]
    pub fn NSCountWindowsForContext(context: NSInteger, count: NonNull<NSInteger>);
}

extern "C-unwind" {
    #[deprecated = "Use +[NSWindow windowNumbersWithOptions:] instead"]
    pub fn NSWindowListForContext(context: NSInteger, size: NSInteger, list: NonNull<NSInteger>);
}

extern "C-unwind" {
    #[deprecated]
    pub fn NSCopyBits(src_g_state: NSInteger, src_rect: NSRect, dest_point: NSPoint);
}
