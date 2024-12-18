//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avsamplebufferrendersynchronizerratedidchangenotification?language=objc)
    pub static AVSampleBufferRenderSynchronizerRateDidChangeNotification:
        &'static NSNotificationName;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avsamplebufferrendersynchronizer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVSampleBufferRenderSynchronizer;
);

unsafe impl NSObjectProtocol for AVSampleBufferRenderSynchronizer {}

extern_methods!(
    unsafe impl AVSampleBufferRenderSynchronizer {
        #[cfg(feature = "objc2-core-media")]
        #[method(timebase)]
        pub unsafe fn timebase(&self) -> CMTimebaseRef;

        #[method(rate)]
        pub unsafe fn rate(&self) -> c_float;

        #[method(setRate:)]
        pub unsafe fn setRate(&self, rate: c_float);

        #[cfg(feature = "objc2-core-media")]
        #[method(currentTime)]
        pub unsafe fn currentTime(&self) -> CMTime;

        #[cfg(feature = "objc2-core-media")]
        #[method(setRate:time:)]
        pub unsafe fn setRate_time(&self, rate: c_float, time: CMTime);

        #[cfg(feature = "objc2-core-media")]
        #[method(setRate:time:atHostTime:)]
        pub unsafe fn setRate_time_atHostTime(
            &self,
            rate: c_float,
            time: CMTime,
            host_time: CMTime,
        );

        #[method(delaysRateChangeUntilHasSufficientMediaData)]
        pub unsafe fn delaysRateChangeUntilHasSufficientMediaData(&self) -> bool;

        #[method(setDelaysRateChangeUntilHasSufficientMediaData:)]
        pub unsafe fn setDelaysRateChangeUntilHasSufficientMediaData(
            &self,
            delays_rate_change_until_has_sufficient_media_data: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVSampleBufferRenderSynchronizer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// AVSampleBufferRenderSynchronizerRendererManagement
    unsafe impl AVSampleBufferRenderSynchronizer {
        #[cfg(feature = "AVQueuedSampleBufferRendering")]
        #[method_id(@__retain_semantics Other renderers)]
        pub unsafe fn renderers(
            &self,
        ) -> Retained<NSArray<ProtocolObject<dyn AVQueuedSampleBufferRendering>>>;

        #[cfg(feature = "AVQueuedSampleBufferRendering")]
        #[method(addRenderer:)]
        pub unsafe fn addRenderer(
            &self,
            renderer: &ProtocolObject<dyn AVQueuedSampleBufferRendering>,
        );

        #[cfg(all(
            feature = "AVQueuedSampleBufferRendering",
            feature = "block2",
            feature = "objc2-core-media"
        ))]
        #[method(removeRenderer:atTime:completionHandler:)]
        pub unsafe fn removeRenderer_atTime_completionHandler(
            &self,
            renderer: &ProtocolObject<dyn AVQueuedSampleBufferRendering>,
            time: CMTime,
            completion_handler: Option<&block2::Block<dyn Fn(Bool)>>,
        );
    }
);

extern_methods!(
    /// AVSampleBufferRenderSynchronizerTimeObservation
    unsafe impl AVSampleBufferRenderSynchronizer {
        #[method(removeTimeObserver:)]
        pub unsafe fn removeTimeObserver(&self, observer: &AnyObject);
    }
);
