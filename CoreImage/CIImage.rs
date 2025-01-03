//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;
#[cfg(feature = "objc2-core-video")]
use objc2_core_video::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-io-surface")]
use objc2_io_surface::*;
#[cfg(feature = "objc2-metal")]
use objc2_metal::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coreimage/ciformat?language=objc)
// NS_TYPED_ENUM
pub type CIFormat = c_int;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatargb8?language=objc)
    pub static kCIFormatARGB8: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatbgra8?language=objc)
    pub static kCIFormatBGRA8: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatrgba8?language=objc)
    pub static kCIFormatRGBA8: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatabgr8?language=objc)
    pub static kCIFormatABGR8: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatrgbah?language=objc)
    pub static kCIFormatRGBAh: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatrgba16?language=objc)
    pub static kCIFormatRGBA16: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatrgbaf?language=objc)
    pub static kCIFormatRGBAf: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatrgbx16?language=objc)
    pub static kCIFormatRGBX16: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatrgbxh?language=objc)
    pub static kCIFormatRGBXh: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatrgbxf?language=objc)
    pub static kCIFormatRGBXf: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatrgb10?language=objc)
    pub static kCIFormatRGB10: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformata8?language=objc)
    pub static kCIFormatA8: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformata16?language=objc)
    pub static kCIFormatA16: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatah?language=objc)
    pub static kCIFormatAh: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformataf?language=objc)
    pub static kCIFormatAf: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatr8?language=objc)
    pub static kCIFormatR8: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatr16?language=objc)
    pub static kCIFormatR16: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatrh?language=objc)
    pub static kCIFormatRh: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatrf?language=objc)
    pub static kCIFormatRf: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatrg8?language=objc)
    pub static kCIFormatRG8: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatrg16?language=objc)
    pub static kCIFormatRG16: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatrgh?language=objc)
    pub static kCIFormatRGh: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatrgf?language=objc)
    pub static kCIFormatRGf: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatl8?language=objc)
    pub static kCIFormatL8: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatl16?language=objc)
    pub static kCIFormatL16: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatlh?language=objc)
    pub static kCIFormatLh: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatlf?language=objc)
    pub static kCIFormatLf: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatla8?language=objc)
    pub static kCIFormatLA8: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatla16?language=objc)
    pub static kCIFormatLA16: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatlah?language=objc)
    pub static kCIFormatLAh: CIFormat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciformatlaf?language=objc)
    pub static kCIFormatLAf: CIFormat;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coreimage/ciimageoption?language=objc)
// NS_TYPED_ENUM
pub type CIImageOption = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimagecolorspace?language=objc)
    pub static kCIImageColorSpace: &'static CIImageOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimagetonemaphdrtosdr?language=objc)
    pub static kCIImageToneMapHDRtoSDR: &'static CIImageOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimageexpandtohdr?language=objc)
    pub static kCIImageExpandToHDR: &'static CIImageOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimagecontentheadroom?language=objc)
    pub static kCIImageContentHeadroom: &'static CIImageOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimagenearestsampling?language=objc)
    pub static kCIImageNearestSampling: &'static CIImageOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimagecacheimmediately?language=objc)
    pub static kCIImageCacheImmediately: &'static CIImageOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimageproperties?language=objc)
    pub static kCIImageProperties: &'static CIImageOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimageapplyorientationproperty?language=objc)
    pub static kCIImageApplyOrientationProperty: &'static CIImageOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimagetexturetarget?language=objc)
    pub static kCIImageTextureTarget: &'static CIImageOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimagetextureformat?language=objc)
    pub static kCIImageTextureFormat: &'static CIImageOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimageauxiliarydepth?language=objc)
    pub static kCIImageAuxiliaryDepth: &'static CIImageOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimageauxiliarydisparity?language=objc)
    pub static kCIImageAuxiliaryDisparity: &'static CIImageOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimageauxiliaryportraiteffectsmatte?language=objc)
    pub static kCIImageAuxiliaryPortraitEffectsMatte: &'static CIImageOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimageauxiliarysemanticsegmentationskinmatte?language=objc)
    pub static kCIImageAuxiliarySemanticSegmentationSkinMatte: &'static CIImageOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimageauxiliarysemanticsegmentationhairmatte?language=objc)
    pub static kCIImageAuxiliarySemanticSegmentationHairMatte: &'static CIImageOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimageauxiliarysemanticsegmentationteethmatte?language=objc)
    pub static kCIImageAuxiliarySemanticSegmentationTeethMatte: &'static CIImageOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimageauxiliarysemanticsegmentationglassesmatte?language=objc)
    pub static kCIImageAuxiliarySemanticSegmentationGlassesMatte: &'static CIImageOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimageauxiliarysemanticsegmentationskymatte?language=objc)
    pub static kCIImageAuxiliarySemanticSegmentationSkyMatte: &'static CIImageOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimageauxiliaryhdrgainmap?language=objc)
    pub static kCIImageAuxiliaryHDRGainMap: &'static CIImageOption;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/ciimage?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIImage;
);

