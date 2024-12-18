//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsnngradientstate?language=objc)
    #[unsafe(super(MPSState, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSState")]
    pub struct MPSNNGradientState;
);

#[cfg(feature = "MPSState")]
unsafe impl NSObjectProtocol for MPSNNGradientState {}

extern_methods!(
    #[cfg(feature = "MPSState")]
    unsafe impl MPSNNGradientState {}
);

extern_methods!(
    /// Methods declared on superclass `MPSState`
    #[cfg(feature = "MPSState")]
    unsafe impl MPSNNGradientState {
        #[method_id(@__retain_semantics Other temporaryStateWithCommandBuffer:bufferSize:)]
        pub unsafe fn temporaryStateWithCommandBuffer_bufferSize(
            cmd_buf: &ProtocolObject<dyn MTLCommandBuffer>,
            buffer_size: usize,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other temporaryStateWithCommandBuffer:textureDescriptor:)]
        pub unsafe fn temporaryStateWithCommandBuffer_textureDescriptor(
            cmd_buf: &ProtocolObject<dyn MTLCommandBuffer>,
            descriptor: &MTLTextureDescriptor,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other temporaryStateWithCommandBuffer:)]
        pub unsafe fn temporaryStateWithCommandBuffer(
            cmd_buf: &ProtocolObject<dyn MTLCommandBuffer>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:bufferSize:)]
        pub unsafe fn initWithDevice_bufferSize(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            buffer_size: usize,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:textureDescriptor:)]
        pub unsafe fn initWithDevice_textureDescriptor(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            descriptor: &MTLTextureDescriptor,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithResource:)]
        pub unsafe fn initWithResource(
            this: Allocated<Self>,
            resource: Option<&ProtocolObject<dyn MTLResource>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithDevice:resourceList:)]
        pub unsafe fn initWithDevice_resourceList(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            resource_list: &MPSStateResourceList,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other temporaryStateWithCommandBuffer:resourceList:)]
        pub unsafe fn temporaryStateWithCommandBuffer_resourceList(
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            resource_list: &MPSStateResourceList,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithResources:)]
        pub unsafe fn initWithResources(
            this: Allocated<Self>,
            resources: Option<&NSArray<ProtocolObject<dyn MTLResource>>>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MPSState")]
    unsafe impl MPSNNGradientState {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsnngradientstatebatch?language=objc)
#[cfg(feature = "MPSState")]
pub type MPSNNGradientStateBatch = NSArray<MPSNNGradientState>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsnnbinarygradientstate?language=objc)
    #[unsafe(super(MPSState, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSState")]
    pub struct MPSNNBinaryGradientState;
);

#[cfg(feature = "MPSState")]
unsafe impl NSObjectProtocol for MPSNNBinaryGradientState {}

extern_methods!(
    #[cfg(feature = "MPSState")]
    unsafe impl MPSNNBinaryGradientState {}
);

extern_methods!(
    /// Methods declared on superclass `MPSState`
    #[cfg(feature = "MPSState")]
    unsafe impl MPSNNBinaryGradientState {
        #[method_id(@__retain_semantics Other temporaryStateWithCommandBuffer:bufferSize:)]
        pub unsafe fn temporaryStateWithCommandBuffer_bufferSize(
            cmd_buf: &ProtocolObject<dyn MTLCommandBuffer>,
            buffer_size: usize,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other temporaryStateWithCommandBuffer:textureDescriptor:)]
        pub unsafe fn temporaryStateWithCommandBuffer_textureDescriptor(
            cmd_buf: &ProtocolObject<dyn MTLCommandBuffer>,
            descriptor: &MTLTextureDescriptor,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other temporaryStateWithCommandBuffer:)]
        pub unsafe fn temporaryStateWithCommandBuffer(
            cmd_buf: &ProtocolObject<dyn MTLCommandBuffer>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:bufferSize:)]
        pub unsafe fn initWithDevice_bufferSize(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            buffer_size: usize,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:textureDescriptor:)]
        pub unsafe fn initWithDevice_textureDescriptor(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            descriptor: &MTLTextureDescriptor,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithResource:)]
        pub unsafe fn initWithResource(
            this: Allocated<Self>,
            resource: Option<&ProtocolObject<dyn MTLResource>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithDevice:resourceList:)]
        pub unsafe fn initWithDevice_resourceList(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            resource_list: &MPSStateResourceList,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other temporaryStateWithCommandBuffer:resourceList:)]
        pub unsafe fn temporaryStateWithCommandBuffer_resourceList(
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            resource_list: &MPSStateResourceList,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithResources:)]
        pub unsafe fn initWithResources(
            this: Allocated<Self>,
            resources: Option<&NSArray<ProtocolObject<dyn MTLResource>>>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MPSState")]
    unsafe impl MPSNNBinaryGradientState {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsnnbinarygradientstatebatch?language=objc)
#[cfg(feature = "MPSState")]
pub type MPSNNBinaryGradientStateBatch = NSArray<MPSNNBinaryGradientState>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsnnmultiarygradientstate?language=objc)
    #[unsafe(super(MPSState, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSState")]
    pub struct MPSNNMultiaryGradientState;
);

#[cfg(feature = "MPSState")]
unsafe impl NSObjectProtocol for MPSNNMultiaryGradientState {}

extern_methods!(
    #[cfg(feature = "MPSState")]
    unsafe impl MPSNNMultiaryGradientState {}
);

extern_methods!(
    /// Methods declared on superclass `MPSState`
    #[cfg(feature = "MPSState")]
    unsafe impl MPSNNMultiaryGradientState {
        #[method_id(@__retain_semantics Other temporaryStateWithCommandBuffer:bufferSize:)]
        pub unsafe fn temporaryStateWithCommandBuffer_bufferSize(
            cmd_buf: &ProtocolObject<dyn MTLCommandBuffer>,
            buffer_size: usize,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other temporaryStateWithCommandBuffer:textureDescriptor:)]
        pub unsafe fn temporaryStateWithCommandBuffer_textureDescriptor(
            cmd_buf: &ProtocolObject<dyn MTLCommandBuffer>,
            descriptor: &MTLTextureDescriptor,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other temporaryStateWithCommandBuffer:)]
        pub unsafe fn temporaryStateWithCommandBuffer(
            cmd_buf: &ProtocolObject<dyn MTLCommandBuffer>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:bufferSize:)]
        pub unsafe fn initWithDevice_bufferSize(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            buffer_size: usize,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:textureDescriptor:)]
        pub unsafe fn initWithDevice_textureDescriptor(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            descriptor: &MTLTextureDescriptor,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithResource:)]
        pub unsafe fn initWithResource(
            this: Allocated<Self>,
            resource: Option<&ProtocolObject<dyn MTLResource>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithDevice:resourceList:)]
        pub unsafe fn initWithDevice_resourceList(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            resource_list: &MPSStateResourceList,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other temporaryStateWithCommandBuffer:resourceList:)]
        pub unsafe fn temporaryStateWithCommandBuffer_resourceList(
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            resource_list: &MPSStateResourceList,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithResources:)]
        pub unsafe fn initWithResources(
            this: Allocated<Self>,
            resources: Option<&NSArray<ProtocolObject<dyn MTLResource>>>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MPSState")]
    unsafe impl MPSNNMultiaryGradientState {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsnnmultiarygradientstatebatch?language=objc)
#[cfg(feature = "MPSState")]
pub type MPSNNMultiaryGradientStateBatch = NSArray<MPSNNMultiaryGradientState>;
