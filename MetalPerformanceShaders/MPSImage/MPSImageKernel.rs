//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscopyallocator?language=objc)
#[cfg(all(feature = "MPSKernel", feature = "block2"))]
pub type MPSCopyAllocator = *mut block2::Block<
    dyn Fn(
        NonNull<MPSKernel>,
        NonNull<ProtocolObject<dyn MTLCommandBuffer>>,
        NonNull<ProtocolObject<dyn MTLTexture>>,
    ) -> NonNull<ProtocolObject<dyn MTLTexture>>,
>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsunaryimagekernel?language=objc)
    #[unsafe(super(MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSKernel")]
    pub struct MPSUnaryImageKernel;
);

#[cfg(feature = "MPSKernel")]
unsafe impl NSCoding for MPSUnaryImageKernel {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSCopying for MPSUnaryImageKernel {}

#[cfg(feature = "MPSKernel")]
unsafe impl CopyingHelper for MPSUnaryImageKernel {
    type Result = Self;
}

#[cfg(feature = "MPSKernel")]
unsafe impl NSObjectProtocol for MPSUnaryImageKernel {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSSecureCoding for MPSUnaryImageKernel {}

extern_methods!(
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSUnaryImageKernel {
        #[cfg(feature = "MPSCoreTypes")]
        #[method(offset)]
        pub unsafe fn offset(&self) -> MPSOffset;

        #[cfg(feature = "MPSCoreTypes")]
        #[method(setOffset:)]
        pub unsafe fn setOffset(&self, offset: MPSOffset);

        #[method(clipRect)]
        pub unsafe fn clipRect(&self) -> MTLRegion;

        #[method(setClipRect:)]
        pub unsafe fn setClipRect(&self, clip_rect: MTLRegion);

        #[cfg(feature = "MPSCoreTypes")]
        #[method(edgeMode)]
        pub unsafe fn edgeMode(&self) -> MPSImageEdgeMode;

        #[cfg(feature = "MPSCoreTypes")]
        #[method(setEdgeMode:)]
        pub unsafe fn setEdgeMode(&self, edge_mode: MPSImageEdgeMode);

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "block2")]
        #[method(encodeToCommandBuffer:inPlaceTexture:fallbackCopyAllocator:)]
        pub unsafe fn encodeToCommandBuffer_inPlaceTexture_fallbackCopyAllocator(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            texture: NonNull<NonNull<ProtocolObject<dyn MTLTexture>>>,
            copy_allocator: MPSCopyAllocator,
        ) -> bool;

        #[method(encodeToCommandBuffer:sourceTexture:destinationTexture:)]
        pub unsafe fn encodeToCommandBuffer_sourceTexture_destinationTexture(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_texture: &ProtocolObject<dyn MTLTexture>,
            destination_texture: &ProtocolObject<dyn MTLTexture>,
        );

        #[cfg(feature = "MPSImage")]
        #[method(encodeToCommandBuffer:sourceImage:destinationImage:)]
        pub unsafe fn encodeToCommandBuffer_sourceImage_destinationImage(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_image: &MPSImage,
            destination_image: &MPSImage,
        );

