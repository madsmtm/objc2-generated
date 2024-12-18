//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpstemporalweighting?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSTemporalWeighting(pub NSUInteger);
impl MPSTemporalWeighting {
    #[doc(alias = "MPSTemporalWeightingAverage")]
    pub const Average: Self = Self(0);
    #[doc(alias = "MPSTemporalWeightingExponentialMovingAverage")]
    pub const ExponentialMovingAverage: Self = Self(1);
}

unsafe impl Encode for MPSTemporalWeighting {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MPSTemporalWeighting {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpssvgf?language=objc)
    #[unsafe(super(MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSKernel")]
    pub struct MPSSVGF;
);

#[cfg(feature = "MPSKernel")]
unsafe impl NSCoding for MPSSVGF {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSCopying for MPSSVGF {}

#[cfg(feature = "MPSKernel")]
unsafe impl CopyingHelper for MPSSVGF {
    type Result = Self;
}

#[cfg(feature = "MPSKernel")]
unsafe impl NSObjectProtocol for MPSSVGF {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSSecureCoding for MPSSVGF {}

extern_methods!(
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSSVGF {
        #[method(depthWeight)]
        pub unsafe fn depthWeight(&self) -> c_float;

        #[method(setDepthWeight:)]
        pub unsafe fn setDepthWeight(&self, depth_weight: c_float);

        #[method(normalWeight)]
        pub unsafe fn normalWeight(&self) -> c_float;

        #[method(setNormalWeight:)]
        pub unsafe fn setNormalWeight(&self, normal_weight: c_float);

        #[method(luminanceWeight)]
        pub unsafe fn luminanceWeight(&self) -> c_float;

        #[method(setLuminanceWeight:)]
        pub unsafe fn setLuminanceWeight(&self, luminance_weight: c_float);

        #[method(temporalWeighting)]
        pub unsafe fn temporalWeighting(&self) -> MPSTemporalWeighting;

        #[method(setTemporalWeighting:)]
        pub unsafe fn setTemporalWeighting(&self, temporal_weighting: MPSTemporalWeighting);

        #[method(temporalReprojectionBlendFactor)]
        pub unsafe fn temporalReprojectionBlendFactor(&self) -> c_float;

        #[method(setTemporalReprojectionBlendFactor:)]
        pub unsafe fn setTemporalReprojectionBlendFactor(
            &self,
            temporal_reprojection_blend_factor: c_float,
        );

        #[method(reprojectionThreshold)]
        pub unsafe fn reprojectionThreshold(&self) -> c_float;

        #[method(setReprojectionThreshold:)]
        pub unsafe fn setReprojectionThreshold(&self, reprojection_threshold: c_float);

        #[method(minimumFramesForVarianceEstimation)]
        pub unsafe fn minimumFramesForVarianceEstimation(&self) -> NSUInteger;

        #[method(setMinimumFramesForVarianceEstimation:)]
        pub unsafe fn setMinimumFramesForVarianceEstimation(
            &self,
            minimum_frames_for_variance_estimation: NSUInteger,
        );

        #[method(varianceEstimationRadius)]
        pub unsafe fn varianceEstimationRadius(&self) -> NSUInteger;

        #[method(setVarianceEstimationRadius:)]
        pub unsafe fn setVarianceEstimationRadius(&self, variance_estimation_radius: NSUInteger);

        #[method(varianceEstimationSigma)]
        pub unsafe fn varianceEstimationSigma(&self) -> c_float;

        #[method(setVarianceEstimationSigma:)]
        pub unsafe fn setVarianceEstimationSigma(&self, variance_estimation_sigma: c_float);

        #[method(variancePrefilterSigma)]
        pub unsafe fn variancePrefilterSigma(&self) -> c_float;

        #[method(setVariancePrefilterSigma:)]
        pub unsafe fn setVariancePrefilterSigma(&self, variance_prefilter_sigma: c_float);

        #[method(variancePrefilterRadius)]
        pub unsafe fn variancePrefilterRadius(&self) -> NSUInteger;

        #[method(setVariancePrefilterRadius:)]
        pub unsafe fn setVariancePrefilterRadius(&self, variance_prefilter_radius: NSUInteger);

        #[method(bilateralFilterSigma)]
        pub unsafe fn bilateralFilterSigma(&self) -> c_float;

        #[method(setBilateralFilterSigma:)]
        pub unsafe fn setBilateralFilterSigma(&self, bilateral_filter_sigma: c_float);

        #[method(bilateralFilterRadius)]
        pub unsafe fn bilateralFilterRadius(&self) -> NSUInteger;

        #[method(setBilateralFilterRadius:)]
        pub unsafe fn setBilateralFilterRadius(&self, bilateral_filter_radius: NSUInteger);

        #[method(channelCount)]
        pub unsafe fn channelCount(&self) -> NSUInteger;

        #[method(setChannelCount:)]
        pub unsafe fn setChannelCount(&self, channel_count: NSUInteger);

        #[method(channelCount2)]
        pub unsafe fn channelCount2(&self) -> NSUInteger;

        #[method(setChannelCount2:)]
        pub unsafe fn setChannelCount2(&self, channel_count2: NSUInteger);

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

        #[method_id(@__retain_semantics Copy copyWithZone:device:)]
        pub unsafe fn copyWithZone_device(
            &self,
            zone: *mut NSZone,
            device: Option<&ProtocolObject<dyn MTLDevice>>,
        ) -> Retained<Self>;

        #[method(encodeWithCoder:)]
        pub unsafe fn encodeWithCoder(&self, coder: &NSCoder);

        #[method(encodeReprojectionToCommandBuffer:sourceTexture:previousTexture:destinationTexture:previousLuminanceMomentsTexture:destinationLuminanceMomentsTexture:previousFrameCountTexture:destinationFrameCountTexture:motionVectorTexture:depthNormalTexture:previousDepthNormalTexture:)]
        pub unsafe fn encodeReprojectionToCommandBuffer_sourceTexture_previousTexture_destinationTexture_previousLuminanceMomentsTexture_destinationLuminanceMomentsTexture_previousFrameCountTexture_destinationFrameCountTexture_motionVectorTexture_depthNormalTexture_previousDepthNormalTexture(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_texture: &ProtocolObject<dyn MTLTexture>,
            previous_texture: &ProtocolObject<dyn MTLTexture>,
            destination_texture: &ProtocolObject<dyn MTLTexture>,
            previous_luminance_moments_texture: &ProtocolObject<dyn MTLTexture>,
            destination_luminance_moments_texture: &ProtocolObject<dyn MTLTexture>,
            previous_frame_count_texture: &ProtocolObject<dyn MTLTexture>,
            destination_frame_count_texture: &ProtocolObject<dyn MTLTexture>,
            motion_vector_texture: Option<&ProtocolObject<dyn MTLTexture>>,
            depth_normal_texture: Option<&ProtocolObject<dyn MTLTexture>>,
            previous_depth_normal_texture: Option<&ProtocolObject<dyn MTLTexture>>,
        );

        #[method(encodeReprojectionToCommandBuffer:sourceTexture:previousTexture:destinationTexture:previousLuminanceMomentsTexture:destinationLuminanceMomentsTexture:sourceTexture2:previousTexture2:destinationTexture2:previousLuminanceMomentsTexture2:destinationLuminanceMomentsTexture2:previousFrameCountTexture:destinationFrameCountTexture:motionVectorTexture:depthNormalTexture:previousDepthNormalTexture:)]
        pub unsafe fn encodeReprojectionToCommandBuffer_sourceTexture_previousTexture_destinationTexture_previousLuminanceMomentsTexture_destinationLuminanceMomentsTexture_sourceTexture2_previousTexture2_destinationTexture2_previousLuminanceMomentsTexture2_destinationLuminanceMomentsTexture2_previousFrameCountTexture_destinationFrameCountTexture_motionVectorTexture_depthNormalTexture_previousDepthNormalTexture(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_texture: &ProtocolObject<dyn MTLTexture>,
            previous_texture: &ProtocolObject<dyn MTLTexture>,
            destination_texture: &ProtocolObject<dyn MTLTexture>,
            previous_luminance_moments_texture: &ProtocolObject<dyn MTLTexture>,
            destination_luminance_moments_texture: &ProtocolObject<dyn MTLTexture>,
            source_texture2: Option<&ProtocolObject<dyn MTLTexture>>,
            previous_texture2: Option<&ProtocolObject<dyn MTLTexture>>,
            destination_texture2: Option<&ProtocolObject<dyn MTLTexture>>,
            previous_luminance_moments_texture2: Option<&ProtocolObject<dyn MTLTexture>>,
            destination_luminance_moments_texture2: Option<&ProtocolObject<dyn MTLTexture>>,
            previous_frame_count_texture: &ProtocolObject<dyn MTLTexture>,
            destination_frame_count_texture: &ProtocolObject<dyn MTLTexture>,
            motion_vector_texture: Option<&ProtocolObject<dyn MTLTexture>>,
            depth_normal_texture: Option<&ProtocolObject<dyn MTLTexture>>,
            previous_depth_normal_texture: Option<&ProtocolObject<dyn MTLTexture>>,
        );

        #[method(encodeVarianceEstimationToCommandBuffer:sourceTexture:luminanceMomentsTexture:destinationTexture:frameCountTexture:depthNormalTexture:)]
        pub unsafe fn encodeVarianceEstimationToCommandBuffer_sourceTexture_luminanceMomentsTexture_destinationTexture_frameCountTexture_depthNormalTexture(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_texture: &ProtocolObject<dyn MTLTexture>,
            luminance_moments_texture: &ProtocolObject<dyn MTLTexture>,
            destination_texture: &ProtocolObject<dyn MTLTexture>,
            frame_count_texture: &ProtocolObject<dyn MTLTexture>,
            depth_normal_texture: Option<&ProtocolObject<dyn MTLTexture>>,
        );

        #[method(encodeVarianceEstimationToCommandBuffer:sourceTexture:luminanceMomentsTexture:destinationTexture:sourceTexture2:luminanceMomentsTexture2:destinationTexture2:frameCountTexture:depthNormalTexture:)]
        pub unsafe fn encodeVarianceEstimationToCommandBuffer_sourceTexture_luminanceMomentsTexture_destinationTexture_sourceTexture2_luminanceMomentsTexture2_destinationTexture2_frameCountTexture_depthNormalTexture(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_texture: &ProtocolObject<dyn MTLTexture>,
            luminance_moments_texture: &ProtocolObject<dyn MTLTexture>,
            destination_texture: &ProtocolObject<dyn MTLTexture>,
            source_texture2: Option<&ProtocolObject<dyn MTLTexture>>,
            luminance_moments_texture2: Option<&ProtocolObject<dyn MTLTexture>>,
            destination_texture2: Option<&ProtocolObject<dyn MTLTexture>>,
            frame_count_texture: &ProtocolObject<dyn MTLTexture>,
            depth_normal_texture: Option<&ProtocolObject<dyn MTLTexture>>,
        );

        #[method(encodeBilateralFilterToCommandBuffer:stepDistance:sourceTexture:destinationTexture:depthNormalTexture:)]
        pub unsafe fn encodeBilateralFilterToCommandBuffer_stepDistance_sourceTexture_destinationTexture_depthNormalTexture(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            step_distance: NSUInteger,
            source_texture: &ProtocolObject<dyn MTLTexture>,
            destination_texture: &ProtocolObject<dyn MTLTexture>,
            depth_normal_texture: &ProtocolObject<dyn MTLTexture>,
        );

        #[method(encodeBilateralFilterToCommandBuffer:stepDistance:sourceTexture:destinationTexture:sourceTexture2:destinationTexture2:depthNormalTexture:)]
        pub unsafe fn encodeBilateralFilterToCommandBuffer_stepDistance_sourceTexture_destinationTexture_sourceTexture2_destinationTexture2_depthNormalTexture(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            step_distance: NSUInteger,
            source_texture: &ProtocolObject<dyn MTLTexture>,
            destination_texture: &ProtocolObject<dyn MTLTexture>,
            source_texture2: Option<&ProtocolObject<dyn MTLTexture>>,
            destination_texture2: Option<&ProtocolObject<dyn MTLTexture>>,
            depth_normal_texture: &ProtocolObject<dyn MTLTexture>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSSVGF {
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
    unsafe impl MPSSVGF {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpssvgftextureallocator?language=objc)
    pub unsafe trait MPSSVGFTextureAllocator: NSObjectProtocol {
        #[method_id(@__retain_semantics Other textureWithPixelFormat:width:height:)]
        unsafe fn textureWithPixelFormat_width_height(
            &self,
            pixel_format: MTLPixelFormat,
            width: NSUInteger,
            height: NSUInteger,
        ) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;

        #[method(returnTexture:)]
        unsafe fn returnTexture(&self, texture: &ProtocolObject<dyn MTLTexture>);
    }

    unsafe impl ProtocolType for dyn MPSSVGFTextureAllocator {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpssvgfdefaulttextureallocator?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPSSVGFDefaultTextureAllocator;
);

unsafe impl MPSSVGFTextureAllocator for MPSSVGFDefaultTextureAllocator {}

unsafe impl NSObjectProtocol for MPSSVGFDefaultTextureAllocator {}

extern_methods!(
    unsafe impl MPSSVGFDefaultTextureAllocator {
        #[method_id(@__retain_semantics Other device)]
        pub unsafe fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        #[method(allocatedTextureCount)]
        pub unsafe fn allocatedTextureCount(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other textureWithPixelFormat:width:height:)]
        pub unsafe fn textureWithPixelFormat_width_height(
            &self,
            pixel_format: MTLPixelFormat,
            width: NSUInteger,
            height: NSUInteger,
        ) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;

        #[method(returnTexture:)]
        pub unsafe fn returnTexture(&self, texture: &ProtocolObject<dyn MTLTexture>);

        #[method(reset)]
        pub unsafe fn reset(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPSSVGFDefaultTextureAllocator {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpssvgfdenoiser?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPSSVGFDenoiser;
);

unsafe impl NSObjectProtocol for MPSSVGFDenoiser {}

extern_methods!(
    unsafe impl MPSSVGFDenoiser {
        #[cfg(feature = "MPSKernel")]
        #[method_id(@__retain_semantics Other svgf)]
        pub unsafe fn svgf(&self) -> Retained<MPSSVGF>;

        #[method_id(@__retain_semantics Other textureAllocator)]
        pub unsafe fn textureAllocator(
            &self,
        ) -> Retained<ProtocolObject<dyn MPSSVGFTextureAllocator>>;

        #[method(bilateralFilterIterations)]
        pub unsafe fn bilateralFilterIterations(&self) -> NSUInteger;

        #[method(setBilateralFilterIterations:)]
        pub unsafe fn setBilateralFilterIterations(&self, bilateral_filter_iterations: NSUInteger);

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[cfg(feature = "MPSKernel")]
        #[method_id(@__retain_semantics Init initWithSVGF:textureAllocator:)]
        pub unsafe fn initWithSVGF_textureAllocator(
            this: Allocated<Self>,
            svgf: &MPSSVGF,
            texture_allocator: &ProtocolObject<dyn MPSSVGFTextureAllocator>,
        ) -> Retained<Self>;

        #[method(clearTemporalHistory)]
        pub unsafe fn clearTemporalHistory(&self);

        #[method(releaseTemporaryTextures)]
        pub unsafe fn releaseTemporaryTextures(&self);

        #[method_id(@__retain_semantics Other encodeToCommandBuffer:sourceTexture:motionVectorTexture:depthNormalTexture:previousDepthNormalTexture:)]
        pub unsafe fn encodeToCommandBuffer_sourceTexture_motionVectorTexture_depthNormalTexture_previousDepthNormalTexture(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_texture: &ProtocolObject<dyn MTLTexture>,
            motion_vector_texture: Option<&ProtocolObject<dyn MTLTexture>>,
            depth_normal_texture: &ProtocolObject<dyn MTLTexture>,
            previous_depth_normal_texture: Option<&ProtocolObject<dyn MTLTexture>>,
        ) -> Retained<ProtocolObject<dyn MTLTexture>>;

        #[method(encodeToCommandBuffer:sourceTexture:destinationTexture:sourceTexture2:destinationTexture2:motionVectorTexture:depthNormalTexture:previousDepthNormalTexture:)]
        pub unsafe fn encodeToCommandBuffer_sourceTexture_destinationTexture_sourceTexture2_destinationTexture2_motionVectorTexture_depthNormalTexture_previousDepthNormalTexture(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_texture: &ProtocolObject<dyn MTLTexture>,
            destination_texture: &mut Retained<ProtocolObject<dyn MTLTexture>>,
            source_texture2: Option<&ProtocolObject<dyn MTLTexture>>,
            destination_texture2: Option<&mut Retained<ProtocolObject<dyn MTLTexture>>>,
            motion_vector_texture: Option<&ProtocolObject<dyn MTLTexture>>,
            depth_normal_texture: &ProtocolObject<dyn MTLTexture>,
            previous_depth_normal_texture: Option<&ProtocolObject<dyn MTLTexture>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPSSVGFDenoiser {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
