//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// The MPSImageHistogram computes the histogram of an image.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimagehistogram?language=objc)
    #[unsafe(super(MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
    pub struct MPSImageHistogram;
);

#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
extern_conformance!(
    unsafe impl NSCoding for MPSImageHistogram {}
);

#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
extern_conformance!(
    unsafe impl NSCopying for MPSImageHistogram {}
);

#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSImageHistogram {
    type Result = Self;
}

#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
extern_conformance!(
    unsafe impl NSObjectProtocol for MPSImageHistogram {}
);

#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
extern_conformance!(
    unsafe impl NSSecureCoding for MPSImageHistogram {}
);

#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
impl MPSImageHistogram {
    extern_methods!(
        /// The source rectangle to use when reading data.
        ///
        /// A MTLRegion that indicates which part of the source to read. If the clipRectSource does not lie
        /// completely within the source image, the intersection of the image bounds and clipRectSource will
        /// be used. The clipRectSource replaces the MPSUnaryImageKernel offset parameter for this filter.
        /// The latter is ignored.   Default: MPSRectNoClip, use the entire source texture.
        #[unsafe(method(clipRectSource))]
        #[unsafe(method_family = none)]
        pub unsafe fn clipRectSource(&self) -> MTLRegion;

        /// Setter for [`clipRectSource`][Self::clipRectSource].
        #[unsafe(method(setClipRectSource:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setClipRectSource(&self, clip_rect_source: MTLRegion);

        /// Zero-initalize the histogram results
        ///
        /// Indicates that the memory region in which the histogram results are to be written in the
        /// histogram buffer are to be zero-initialized or not. Default: YES.
        #[unsafe(method(zeroHistogram))]
        #[unsafe(method_family = none)]
        pub unsafe fn zeroHistogram(&self) -> bool;

        /// Setter for [`zeroHistogram`][Self::zeroHistogram].
        #[unsafe(method(setZeroHistogram:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setZeroHistogram(&self, zero_histogram: bool);

        /// NSSecureCoding compatability
        ///
        /// While the standard NSSecureCoding/NSCoding method
        /// -initWithCoder: should work, since the file can't
        /// know which device your data is allocated on, we
        /// have to guess and may guess incorrectly.  To avoid
        /// that problem, use initWithCoder:device instead.
        ///
        /// Parameter `aDecoder`: The NSCoder subclass with your serialized MPSKernel
        ///
        /// Parameter `device`: The MTLDevice on which to make the MPSKernel
        ///
        /// Returns: A new MPSKernel object, or nil if failure.
        #[unsafe(method(initWithCoder:device:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        /// Encode the filter to a command buffer using a MTLComputeCommandEncoder.
        ///
        /// The filter will not begin to execute until after the command
        /// buffer has been enqueued and committed.
        ///
        ///
        /// Parameter `commandBuffer`: A valid MTLCommandBuffer.
        ///
        /// Parameter `source`: A valid MTLTexture containing the source image for the filter
        ///
        /// Parameter `histogram`: A valid MTLBuffer to receive the histogram results.
        ///
        /// Parameter `histogramOffset`: Byte offset into histogram buffer at which to write the histogram results. Must be a multiple of 32 bytes.
        /// The histogram results / channel are stored together.  The number of channels for which
        /// histogram results are stored is determined by the number of channels in the image.
        /// If histogramInfo.histogramForAlpha is false and the source image is RGBA then only histogram
        /// results for RGB channels are stored.
        ///
        /// The histogram results are stored in the histogram buffer as follows:
        /// - histogram results for the R channel for all bins followed by
        /// - histogram results for the G channel for all bins followed by
        /// - histogram results for the B channel for all bins followed by
        /// - histogram results for the A channel for all bins
        #[unsafe(method(encodeToCommandBuffer:sourceTexture:histogram:histogramOffset:))]
        #[unsafe(method_family = none)]
        pub unsafe fn encodeToCommandBuffer_sourceTexture_histogram_histogramOffset(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source: &ProtocolObject<dyn MTLTexture>,
            histogram: &ProtocolObject<dyn MTLBuffer>,
            histogram_offset: NSUInteger,
        );

        /// The amount of space in the output MTLBuffer the histogram will take up.
        ///
        /// This convenience function calculates the minimum amount of space
        /// needed in the output histogram for the results.  The MTLBuffer should
        /// be at least this length, longer if histogramOffset is non-zero.
        ///
        /// Parameter `sourceFormat`: The MTLPixelFormat of the source image. This is
        /// the source parameter of -encodeToCommandBuffer:
        /// sourceTexture:histogram:histogramOffset
        ///
        /// Returns: The number of bytes needed to store the result histograms.
        #[unsafe(method(histogramSizeForSourceFormat:))]
        #[unsafe(method_family = none)]
        pub unsafe fn histogramSizeForSourceFormat(&self, source_format: MTLPixelFormat) -> usize;
    );
}