unsafe impl NSCoding for CIImage {}

unsafe impl NSCopying for CIImage {}

unsafe impl CopyingHelper for CIImage {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CIImage {}

unsafe impl NSSecureCoding for CIImage {}

extern_methods!(
    unsafe impl CIImage {
        #[cfg(feature = "objc2-core-graphics")]
        #[method_id(@__retain_semantics Other imageWithCGImage:)]
        pub unsafe fn imageWithCGImage(image: CGImageRef) -> Retained<CIImage>;

        #[cfg(feature = "objc2-core-graphics")]
        #[method_id(@__retain_semantics Other imageWithCGImage:options:)]
        pub unsafe fn imageWithCGImage_options(
            image: CGImageRef,
            options: Option<&NSDictionary<CIImageOption, AnyObject>>,
        ) -> Retained<CIImage>;

        #[cfg(feature = "objc2-core-graphics")]
        #[deprecated]
        #[method_id(@__retain_semantics Other imageWithCGLayer:)]
        pub unsafe fn imageWithCGLayer(layer: CGLayerRef) -> Retained<CIImage>;

        #[cfg(feature = "objc2-core-graphics")]
        #[deprecated]
        #[method_id(@__retain_semantics Other imageWithCGLayer:options:)]
        pub unsafe fn imageWithCGLayer_options(
            layer: CGLayerRef,
            options: Option<&NSDictionary<CIImageOption, AnyObject>>,
        ) -> Retained<CIImage>;

        #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-core-graphics"))]
        #[method_id(@__retain_semantics Other imageWithBitmapData:bytesPerRow:size:format:colorSpace:)]
        pub unsafe fn imageWithBitmapData_bytesPerRow_size_format_colorSpace(
            data: &NSData,
            bytes_per_row: usize,
            size: CGSize,
            format: CIFormat,
            color_space: CGColorSpaceRef,
        ) -> Retained<CIImage>;

        #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-core-graphics"))]
        #[deprecated = "Core Image OpenGL API deprecated. (Define CI_SILENCE_GL_DEPRECATION to silence these warnings)"]
        #[method_id(@__retain_semantics Other imageWithTexture:size:flipped:colorSpace:)]
        pub unsafe fn imageWithTexture_size_flipped_colorSpace(
            name: c_uint,
            size: CGSize,
            flipped: bool,
            color_space: CGColorSpaceRef,
        ) -> Retained<CIImage>;

        #[cfg(feature = "objc2-core-foundation")]
        #[deprecated = "Core Image OpenGL API deprecated. (Define CI_SILENCE_GL_DEPRECATION to silence these warnings)"]
        #[method_id(@__retain_semantics Other imageWithTexture:size:flipped:options:)]
        pub unsafe fn imageWithTexture_size_flipped_options(
            name: c_uint,
            size: CGSize,
            flipped: bool,
            options: Option<&NSDictionary<CIImageOption, AnyObject>>,
        ) -> Retained<CIImage>;

        #[cfg(feature = "objc2-metal")]
        #[method_id(@__retain_semantics Other imageWithMTLTexture:options:)]
        pub unsafe fn imageWithMTLTexture_options(
            texture: &ProtocolObject<dyn MTLTexture>,
            options: Option<&NSDictionary<CIImageOption, AnyObject>>,
        ) -> Option<Retained<CIImage>>;

        #[method_id(@__retain_semantics Other imageWithContentsOfURL:)]
        pub unsafe fn imageWithContentsOfURL(url: &NSURL) -> Option<Retained<CIImage>>;

        #[method_id(@__retain_semantics Other imageWithContentsOfURL:options:)]
        pub unsafe fn imageWithContentsOfURL_options(
            url: &NSURL,
            options: Option<&NSDictionary<CIImageOption, AnyObject>>,
        ) -> Option<Retained<CIImage>>;

        #[method_id(@__retain_semantics Other imageWithData:)]
        pub unsafe fn imageWithData(data: &NSData) -> Option<Retained<CIImage>>;

        #[method_id(@__retain_semantics Other imageWithData:options:)]
        pub unsafe fn imageWithData_options(
            data: &NSData,
            options: Option<&NSDictionary<CIImageOption, AnyObject>>,
        ) -> Option<Retained<CIImage>>;

        #[cfg(feature = "objc2-core-video")]
        #[method_id(@__retain_semantics Other imageWithCVImageBuffer:)]
        pub unsafe fn imageWithCVImageBuffer(image_buffer: CVImageBufferRef) -> Retained<CIImage>;

        #[cfg(feature = "objc2-core-video")]
        #[method_id(@__retain_semantics Other imageWithCVImageBuffer:options:)]
        pub unsafe fn imageWithCVImageBuffer_options(
            image_buffer: CVImageBufferRef,
            options: Option<&NSDictionary<CIImageOption, AnyObject>>,
        ) -> Retained<CIImage>;

        #[cfg(feature = "objc2-core-video")]
        #[method_id(@__retain_semantics Other imageWithCVPixelBuffer:)]
        pub unsafe fn imageWithCVPixelBuffer(pixel_buffer: CVPixelBufferRef) -> Retained<CIImage>;

        #[cfg(feature = "objc2-core-video")]
        #[method_id(@__retain_semantics Other imageWithCVPixelBuffer:options:)]
        pub unsafe fn imageWithCVPixelBuffer_options(
            pixel_buffer: CVPixelBufferRef,
            options: Option<&NSDictionary<CIImageOption, AnyObject>>,
        ) -> Retained<CIImage>;

        #[cfg(feature = "objc2-io-surface")]
        #[method_id(@__retain_semantics Other imageWithIOSurface:)]
        pub unsafe fn imageWithIOSurface(surface: IOSurfaceRef) -> Retained<CIImage>;

        #[cfg(feature = "objc2-io-surface")]
        #[method_id(@__retain_semantics Other imageWithIOSurface:options:)]
        pub unsafe fn imageWithIOSurface_options(
            surface: IOSurfaceRef,
            options: Option<&NSDictionary<CIImageOption, AnyObject>>,
        ) -> Retained<CIImage>;

        #[cfg(feature = "CIColor")]
        #[method_id(@__retain_semantics Other imageWithColor:)]
        pub unsafe fn imageWithColor(color: &CIColor) -> Retained<CIImage>;

        #[method_id(@__retain_semantics Other emptyImage)]
        pub unsafe fn emptyImage() -> Retained<CIImage>;

        #[method_id(@__retain_semantics Other blackImage)]
        pub unsafe fn blackImage() -> Retained<CIImage>;

        #[method_id(@__retain_semantics Other whiteImage)]
        pub unsafe fn whiteImage() -> Retained<CIImage>;

        #[method_id(@__retain_semantics Other grayImage)]
        pub unsafe fn grayImage() -> Retained<CIImage>;

        #[method_id(@__retain_semantics Other redImage)]
        pub unsafe fn redImage() -> Retained<CIImage>;

        #[method_id(@__retain_semantics Other greenImage)]
        pub unsafe fn greenImage() -> Retained<CIImage>;

        #[method_id(@__retain_semantics Other blueImage)]
        pub unsafe fn blueImage() -> Retained<CIImage>;

        #[method_id(@__retain_semantics Other cyanImage)]
        pub unsafe fn cyanImage() -> Retained<CIImage>;

        #[method_id(@__retain_semantics Other magentaImage)]
        pub unsafe fn magentaImage() -> Retained<CIImage>;

        #[method_id(@__retain_semantics Other yellowImage)]
        pub unsafe fn yellowImage() -> Retained<CIImage>;

        #[method_id(@__retain_semantics Other clearImage)]
        pub unsafe fn clearImage() -> Retained<CIImage>;

        #[cfg(feature = "objc2-core-graphics")]
        #[method_id(@__retain_semantics Init initWithCGImage:)]
        pub unsafe fn initWithCGImage(this: Allocated<Self>, image: CGImageRef) -> Retained<Self>;

        #[cfg(feature = "objc2-core-graphics")]
        #[method_id(@__retain_semantics Init initWithCGImage:options:)]
        pub unsafe fn initWithCGImage_options(
            this: Allocated<Self>,
            image: CGImageRef,
            options: Option<&NSDictionary<CIImageOption, AnyObject>>,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-graphics")]
        #[deprecated = "Use initWithCGImage: instead."]
        #[method_id(@__retain_semantics Init initWithCGLayer:)]
        pub unsafe fn initWithCGLayer(this: Allocated<Self>, layer: CGLayerRef) -> Retained<Self>;

        #[cfg(feature = "objc2-core-graphics")]
        #[deprecated = "Use initWithCGImage:options instead."]
        #[method_id(@__retain_semantics Init initWithCGLayer:options:)]
        pub unsafe fn initWithCGLayer_options(
            this: Allocated<Self>,
            layer: CGLayerRef,
            options: Option<&NSDictionary<CIImageOption, AnyObject>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(this: Allocated<Self>, data: &NSData) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithData:options:)]
        pub unsafe fn initWithData_options(
            this: Allocated<Self>,
            data: &NSData,
            options: Option<&NSDictionary<CIImageOption, AnyObject>>,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-core-graphics"))]
        #[method_id(@__retain_semantics Init initWithBitmapData:bytesPerRow:size:format:colorSpace:)]
        pub unsafe fn initWithBitmapData_bytesPerRow_size_format_colorSpace(
            this: Allocated<Self>,
            data: &NSData,
            bytes_per_row: usize,
            size: CGSize,
            format: CIFormat,
            color_space: CGColorSpaceRef,
        ) -> Retained<Self>;

        #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-core-graphics"))]
        #[deprecated = "Core Image OpenGL API deprecated. (Define CI_SILENCE_GL_DEPRECATION to silence these warnings)"]
        #[method_id(@__retain_semantics Init initWithTexture:size:flipped:colorSpace:)]
        pub unsafe fn initWithTexture_size_flipped_colorSpace(
            this: Allocated<Self>,
            name: c_uint,
            size: CGSize,
            flipped: bool,
            color_space: CGColorSpaceRef,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[deprecated = "Core Image OpenGL API deprecated. (Define CI_SILENCE_GL_DEPRECATION to silence these warnings)"]
        #[method_id(@__retain_semantics Init initWithTexture:size:flipped:options:)]
        pub unsafe fn initWithTexture_size_flipped_options(
            this: Allocated<Self>,
            name: c_uint,
            size: CGSize,
            flipped: bool,
            options: Option<&NSDictionary<CIImageOption, AnyObject>>,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-metal")]
        #[method_id(@__retain_semantics Init initWithMTLTexture:options:)]
        pub unsafe fn initWithMTLTexture_options(
            this: Allocated<Self>,
            texture: &ProtocolObject<dyn MTLTexture>,
            options: Option<&NSDictionary<CIImageOption, AnyObject>>,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Allocated<Self>,
            url: &NSURL,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithContentsOfURL:options:)]
        pub unsafe fn initWithContentsOfURL_options(
            this: Allocated<Self>,
            url: &NSURL,
            options: Option<&NSDictionary<CIImageOption, AnyObject>>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "objc2-io-surface")]
        #[method_id(@__retain_semantics Init initWithIOSurface:)]
        pub unsafe fn initWithIOSurface(
            this: Allocated<Self>,
            surface: IOSurfaceRef,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-io-surface")]
        #[method_id(@__retain_semantics Init initWithIOSurface:options:)]
        pub unsafe fn initWithIOSurface_options(
            this: Allocated<Self>,
            surface: IOSurfaceRef,
            options: Option<&NSDictionary<CIImageOption, AnyObject>>,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-io-surface")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithIOSurface:plane:format:options:)]
        pub unsafe fn initWithIOSurface_plane_format_options(
            this: Allocated<Self>,
            surface: IOSurfaceRef,
            plane: usize,
            format: CIFormat,
            options: Option<&NSDictionary<CIImageOption, AnyObject>>,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-video")]
        #[method_id(@__retain_semantics Init initWithCVImageBuffer:)]
        pub unsafe fn initWithCVImageBuffer(
            this: Allocated<Self>,
            image_buffer: CVImageBufferRef,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-video")]
        #[method_id(@__retain_semantics Init initWithCVImageBuffer:options:)]
        pub unsafe fn initWithCVImageBuffer_options(
            this: Allocated<Self>,
            image_buffer: CVImageBufferRef,
            options: Option<&NSDictionary<CIImageOption, AnyObject>>,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-video")]
        #[method_id(@__retain_semantics Init initWithCVPixelBuffer:)]
        pub unsafe fn initWithCVPixelBuffer(
            this: Allocated<Self>,
            pixel_buffer: CVPixelBufferRef,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-video")]
        #[method_id(@__retain_semantics Init initWithCVPixelBuffer:options:)]
        pub unsafe fn initWithCVPixelBuffer_options(
            this: Allocated<Self>,
            pixel_buffer: CVPixelBufferRef,
            options: Option<&NSDictionary<CIImageOption, AnyObject>>,
        ) -> Retained<Self>;

        #[cfg(feature = "CIColor")]
        #[method_id(@__retain_semantics Init initWithColor:)]
        pub unsafe fn initWithColor(this: Allocated<Self>, color: &CIColor) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other imageByApplyingTransform:)]
        pub unsafe fn imageByApplyingTransform(
            &self,
            matrix: CGAffineTransform,
        ) -> Retained<CIImage>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other imageByApplyingTransform:highQualityDownsample:)]
        pub unsafe fn imageByApplyingTransform_highQualityDownsample(
            &self,
            matrix: CGAffineTransform,
            high_quality_downsample: bool,
        ) -> Retained<CIImage>;

        #[method_id(@__retain_semantics Other imageByApplyingOrientation:)]
        pub unsafe fn imageByApplyingOrientation(&self, orientation: c_int) -> Retained<CIImage>;

        #[method_id(@__retain_semantics Other imageByCompositingOverImage:)]
        pub unsafe fn imageByCompositingOverImage(&self, dest: &CIImage) -> Retained<CIImage>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other imageByCroppingToRect:)]
        pub unsafe fn imageByCroppingToRect(&self, rect: CGRect) -> Retained<CIImage>;

        #[method_id(@__retain_semantics Other imageByClampingToExtent)]
        pub unsafe fn imageByClampingToExtent(&self) -> Retained<CIImage>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other imageByClampingToRect:)]
        pub unsafe fn imageByClampingToRect(&self, rect: CGRect) -> Retained<CIImage>;

        #[method_id(@__retain_semantics Other imageByApplyingFilter:withInputParameters:)]
        pub unsafe fn imageByApplyingFilter_withInputParameters(
            &self,
            filter_name: &NSString,
            params: &NSDictionary<NSString, AnyObject>,
        ) -> Retained<CIImage>;

        #[method_id(@__retain_semantics Other imageByApplyingFilter:)]
        pub unsafe fn imageByApplyingFilter(&self, filter_name: &NSString) -> Retained<CIImage>;

        #[cfg(feature = "objc2-core-graphics")]
        #[method_id(@__retain_semantics Other imageByColorMatchingColorSpaceToWorkingSpace:)]
        pub unsafe fn imageByColorMatchingColorSpaceToWorkingSpace(
            &self,
            color_space: CGColorSpaceRef,
        ) -> Option<Retained<CIImage>>;

        #[cfg(feature = "objc2-core-graphics")]
        #[method_id(@__retain_semantics Other imageByColorMatchingWorkingSpaceToColorSpace:)]
        pub unsafe fn imageByColorMatchingWorkingSpaceToColorSpace(
            &self,
            color_space: CGColorSpaceRef,
        ) -> Option<Retained<CIImage>>;

        #[method_id(@__retain_semantics Other imageByPremultiplyingAlpha)]
        pub unsafe fn imageByPremultiplyingAlpha(&self) -> Retained<CIImage>;

        #[method_id(@__retain_semantics Other imageByUnpremultiplyingAlpha)]
        pub unsafe fn imageByUnpremultiplyingAlpha(&self) -> Retained<CIImage>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other imageBySettingAlphaOneInExtent:)]
        pub unsafe fn imageBySettingAlphaOneInExtent(&self, extent: CGRect) -> Retained<CIImage>;

        #[method_id(@__retain_semantics Other imageByApplyingGaussianBlurWithSigma:)]
        pub unsafe fn imageByApplyingGaussianBlurWithSigma(
            &self,
            sigma: c_double,
        ) -> Retained<CIImage>;

        #[method_id(@__retain_semantics Other imageBySettingProperties:)]
        pub unsafe fn imageBySettingProperties(
            &self,
            properties: &NSDictionary,
        ) -> Retained<CIImage>;

        #[method_id(@__retain_semantics Other imageBySamplingLinear)]
        pub unsafe fn imageBySamplingLinear(&self) -> Retained<CIImage>;

        #[method_id(@__retain_semantics Other imageBySamplingNearest)]
        pub unsafe fn imageBySamplingNearest(&self) -> Retained<CIImage>;

        #[method_id(@__retain_semantics Other imageByInsertingIntermediate)]
        pub unsafe fn imageByInsertingIntermediate(&self) -> Retained<CIImage>;

        #[method_id(@__retain_semantics Other imageByInsertingIntermediate:)]
        pub unsafe fn imageByInsertingIntermediate_(&self, cache: bool) -> Retained<CIImage>;

        #[method_id(@__retain_semantics Other imageByApplyingGainMap:)]
        pub unsafe fn imageByApplyingGainMap(&self, gainmap: &CIImage) -> Retained<CIImage>;

        #[method_id(@__retain_semantics Other imageByApplyingGainMap:headroom:)]
        pub unsafe fn imageByApplyingGainMap_headroom(
            &self,
            gainmap: &CIImage,
            headroom: c_float,
        ) -> Retained<CIImage>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(extent)]
        pub unsafe fn extent(&self) -> CGRect;

        #[method(isOpaque)]
        pub unsafe fn isOpaque(&self) -> bool;

        #[method_id(@__retain_semantics Other properties)]
        pub unsafe fn properties(&self) -> Retained<NSDictionary<NSString, AnyObject>>;

        #[cfg(feature = "CIFilterShape")]
        #[method_id(@__retain_semantics Other definition)]
        pub unsafe fn definition(&self) -> Retained<CIFilterShape>;

        #[method_id(@__retain_semantics Other url)]
        pub unsafe fn url(&self) -> Option<Retained<NSURL>>;

        #[method(contentHeadroom)]
        pub unsafe fn contentHeadroom(&self) -> c_float;

        #[cfg(feature = "objc2-core-video")]
        #[method(pixelBuffer)]
        pub unsafe fn pixelBuffer(&self) -> CVPixelBufferRef;

        #[cfg(feature = "objc2-core-graphics")]
        #[method(CGImage)]
        pub unsafe fn CGImage(&self) -> CGImageRef;

        #[cfg(feature = "objc2-metal")]
        #[method_id(@__retain_semantics Other metalTexture)]
        pub unsafe fn metalTexture(&self) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(regionOfInterestForImage:inRect:)]
        pub unsafe fn regionOfInterestForImage_inRect(
            &self,
            image: &CIImage,
            rect: CGRect,
        ) -> CGRect;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIImage {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/coreimage/ciimageautoadjustmentoption?language=objc)
