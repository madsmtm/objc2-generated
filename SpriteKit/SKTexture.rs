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
#[cfg(feature = "objc2-core-image")]
#[cfg(not(target_os = "watchos"))]
use objc2_core_image::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/spritekit/sktexturefilteringmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SKTextureFilteringMode(pub NSInteger);
impl SKTextureFilteringMode {
    #[doc(alias = "SKTextureFilteringNearest")]
    pub const Nearest: Self = Self(0);
    #[doc(alias = "SKTextureFilteringLinear")]
    pub const Linear: Self = Self(1);
}

unsafe impl Encode for SKTextureFilteringMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SKTextureFilteringMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// A texture to be mapped onto SKSpriteNode instances.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/spritekit/sktexture?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SKTexture;
);

extern_conformance!(
    unsafe impl NSCoding for SKTexture {}
);

extern_conformance!(
    unsafe impl NSCopying for SKTexture {}
);

unsafe impl CopyingHelper for SKTexture {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for SKTexture {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for SKTexture {}
);

impl SKTexture {
    extern_methods!(
        /// Create a texture from an image file. Behaves similar to imageNamed: in UIImage or NSImage
        ///
        ///
        /// Parameter `name`: the name or path of the image to load.
        #[unsafe(method(textureWithImageNamed:))]
        #[unsafe(method_family = none)]
        pub unsafe fn textureWithImageNamed(name: &NSString) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Create a texture that is a subrect of an existing texture. See textureRect property for details.
        ///
        ///
        /// Parameter `rect`: the source rectangle to use in creating a logical copy of the given texture.
        ///
        /// Parameter `texture`: the existing texture to reference in the copy.
        #[unsafe(method(textureWithRect:inTexture:))]
        #[unsafe(method_family = none)]
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
        #[unsafe(method(textureVectorNoiseWithSmoothness:size:))]
        #[unsafe(method_family = none)]
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
        #[unsafe(method(textureNoiseWithSmoothness:size:grayscale:))]
        #[unsafe(method_family = none)]
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
        #[unsafe(method(textureWithCGImage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn textureWithCGImage(image: &CGImage) -> Retained<Self>;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[unsafe(method(textureWithImage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn textureWithImage(image: &NSImage) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Create new texture with bitmap RGBA data in unsigned bytes. Data is copied once, additional changes to the data does not affect the texture. All pixel data is assumed to be premultiplied alpha.
        ///
        ///
        /// Parameter `pixelData`: the pixelData to read in creating the texture.
        ///
        /// Parameter `size`: the dimensions of the pixelData given.
        #[unsafe(method(textureWithData:size:))]
        #[unsafe(method_family = none)]
        pub unsafe fn textureWithData_size(pixel_data: &NSData, size: CGSize) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(textureWithData:size:flipped:))]
        #[unsafe(method_family = none)]
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
        #[unsafe(method(textureWithData:size:rowLength:alignment:))]
        #[unsafe(method_family = none)]
        pub unsafe fn textureWithData_size_rowLength_alignment(
            pixel_data: &NSData,
            size: CGSize,
            row_length: c_uint,
            alignment: c_uint,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-image")]
        #[cfg(not(target_os = "watchos"))]
        /// Create new texture by applying a CIFilter to an existing one. Any CIFilter that requires only a single "inputImage" and produces an "outputImage" is allowed.
        ///
        ///
        /// Parameter `filter`: the CI filter to apply in the copy.
        #[unsafe(method(textureByApplyingCIFilter:))]
        #[unsafe(method_family = none)]
        pub unsafe fn textureByApplyingCIFilter(&self, filter: &CIFilter) -> Retained<Self>;

        /// Create new texture by generating a normal map texture.
        #[unsafe(method(textureByGeneratingNormalMap))]
        #[unsafe(method_family = none)]
        pub unsafe fn textureByGeneratingNormalMap(&self) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Create new texture by generating a normal map texture.
        ///
        ///
        /// Parameter `smoothness`: the smooth level of the generated normal map.
        ///
        /// Parameter `contrast`: the scale applied to the generated normal map.
        #[unsafe(method(textureByGeneratingNormalMapWithSmoothness:contrast:))]
        #[unsafe(method_family = none)]
        pub unsafe fn textureByGeneratingNormalMapWithSmoothness_contrast(
            &self,
            smoothness: CGFloat,
            contrast: CGFloat,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Used to choose the area of the texture you want to display. The origin and size should both be in the range 0.0 - 1.0, values outside of this range produces unpredictable results. Defaults to the entire texture {(0,0) (1,1)}.
        #[unsafe(method(textureRect))]
        #[unsafe(method_family = none)]
        pub unsafe fn textureRect(&self) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        /// The size of the texture's bitmap data in points.
        #[unsafe(method(size))]
        #[unsafe(method_family = none)]
        pub unsafe fn size(&self) -> CGSize;

        /// The filtering mode the texture should use when not drawn at native size. Defaults to SKTextureFilteringLinear.
        #[unsafe(method(filteringMode))]
        #[unsafe(method_family = none)]
        pub unsafe fn filteringMode(&self) -> SKTextureFilteringMode;

        /// Setter for [`filteringMode`][Self::filteringMode].
        #[unsafe(method(setFilteringMode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFilteringMode(&self, filtering_mode: SKTextureFilteringMode);

        /// Request that the texture have mipmaps generated if possible. Only supported for power of 2 texture sizes.
        #[unsafe(method(usesMipmaps))]
        #[unsafe(method_family = none)]
        pub unsafe fn usesMipmaps(&self) -> bool;

        /// Setter for [`usesMipmaps`][Self::usesMipmaps].
        #[unsafe(method(setUsesMipmaps:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setUsesMipmaps(&self, uses_mipmaps: bool);

        #[cfg(feature = "objc2-core-graphics")]
        /// Convert the current SKTexture into a CGImageRef object
        #[unsafe(method(CGImage))]
        // required for soundness, method has `returns_retained` attribute.
        #[unsafe(method_family = copy)]
        pub unsafe fn CGImage(&self) -> Retained<CGImage>;

        #[cfg(feature = "block2")]
        /// Start a texture preload operation on an array of textures
        ///
        ///
        /// Parameter `textures`: an array of SKTextures to be preloaded
        ///
        /// Parameter `completionHandler`: will be called upon the preload completion
        #[unsafe(method(preloadTextures:withCompletionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn preloadTextures_withCompletionHandler(
            textures: &NSArray<SKTexture>,
            completion_handler: &block2::DynBlock<dyn Fn()>,
        );

        #[cfg(feature = "block2")]
        /// Request that this texture be loaded into vram on the next render update, with a callback handler.
        #[unsafe(method(preloadWithCompletionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn preloadWithCompletionHandler(
            &self,
            completion_handler: &block2::DynBlock<dyn Fn()>,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl SKTexture {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
