//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
#[cfg(target_vendor = "apple")]
use objc2_core_graphics::*;
#[cfg(feature = "objc2-core-image")]
#[cfg(target_vendor = "apple")]
use objc2_core_image::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstiffcompression?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTIFFCompression(pub NSUInteger);
impl NSTIFFCompression {
    #[doc(alias = "NSTIFFCompressionNone")]
    pub const None: Self = Self(1);
    #[doc(alias = "NSTIFFCompressionCCITTFAX3")]
    pub const CCITTFAX3: Self = Self(3);
    #[doc(alias = "NSTIFFCompressionCCITTFAX4")]
    pub const CCITTFAX4: Self = Self(4);
    #[doc(alias = "NSTIFFCompressionLZW")]
    pub const LZW: Self = Self(5);
    #[doc(alias = "NSTIFFCompressionJPEG")]
    pub const JPEG: Self = Self(6);
    #[doc(alias = "NSTIFFCompressionNEXT")]
    pub const NEXT: Self = Self(32766);
    #[doc(alias = "NSTIFFCompressionPackBits")]
    pub const PackBits: Self = Self(32773);
    #[doc(alias = "NSTIFFCompressionOldJPEG")]
    pub const OldJPEG: Self = Self(32865);
}

unsafe impl Encode for NSTIFFCompression {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSTIFFCompression {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsbitmapimagefiletype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSBitmapImageFileType(pub NSUInteger);
impl NSBitmapImageFileType {
    #[doc(alias = "NSBitmapImageFileTypeTIFF")]
    pub const TIFF: Self = Self(0);
    #[doc(alias = "NSBitmapImageFileTypeBMP")]
    pub const BMP: Self = Self(1);
    #[doc(alias = "NSBitmapImageFileTypeGIF")]
    pub const GIF: Self = Self(2);
    #[doc(alias = "NSBitmapImageFileTypeJPEG")]
    pub const JPEG: Self = Self(3);
    #[doc(alias = "NSBitmapImageFileTypePNG")]
    pub const PNG: Self = Self(4);
    #[doc(alias = "NSBitmapImageFileTypeJPEG2000")]
    pub const JPEG2000: Self = Self(5);
}

unsafe impl Encode for NSBitmapImageFileType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSBitmapImageFileType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsimagereploadstatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSImageRepLoadStatus(pub NSInteger);
impl NSImageRepLoadStatus {
    #[doc(alias = "NSImageRepLoadStatusUnknownType")]
    pub const UnknownType: Self = Self(-1);
    #[doc(alias = "NSImageRepLoadStatusReadingHeader")]
    pub const ReadingHeader: Self = Self(-2);
    #[doc(alias = "NSImageRepLoadStatusWillNeedAllData")]
    pub const WillNeedAllData: Self = Self(-3);
    #[doc(alias = "NSImageRepLoadStatusInvalidData")]
    pub const InvalidData: Self = Self(-4);
    #[doc(alias = "NSImageRepLoadStatusUnexpectedEOF")]
    pub const UnexpectedEOF: Self = Self(-5);
    #[doc(alias = "NSImageRepLoadStatusCompleted")]
    pub const Completed: Self = Self(-6);
}

unsafe impl Encode for NSImageRepLoadStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSImageRepLoadStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsbitmapformat?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSBitmapFormat(pub NSUInteger);
bitflags::bitflags! {
    impl NSBitmapFormat: NSUInteger {
        #[doc(alias = "NSBitmapFormatAlphaFirst")]
        const AlphaFirst = 1<<0;
        #[doc(alias = "NSBitmapFormatAlphaNonpremultiplied")]
        const AlphaNonpremultiplied = 1<<1;
        #[doc(alias = "NSBitmapFormatFloatingPointSamples")]
        const FloatingPointSamples = 1<<2;
        #[doc(alias = "NSBitmapFormatSixteenBitLittleEndian")]
        const SixteenBitLittleEndian = 1<<8;
        #[doc(alias = "NSBitmapFormatThirtyTwoBitLittleEndian")]
        const ThirtyTwoBitLittleEndian = 1<<9;
        #[doc(alias = "NSBitmapFormatSixteenBitBigEndian")]
        const SixteenBitBigEndian = 1<<10;
        #[doc(alias = "NSBitmapFormatThirtyTwoBitBigEndian")]
        const ThirtyTwoBitBigEndian = 1<<11;
    }
}

unsafe impl Encode for NSBitmapFormat {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSBitmapFormat {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsbitmapimagereppropertykey?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type NSBitmapImageRepPropertyKey = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsimagecompressionmethod?language=objc)
    pub static NSImageCompressionMethod: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsimagecompressionfactor?language=objc)
    pub static NSImageCompressionFactor: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsimagedithertransparency?language=objc)
    pub static NSImageDitherTransparency: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsimagergbcolortable?language=objc)
    pub static NSImageRGBColorTable: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsimageinterlaced?language=objc)
    pub static NSImageInterlaced: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsimagecolorsyncprofiledata?language=objc)
    pub static NSImageColorSyncProfileData: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsimageframecount?language=objc)
    pub static NSImageFrameCount: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsimagecurrentframe?language=objc)
    pub static NSImageCurrentFrame: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsimagecurrentframeduration?language=objc)
    pub static NSImageCurrentFrameDuration: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsimageloopcount?language=objc)
    pub static NSImageLoopCount: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsimagegamma?language=objc)
    pub static NSImageGamma: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsimageprogressive?language=objc)
    pub static NSImageProgressive: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsimageexifdata?language=objc)
    pub static NSImageEXIFData: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsimageiptcdata?language=objc)
    pub static NSImageIPTCData: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsimagefallbackbackgroundcolor?language=objc)
    pub static NSImageFallbackBackgroundColor: &'static NSBitmapImageRepPropertyKey;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsbitmapimagerep?language=objc)
    #[unsafe(super(NSImageRep, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSImageRep")]
    pub struct NSBitmapImageRep;
);

