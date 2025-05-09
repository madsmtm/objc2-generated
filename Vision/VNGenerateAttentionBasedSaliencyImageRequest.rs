//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Generates an image that identifies which part(s) of a given image is most interesting (i.e. something that a human is likely to look at - hence attention based).
    /// The resulting observation, VNSaliencyImageObservation, encodes this data as a heat map which can be used to highlight regions of interest.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/vision/vngenerateattentionbasedsaliencyimagerequest?language=objc)
    #[unsafe(super(VNImageBasedRequest, VNRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VNRequest")]
    pub struct VNGenerateAttentionBasedSaliencyImageRequest;
);

#[cfg(feature = "VNRequest")]
extern_conformance!(
    unsafe impl NSCopying for VNGenerateAttentionBasedSaliencyImageRequest {}
);

#[cfg(feature = "VNRequest")]
unsafe impl CopyingHelper for VNGenerateAttentionBasedSaliencyImageRequest {
    type Result = Self;
}

#[cfg(feature = "VNRequest")]
extern_conformance!(
    unsafe impl NSObjectProtocol for VNGenerateAttentionBasedSaliencyImageRequest {}
);

#[cfg(feature = "VNRequest")]
impl VNGenerateAttentionBasedSaliencyImageRequest {
    extern_methods!(
        #[cfg(feature = "VNObservation")]
        /// VNSaliencyImageObservation results.
        #[unsafe(method(results))]
        #[unsafe(method_family = none)]
        pub unsafe fn results(&self) -> Option<Retained<NSArray<VNSaliencyImageObservation>>>;
    );
}

/// Methods declared on superclass `VNRequest`.
#[cfg(feature = "VNRequest")]
impl VNGenerateAttentionBasedSaliencyImageRequest {
    extern_methods!(
        /// Creates a new VNRequest with no completion handler.
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        /// Creates a new VNRequest with an optional completion handler.
        ///
        ///
        /// Parameter `completionHandler`: The block to be invoked after the request has completed its processing. The completion handler gets executed on the same dispatch queue as the request being executed.
        #[unsafe(method(initWithCompletionHandler:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCompletionHandler(
            this: Allocated<Self>,
            completion_handler: VNRequestCompletionHandler,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "VNRequest")]
impl VNGenerateAttentionBasedSaliencyImageRequest {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

/// [Apple's documentation](https://developer.apple.com/documentation/vision/vngenerateattentionbasedsaliencyimagerequestrevision1?language=objc)
pub static VNGenerateAttentionBasedSaliencyImageRequestRevision1: NSUInteger = 1;

/// Improved accuracy, reduced latency and memory utilization.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/vision/vngenerateattentionbasedsaliencyimagerequestrevision2?language=objc)
pub static VNGenerateAttentionBasedSaliencyImageRequestRevision2: NSUInteger = 2;
