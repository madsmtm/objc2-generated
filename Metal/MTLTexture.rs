//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-io-surface")]
use objc2_io_surface::*;

use crate::*;

/// MTLTextureType describes the dimensionality of each image, and if multiple images are arranged into an array or cube.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtltexturetype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLTextureType(pub NSUInteger);
impl MTLTextureType {
    #[doc(alias = "MTLTextureType1D")]
    pub const Type1D: Self = Self(0);
    #[doc(alias = "MTLTextureType1DArray")]
    pub const Type1DArray: Self = Self(1);
    #[doc(alias = "MTLTextureType2D")]
    pub const Type2D: Self = Self(2);
    #[doc(alias = "MTLTextureType2DArray")]
    pub const Type2DArray: Self = Self(3);
    #[doc(alias = "MTLTextureType2DMultisample")]
    pub const Type2DMultisample: Self = Self(4);
    #[doc(alias = "MTLTextureTypeCube")]
    pub const TypeCube: Self = Self(5);
    #[doc(alias = "MTLTextureTypeCubeArray")]
    pub const TypeCubeArray: Self = Self(6);
    #[doc(alias = "MTLTextureType3D")]
    pub const Type3D: Self = Self(7);
    #[doc(alias = "MTLTextureType2DMultisampleArray")]
    pub const Type2DMultisampleArray: Self = Self(8);
    #[doc(alias = "MTLTextureTypeTextureBuffer")]
    pub const TypeTextureBuffer: Self = Self(9);
}

unsafe impl Encode for MTLTextureType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLTextureType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtltextureswizzle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLTextureSwizzle(pub u8);
impl MTLTextureSwizzle {
    #[doc(alias = "MTLTextureSwizzleZero")]
    pub const Zero: Self = Self(0);
    #[doc(alias = "MTLTextureSwizzleOne")]
    pub const One: Self = Self(1);
    #[doc(alias = "MTLTextureSwizzleRed")]
    pub const Red: Self = Self(2);
    #[doc(alias = "MTLTextureSwizzleGreen")]
    pub const Green: Self = Self(3);
    #[doc(alias = "MTLTextureSwizzleBlue")]
    pub const Blue: Self = Self(4);
    #[doc(alias = "MTLTextureSwizzleAlpha")]
    pub const Alpha: Self = Self(5);
}

unsafe impl Encode for MTLTextureSwizzle {
    const ENCODING: Encoding = u8::ENCODING;
}

unsafe impl RefEncode for MTLTextureSwizzle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtltextureswizzlechannels?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLTextureSwizzleChannels {
    pub red: MTLTextureSwizzle,
    pub green: MTLTextureSwizzle,
    pub blue: MTLTextureSwizzle,
    pub alpha: MTLTextureSwizzle,
}

