//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-foundation")]
use objc2_foundation::*;

use crate::*;

/// Quality prioritization levels to favor quality or performance.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/videotoolbox/vtmotionblurconfigurationqualityprioritization?language=objc)
// NS_ENUM
#[cfg(feature = "objc2")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VTMotionBlurConfigurationQualityPrioritization(pub NSInteger);
#[cfg(feature = "objc2")]
impl VTMotionBlurConfigurationQualityPrioritization {
    #[doc(alias = "VTMotionBlurConfigurationQualityPrioritizationNormal")]
    pub const Normal: Self = Self(1);
    #[doc(alias = "VTMotionBlurConfigurationQualityPrioritizationQuality")]
    pub const Quality: Self = Self(2);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for VTMotionBlurConfigurationQualityPrioritization {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for VTMotionBlurConfigurationQualityPrioritization {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// List of existing algorithm revisions with the highest being the latest. Clients can read defaultRevision property to find the default revision.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/videotoolbox/vtmotionblurconfigurationrevision?language=objc)
// NS_ENUM
#[cfg(feature = "objc2")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VTMotionBlurConfigurationRevision(pub NSInteger);
#[cfg(feature = "objc2")]
impl VTMotionBlurConfigurationRevision {
    #[doc(alias = "VTMotionBlurConfigurationRevision1")]
    pub const Revision1: Self = Self(1);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for VTMotionBlurConfigurationRevision {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for VTMotionBlurConfigurationRevision {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Hint to let the processor know whether frames are being submitted in presenatation sequence, allowing performance optimizations based on previous processing requests
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/videotoolbox/vtmotionblurparameterssubmissionmode?language=objc)
// NS_ENUM
#[cfg(feature = "objc2")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VTMotionBlurParametersSubmissionMode(pub NSInteger);
#[cfg(feature = "objc2")]
impl VTMotionBlurParametersSubmissionMode {
    #[doc(alias = "VTMotionBlurParametersSubmissionModeRandom")]
    pub const Random: Self = Self(1);
    #[doc(alias = "VTMotionBlurParametersSubmissionModeSequential")]
    pub const Sequential: Self = Self(2);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for VTMotionBlurParametersSubmissionMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for VTMotionBlurParametersSubmissionMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(feature = "objc2")]
extern_class!(
    /// Configuration that is used to set up the MotionBlur Processor.
    ///
    ///
    /// This configuration enables the MotionBlur on a VTFrameProcesing session.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/videotoolbox/vtmotionblurconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2")]
    pub struct VTMotionBlurConfiguration;
);

#[cfg(feature = "objc2")]
unsafe impl Send for VTMotionBlurConfiguration {}

#[cfg(feature = "objc2")]
unsafe impl Sync for VTMotionBlurConfiguration {}

#[cfg(feature = "objc2")]
extern_conformance!(
    unsafe impl NSObjectProtocol for VTMotionBlurConfiguration {}
);

#[cfg(all(feature = "VTFrameProcessorConfiguration", feature = "objc2"))]
extern_conformance!(
    unsafe impl VTFrameProcessorConfiguration for VTMotionBlurConfiguration {}
);

#[cfg(feature = "objc2")]
impl VTMotionBlurConfiguration {
    extern_methods!(
        /// Creates a new VTMotionBlurConfiguration with specified flow width and height.
        ///
        ///
        /// init will return nil if dimensions are out of range or revision is unsupported.
        ///
        ///
        /// Parameter `frameWidth`: Width of source frame in pixels. Maximum value is 8192 for macOS, and 4096 for iOS.
        ///
        ///
        /// Parameter `frameHeight`: Height of source frame in pixels. Maximum value is 4320 for macOS, and 2160 for iOS.
        ///
        ///
        /// Parameter `usePrecomputedFlow`: Boolean value to indicate that Optical Flow will be provided by the user, if false this configuration will compute the optical flow on the fly.
        ///
        ///
        /// Parameter `qualityPrioritization`: Used to control quality and performance levels. See VTMotionBlurConfigurationQualityPrioritization for more info.
        ///
        ///
        /// Parameter `revision`: The specific algorithm or configuration revision that is to be used to perform the request.
        #[unsafe(method(initWithFrameWidth:frameHeight:usePrecomputedFlow:qualityPrioritization:revision:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithFrameWidth_frameHeight_usePrecomputedFlow_qualityPrioritization_revision(
            this: Allocated<Self>,
            frame_width: NSInteger,
            frame_height: NSInteger,
            use_precomputed_flow: bool,
            quality_prioritization: VTMotionBlurConfigurationQualityPrioritization,
            revision: VTMotionBlurConfigurationRevision,
        ) -> Option<Retained<Self>>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        /// Width of source frame in pixels.
        #[unsafe(method(frameWidth))]
        #[unsafe(method_family = none)]
        pub unsafe fn frameWidth(&self) -> NSInteger;

        /// Height of source frame in pixels.
        #[unsafe(method(frameHeight))]
        #[unsafe(method_family = none)]
        pub unsafe fn frameHeight(&self) -> NSInteger;

        /// Indicates that caller will provide optical flow.
        #[unsafe(method(usePrecomputedFlow))]
        #[unsafe(method_family = none)]
        pub unsafe fn usePrecomputedFlow(&self) -> bool;

        /// parameter used to control quality and performance levels. See VTMotionBlurConfigurationQualityPrioritization for more info.
        #[unsafe(method(qualityPrioritization))]
        #[unsafe(method_family = none)]
        pub unsafe fn qualityPrioritization(
            &self,
        ) -> VTMotionBlurConfigurationQualityPrioritization;

        /// The specific algorithm or configuration revision that is to be used to perform the request.
        #[unsafe(method(revision))]
        #[unsafe(method_family = none)]
        pub unsafe fn revision(&self) -> VTMotionBlurConfigurationRevision;

        #[cfg(feature = "objc2-foundation")]
        /// Provides the collection of currently-supported algorithm or configuration revisions for the class of configuration.
        ///
        /// This property allows clients to introspect at runtime what revisions are available for each configuration.
        #[unsafe(method(supportedRevisions))]
        #[unsafe(method_family = none)]
        pub unsafe fn supportedRevisions() -> Retained<NSIndexSet>;

        /// Provides the default revision of a particular algorithm or configuration.
        #[unsafe(method(defaultRevision))]
        #[unsafe(method_family = none)]
        pub unsafe fn defaultRevision() -> VTMotionBlurConfigurationRevision;

        #[cfg(feature = "objc2-foundation")]
        /// list of source frame supported pixel formats for current configuration
        #[unsafe(method(frameSupportedPixelFormats))]
        #[unsafe(method_family = none)]
        pub unsafe fn frameSupportedPixelFormats(&self) -> Retained<NSArray<NSNumber>>;

        #[cfg(feature = "objc2-foundation")]
        /// returns a pixelBufferAttributes dictionary describing requirements for pixelBuffers used as source frames and reference frames.
        #[unsafe(method(sourcePixelBufferAttributes))]
        #[unsafe(method_family = none)]
        pub unsafe fn sourcePixelBufferAttributes(
            &self,
        ) -> Retained<NSDictionary<NSString, AnyObject>>;

        #[cfg(feature = "objc2-foundation")]
        /// returns a pixelBufferAttributes dictionary describing requirements for pixelBuffers used as destination frames.
        #[unsafe(method(destinationPixelBufferAttributes))]
        #[unsafe(method_family = none)]
        pub unsafe fn destinationPixelBufferAttributes(
            &self,
        ) -> Retained<NSDictionary<NSString, AnyObject>>;

        /// reports that this processor is  supported
        #[unsafe(method(processorSupported))]
        #[unsafe(method_family = none)]
        pub unsafe fn processorSupported() -> Boolean;
    );
}

#[cfg(feature = "objc2")]
extern_class!(
    /// VTMotionBlurParameters object contains both input and output parameters needed to run the MotionBlur processor on a frame. This object is used in the processWithParameters call of VTFrameProcessor class. The output parameter for this class is destinationFrame where the output frame is returned (as VTFrameProcessorFrame) back to the caller function once the processWithParameters completes.
    ///
    ///
    /// VTMotionBlurParameters are frame level parameters.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/videotoolbox/vtmotionblurparameters?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2")]
    pub struct VTMotionBlurParameters;
);

#[cfg(feature = "objc2")]
extern_conformance!(
    unsafe impl NSObjectProtocol for VTMotionBlurParameters {}
);

#[cfg(all(feature = "VTFrameProcessorParameters", feature = "objc2"))]
extern_conformance!(
    unsafe impl VTFrameProcessorParameters for VTMotionBlurParameters {}
);

#[cfg(feature = "objc2")]
impl VTMotionBlurParameters {
    extern_methods!(
        #[cfg(feature = "VTFrameProcessorFrame")]
        /// Creates a new VTMotionBlurParameters .
        ///
        ///
        /// init will return nil if sourceFrame or destinationFrame is nil, sourceFrame and reference frames  are different pixelFormats, or motionBlurStrength is out of range.
        ///
        ///
        /// Parameter `sourceFrame`: Current source frame. Must be non nil.
        ///
        ///
        /// Parameter `nextFrame`: Next source frame in presentation time order. For the last frame this can be set to nil.
        ///
        ///
        /// Parameter `previousFrame`: Previous source frame in presentation time order. For the first frame this can be set to nil.
        ///
        ///
        /// Parameter `nextOpticalFlow`: Optional VTFrameProcessorOpticalFlow object that contains forward and backward optical flow with next frame. Only needed if optical flow is pre-computed. For the last frame this will always be nil.
        ///
        ///
        /// Parameter `previousOpticalFlow`: Optional VTFrameProcessorOpticalFlow object that contains forward and backward optical flow with previous frame. Only needed if optical flow is pre-computed. For the first frame this will always be nil.
        ///
        ///
        /// Parameter `motionBlurStrength`: NSInteger number to indicate the strength of blur to apply. Range is from 1 to 100. Default value is 50.
        ///
        ///
        /// Parameter `submissionMode`: Set to VTMotionBlurParametersSubmissionModeSequential to indicate that current submission follow presentation time order without jump or skip when compared to previous submission. VTMotionBlurParametersSubmissionModeSequential will yield better performance. Set to VTMotionBlurParametersSubmissionModeRandom to indicate a skip or a jump in frame sequence. If VTMotionBlurParametersSubmissionModeRandom is set internal cache will be cleared during processWithParameters call.
        ///
        ///
        /// Parameter `destinationFrame`: User allocated pixel buffer that will receive the results.
        #[unsafe(method(initWithSourceFrame:nextFrame:previousFrame:nextOpticalFlow:previousOpticalFlow:motionBlurStrength:submissionMode:destinationFrame:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithSourceFrame_nextFrame_previousFrame_nextOpticalFlow_previousOpticalFlow_motionBlurStrength_submissionMode_destinationFrame(
            this: Allocated<Self>,
            source_frame: &VTFrameProcessorFrame,
            next_frame: Option<&VTFrameProcessorFrame>,
            previous_frame: Option<&VTFrameProcessorFrame>,
            next_optical_flow: Option<&VTFrameProcessorOpticalFlow>,
            previous_optical_flow: Option<&VTFrameProcessorOpticalFlow>,
            motion_blur_strength: NSInteger,
            submission_mode: VTMotionBlurParametersSubmissionMode,
            destination_frame: &VTFrameProcessorFrame,
        ) -> Option<Retained<Self>>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "VTFrameProcessorFrame")]
        /// sourceFrame Current source frame. Must be non nil
        #[unsafe(method(sourceFrame))]
        #[unsafe(method_family = none)]
        pub unsafe fn sourceFrame(&self) -> Retained<VTFrameProcessorFrame>;

        #[cfg(feature = "VTFrameProcessorFrame")]
        /// Next source frame in presentation time order. For the last frame this will be nil.
        #[unsafe(method(nextFrame))]
        #[unsafe(method_family = none)]
        pub unsafe fn nextFrame(&self) -> Option<Retained<VTFrameProcessorFrame>>;

        #[cfg(feature = "VTFrameProcessorFrame")]
        /// Previous source frame in presentation time order. For the first frame this will be nil.
        #[unsafe(method(previousFrame))]
        #[unsafe(method_family = none)]
        pub unsafe fn previousFrame(&self) -> Option<Retained<VTFrameProcessorFrame>>;

        #[cfg(feature = "VTFrameProcessorFrame")]
        /// Optional VTFrameProcessorOpticalFlow object that contains forward and backward optical flow with next frame. Only needed if optical flow is pre-computed. For the last frame this will be nil.
        #[unsafe(method(nextOpticalFlow))]
        #[unsafe(method_family = none)]
        pub unsafe fn nextOpticalFlow(&self) -> Option<Retained<VTFrameProcessorOpticalFlow>>;

        #[cfg(feature = "VTFrameProcessorFrame")]
        /// Optional VTFrameProcessorOpticalFlow object  that contains forward and backward optical flow with previous frame. Only needed if optical flow is pre-computed. For the first frame this will be nil.
        #[unsafe(method(previousOpticalFlow))]
        #[unsafe(method_family = none)]
        pub unsafe fn previousOpticalFlow(&self) -> Option<Retained<VTFrameProcessorOpticalFlow>>;

        /// motionBlurStrength NSInteger number to indicate the strength of blur to apply. Range is from 1 to 100. Default value is 50.
        #[unsafe(method(motionBlurStrength))]
        #[unsafe(method_family = none)]
        pub unsafe fn motionBlurStrength(&self) -> NSInteger;

        /// A VTMotionBlurParametersSubmissionMode value describing the processing request in this Parameters object .
        #[unsafe(method(submissionMode))]
        #[unsafe(method_family = none)]
        pub unsafe fn submissionMode(&self) -> VTMotionBlurParametersSubmissionMode;

        #[cfg(feature = "VTFrameProcessorFrame")]
        /// VTFrameProcessorFrame that contains user allocated pixel buffer that will receive the results.
        #[unsafe(method(destinationFrame))]
        #[unsafe(method_family = none)]
        pub unsafe fn destinationFrame(&self) -> Retained<VTFrameProcessorFrame>;
    );
}
