//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimagehistogram?language=objc)
    #[unsafe(super(MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSKernel")]
    pub struct MPSImageHistogram;
);

#[cfg(feature = "MPSKernel")]
unsafe impl NSCoding for MPSImageHistogram {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSCopying for MPSImageHistogram {}

#[cfg(feature = "MPSKernel")]
unsafe impl CopyingHelper for MPSImageHistogram {
    type Result = Self;
}

#[cfg(feature = "MPSKernel")]
unsafe impl NSObjectProtocol for MPSImageHistogram {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSSecureCoding for MPSImageHistogram {}

extern_methods!(
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSImageHistogram {
        #[method(clipRectSource)]
        pub unsafe fn clipRectSource(&self) -> MTLRegion;

        #[method(setClipRectSource:)]
        pub unsafe fn setClipRectSource(&self, clip_rect_source: MTLRegion);

        #[method(zeroHistogram)]
        pub unsafe fn zeroHistogram(&self) -> bool;

        #[method(setZeroHistogram:)]
        pub unsafe fn setZeroHistogram(&self, zero_histogram: bool);

        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        #[method(encodeToCommandBuffer:sourceTexture:histogram:histogramOffset:)]
        pub unsafe fn encodeToCommandBuffer_sourceTexture_histogram_histogramOffset(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source: &ProtocolObject<dyn MTLTexture>,
            histogram: &ProtocolObject<dyn MTLBuffer>,
            histogram_offset: NSUInteger,
        );

        #[method(histogramSizeForSourceFormat:)]
        pub unsafe fn histogramSizeForSourceFormat(&self, source_format: MTLPixelFormat) -> usize;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSImageHistogram {
        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

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
    unsafe impl MPSImageHistogram {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimagenormalizedhistogram?language=objc)
    #[unsafe(super(MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSKernel")]
    pub struct MPSImageNormalizedHistogram;
);

#[cfg(feature = "MPSKernel")]
unsafe impl NSCoding for MPSImageNormalizedHistogram {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSCopying for MPSImageNormalizedHistogram {}

#[cfg(feature = "MPSKernel")]
unsafe impl CopyingHelper for MPSImageNormalizedHistogram {
    type Result = Self;
}

#[cfg(feature = "MPSKernel")]
unsafe impl NSObjectProtocol for MPSImageNormalizedHistogram {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSSecureCoding for MPSImageNormalizedHistogram {}

extern_methods!(
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSImageNormalizedHistogram {
        #[method(clipRectSource)]
        pub unsafe fn clipRectSource(&self) -> MTLRegion;

        #[method(setClipRectSource:)]
        pub unsafe fn setClipRectSource(&self, clip_rect_source: MTLRegion);

        #[method(zeroHistogram)]
        pub unsafe fn zeroHistogram(&self) -> bool;

        #[method(setZeroHistogram:)]
        pub unsafe fn setZeroHistogram(&self, zero_histogram: bool);

        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        #[method(encodeToCommandBuffer:sourceTexture:minmaxTexture:histogram:histogramOffset:)]
        pub unsafe fn encodeToCommandBuffer_sourceTexture_minmaxTexture_histogram_histogramOffset(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source: &ProtocolObject<dyn MTLTexture>,
            minmax_texture: &ProtocolObject<dyn MTLTexture>,
            histogram: &ProtocolObject<dyn MTLBuffer>,
            histogram_offset: NSUInteger,
        );

        #[method(histogramSizeForSourceFormat:)]
        pub unsafe fn histogramSizeForSourceFormat(&self, source_format: MTLPixelFormat) -> usize;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSImageNormalizedHistogram {
        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

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
    unsafe impl MPSImageNormalizedHistogram {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimagehistogramequalization?language=objc)
    #[unsafe(super(MPSUnaryImageKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    pub struct MPSImageHistogramEqualization;
);

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSImageHistogramEqualization {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSImageHistogramEqualization {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSImageHistogramEqualization {
    type Result = Self;
}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSImageHistogramEqualization {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSImageHistogramEqualization {}

extern_methods!(
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageHistogramEqualization {
        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        #[method(encodeTransformToCommandBuffer:sourceTexture:histogram:histogramOffset:)]
        pub unsafe fn encodeTransformToCommandBuffer_sourceTexture_histogram_histogramOffset(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source: &ProtocolObject<dyn MTLTexture>,
            histogram: &ProtocolObject<dyn MTLBuffer>,
            histogram_offset: NSUInteger,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSUnaryImageKernel`
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageHistogramEqualization {
        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageHistogramEqualization {
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageHistogramEqualization {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimagehistogramspecification?language=objc)
    #[unsafe(super(MPSUnaryImageKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    pub struct MPSImageHistogramSpecification;
);

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSImageHistogramSpecification {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSImageHistogramSpecification {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSImageHistogramSpecification {
    type Result = Self;
}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSImageHistogramSpecification {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSImageHistogramSpecification {}

extern_methods!(
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageHistogramSpecification {
        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        #[method(encodeTransformToCommandBuffer:sourceTexture:sourceHistogram:sourceHistogramOffset:desiredHistogram:desiredHistogramOffset:)]
        pub unsafe fn encodeTransformToCommandBuffer_sourceTexture_sourceHistogram_sourceHistogramOffset_desiredHistogram_desiredHistogramOffset(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source: &ProtocolObject<dyn MTLTexture>,
            source_histogram: &ProtocolObject<dyn MTLBuffer>,
            source_histogram_offset: NSUInteger,
            desired_histogram: &ProtocolObject<dyn MTLBuffer>,
            desired_histogram_offset: NSUInteger,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSUnaryImageKernel`
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageHistogramSpecification {
        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageHistogramSpecification {
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageHistogramSpecification {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);