// NS_TYPED_ENUM
pub type CIImageAutoAdjustmentOption = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimageautoadjustenhance?language=objc)
    pub static kCIImageAutoAdjustEnhance: &'static CIImageAutoAdjustmentOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimageautoadjustredeye?language=objc)
    pub static kCIImageAutoAdjustRedEye: &'static CIImageAutoAdjustmentOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimageautoadjustfeatures?language=objc)
    pub static kCIImageAutoAdjustFeatures: &'static CIImageAutoAdjustmentOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimageautoadjustcrop?language=objc)
    pub static kCIImageAutoAdjustCrop: &'static CIImageAutoAdjustmentOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimageautoadjustlevel?language=objc)
    pub static kCIImageAutoAdjustLevel: &'static CIImageAutoAdjustmentOption;
}

extern_methods!(
    /// AutoAdjustment
    unsafe impl CIImage {
        #[cfg(feature = "CIFilter")]
        #[method_id(@__retain_semantics Other autoAdjustmentFilters)]
        pub unsafe fn autoAdjustmentFilters(&self) -> Retained<NSArray<CIFilter>>;

        #[cfg(feature = "CIFilter")]
        #[method_id(@__retain_semantics Other autoAdjustmentFiltersWithOptions:)]
        pub unsafe fn autoAdjustmentFiltersWithOptions(
            &self,
            options: Option<&NSDictionary<CIImageAutoAdjustmentOption, AnyObject>>,
        ) -> Retained<NSArray<CIFilter>>;
    }
);

extern_methods!(
    /// LabConversion
    unsafe impl CIImage {
        #[method_id(@__retain_semantics Other imageByConvertingWorkingSpaceToLab)]
        pub unsafe fn imageByConvertingWorkingSpaceToLab(&self) -> Retained<CIImage>;

        #[method_id(@__retain_semantics Other imageByConvertingLabToWorkingSpace)]
        pub unsafe fn imageByConvertingLabToWorkingSpace(&self) -> Retained<CIImage>;
    }
);