#[cfg(feature = "NSImageRep")]
unsafe impl NSCoding for NSBitmapImageRep {}

#[cfg(feature = "NSImageRep")]
unsafe impl NSCopying for NSBitmapImageRep {}

#[cfg(feature = "NSImageRep")]
unsafe impl CopyingHelper for NSBitmapImageRep {
    type Result = Self;
}

#[cfg(feature = "NSImageRep")]
unsafe impl NSObjectProtocol for NSBitmapImageRep {}

#[cfg(feature = "NSImageRep")]
unsafe impl NSSecureCoding for NSBitmapImageRep {}

extern_methods!(
    #[cfg(feature = "NSImageRep")]
    unsafe impl NSBitmapImageRep {
        #[deprecated = "Use -[NSView cacheDisplayInRect:toBitmapImageRep:] to snapshot a view."]
        #[unsafe(method_family(init))]
        #[method_id(initWithFocusedViewRect:)]
        pub unsafe fn initWithFocusedViewRect(
            this: Allocated<Self>,
            rect: NSRect,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSGraphics")]
        #[unsafe(method_family(init))]
        #[method_id(initWithBitmapDataPlanes:pixelsWide:pixelsHigh:bitsPerSample:samplesPerPixel:hasAlpha:isPlanar:colorSpaceName:bytesPerRow:bitsPerPixel:)]
        pub unsafe fn initWithBitmapDataPlanes_pixelsWide_pixelsHigh_bitsPerSample_samplesPerPixel_hasAlpha_isPlanar_colorSpaceName_bytesPerRow_bitsPerPixel(
            this: Allocated<Self>,
            planes: *mut *mut c_uchar,
            width: NSInteger,
            height: NSInteger,
            bps: NSInteger,
            spp: NSInteger,
            alpha: bool,
            is_planar: bool,
            color_space_name: &NSColorSpaceName,
            r_bytes: NSInteger,
            p_bits: NSInteger,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSGraphics")]
        #[unsafe(method_family(init))]
        #[method_id(initWithBitmapDataPlanes:pixelsWide:pixelsHigh:bitsPerSample:samplesPerPixel:hasAlpha:isPlanar:colorSpaceName:bitmapFormat:bytesPerRow:bitsPerPixel:)]
        pub unsafe fn initWithBitmapDataPlanes_pixelsWide_pixelsHigh_bitsPerSample_samplesPerPixel_hasAlpha_isPlanar_colorSpaceName_bitmapFormat_bytesPerRow_bitsPerPixel(
            this: Allocated<Self>,
            planes: *mut *mut c_uchar,
            width: NSInteger,
            height: NSInteger,
            bps: NSInteger,
            spp: NSInteger,
            alpha: bool,
            is_planar: bool,
            color_space_name: &NSColorSpaceName,
            bitmap_format: NSBitmapFormat,
            r_bytes: NSInteger,
            p_bits: NSInteger,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "objc2-core-graphics")]
        #[cfg(target_vendor = "apple")]
        #[unsafe(method_family(init))]
        #[method_id(initWithCGImage:)]
        pub unsafe fn initWithCGImage(this: Allocated<Self>, cg_image: &CGImage) -> Retained<Self>;

        #[cfg(feature = "objc2-core-image")]
        #[cfg(target_vendor = "apple")]
        #[unsafe(method_family(init))]
        #[method_id(initWithCIImage:)]
        pub unsafe fn initWithCIImage(this: Allocated<Self>, ci_image: &CIImage) -> Retained<Self>;

        #[unsafe(method_family(none))]
        #[method_id(imageRepsWithData:)]
        pub unsafe fn imageRepsWithData(data: &NSData) -> Retained<NSArray<NSImageRep>>;

        #[unsafe(method_family(none))]
        #[method_id(imageRepWithData:)]
        pub unsafe fn imageRepWithData(data: &NSData) -> Option<Retained<Self>>;

        #[unsafe(method_family(init))]
        #[method_id(initWithData:)]
        pub unsafe fn initWithData(this: Allocated<Self>, data: &NSData) -> Option<Retained<Self>>;

        #[method(bitmapData)]
        pub unsafe fn bitmapData(&self) -> *mut c_uchar;

        #[method(getBitmapDataPlanes:)]
        pub unsafe fn getBitmapDataPlanes(&self, data: NonNull<*mut c_uchar>);

        #[method(isPlanar)]
        pub unsafe fn isPlanar(&self) -> bool;

        #[method(samplesPerPixel)]
        pub unsafe fn samplesPerPixel(&self) -> NSInteger;

        #[method(bitsPerPixel)]
        pub unsafe fn bitsPerPixel(&self) -> NSInteger;

        #[method(bytesPerRow)]
        pub unsafe fn bytesPerRow(&self) -> NSInteger;

        #[method(bytesPerPlane)]
        pub unsafe fn bytesPerPlane(&self) -> NSInteger;

        #[method(numberOfPlanes)]
        pub unsafe fn numberOfPlanes(&self) -> NSInteger;

        #[method(bitmapFormat)]
        pub unsafe fn bitmapFormat(&self) -> NSBitmapFormat;

        #[method(getCompression:factor:)]
        pub unsafe fn getCompression_factor(
            &self,
            compression: *mut NSTIFFCompression,
            factor: *mut c_float,
        );

        #[method(setCompression:factor:)]
        pub unsafe fn setCompression_factor(&self, compression: NSTIFFCompression, factor: c_float);

        #[unsafe(method_family(none))]
        #[method_id(TIFFRepresentation)]
        pub unsafe fn TIFFRepresentation(&self) -> Option<Retained<NSData>>;

        #[unsafe(method_family(none))]
        #[method_id(TIFFRepresentationUsingCompression:factor:)]
        pub unsafe fn TIFFRepresentationUsingCompression_factor(
            &self,
            comp: NSTIFFCompression,
            factor: c_float,
        ) -> Option<Retained<NSData>>;

        #[unsafe(method_family(none))]
        #[method_id(TIFFRepresentationOfImageRepsInArray:)]
        pub unsafe fn TIFFRepresentationOfImageRepsInArray(
            array: &NSArray<NSImageRep>,
        ) -> Option<Retained<NSData>>;

        #[unsafe(method_family(none))]
        #[method_id(TIFFRepresentationOfImageRepsInArray:usingCompression:factor:)]
        pub unsafe fn TIFFRepresentationOfImageRepsInArray_usingCompression_factor(
            array: &NSArray<NSImageRep>,
            comp: NSTIFFCompression,
            factor: c_float,
        ) -> Option<Retained<NSData>>;

        #[method(getTIFFCompressionTypes:count:)]
        pub unsafe fn getTIFFCompressionTypes_count(
            list: NonNull<*const NSTIFFCompression>,
            num_types: NonNull<NSInteger>,
        );

        #[unsafe(method_family(none))]
        #[method_id(localizedNameForTIFFCompressionType:)]
        pub unsafe fn localizedNameForTIFFCompressionType(
            compression: NSTIFFCompression,
        ) -> Option<Retained<NSString>>;

        #[method(canBeCompressedUsing:)]
        pub unsafe fn canBeCompressedUsing(&self, compression: NSTIFFCompression) -> bool;

        #[cfg(all(feature = "NSColor", feature = "objc2-core-foundation"))]
        #[method(colorizeByMappingGray:toColor:blackMapping:whiteMapping:)]
        pub unsafe fn colorizeByMappingGray_toColor_blackMapping_whiteMapping(
            &self,
            mid_point: CGFloat,
            mid_point_color: Option<&NSColor>,
            shadow_color: Option<&NSColor>,
            light_color: Option<&NSColor>,
        );

        #[unsafe(method_family(init))]
        #[method_id(initForIncrementalLoad)]
        pub unsafe fn initForIncrementalLoad(this: Allocated<Self>) -> Retained<Self>;

        #[method(incrementalLoadFromData:complete:)]
        pub unsafe fn incrementalLoadFromData_complete(
            &self,
            data: &NSData,
            complete: bool,
        ) -> NSInteger;

        #[cfg(feature = "NSColor")]
        #[method(setColor:atX:y:)]
        pub unsafe fn setColor_atX_y(&self, color: &NSColor, x: NSInteger, y: NSInteger);

        #[cfg(feature = "NSColor")]
        #[unsafe(method_family(none))]
        #[method_id(colorAtX:y:)]
        pub unsafe fn colorAtX_y(&self, x: NSInteger, y: NSInteger) -> Option<Retained<NSColor>>;

        #[method(getPixel:atX:y:)]
        pub unsafe fn getPixel_atX_y(&self, p: NonNull<NSUInteger>, x: NSInteger, y: NSInteger);

        #[method(setPixel:atX:y:)]
        pub unsafe fn setPixel_atX_y(&self, p: NonNull<NSUInteger>, x: NSInteger, y: NSInteger);

        #[cfg(feature = "objc2-core-graphics")]
        #[cfg(target_vendor = "apple")]
        #[unsafe(method_family(none))]
        #[method_id(CGImage)]
        pub unsafe fn CGImage(&self) -> Option<Retained<CGImage>>;

        #[cfg(feature = "NSColorSpace")]
        #[unsafe(method_family(none))]
        #[method_id(colorSpace)]
        pub unsafe fn colorSpace(&self) -> Retained<NSColorSpace>;

        #[cfg(all(feature = "NSColorSpace", feature = "NSGraphics"))]
        #[unsafe(method_family(none))]
        #[method_id(bitmapImageRepByConvertingToColorSpace:renderingIntent:)]
        pub unsafe fn bitmapImageRepByConvertingToColorSpace_renderingIntent(
            &self,
            target_space: &NSColorSpace,
            rendering_intent: NSColorRenderingIntent,
        ) -> Option<Retained<NSBitmapImageRep>>;

        #[cfg(feature = "NSColorSpace")]
        #[unsafe(method_family(none))]
        #[method_id(bitmapImageRepByRetaggingWithColorSpace:)]
        pub unsafe fn bitmapImageRepByRetaggingWithColorSpace(
            &self,
            new_space: &NSColorSpace,
        ) -> Option<Retained<NSBitmapImageRep>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSImageRep`
    #[cfg(feature = "NSImageRep")]
    unsafe impl NSBitmapImageRep {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSImageRep")]
    unsafe impl NSBitmapImageRep {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSBitmapImageFileTypeExtensions
    #[cfg(feature = "NSImageRep")]
    unsafe impl NSBitmapImageRep {
        #[unsafe(method_family(none))]
        #[method_id(representationOfImageRepsInArray:usingType:properties:)]
        pub unsafe fn representationOfImageRepsInArray_usingType_properties(
            image_reps: &NSArray<NSImageRep>,
            storage_type: NSBitmapImageFileType,
            properties: &NSDictionary<NSBitmapImageRepPropertyKey, AnyObject>,
        ) -> Option<Retained<NSData>>;

        #[unsafe(method_family(none))]
        #[method_id(representationUsingType:properties:)]
        pub unsafe fn representationUsingType_properties(
            &self,
            storage_type: NSBitmapImageFileType,
            properties: &NSDictionary<NSBitmapImageRepPropertyKey, AnyObject>,
        ) -> Option<Retained<NSData>>;

        #[method(setProperty:withValue:)]
        pub unsafe fn setProperty_withValue(
            &self,
            property: &NSBitmapImageRepPropertyKey,
            value: Option<&AnyObject>,
        );

        #[unsafe(method_family(none))]
        #[method_id(valueForProperty:)]
        pub unsafe fn valueForProperty(
            &self,
            property: &NSBitmapImageRepPropertyKey,
        ) -> Option<Retained<AnyObject>>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstifffiletype?language=objc)
pub static NSTIFFFileType: NSBitmapImageFileType =
    NSBitmapImageFileType(NSBitmapImageFileType::TIFF.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsbmpfiletype?language=objc)
pub static NSBMPFileType: NSBitmapImageFileType =
    NSBitmapImageFileType(NSBitmapImageFileType::BMP.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsgiffiletype?language=objc)
pub static NSGIFFileType: NSBitmapImageFileType =
    NSBitmapImageFileType(NSBitmapImageFileType::GIF.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsjpegfiletype?language=objc)
pub static NSJPEGFileType: NSBitmapImageFileType =
    NSBitmapImageFileType(NSBitmapImageFileType::JPEG.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspngfiletype?language=objc)
pub static NSPNGFileType: NSBitmapImageFileType =
    NSBitmapImageFileType(NSBitmapImageFileType::PNG.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsjpeg2000filetype?language=objc)
pub static NSJPEG2000FileType: NSBitmapImageFileType =
    NSBitmapImageFileType(NSBitmapImageFileType::JPEG2000.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsalphafirstbitmapformat?language=objc)
pub static NSAlphaFirstBitmapFormat: NSBitmapFormat = NSBitmapFormat(NSBitmapFormat::AlphaFirst.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsalphanonpremultipliedbitmapformat?language=objc)
pub static NSAlphaNonpremultipliedBitmapFormat: NSBitmapFormat =
    NSBitmapFormat(NSBitmapFormat::AlphaNonpremultiplied.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfloatingpointsamplesbitmapformat?language=objc)
pub static NSFloatingPointSamplesBitmapFormat: NSBitmapFormat =
    NSBitmapFormat(NSBitmapFormat::FloatingPointSamples.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/ns16bitlittleendianbitmapformat?language=objc)
pub static NS16BitLittleEndianBitmapFormat: NSBitmapFormat =
    NSBitmapFormat(NSBitmapFormat::SixteenBitLittleEndian.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/ns32bitlittleendianbitmapformat?language=objc)
pub static NS32BitLittleEndianBitmapFormat: NSBitmapFormat =
    NSBitmapFormat(NSBitmapFormat::ThirtyTwoBitLittleEndian.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/ns16bitbigendianbitmapformat?language=objc)
pub static NS16BitBigEndianBitmapFormat: NSBitmapFormat =
    NSBitmapFormat(NSBitmapFormat::SixteenBitBigEndian.0);

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/ns32bitbigendianbitmapformat?language=objc)
pub static NS32BitBigEndianBitmapFormat: NSBitmapFormat =
    NSBitmapFormat(NSBitmapFormat::ThirtyTwoBitBigEndian.0);