/// Methods declared on superclass `MPSKernel`.
#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
impl MPSImageHistogram {
    extern_methods!(
        /// Standard init with default properties per filter type
        ///
        /// Parameter `device`: The device that the filter will be used on. May not be NULL.
        ///
        /// Returns: a pointer to the newly initialized object. This will fail, returning
        /// nil if the device is not supported. Devices must be
        /// MTLFeatureSet_iOS_GPUFamily2_v1 or later.
        #[unsafe(method(initWithDevice:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        /// Called by NSCoder to decode MPSKernels
        ///
        /// This isn't the right interface to decode a MPSKernel, but
        /// it is the one that NSCoder uses. To enable your NSCoder
        /// (e.g. NSKeyedUnarchiver) to set which device to use
        /// extend the object to adopt the MPSDeviceProvider
        /// protocol. Otherwise, the Metal system default device
        /// will be used.
        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
impl MPSImageHistogram {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// The MPSImageNormalizedHistogram computes the normalized histogram of an image.
    /// The minimum and maximum pixel values for a given region of an image are first computed.
    /// The max(computed minimum pixel value, MPSImageHistogramInfo.minPixelValue) and the
    /// min(computed maximum pixel value, MPSImageHistogramInfo.maxPixelValue) are used to
    /// compute the normalized histogram.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimagenormalizedhistogram?language=objc)
    #[unsafe(super(MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
    pub struct MPSImageNormalizedHistogram;
);

#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
extern_conformance!(
    unsafe impl NSCoding for MPSImageNormalizedHistogram {}
);

#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
extern_conformance!(
    unsafe impl NSCopying for MPSImageNormalizedHistogram {}
);

#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSImageNormalizedHistogram {
    type Result = Self;
}

#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
extern_conformance!(
    unsafe impl NSObjectProtocol for MPSImageNormalizedHistogram {}
);

#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
extern_conformance!(
    unsafe impl NSSecureCoding for MPSImageNormalizedHistogram {}
);

#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
impl MPSImageNormalizedHistogram {
    extern_methods!(
        /// The source rectangle to use when reading data.
        ///
        /// A MTLRegion that indicates which part of the source to read. If the clipRectSource does not lie
        /// completely within the source image, the intersection of the image bounds and clipRectSource will
        /// be used. The clipRectSource replaces the MPSUnaryImageKernel offset parameter for this filter.
        /// The latter is ignored.   Default: MPSRectNoClip, use the entire source texture.
        #[unsafe(method(clipRectSource))]
        #[unsafe(method_family = none)]
        pub unsafe fn clipRectSource(&self) -> MTLRegion;

        /// Setter for [`clipRectSource`][Self::clipRectSource].
        #[unsafe(method(setClipRectSource:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setClipRectSource(&self, clip_rect_source: MTLRegion);

        /// Zero-initalize the histogram results
        ///
        /// Indicates that the memory region in which the histogram results are to be written in the
        /// histogram buffer are to be zero-initialized or not. Default: YES.
        #[unsafe(method(zeroHistogram))]
        #[unsafe(method_family = none)]
        pub unsafe fn zeroHistogram(&self) -> bool;

        /// Setter for [`zeroHistogram`][Self::zeroHistogram].
        #[unsafe(method(setZeroHistogram:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setZeroHistogram(&self, zero_histogram: bool);

        /// NSSecureCoding compatability
        ///
        /// While the standard NSSecureCoding/NSCoding method
        /// -initWithCoder: should work, since the file can't
        /// know which device your data is allocated on, we
        /// have to guess and may guess incorrectly.  To avoid
        /// that problem, use initWithCoder:device instead.
        ///
        /// Parameter `aDecoder`: The NSCoder subclass with your serialized MPSKernel
        ///
        /// Parameter `device`: The MTLDevice on which to make the MPSKernel
        ///
        /// Returns: A new MPSKernel object, or nil if failure.
        #[unsafe(method(initWithCoder:device:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        /// Encode the filter to a command buffer using a MTLComputeCommandEncoder.
        ///
        /// The filter will not begin to execute until after the command
        /// buffer has been enqueued and committed.
        ///
        ///
        /// Parameter `commandBuffer`: A valid MTLCommandBuffer.
        ///
        /// Parameter `source`: A valid MTLTexture containing the source image for the filter
        ///
        /// Parameter `minmaxTexture`: A valid MTLTexture in which the min/max pixel values from source will be returned
        ///
        /// Parameter `histogram`: A valid MTLBuffer to receive the histogram results.
        ///
        /// Parameter `histogramOffset`: Byte offset into histogram buffer at which to write the histogram results. Must be a multiple of 32 bytes.
        /// The histogram results / channel are stored together.  The number of channels for which
        /// histogram results are stored is determined by the number of channels in the image.
        /// If histogramInfo.histogramForAlpha is false and the source image is RGBA then only histogram
        /// results for RGB channels are stored.
        ///
        /// The histogram results are stored in the histogram buffer as follows:
        /// - histogram results for the R channel for all bins followed by
        /// - histogram results for the G channel for all bins followed by
        /// - histogram results for the B channel for all bins followed by
        /// - histogram results for the A channel for all bins
        #[unsafe(method(encodeToCommandBuffer:sourceTexture:minmaxTexture:histogram:histogramOffset:))]
        #[unsafe(method_family = none)]
        pub unsafe fn encodeToCommandBuffer_sourceTexture_minmaxTexture_histogram_histogramOffset(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source: &ProtocolObject<dyn MTLTexture>,
            minmax_texture: &ProtocolObject<dyn MTLTexture>,
            histogram: &ProtocolObject<dyn MTLBuffer>,
            histogram_offset: NSUInteger,
        );

        /// The amount of space in the output MTLBuffer the histogram will take up.
        ///
        /// This convenience function calculates the minimum amount of space
        /// needed in the output histogram for the results.  The MTLBuffer should
        /// be at least this length, longer if histogramOffset is non-zero.
        ///
        /// Parameter `sourceFormat`: The MTLPixelFormat of the source image. This is
        /// the source parameter of -encodeToCommandBuffer:
        /// sourceTexture:histogram:histogramOffset
        ///
        /// Returns: The number of bytes needed to store the result histograms.
        #[unsafe(method(histogramSizeForSourceFormat:))]
        #[unsafe(method_family = none)]
        pub unsafe fn histogramSizeForSourceFormat(&self, source_format: MTLPixelFormat) -> usize;
    );
}

/// Methods declared on superclass `MPSKernel`.
#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
impl MPSImageNormalizedHistogram {
    extern_methods!(
        /// Standard init with default properties per filter type
        ///
        /// Parameter `device`: The device that the filter will be used on. May not be NULL.
        ///
        /// Returns: a pointer to the newly initialized object. This will fail, returning
        /// nil if the device is not supported. Devices must be
        /// MTLFeatureSet_iOS_GPUFamily2_v1 or later.
        #[unsafe(method(initWithDevice:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        /// Called by NSCoder to decode MPSKernels
        ///
        /// This isn't the right interface to decode a MPSKernel, but
        /// it is the one that NSCoder uses. To enable your NSCoder
        /// (e.g. NSKeyedUnarchiver) to set which device to use
        /// extend the object to adopt the MPSDeviceProvider
        /// protocol. Otherwise, the Metal system default device
        /// will be used.
        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
impl MPSImageNormalizedHistogram {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// The MPSImageHistogramEqualization performs equalizes the histogram of an image.
    /// The process is divided into three steps.
    ///
    /// -# Call -initWithDevice:histogramInfo:   This creates a MPSImageHistogramEqualization
    /// object.   It is done when the method returns.
    ///
    /// -# Call -encodeTransform:sourceTexture:histogram:histogramOffset:  This creates a privately held
    /// image transform (i.e. a cumulative distribution function of the histogram) which will be used to
    /// equalize the distribution of the histogram of the source image. This process runs on a MTLCommandBuffer
    /// when it is committed to a MTLCommandQueue. It must complete before the next step can be run.
    /// It may be performed on the same MTLCommandBuffer.  The histogram argument specifies the histogram
    /// buffer which contains the histogram values for sourceTexture.  The sourceTexture argument is used by
    /// encodeTransform to determine the number of channels and therefore which histogram data in histogram
    /// buffer to use. The histogram for sourceTexture must have been computed either on the CPU or using
    /// the MPSImageHistogram kernel
    ///
    /// -# Call -encodeToCommandBuffer:sourceTexture:destinationTexture: to read data from
    /// sourceTexture, apply the equalization transform to it and write to destination texture.
    /// This step is also done on the GPU on a MTLCommandQueue.
    ///
    /// You can reuse the same equalization transform on other images to perform the
    /// same transform on those images. (Since their distribution is probably different,
    /// they will probably not be equalized by it.) This filter usually will not be able
    /// to work in place.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimagehistogramequalization?language=objc)
    #[unsafe(super(MPSUnaryImageKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
    pub struct MPSImageHistogramEqualization;
);

#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
extern_conformance!(
    unsafe impl NSCoding for MPSImageHistogramEqualization {}
);

#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
extern_conformance!(
    unsafe impl NSCopying for MPSImageHistogramEqualization {}
);

#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSImageHistogramEqualization {
    type Result = Self;
}

#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
extern_conformance!(
    unsafe impl NSObjectProtocol for MPSImageHistogramEqualization {}
);

#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
extern_conformance!(
    unsafe impl NSSecureCoding for MPSImageHistogramEqualization {}
);

#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
impl MPSImageHistogramEqualization {
    extern_methods!(
        /// NSSecureCoding compatability
        ///
        /// While the standard NSSecureCoding/NSCoding method
        /// -initWithCoder: should work, since the file can't
        /// know which device your data is allocated on, we
        /// have to guess and may guess incorrectly.  To avoid
        /// that problem, use initWithCoder:device instead.
        ///
        /// Parameter `aDecoder`: The NSCoder subclass with your serialized MPSKernel
        ///
        /// Parameter `device`: The MTLDevice on which to make the MPSKernel
        ///
        /// Returns: A new MPSKernel object, or nil if failure.
        #[unsafe(method(initWithCoder:device:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        /// Encode the transform function to a command buffer using a MTLComputeCommandEncoder.
        /// The transform function computes the equalization lookup table.
        ///
        /// The transform function will not begin to execute until after the command
        /// buffer has been enqueued and committed.  This step will need to be repeated
        /// with the new MPSKernel if -copyWithZone:device or -copyWithZone: is called.
        /// The transform is stored as internal state to the object. You still need to
        /// call -encodeToCommandBuffer:sourceTexture:destinationTexture: afterward
        /// to apply the transform to produce a result texture.
        ///
        ///
        /// Parameter `commandBuffer`: A valid MTLCommandBuffer.
        ///
        /// Parameter `source`: A valid MTLTexture containing the source image for the filter.
        ///
        /// Parameter `histogram`: A valid MTLBuffer containing the histogram results for an image.  This filter
        /// will use these histogram results to generate the cumulative histogram for equalizing
        /// the image.  The histogram results / channel are stored together.  The number of channels
        /// for which histogram results are stored is determined by the number of channels in the image.
        /// If histogramInfo.histogramForAlpha is false and the source image is RGBA then only histogram
        /// results for RGB channels are stored.
        ///
        /// Parameter `histogramOffset`: A byte offset into the histogram MTLBuffer where the histogram starts. Must conform to
        /// alignment requirements for [MTLComputeCommandEncoder setBuffer:offset:atIndex:] offset
        /// parameter.
        #[unsafe(method(encodeTransformToCommandBuffer:sourceTexture:histogram:histogramOffset:))]
        #[unsafe(method_family = none)]
        pub unsafe fn encodeTransformToCommandBuffer_sourceTexture_histogram_histogramOffset(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source: &ProtocolObject<dyn MTLTexture>,
            histogram: &ProtocolObject<dyn MTLBuffer>,
            histogram_offset: NSUInteger,
        );
    );
}

/// Methods declared on superclass `MPSUnaryImageKernel`.
#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
impl MPSImageHistogramEqualization {
    extern_methods!(
        /// Standard init with default properties per filter type
        ///
        /// Parameter `device`: The device that the filter will be used on. May not be NULL.
        ///
        /// Returns: a pointer to the newly initialized object. This will fail, returning
        /// nil if the device is not supported. Devices must be
        /// MTLFeatureSet_iOS_GPUFamily2_v1 or later.
        #[unsafe(method(initWithDevice:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `MPSKernel`.
#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
impl MPSImageHistogramEqualization {
    extern_methods!(
        /// Called by NSCoder to decode MPSKernels
        ///
        /// This isn't the right interface to decode a MPSKernel, but
        /// it is the one that NSCoder uses. To enable your NSCoder
        /// (e.g. NSKeyedUnarchiver) to set which device to use
        /// extend the object to adopt the MPSDeviceProvider
        /// protocol. Otherwise, the Metal system default device
        /// will be used.
        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
impl MPSImageHistogramEqualization {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// The MPSImageHistogramSpecification performs a histogram specification operation on an image.
    /// It is a generalized version of histogram equalization operation.  The histogram specificaiton filter
    /// converts the image so that its histogram matches the desired histogram.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimagehistogramspecification?language=objc)
    #[unsafe(super(MPSUnaryImageKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
    pub struct MPSImageHistogramSpecification;
);

#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
extern_conformance!(
    unsafe impl NSCoding for MPSImageHistogramSpecification {}
);

#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
extern_conformance!(
    unsafe impl NSCopying for MPSImageHistogramSpecification {}
);

#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSImageHistogramSpecification {
    type Result = Self;
}

#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
extern_conformance!(
    unsafe impl NSObjectProtocol for MPSImageHistogramSpecification {}
);

#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
extern_conformance!(
    unsafe impl NSSecureCoding for MPSImageHistogramSpecification {}
);

#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
impl MPSImageHistogramSpecification {
    extern_methods!(
        /// NSSecureCoding compatability
        ///
        /// While the standard NSSecureCoding/NSCoding method
        /// -initWithCoder: should work, since the file can't
        /// know which device your data is allocated on, we
        /// have to guess and may guess incorrectly.  To avoid
        /// that problem, use initWithCoder:device instead.
        ///
        /// Parameter `aDecoder`: The NSCoder subclass with your serialized MPSKernel
        ///
        /// Parameter `device`: The MTLDevice on which to make the MPSKernel
        ///
        /// Returns: A new MPSKernel object, or nil if failure.
        #[unsafe(method(initWithCoder:device:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        /// Encode the transform function to a command buffer using a MTLComputeCommandEncoder.
        /// The transform function computes the specification lookup table.
        ///
        /// The transform function will not begin to execute until after the command
        /// buffer has been enqueued and committed. This step will need to be repeated
        /// with the new MPSKernel if -copyWithZone:device or -copyWithZone: is called.
        ///
        ///
        /// Parameter `commandBuffer`: A valid MTLCommandBuffer.
        ///
        /// Parameter `source`: A valid MTLTexture containing the source image for the filter.
        ///
        /// Parameter `sourceHistogram`: A valid MTLBuffer containing the histogram results for the source image.  This filter
        /// will use these histogram results to generate the cumulative histogram for equalizing
        /// the image.  The histogram results / channel are stored together.  The number of channels
        /// for which histogram results are stored is determined by the number of channels in the image.
        /// If histogramInfo.histogramForAlpha is false and the source image is RGBA then only histogram
        /// results for RGB channels are stored.
        ///
        /// Parameter `sourceHistogramOffset`: A byte offset into the sourceHistogram MTLBuffer where the histogram starts. Must conform to
        /// alignment requirements for [MTLComputeCommandEncoder setBuffer:offset:atIndex:] offset
        /// parameter.
        ///
        /// Parameter `desiredHistogram`: A valid MTLBuffer containing the desired histogram results for the source image.
        /// The histogram results / channel are stored together.  The number of channels
        /// for which histogram results are stored is determined by the number of channels in the image.
        /// If histogramInfo.histogramForAlpha is false and the source image is RGBA then only histogram
        /// results for RGB channels are stored.
        ///
        /// Parameter `desiredHistogramOffset`: A byte offset into the desiredHistogram MTLBuffer where the histogram starts. Must conform to
        /// alignment requirements for [MTLComputeCommandEncoder setBuffer:offset:atIndex:] offset
        /// parameter.
        #[unsafe(method(encodeTransformToCommandBuffer:sourceTexture:sourceHistogram:sourceHistogramOffset:desiredHistogram:desiredHistogramOffset:))]
        #[unsafe(method_family = none)]
        pub unsafe fn encodeTransformToCommandBuffer_sourceTexture_sourceHistogram_sourceHistogramOffset_desiredHistogram_desiredHistogramOffset(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source: &ProtocolObject<dyn MTLTexture>,
            source_histogram: &ProtocolObject<dyn MTLBuffer>,
            source_histogram_offset: NSUInteger,
            desired_histogram: &ProtocolObject<dyn MTLBuffer>,
            desired_histogram_offset: NSUInteger,
        );
    );
}

/// Methods declared on superclass `MPSUnaryImageKernel`.
#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
impl MPSImageHistogramSpecification {
    extern_methods!(
        /// Standard init with default properties per filter type
        ///
        /// Parameter `device`: The device that the filter will be used on. May not be NULL.
        ///
        /// Returns: a pointer to the newly initialized object. This will fail, returning
        /// nil if the device is not supported. Devices must be
        /// MTLFeatureSet_iOS_GPUFamily2_v1 or later.
        #[unsafe(method(initWithDevice:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `MPSKernel`.
#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
impl MPSImageHistogramSpecification {
    extern_methods!(
        /// Called by NSCoder to decode MPSKernels
        ///
        /// This isn't the right interface to decode a MPSKernel, but
        /// it is the one that NSCoder uses. To enable your NSCoder
        /// (e.g. NSKeyedUnarchiver) to set which device to use
        /// extend the object to adopt the MPSDeviceProvider
        /// protocol. Otherwise, the Metal system default device
        /// will be used.
        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(feature = "MPSCore", feature = "MPSImageKernel", feature = "MPSKernel"))]
impl MPSImageHistogramSpecification {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
