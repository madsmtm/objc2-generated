//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
#[cfg(feature = "objc2-core-video")]
use objc2_core_video::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avsamplebuffervideorendererdidfailtodecodenotification?language=objc)
    pub static AVSampleBufferVideoRendererDidFailToDecodeNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avsamplebuffervideorendererdidfailtodecodenotificationerrorkey?language=objc)
    pub static AVSampleBufferVideoRendererDidFailToDecodeNotificationErrorKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avsamplebuffervideorendererrequiresflushtoresumedecodingdidchangenotification?language=objc)
    pub static AVSampleBufferVideoRendererRequiresFlushToResumeDecodingDidChangeNotification:
        &'static NSNotificationName;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avsamplebuffervideorenderer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVSampleBufferVideoRenderer;
);

#[cfg(feature = "AVQueuedSampleBufferRendering")]
extern_conformance!(
    unsafe impl AVQueuedSampleBufferRendering for AVSampleBufferVideoRenderer {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for AVSampleBufferVideoRenderer {}
);

impl AVSampleBufferVideoRenderer {
    extern_methods!(
        #[cfg(feature = "AVQueuedSampleBufferRendering")]
        /// The ability of the video renderer to be used for enqueueing sample buffers.
        ///
        /// The value of this property is an AVQueuedSampleBufferRenderingStatus that indicates whether the receiver can be used for enqueueing and rendering sample buffers. When the value of this property is AVQueuedSampleBufferRenderingStatusFailed, clients can check the value of the error property to determine the failure. To resume rendering sample buffers using the video renderer after a failure, clients must first reset the status to AVQueuedSampleBufferRenderingStatusUnknown. This can be achieved by invoking -flush on the video renderer.
        /// This property is key value observable.
        #[unsafe(method(status))]
        #[unsafe(method_family = none)]
        pub unsafe fn status(&self) -> AVQueuedSampleBufferRenderingStatus;

        /// If the video renderer's status is AVQueuedSampleBufferRenderingStatusFailed, this describes the error that caused the failure.
        ///
        /// The value of this property is an NSError that describes what caused the video renderer to no longer be able to enqueue sample buffers. If the status is not AVQueuedSampleBufferRenderingStatusFailed, the value of this property is nil.
        #[unsafe(method(error))]
        #[unsafe(method_family = none)]
        pub unsafe fn error(&self) -> Option<Retained<NSError>>;

        /// Indicates that the receiver is in a state where it requires a call to -flush to continue decoding frames.
        ///
        /// When the application enters a state where use of video decoder resources is not permissible, the value of this property changes to YES along with the video renderer's status changing to AVQueuedSampleBufferRenderingStatusFailed.
        /// To resume rendering sample buffers using the video renderer after this property's value is YES, clients must first reset the video renderer by calling flush or flushWithRemovalOfDisplayedImage:completionHandler:.
        /// Clients can track changes to this property via AVSampleBufferVideoRendererRequiresFlushToResumeDecodingDidChangeNotification.
        /// This property is not key value observable.
        #[unsafe(method(requiresFlushToResumeDecoding))]
        #[unsafe(method_family = none)]
        pub unsafe fn requiresFlushToResumeDecoding(&self) -> bool;

        #[cfg(feature = "block2")]
        /// Instructs the video renderer to discard pending enqueued sample buffers and call the provided block when complete.
        ///
        /// Parameter `removeDisplayedImage`: Set YES to remove any currently displayed image, NO to preserve any current image.
        ///
        /// Parameter `handler`: The handler to invoke when flush operation is complete. May be nil.
        ///
        /// A flush resets decoder state. The next frame passed to enqueueSampleBuffer: should be an IDR frame (also known as a key frame or sync sample).
        #[unsafe(method(flushWithRemovalOfDisplayedImage:completionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn flushWithRemovalOfDisplayedImage_completionHandler(
            &self,
            remove_displayed_image: bool,
            handler: Option<&block2::DynBlock<dyn Fn()>>,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl AVSampleBufferVideoRenderer {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

/// AVSampleBufferVideoRendererPixelBufferOutput.
impl AVSampleBufferVideoRenderer {
    extern_methods!(
        #[cfg(feature = "objc2-core-video")]
        /// Returns a retained reference to the pixel buffer currently displayed in the AVSampleBufferVideoRenderer's target. This will return NULL if the displayed pixel buffer is protected, no image is currently being displayed, or if the image is unavailable.
        ///
        /// This will return NULL if the rate is non-zero.  Clients must release the pixel buffer after use.
        ///
        /// Do not write to the returned CVPixelBuffer's attachments or pixel data.
        #[unsafe(method(copyDisplayedPixelBuffer))]
        #[unsafe(method_family = copy)]
        pub unsafe fn copyDisplayedPixelBuffer(&self) -> Option<Retained<CVPixelBuffer>>;
    );
}

/// AVSampleBufferVideoRendererPowerOptimization.
impl AVSampleBufferVideoRenderer {
    extern_methods!(
        #[cfg(feature = "objc2-core-media")]
        /// Promises, for the purpose of enabling power optimizations, that future sample buffers will have PTS values no less than a specified lower-bound PTS.
        ///
        /// Only applicable for forward playback.
        /// Sending this message and later calling -enqueueSampleBuffer: with a buffer with a lower PTS has the potential to lead to dropping that later buffer.
        /// For best results, call -expectMinimumUpcomingSampleBufferPresentationTime: regularly, in between calls to -enqueueSampleBuffer:, to advance the lower-bound PTS.
        /// Messaging -flush resets such expectations.
        /// (For example, it's OK to make this expectation, then in response to a seek back, flush and then enqueue buffers with lower PTS values.)
        ///
        /// Parameter `minimumUpcomingPresentationTime`: A lower bound on PTS values for buffers that will be passed to -enqueueSampleBuffer: in the future.
        #[unsafe(method(expectMinimumUpcomingSampleBufferPresentationTime:))]
        #[unsafe(method_family = none)]
        pub unsafe fn expectMinimumUpcomingSampleBufferPresentationTime(
            &self,
            minimum_upcoming_presentation_time: CMTime,
        );

        /// Promises, for the purpose of enabling power optimizations, that future sample buffers will have monotonically increasing PTS values.
        ///
        /// Only applicable for forward playback.
        /// Sending this message and later calling -enqueueSampleBuffer: with a buffer with a lower PTS than any previously enqueued PTS has the potential to lead to dropped buffers.
        /// Messaging -flush resets such expectations.
        #[unsafe(method(expectMonotonicallyIncreasingUpcomingSampleBufferPresentationTimes))]
        #[unsafe(method_family = none)]
        pub unsafe fn expectMonotonicallyIncreasingUpcomingSampleBufferPresentationTimes(&self);

        /// Resets previously-promised expectations about upcoming sample buffer PTSs.
        ///
        /// This undoes the state set by messaging -expectMinimumUpcomingSampleBufferPresentationTime: or -expectMonotonicallyIncreasingUpcomingSampleBufferPresentationTimes.
        /// If you didn't use either of those, you don't have to use this.
        #[unsafe(method(resetUpcomingSampleBufferPresentationTimeExpectations))]
        #[unsafe(method_family = none)]
        pub unsafe fn resetUpcomingSampleBufferPresentationTimeExpectations(&self);
    );
}

/// AVSampleBufferVideoRendererVideoPerformanceMetrics.
impl AVSampleBufferVideoRenderer {
    extern_methods!(
        #[cfg(all(feature = "AVVideoPerformanceMetrics", feature = "block2"))]
        /// Gathers a snapshot of the video performance metrics and calls the completion handler with the results.
        ///
        /// Parameter `completionHandler`: The handler to invoke with the video performance metrics.
        ///
        /// If there are no performance metrics available, the completion handler will be called with nil videoPerformanceMetrics.
        #[unsafe(method(loadVideoPerformanceMetricsWithCompletionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn loadVideoPerformanceMetricsWithCompletionHandler(
            &self,
            completion_handler: &block2::DynBlock<dyn Fn(*mut AVVideoPerformanceMetrics)>,
        );
    );
}