unsafe impl Encode for MTLTextureSwizzleChannels {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <MTLTextureSwizzle>::ENCODING,
            <MTLTextureSwizzle>::ENCODING,
            <MTLTextureSwizzle>::ENCODING,
            <MTLTextureSwizzle>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTLTextureSwizzleChannels {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

impl MTLTextureSwizzleChannels {
    // TODO: pub fn MTLTextureSwizzleChannelsMake(r: MTLTextureSwizzle,g: MTLTextureSwizzle,b: MTLTextureSwizzle,a: MTLTextureSwizzle,) -> MTLTextureSwizzleChannels;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlsharedtexturehandle?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLSharedTextureHandle;
);

extern_conformance!(
    unsafe impl NSCoding for MTLSharedTextureHandle {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLSharedTextureHandle {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for MTLSharedTextureHandle {}
);

impl MTLSharedTextureHandle {
    extern_methods!(
        #[cfg(feature = "MTLDevice")]
        /// The device this texture was created against.
        ///
        /// This shared texture handle can only be used with this device.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        pub fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// A copy of the original texture's label property, if any
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        pub fn label(&self) -> Option<Retained<NSString>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLSharedTextureHandle {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

/// MTLTextureUsage declares how the texture will be used over its lifetime (bitwise OR for multiple uses).
///
/// This information may be used by the driver to make optimization decisions.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtltextureusage?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLTextureUsage(pub NSUInteger);
bitflags::bitflags! {
    impl MTLTextureUsage: NSUInteger {
        #[doc(alias = "MTLTextureUsageUnknown")]
        const Unknown = 0x0000;
        #[doc(alias = "MTLTextureUsageShaderRead")]
        const ShaderRead = 0x0001;
        #[doc(alias = "MTLTextureUsageShaderWrite")]
        const ShaderWrite = 0x0002;
        #[doc(alias = "MTLTextureUsageRenderTarget")]
        const RenderTarget = 0x0004;
        #[doc(alias = "MTLTextureUsagePixelFormatView")]
        const PixelFormatView = 0x0010;
        #[doc(alias = "MTLTextureUsageShaderAtomic")]
        const ShaderAtomic = 0x0020;
    }
}

unsafe impl Encode for MTLTextureUsage {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLTextureUsage {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtltexturecompressiontype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLTextureCompressionType(pub NSInteger);
impl MTLTextureCompressionType {
    #[doc(alias = "MTLTextureCompressionTypeLossless")]
    pub const Lossless: Self = Self(0);
    #[doc(alias = "MTLTextureCompressionTypeLossy")]
    pub const Lossy: Self = Self(1);
}

unsafe impl Encode for MTLTextureCompressionType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MTLTextureCompressionType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtltexturedescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLTextureDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLTextureDescriptor {}
);

unsafe impl CopyingHelper for MTLTextureDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLTextureDescriptor {}
);

impl MTLTextureDescriptor {
    extern_methods!(
        #[cfg(feature = "MTLPixelFormat")]
        /// Create a TextureDescriptor for a common 2D texture.
        #[unsafe(method(texture2DDescriptorWithPixelFormat:width:height:mipmapped:))]
        #[unsafe(method_family = none)]
        pub unsafe fn texture2DDescriptorWithPixelFormat_width_height_mipmapped(
            pixel_format: MTLPixelFormat,
            width: NSUInteger,
            height: NSUInteger,
            mipmapped: bool,
        ) -> Retained<MTLTextureDescriptor>;

        #[cfg(feature = "MTLPixelFormat")]
        /// Create a TextureDescriptor for a common Cube texture.
        #[unsafe(method(textureCubeDescriptorWithPixelFormat:size:mipmapped:))]
        #[unsafe(method_family = none)]
        pub unsafe fn textureCubeDescriptorWithPixelFormat_size_mipmapped(
            pixel_format: MTLPixelFormat,
            size: NSUInteger,
            mipmapped: bool,
        ) -> Retained<MTLTextureDescriptor>;

        #[cfg(all(feature = "MTLPixelFormat", feature = "MTLResource"))]
        /// Create a TextureDescriptor for a common texture buffer.
        #[unsafe(method(textureBufferDescriptorWithPixelFormat:width:resourceOptions:usage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn textureBufferDescriptorWithPixelFormat_width_resourceOptions_usage(
            pixel_format: MTLPixelFormat,
            width: NSUInteger,
            resource_options: MTLResourceOptions,
            usage: MTLTextureUsage,
        ) -> Retained<MTLTextureDescriptor>;

        /// The overall type of the texture to be created. The default value is MTLTextureType2D.
        #[unsafe(method(textureType))]
        #[unsafe(method_family = none)]
        pub fn textureType(&self) -> MTLTextureType;

        /// Setter for [`textureType`][Self::textureType].
        #[unsafe(method(setTextureType:))]
        #[unsafe(method_family = none)]
        pub fn setTextureType(&self, texture_type: MTLTextureType);

        #[cfg(feature = "MTLPixelFormat")]
        /// The pixel format to use when allocating this texture. This is also the pixel format that will be used to when the caller writes or reads pixels from this texture. The default value is MTLPixelFormatRGBA8Unorm.
        #[unsafe(method(pixelFormat))]
        #[unsafe(method_family = none)]
        pub fn pixelFormat(&self) -> MTLPixelFormat;

        #[cfg(feature = "MTLPixelFormat")]
        /// Setter for [`pixelFormat`][Self::pixelFormat].
        #[unsafe(method(setPixelFormat:))]
        #[unsafe(method_family = none)]
        pub fn setPixelFormat(&self, pixel_format: MTLPixelFormat);

        /// The width of the texture to create. The default value is 1.
        #[unsafe(method(width))]
        #[unsafe(method_family = none)]
        pub fn width(&self) -> NSUInteger;

        /// Setter for [`width`][Self::width].
        #[unsafe(method(setWidth:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setWidth(&self, width: NSUInteger);

        /// The height of the texture to create. The default value is 1.
        ///
        /// height If allocating a 1D texture, height must be 1.
        #[unsafe(method(height))]
        #[unsafe(method_family = none)]
        pub fn height(&self) -> NSUInteger;

        /// Setter for [`height`][Self::height].
        #[unsafe(method(setHeight:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setHeight(&self, height: NSUInteger);

        /// The depth of the texture to create. The default value is 1.
        ///
        /// depth When allocating any texture types other than 3D, depth must be 1.
        #[unsafe(method(depth))]
        #[unsafe(method_family = none)]
        pub fn depth(&self) -> NSUInteger;

        /// Setter for [`depth`][Self::depth].
        #[unsafe(method(setDepth:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDepth(&self, depth: NSUInteger);

        /// The number of mipmap levels to allocate. The default value is 1.
        ///
        /// When creating Buffer and Multisample textures, mipmapLevelCount must be 1.
        #[unsafe(method(mipmapLevelCount))]
        #[unsafe(method_family = none)]
        pub fn mipmapLevelCount(&self) -> NSUInteger;

        /// Setter for [`mipmapLevelCount`][Self::mipmapLevelCount].
        #[unsafe(method(setMipmapLevelCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMipmapLevelCount(&self, mipmap_level_count: NSUInteger);

        /// The number of samples in the texture to create. The default value is 1.
        ///
        /// When creating Buffer textures sampleCount must be 1. Implementations may round sample counts up to the next supported value.
        #[unsafe(method(sampleCount))]
        #[unsafe(method_family = none)]
        pub fn sampleCount(&self) -> NSUInteger;

        /// Setter for [`sampleCount`][Self::sampleCount].
        #[unsafe(method(setSampleCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSampleCount(&self, sample_count: NSUInteger);

        /// The number of array elements to allocate. The default value is 1.
        ///
        /// When allocating any non-Array texture type, arrayLength has to be 1. Otherwise it must be set to something greater than 1 and less than 2048.
        #[unsafe(method(arrayLength))]
        #[unsafe(method_family = none)]
        pub fn arrayLength(&self) -> NSUInteger;

        /// Setter for [`arrayLength`][Self::arrayLength].
        #[unsafe(method(setArrayLength:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setArrayLength(&self, array_length: NSUInteger);

        #[cfg(feature = "MTLResource")]
        /// Options to control memory allocation parameters, etc.
        ///
        /// Contains a packed set of the storageMode, cpuCacheMode and hazardTrackingMode properties.
        #[unsafe(method(resourceOptions))]
        #[unsafe(method_family = none)]
        pub fn resourceOptions(&self) -> MTLResourceOptions;

        #[cfg(feature = "MTLResource")]
        /// Setter for [`resourceOptions`][Self::resourceOptions].
        #[unsafe(method(setResourceOptions:))]
        #[unsafe(method_family = none)]
        pub fn setResourceOptions(&self, resource_options: MTLResourceOptions);

        #[cfg(feature = "MTLResource")]
        /// Options to specify CPU cache mode of texture resource.
        #[unsafe(method(cpuCacheMode))]
        #[unsafe(method_family = none)]
        pub fn cpuCacheMode(&self) -> MTLCPUCacheMode;

        #[cfg(feature = "MTLResource")]
        /// Setter for [`cpuCacheMode`][Self::cpuCacheMode].
        #[unsafe(method(setCpuCacheMode:))]
        #[unsafe(method_family = none)]
        pub fn setCpuCacheMode(&self, cpu_cache_mode: MTLCPUCacheMode);

        #[cfg(feature = "MTLResource")]
        /// To specify storage mode of texture resource.
        #[unsafe(method(storageMode))]
        #[unsafe(method_family = none)]
        pub fn storageMode(&self) -> MTLStorageMode;

        #[cfg(feature = "MTLResource")]
        /// Setter for [`storageMode`][Self::storageMode].
        #[unsafe(method(setStorageMode:))]
        #[unsafe(method_family = none)]
        pub fn setStorageMode(&self, storage_mode: MTLStorageMode);

        #[cfg(feature = "MTLResource")]
        /// Set hazard tracking mode for the texture. The default value is MTLHazardTrackingModeDefault.
        ///
        /// For resources created from the device, MTLHazardTrackingModeDefault is treated as MTLHazardTrackingModeTracked.
        /// For resources created on a heap, MTLHazardTrackingModeDefault is treated as the hazardTrackingMode of the heap itself.
        /// In either case, it is possible to opt-out of hazard tracking by setting MTLHazardTrackingModeUntracked.
        /// It is not possible to opt-in to hazard tracking on a heap that itself is not hazard tracked.
        /// For optimal performance, perform hazard tracking manually through MTLFence or MTLEvent instead.
        #[unsafe(method(hazardTrackingMode))]
        #[unsafe(method_family = none)]
        pub fn hazardTrackingMode(&self) -> MTLHazardTrackingMode;

        #[cfg(feature = "MTLResource")]
        /// Setter for [`hazardTrackingMode`][Self::hazardTrackingMode].
        #[unsafe(method(setHazardTrackingMode:))]
        #[unsafe(method_family = none)]
        pub fn setHazardTrackingMode(&self, hazard_tracking_mode: MTLHazardTrackingMode);

        /// Description of texture usage
        #[unsafe(method(usage))]
        #[unsafe(method_family = none)]
        pub fn usage(&self) -> MTLTextureUsage;

        /// Setter for [`usage`][Self::usage].
        #[unsafe(method(setUsage:))]
        #[unsafe(method_family = none)]
        pub fn setUsage(&self, usage: MTLTextureUsage);

        /// Allow GPU-optimization for the contents of this texture. The default value is true.
        ///
        /// Useful for opting-out of GPU-optimization when implicit optimization (e.g. RT writes) is regressing CPU-read-back performance. See the documentation for optimizeContentsForGPUAccess: and optimizeContentsForCPUAccess: APIs.
        #[unsafe(method(allowGPUOptimizedContents))]
        #[unsafe(method_family = none)]
        pub fn allowGPUOptimizedContents(&self) -> bool;

        /// Setter for [`allowGPUOptimizedContents`][Self::allowGPUOptimizedContents].
        #[unsafe(method(setAllowGPUOptimizedContents:))]
        #[unsafe(method_family = none)]
        pub fn setAllowGPUOptimizedContents(&self, allow_gpu_optimized_contents: bool);

        /// Controls how the texture contents will be compressed when written to by the GPU. Compression can be used to reduce the bandwidth usage and storage requirements of a texture.
        ///
        /// The default compression type is lossless, meaning that no loss of precision will occur when the texture content is modified.
        /// Losslessly compressed textures may benefit from reduced bandwidth usage when regions of correlated color values are written, but do not benefit from reduced storage requirements.
        /// Enabling lossy compression for textures that can tolerate some precision loss will guarantee both reduced bandwidth usage and reduced storage requirements.
        /// The amount of precision loss depends on the color values stored; regions with correlated color values can be represented with limited to no precision loss, whereas regions with unrelated color values suffer more precision loss.
        /// Enabling lossy compression requires both storageMode == MTLStorageModePrivate, allowGPUOptimizedContents == YES, and cannot be combined with either MTLTextureUsagePixelFormatView, MTLTextureUsageShaderWrite, MTLTextureUsageShaderAtomic, MTLTextureType1D(Array) or MTLTextureTypeTextureBuffer.
        /// Moreover, not all MTLPixelFormat are supported with lossy compression, verify that the MTLDevice's GPU family supports the lossy compression feature for the pixelFormat requested.
        /// Set allowGPUOptimizedContents to NO to opt out of both lossless and lossy compression; such textures do not benefit from either reduced bandwidth usage or reduced storage requirements, but have predictable CPU readback performance.
        #[unsafe(method(compressionType))]
        #[unsafe(method_family = none)]
        pub unsafe fn compressionType(&self) -> MTLTextureCompressionType;

        /// Setter for [`compressionType`][Self::compressionType].
        #[unsafe(method(setCompressionType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCompressionType(&self, compression_type: MTLTextureCompressionType);

        /// Channel swizzle to use when reading or sampling from the texture, the default value is MTLTextureSwizzleChannelsDefault.
        #[unsafe(method(swizzle))]
        #[unsafe(method_family = none)]
        pub fn swizzle(&self) -> MTLTextureSwizzleChannels;

        /// Setter for [`swizzle`][Self::swizzle].
        #[unsafe(method(setSwizzle:))]
        #[unsafe(method_family = none)]
        pub fn setSwizzle(&self, swizzle: MTLTextureSwizzleChannels);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLTextureDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_protocol!(
    /// MTLTexture represents a collection of 1D, 2D, or 3D images.
    ///
    /// Each image in a texture is a 1D, 2D, 2DMultisample, or 3D image. The texture contains one or more images arranged in a mipmap stack. If there are multiple mipmap stacks, each one is referred to as a slice of the texture. 1D, 2D, 2DMultisample, and 3D textures have a single slice. In 1DArray and 2DArray textures, every slice is an array element. A Cube texture always has 6 slices, one for each face. In a CubeArray texture, each set of six slices is one element in the array.
    ///
    /// Most APIs that operate on individual images in a texture address those images via a tuple of a Slice, and Mipmap Level within that slice.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtltexture?language=objc)
    #[cfg(all(feature = "MTLAllocation", feature = "MTLResource"))]
    pub unsafe trait MTLTexture: MTLResource {
        /// The resource this texture was created from. It may be a texture or a buffer. If this texture is not reusing storage of another MTLResource, then nil is returned.
        #[deprecated = "Use parentTexture or buffer instead"]
        #[unsafe(method(rootResource))]
        #[unsafe(method_family = none)]
        fn rootResource(&self) -> Option<Retained<ProtocolObject<dyn MTLResource>>>;

        /// The texture this texture view was created from, or nil if this is not a texture view or it was not created from a texture.
        #[unsafe(method(parentTexture))]
        #[unsafe(method_family = none)]
        fn parentTexture(&self) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;

        /// The base level of the texture this texture view was created from, or 0 if this is not a texture view.
        #[unsafe(method(parentRelativeLevel))]
        #[unsafe(method_family = none)]
        fn parentRelativeLevel(&self) -> NSUInteger;

        /// The base slice of the texture this texture view was created from, or 0 if this is not a texture view.
        #[unsafe(method(parentRelativeSlice))]
        #[unsafe(method_family = none)]
        fn parentRelativeSlice(&self) -> NSUInteger;

        #[cfg(feature = "MTLBuffer")]
        /// The buffer this texture view was created from, or nil if this is not a texture view or it was not created from a buffer.
        #[unsafe(method(buffer))]
        #[unsafe(method_family = none)]
        fn buffer(&self) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// The offset of the buffer this texture view was created from, or 0 if this is not a texture view.
        #[unsafe(method(bufferOffset))]
        #[unsafe(method_family = none)]
        fn bufferOffset(&self) -> NSUInteger;

        /// The bytesPerRow of the buffer this texture view was created from, or 0 if this is not a texture view.
        #[unsafe(method(bufferBytesPerRow))]
        #[unsafe(method_family = none)]
        fn bufferBytesPerRow(&self) -> NSUInteger;

        #[cfg(feature = "objc2-io-surface")]
        /// If this texture was created from an IOSurface, this returns a reference to that IOSurface. iosurface is nil if this texture was not created from an IOSurface.
        #[unsafe(method(iosurface))]
        #[unsafe(method_family = none)]
        unsafe fn iosurface(&self) -> Option<Retained<IOSurfaceRef>>;

        /// If this texture was created from an IOSurface, this returns the plane of the IOSurface from which the texture was created. iosurfacePlane is 0 if this texture was not created from an IOSurface.
        #[unsafe(method(iosurfacePlane))]
        #[unsafe(method_family = none)]
        fn iosurfacePlane(&self) -> NSUInteger;

        /// The type of this texture.
        #[unsafe(method(textureType))]
        #[unsafe(method_family = none)]
        fn textureType(&self) -> MTLTextureType;

        #[cfg(feature = "MTLPixelFormat")]
        /// The MTLPixelFormat that is used to interpret this texture's contents.
        #[unsafe(method(pixelFormat))]
        #[unsafe(method_family = none)]
        fn pixelFormat(&self) -> MTLPixelFormat;

        /// The width of the MTLTexture instance in pixels.
        #[unsafe(method(width))]
        #[unsafe(method_family = none)]
        fn width(&self) -> NSUInteger;

        /// The height of the MTLTexture instance in pixels.
        ///
        /// . height is 1 if the texture is 1D.
        #[unsafe(method(height))]
        #[unsafe(method_family = none)]
        fn height(&self) -> NSUInteger;

        /// The depth of this MTLTexture instance in pixels.
        ///
        /// If this MTLTexture is not a 3D texture, the depth is 1
        #[unsafe(method(depth))]
        #[unsafe(method_family = none)]
        fn depth(&self) -> NSUInteger;

        /// The number of mipmap levels in each slice of this MTLTexture.
        #[unsafe(method(mipmapLevelCount))]
        #[unsafe(method_family = none)]
        fn mipmapLevelCount(&self) -> NSUInteger;

        /// The number of samples in each pixel of this MTLTexture.
        ///
        /// If this texture is any type other than 2DMultisample, samples is 1.
        #[unsafe(method(sampleCount))]
        #[unsafe(method_family = none)]
        fn sampleCount(&self) -> NSUInteger;

        /// The number of array elements in this MTLTexture.
        ///
        /// For non-Array texture types, arrayLength is 1.
        #[unsafe(method(arrayLength))]
        #[unsafe(method_family = none)]
        fn arrayLength(&self) -> NSUInteger;

        /// Description of texture usage.
        #[unsafe(method(usage))]
        #[unsafe(method_family = none)]
        fn usage(&self) -> MTLTextureUsage;

        /// If YES, this texture can be shared with other processes.
        ///
        /// Texture can be shared across process addres space boundaries through use of sharedTextureHandle and XPC.
        #[unsafe(method(isShareable))]
        #[unsafe(method_family = none)]
        fn isShareable(&self) -> bool;

        /// If YES, this texture can only be used with a MTLAttachmentDescriptor, and cannot be used as a texture argument for MTLRenderCommandEncoder, MTLBlitCommandEncoder, or MTLComputeCommandEncoder. Furthermore, when this property's value is YES, readPixels/writePixels may not be used with this texture.
        ///
        /// Textures obtained from CAMetalDrawables may have this property set to YES, depending on the value of frameBufferOnly passed to their parent CAMetalLayer. Textures created directly by the application will not have any restrictions.
        #[unsafe(method(isFramebufferOnly))]
        #[unsafe(method_family = none)]
        fn isFramebufferOnly(&self) -> bool;

        /// For sparse textures this property returns index of first mipmap that is packed in tail.
        /// Mapping this mipmap level will map all subsequent mipmap levels.
        #[optional]
        #[unsafe(method(firstMipmapInTail))]
        #[unsafe(method_family = none)]
        fn firstMipmapInTail(&self) -> NSUInteger;

        /// Amount of memory in bytes required to map sparse texture tail.
        #[optional]
        #[unsafe(method(tailSizeInBytes))]
        #[unsafe(method_family = none)]
        fn tailSizeInBytes(&self) -> NSUInteger;

        #[optional]
        #[unsafe(method(isSparse))]
        #[unsafe(method_family = none)]
        fn isSparse(&self) -> bool;

        /// Allow GPU-optimization for the contents texture. The default value is true.
        ///
        /// Useful for opting-out of GPU-optimization when implicit optimization (e.g. RT writes) is regressing CPU-read-back performance. See the documentation for optimizeContentsForGPUAccess: and optimizeContentsForCPUAccess: APIs.
        #[unsafe(method(allowGPUOptimizedContents))]
        #[unsafe(method_family = none)]
        fn allowGPUOptimizedContents(&self) -> bool;

        /// Returns the compression type of the texture
        ///
        /// See the compressionType property on MTLTextureDescriptor
        #[unsafe(method(compressionType))]
        #[unsafe(method_family = none)]
        unsafe fn compressionType(&self) -> MTLTextureCompressionType;

        #[cfg(feature = "MTLTypes")]
        /// Handle of the GPU resource suitable for storing in an Argument Buffer
        #[unsafe(method(gpuResourceID))]
        #[unsafe(method_family = none)]
        unsafe fn gpuResourceID(&self) -> MTLResourceID;

        #[cfg(feature = "MTLTypes")]
        /// Copies a block of pixels from a texture slice into the application's memory.
        #[unsafe(method(getBytes:bytesPerRow:bytesPerImage:fromRegion:mipmapLevel:slice:))]
        #[unsafe(method_family = none)]
        unsafe fn getBytes_bytesPerRow_bytesPerImage_fromRegion_mipmapLevel_slice(
            &self,
            pixel_bytes: NonNull<c_void>,
            bytes_per_row: NSUInteger,
            bytes_per_image: NSUInteger,
            region: MTLRegion,
            level: NSUInteger,
            slice: NSUInteger,
        );

        #[cfg(feature = "MTLTypes")]
        /// Copy a block of pixel data from the caller's pointer into a texture slice.
        #[unsafe(method(replaceRegion:mipmapLevel:slice:withBytes:bytesPerRow:bytesPerImage:))]
        #[unsafe(method_family = none)]
        unsafe fn replaceRegion_mipmapLevel_slice_withBytes_bytesPerRow_bytesPerImage(
            &self,
            region: MTLRegion,
            level: NSUInteger,
            slice: NSUInteger,
            pixel_bytes: NonNull<c_void>,
            bytes_per_row: NSUInteger,
            bytes_per_image: NSUInteger,
        );

        #[cfg(feature = "MTLTypes")]
        /// Convenience for getBytes:bytesPerRow:bytesPerImage:fromRegion:mipmapLevel:slice: that doesn't require slice related arguments
        #[unsafe(method(getBytes:bytesPerRow:fromRegion:mipmapLevel:))]
        #[unsafe(method_family = none)]
        unsafe fn getBytes_bytesPerRow_fromRegion_mipmapLevel(
            &self,
            pixel_bytes: NonNull<c_void>,
            bytes_per_row: NSUInteger,
            region: MTLRegion,
            level: NSUInteger,
        );

        #[cfg(feature = "MTLTypes")]
        /// Convenience for replaceRegion:mipmapLevel:slice:withBytes:bytesPerRow:bytesPerImage: that doesn't require slice related arguments
        #[unsafe(method(replaceRegion:mipmapLevel:withBytes:bytesPerRow:))]
        #[unsafe(method_family = none)]
        unsafe fn replaceRegion_mipmapLevel_withBytes_bytesPerRow(
            &self,
            region: MTLRegion,
            level: NSUInteger,
            pixel_bytes: NonNull<c_void>,
            bytes_per_row: NSUInteger,
        );

        #[cfg(feature = "MTLPixelFormat")]
        /// Create a new texture which shares the same storage as the source texture, but with a different (but compatible) pixel format.
        #[unsafe(method(newTextureViewWithPixelFormat:))]
        #[unsafe(method_family = new)]
        fn newTextureViewWithPixelFormat(
            &self,
            pixel_format: MTLPixelFormat,
        ) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;

        #[cfg(feature = "MTLPixelFormat")]
        /// Create a new texture which shares the same storage as the source texture, but with a different (but compatible) pixel format, texture type, levels and slices.
        #[unsafe(method(newTextureViewWithPixelFormat:textureType:levels:slices:))]
        #[unsafe(method_family = new)]
        unsafe fn newTextureViewWithPixelFormat_textureType_levels_slices(
            &self,
            pixel_format: MTLPixelFormat,
            texture_type: MTLTextureType,
            level_range: NSRange,
            slice_range: NSRange,
        ) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;

        /// Create a new texture handle, that can be shared across process addres space boundaries.
        #[unsafe(method(newSharedTextureHandle))]
        #[unsafe(method_family = new)]
        fn newSharedTextureHandle(&self) -> Option<Retained<MTLSharedTextureHandle>>;

        /// For Metal texture objects that are remote views, this returns the texture associated with the storage on the originating device.
        #[unsafe(method(remoteStorageTexture))]
        #[unsafe(method_family = none)]
        fn remoteStorageTexture(&self) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;

        #[cfg(feature = "MTLDevice")]
        /// On Metal devices that support peer to peer transfers, this method is used to create a remote texture view on another device
        /// within the peer group.  The receiver must use MTLStorageModePrivate or be backed by an IOSurface.
        #[unsafe(method(newRemoteTextureViewForDevice:))]
        #[unsafe(method_family = new)]
        unsafe fn newRemoteTextureViewForDevice(
            &self,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;

        /// The channel swizzle used when reading or sampling from this texture
        #[unsafe(method(swizzle))]
        #[unsafe(method_family = none)]
        fn swizzle(&self) -> MTLTextureSwizzleChannels;

        #[cfg(feature = "MTLPixelFormat")]
        /// Create a new texture which shares the same storage as the source texture, but with a different (but compatible) pixel format, texture type, levels, slices and swizzle.
        #[unsafe(method(newTextureViewWithPixelFormat:textureType:levels:slices:swizzle:))]
        #[unsafe(method_family = new)]
        unsafe fn newTextureViewWithPixelFormat_textureType_levels_slices_swizzle(
            &self,
            pixel_format: MTLPixelFormat,
            texture_type: MTLTextureType,
            level_range: NSRange,
            slice_range: NSRange,
            swizzle: MTLTextureSwizzleChannels,
        ) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;
    }
);
