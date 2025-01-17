//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// An image registration request that will produce a translational transformation which will morph one image to another.
    ///
    /// Because this is a stateful request, it must be performed on at least two images in order to produce an observation.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/vision/vntracktranslationalimageregistrationrequest?language=objc)
    #[unsafe(super(VNStatefulRequest, VNImageBasedRequest, VNRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
    pub struct VNTrackTranslationalImageRegistrationRequest;
);

#[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
unsafe impl NSCopying for VNTrackTranslationalImageRegistrationRequest {}

#[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
unsafe impl CopyingHelper for VNTrackTranslationalImageRegistrationRequest {
    type Result = Self;
}

#[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
unsafe impl NSObjectProtocol for VNTrackTranslationalImageRegistrationRequest {}

extern_methods!(
    #[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
    unsafe impl VNTrackTranslationalImageRegistrationRequest {
        /// Create a new request that can statefully track the translational registration of two images.
        ///
        /// This is a convenience initializer for a frame analysis spacing of kCMTimeZero and a nil completion handler.
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        /// Create a new request that can statefully track the translational registration of two images.
        ///
        /// This is a convenience initializer for a frame analysis spacing of kCMTimeZero.
        #[unsafe(method_family(init))]
        #[method_id(initWithCompletionHandler:)]
        pub unsafe fn initWithCompletionHandler(
            this: Allocated<Self>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;

        #[cfg(feature = "VNObservation")]
        /// VNImageTranslationAlignmentObservation results.
        #[unsafe(method_family(none))]
        #[method_id(results)]
        pub unsafe fn results(
            &self,
        ) -> Option<Retained<NSArray<VNImageTranslationAlignmentObservation>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VNStatefulRequest`
    #[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
    unsafe impl VNTrackTranslationalImageRegistrationRequest {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(all(feature = "block2", feature = "objc2-core-media"))]
        /// Create a new video-based stateful request.
        ///
        ///
        /// Parameter `frameAnalysisSpacing`: The reciprocal of maximum rate at which buffers will be processed. The request will not process buffers that fall within the frameAnalysisSpacing after it has performed the analysis. The analysis is not done by wall time but by analysis of of the time stamps of the samplebuffers being processed.
        ///
        /// Parameter `completionHandler`: The block to be invoked after the request has completed its processing. The completion handler gets executed on the same dispatch queue as the request being executed.
        #[unsafe(method_family(init))]
        #[method_id(initWithFrameAnalysisSpacing:completionHandler:)]
        pub unsafe fn initWithFrameAnalysisSpacing_completionHandler(
            this: Allocated<Self>,
            frame_analysis_spacing: CMTime,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vntracktranslationalimageregistrationrequestrevision1?language=objc)
pub static VNTrackTranslationalImageRegistrationRequestRevision1: NSUInteger = 1;
