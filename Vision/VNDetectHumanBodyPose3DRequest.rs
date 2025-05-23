//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A request that detects specific landmark points on human bodies in 3D space relative to the camera.
    /// When possible,`AVDepthData` depth information is used to produce more accurate results, but the request does not require it to run.
    ///
    /// This request generates a collection of VNHumanBodyPose3DObservation objects which describe the position of each detected body
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/vision/vndetecthumanbodypose3drequest?language=objc)
    #[unsafe(super(VNStatefulRequest, VNImageBasedRequest, VNRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
    pub struct VNDetectHumanBodyPose3DRequest;
);

#[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
extern_conformance!(
    unsafe impl NSCopying for VNDetectHumanBodyPose3DRequest {}
);

#[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
unsafe impl CopyingHelper for VNDetectHumanBodyPose3DRequest {
    type Result = Self;
}

#[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
extern_conformance!(
    unsafe impl NSObjectProtocol for VNDetectHumanBodyPose3DRequest {}
);

#[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
impl VNDetectHumanBodyPose3DRequest {
    extern_methods!(
        /// Creates a new VNDetectHumanBodyPose3DRequest with no completion handler.
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        /// Creates a new VNDetectHumanBodyPose3DRequest with completion handler.
        #[unsafe(method(initWithCompletionHandler:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCompletionHandler(
            this: Allocated<Self>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;

        #[cfg(feature = "VNTypes")]
        /// Obtain the collection of human body joint names that are supported by a given request revision.
        ///
        /// Parameter `error`: The address of a variable that will be populated with an error upon failure.  If the caller does not need this information, NULL can be passed.
        ///
        /// Returns: An array of VNHumanBodyPose3DObservationJointName symbols that are supported by the request revision, or nil if a failure occurs.
        #[unsafe(method(supportedJointNamesAndReturnError:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn supportedJointNamesAndReturnError(
            &self,
        ) -> Result<Retained<NSArray<VNHumanBodyPose3DObservationJointName>>, Retained<NSError>>;

        #[cfg(feature = "VNTypes")]
        /// Obtain the collection of human body joints group names that are supported by a request object configured with a request revision.
        ///
        /// Parameter `error`: The address of a variable that will be populated with an error upon failure.  If the caller does not need this information, NULL can be passed.
        ///
        /// Returns: An array of VNHumanBody3DPoseObservationJointsGroupName symbols that are supported by the request, or nil if a failure occurs.
        #[unsafe(method(supportedJointsGroupNamesAndReturnError:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn supportedJointsGroupNamesAndReturnError(
            &self,
        ) -> Result<Retained<NSArray<VNHumanBodyPose3DObservationJointsGroupName>>, Retained<NSError>>;

        #[cfg(feature = "VNObservation")]
        /// VNHumanBodyPose3DObservation results.
        #[unsafe(method(results))]
        #[unsafe(method_family = none)]
        pub unsafe fn results(&self) -> Option<Retained<NSArray<VNHumanBodyPose3DObservation>>>;
    );
}

/// Methods declared on superclass `VNStatefulRequest`.
#[cfg(all(feature = "VNRequest", feature = "VNStatefulRequest"))]
impl VNDetectHumanBodyPose3DRequest {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(all(feature = "block2", feature = "objc2-core-media"))]
        /// Create a new video-based stateful request.
        ///
        ///
        /// Parameter `frameAnalysisSpacing`: The reciprocal of maximum rate at which buffers will be processed. The request will not process buffers that fall within the frameAnalysisSpacing after it has performed the analysis. The analysis is not done by wall time but by analysis of of the time stamps of the samplebuffers being processed.
        ///
        /// Parameter `completionHandler`: The block to be invoked after the request has completed its processing. The completion handler gets executed on the same dispatch queue as the request being executed.
        #[unsafe(method(initWithFrameAnalysisSpacing:completionHandler:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithFrameAnalysisSpacing_completionHandler(
            this: Allocated<Self>,
            frame_analysis_spacing: CMTime,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;
    );
}

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vndetecthumanbodypose3drequestrevision1?language=objc)
pub static VNDetectHumanBodyPose3DRequestRevision1: NSUInteger = 1;
