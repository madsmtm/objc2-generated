//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-video")]
use objc2_core_video::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// An object wrapping a matting image used for high quality rendering of portrait style effects onto an image (i.e. shallow depth of field, stage lighting, etc).
    ///
    ///
    /// The pixel data in the matting image is represented in CVPixelBuffers as kCVPixelFormatType_OneComponent8 ('L008'). It's stored in image files as an auxiliary image, accessible using CGImageSourceCopyAuxiliaryDataInfoAtIndex with the data type kCGImageAuxiliaryDataTypePortraitEffectsMatte (see
    /// <ImageIO
    /// /CGImageProperties.h>).
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avportraiteffectsmatte?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVPortraitEffectsMatte;
);

unsafe impl NSObjectProtocol for AVPortraitEffectsMatte {}

extern_methods!(
    unsafe impl AVPortraitEffectsMatte {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        /// Returns an AVPortraitEffectsMatte instance from auxiliary image information in an image file.
        ///
        ///
        /// Parameter `imageSourceAuxDataInfoDictionary`: A dictionary of primitive portrait effects matte related information obtained from CGImageSourceCopyAuxiliaryDataInfoAtIndex.
        ///
        /// Parameter `outError`: On return, if the portrait effects matte cannot be created, points to an NSError describing the problem.
        ///
        /// Returns: An AVPortraitEffectsMatte instance, or nil if the auxiliary data info dictionary was malformed.
        ///
        ///
        /// When using ImageIO framework's CGImageSource API to read from a HEIF or JPEG file containing a portrait effects matte, AVPortraitEffectsMatte can be instantiated using the result of CGImageSourceCopyAuxiliaryDataInfoAtIndex, which returns a CFDictionary of primitive map information.
        #[method_id(@__retain_semantics Other portraitEffectsMatteFromDictionaryRepresentation:error:_)]
        pub unsafe fn portraitEffectsMatteFromDictionaryRepresentation_error(
            image_source_aux_data_info_dictionary: &NSDictionary,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "objc2-core-video")]
        /// Returns an AVPortraitEffectsMatte instance wrapping the replacement pixel buffer.
        ///
        ///
        /// Parameter `pixelBuffer`: A pixel buffer containing a portrait effects matting image, represented as kCVPixelFormatType_OneComponent8 with a kCVImageBufferTransferFunction_Linear transfer function.
        ///
        /// Parameter `outError`: On return, if the AVPortraitEffectsMatte cannot be created, points to an NSError describing the problem.
        ///
        /// Returns: An AVPortraitEffectsMatte instance, or nil if the pixel buffer is malformed.
        ///
        ///
        /// When applying complex edits to media containing a portrait effects matte, you may create a derivative matte with arbitrary transforms applied to it, then use this initializer to create a new AVPortraitEffectsMatte.
        #[method_id(@__retain_semantics Other portraitEffectsMatteByReplacingPortraitEffectsMatteWithPixelBuffer:error:_)]
        pub unsafe fn portraitEffectsMatteByReplacingPortraitEffectsMatteWithPixelBuffer_error(
            &self,
            pixel_buffer: &CVPixelBuffer,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        /// Returns a dictionary of primitive map information to be used when writing an image file with a portrait effects matte.
        ///
        ///
        /// Parameter `outAuxDataType`: On output, the auxiliary data type to be used when calling CGImageDestinationAddAuxiliaryDataInfo. Currently the only supported auxiliary data type is kCGImageAuxiliaryDataTypePortraitEffectsMatte.
        ///
        /// Returns: A dictionary of CGImageDestination compatible portrait effects matte information, or nil if the auxDataType is unsupported.
        ///
        ///
        /// When using ImageIO framework's CGImageDestination API to write portrait effects matte information to a HEIF or JPEG file, you may use this method to generate a dictionary of primitive map information consumed by CGImageDestinationAddAuxiliaryDataInfo.
        #[method_id(@__retain_semantics Other dictionaryRepresentationForAuxiliaryDataType:)]
        pub unsafe fn dictionaryRepresentationForAuxiliaryDataType(
            &self,
            out_aux_data_type: Option<&mut Option<Retained<NSString>>>,
        ) -> Option<Retained<NSDictionary>>;

        /// Specifies the pixel format type of this object's internal matting image.
        ///
        ///
        /// Currently the only supported CV pixel format type for the matting image is kCVPixelFormatType_OneComponent8.
        #[method(pixelFormatType)]
        pub unsafe fn pixelFormatType(&self) -> OSType;

        #[cfg(feature = "objc2-core-video")]
        /// Provides access to the portrait effects matte's internal image.
        ///
        ///
        /// The pixel format can be queried using the pixelFormatType property.
        #[method_id(@__retain_semantics Other mattingImage)]
        pub unsafe fn mattingImage(&self) -> Retained<CVPixelBuffer>;
    }
);