        #[cfg(feature = "MPSCoreTypes")]
        #[method(sourceRegionForDestinationSize:)]
        pub unsafe fn sourceRegionForDestinationSize(&self, destination_size: MTLSize)
            -> MPSRegion;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSUnaryImageKernel {
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSUnaryImageKernel {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsbinaryimagekernel?language=objc)
    #[unsafe(super(MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSKernel")]
    pub struct MPSBinaryImageKernel;
);

#[cfg(feature = "MPSKernel")]
unsafe impl NSCoding for MPSBinaryImageKernel {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSCopying for MPSBinaryImageKernel {}

#[cfg(feature = "MPSKernel")]
unsafe impl CopyingHelper for MPSBinaryImageKernel {
    type Result = Self;
}

#[cfg(feature = "MPSKernel")]
unsafe impl NSObjectProtocol for MPSBinaryImageKernel {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSSecureCoding for MPSBinaryImageKernel {}

extern_methods!(
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSBinaryImageKernel {
        #[cfg(feature = "MPSCoreTypes")]
        #[method(primaryOffset)]
        pub unsafe fn primaryOffset(&self) -> MPSOffset;

        #[cfg(feature = "MPSCoreTypes")]
        #[method(setPrimaryOffset:)]
        pub unsafe fn setPrimaryOffset(&self, primary_offset: MPSOffset);

        #[cfg(feature = "MPSCoreTypes")]
        #[method(secondaryOffset)]
        pub unsafe fn secondaryOffset(&self) -> MPSOffset;

        #[cfg(feature = "MPSCoreTypes")]
        #[method(setSecondaryOffset:)]
        pub unsafe fn setSecondaryOffset(&self, secondary_offset: MPSOffset);

        #[cfg(feature = "MPSCoreTypes")]
        #[method(primaryEdgeMode)]
        pub unsafe fn primaryEdgeMode(&self) -> MPSImageEdgeMode;

        #[cfg(feature = "MPSCoreTypes")]
        #[method(setPrimaryEdgeMode:)]
        pub unsafe fn setPrimaryEdgeMode(&self, primary_edge_mode: MPSImageEdgeMode);

        #[cfg(feature = "MPSCoreTypes")]
        #[method(secondaryEdgeMode)]
        pub unsafe fn secondaryEdgeMode(&self) -> MPSImageEdgeMode;

        #[cfg(feature = "MPSCoreTypes")]
        #[method(setSecondaryEdgeMode:)]
        pub unsafe fn setSecondaryEdgeMode(&self, secondary_edge_mode: MPSImageEdgeMode);

        #[method(clipRect)]
        pub unsafe fn clipRect(&self) -> MTLRegion;

        #[method(setClipRect:)]
        pub unsafe fn setClipRect(&self, clip_rect: MTLRegion);

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "block2")]
        #[method(encodeToCommandBuffer:primaryTexture:inPlaceSecondaryTexture:fallbackCopyAllocator:)]
        pub unsafe fn encodeToCommandBuffer_primaryTexture_inPlaceSecondaryTexture_fallbackCopyAllocator(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            primary_texture: &ProtocolObject<dyn MTLTexture>,
            in_place_secondary_texture: NonNull<NonNull<ProtocolObject<dyn MTLTexture>>>,
            copy_allocator: MPSCopyAllocator,
        ) -> bool;

        #[cfg(feature = "block2")]
        #[method(encodeToCommandBuffer:inPlacePrimaryTexture:secondaryTexture:fallbackCopyAllocator:)]
        pub unsafe fn encodeToCommandBuffer_inPlacePrimaryTexture_secondaryTexture_fallbackCopyAllocator(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            in_place_primary_texture: NonNull<NonNull<ProtocolObject<dyn MTLTexture>>>,
            secondary_texture: &ProtocolObject<dyn MTLTexture>,
            copy_allocator: MPSCopyAllocator,
        ) -> bool;

        #[method(encodeToCommandBuffer:primaryTexture:secondaryTexture:destinationTexture:)]
        pub unsafe fn encodeToCommandBuffer_primaryTexture_secondaryTexture_destinationTexture(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            primary_texture: &ProtocolObject<dyn MTLTexture>,
            secondary_texture: &ProtocolObject<dyn MTLTexture>,
            destination_texture: &ProtocolObject<dyn MTLTexture>,
        );

        #[cfg(feature = "MPSImage")]
        #[method(encodeToCommandBuffer:primaryImage:secondaryImage:destinationImage:)]
        pub unsafe fn encodeToCommandBuffer_primaryImage_secondaryImage_destinationImage(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            primary_image: &MPSImage,
            secondary_image: &MPSImage,
            destination_image: &MPSImage,
        );

        #[cfg(feature = "MPSCoreTypes")]
        #[method(primarySourceRegionForDestinationSize:)]
        pub unsafe fn primarySourceRegionForDestinationSize(
            &self,
            destination_size: MTLSize,
        ) -> MPSRegion;

        #[cfg(feature = "MPSCoreTypes")]
        #[method(secondarySourceRegionForDestinationSize:)]
        pub unsafe fn secondarySourceRegionForDestinationSize(
            &self,
            destination_size: MTLSize,
        ) -> MPSRegion;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSBinaryImageKernel {
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSBinaryImageKernel {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);