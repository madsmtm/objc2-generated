//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/spritekit/skmutabletexture?language=objc)
    #[unsafe(super(SKTexture, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "SKTexture")]
    pub struct SKMutableTexture;
);

#[cfg(feature = "SKTexture")]
unsafe impl NSCoding for SKMutableTexture {}

#[cfg(feature = "SKTexture")]
unsafe impl NSCopying for SKMutableTexture {}

#[cfg(feature = "SKTexture")]
unsafe impl CopyingHelper for SKMutableTexture {
    type Result = Self;
}

#[cfg(feature = "SKTexture")]
unsafe impl NSObjectProtocol for SKMutableTexture {}

#[cfg(feature = "SKTexture")]
unsafe impl NSSecureCoding for SKMutableTexture {}

extern_methods!(
    #[cfg(feature = "SKTexture")]
    unsafe impl SKMutableTexture {
        #[cfg(feature = "objc2-core-foundation")]
        /// Create a mutable texture with a specfic size.
        ///
        ///
        /// Parameter `size`: the dimension to use when creating the given texture.
        #[method_id(@__retain_semantics Init initWithSize:)]
        pub unsafe fn initWithSize(this: Allocated<Self>, size: CGSize) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other mutableTextureWithSize:)]
        pub unsafe fn mutableTextureWithSize(size: CGSize) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Create a mutable texture with a specfic size and type.
        ///
        ///
        /// Parameter `size`: the dimension to use when creating the given texture.
        ///
        /// Parameter `format`: the CoreVideo format type.  supported types include 'RGBA', 'RGhA', and 'RGfA' for byte, half-float, and float components.
        #[method_id(@__retain_semantics Init initWithSize:pixelFormat:)]
        pub unsafe fn initWithSize_pixelFormat(
            this: Allocated<Self>,
            size: CGSize,
            format: c_int,
        ) -> Retained<Self>;

        #[cfg(feature = "block2")]
        /// Modify the created mutable texture.
        #[method(modifyPixelDataWithBlock:)]
        pub unsafe fn modifyPixelDataWithBlock(
            &self,
            block: &block2::Block<dyn Fn(*mut c_void, usize)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `SKTexture`
    #[cfg(feature = "SKTexture")]
    unsafe impl SKMutableTexture {
        /// Create a texture from an image file. Behaves similar to imageNamed: in UIImage or NSImage
        ///
        ///
        /// Parameter `name`: the name or path of the image to load.
        #[method_id(@__retain_semantics Other textureWithImageNamed:)]
        pub unsafe fn textureWithImageNamed(name: &NSString) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Create a texture that is a subrect of an existing texture. See textureRect property for details.
        ///
        ///
        /// Parameter `rect`: the source rectangle to use in creating a logical copy of the given texture.
        ///
        /// Parameter `texture`: the existing texture to reference in the copy.
        #[method_id(@__retain_semantics Other textureWithRect:inTexture:)]
        pub unsafe fn textureWithRect_inTexture(
            rect: CGRect,
            texture: &SKTexture,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Create a texture containing directional noise. The RGBA values in this
        /// texture can be used as a normal map or as direction possibly with length. XYZ are a three dimensional direction, and A is a magnitude.
        ///
        ///
        /// Parameter `size`: the size of the resulting texture.
        ///
        /// Parameter `smoothness`: how similar neighboring pixels are. A value of zero is like static, one is smooth.
        #[method_id(@__retain_semantics Other textureVectorNoiseWithSmoothness:size:)]
        pub unsafe fn textureVectorNoiseWithSmoothness_size(
            smoothness: CGFloat,
            size: CGSize,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Create a texture containing colored noise. The noise texture is tileable with itself.
        ///
        ///
        /// Parameter `size`: the size of the resulting texture.
        ///
        /// Parameter `smoothness`: how similar neighboring pixels are. A value of zero is like static, one is smooth.
        ///
        /// Parameter `grayscale`: if YES, RGB and A will all be the same. If no, RGB and A will all be different. A is not pre-multiplied, because the intent is that if you read a texel in a shader, all four values will be exactly the same value if grayscale, or four different, uncorrelated values if not grayscale.
        #[method_id(@__retain_semantics Other textureNoiseWithSmoothness:size:grayscale:)]
        pub unsafe fn textureNoiseWithSmoothness_size_grayscale(
            smoothness: CGFloat,
            size: CGSize,
            grayscale: bool,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-graphics")]
        /// Create a texture from a CGImageRef.
        ///
        ///
        /// Parameter `image`: the CGImageRef to create the texture from
        #[method_id(@__retain_semantics Other textureWithCGImage:)]
        pub unsafe fn textureWithCGImage(image: &CGImage) -> Retained<Self>;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Other textureWithImage:)]
        pub unsafe fn textureWithImage(image: &NSImage) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Create new texture with bitmap RGBA data in unsigned bytes. Data is copied once, additional changes to the data does not affect the texture. All pixel data is assumed to be premultiplied alpha.
        ///
        ///
        /// Parameter `pixelData`: the pixelData to read in creating the texture.
        ///
        /// Parameter `size`: the dimensions of the pixelData given.
        #[method_id(@__retain_semantics Other textureWithData:size:)]
        pub unsafe fn textureWithData_size(pixel_data: &NSData, size: CGSize) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other textureWithData:size:flipped:)]
        pub unsafe fn textureWithData_size_flipped(
            pixel_data: &NSData,
            size: CGSize,
            flipped: bool,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Create new texture with bitmap RGBA data in unsigned bytes using a custom row length and row alignment. Data is copied once, additional changes to the data does not affect the texture. All pixel data is assumed to be premultiplied alpha.
        ///
        ///
        /// Parameter `pixelData`: the data to use
        ///
        /// Parameter `size`: the size in texels
        ///
        /// Parameter `rowLength`: the length of each row in pixels (allows byte row pitches greater than the width for aligned data)
        ///
        /// Parameter `alignment`: the byte alignment of the data, provide 0 for tightly packed data.
        #[method_id(@__retain_semantics Other textureWithData:size:rowLength:alignment:)]
        pub unsafe fn textureWithData_size_rowLength_alignment(
            pixel_data: &NSData,
            size: CGSize,
            row_length: c_uint,
            alignment: c_uint,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "SKTexture")]
    unsafe impl SKMutableTexture {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);