//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-video")]
use objc2_core_video::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avplayerlayer?language=objc)
    #[unsafe(super(CALayer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-quartz-core")]
    #[cfg(not(target_os = "watchos"))]
    pub struct AVPlayerLayer;
);

#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
unsafe impl CAMediaTiming for AVPlayerLayer {}

#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
unsafe impl NSCoding for AVPlayerLayer {}

#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
unsafe impl NSObjectProtocol for AVPlayerLayer {}

#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
unsafe impl NSSecureCoding for AVPlayerLayer {}

extern_methods!(
    #[cfg(feature = "objc2-quartz-core")]
    #[cfg(not(target_os = "watchos"))]
    unsafe impl AVPlayerLayer {
        #[cfg(feature = "AVPlayer")]
        #[method_id(@__retain_semantics Other playerLayerWithPlayer:)]
        pub unsafe fn playerLayerWithPlayer(player: Option<&AVPlayer>) -> Retained<AVPlayerLayer>;

        #[cfg(feature = "AVPlayer")]
        #[method_id(@__retain_semantics Other player)]
        pub unsafe fn player(&self, mtm: MainThreadMarker) -> Option<Retained<AVPlayer>>;

        #[cfg(feature = "AVPlayer")]
        #[method(setPlayer:)]
        pub unsafe fn setPlayer(&self, player: Option<&AVPlayer>);

        #[cfg(feature = "AVAnimation")]
        #[method_id(@__retain_semantics Other videoGravity)]
        pub unsafe fn videoGravity(&self) -> Retained<AVLayerVideoGravity>;

        #[cfg(feature = "AVAnimation")]
        #[method(setVideoGravity:)]
        pub unsafe fn setVideoGravity(&self, video_gravity: &AVLayerVideoGravity);

        #[method(isReadyForDisplay)]
        pub unsafe fn isReadyForDisplay(&self) -> bool;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(videoRect)]
        pub unsafe fn videoRect(&self) -> CGRect;

        #[method_id(@__retain_semantics Other pixelBufferAttributes)]
        pub unsafe fn pixelBufferAttributes(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[method(setPixelBufferAttributes:)]
        pub unsafe fn setPixelBufferAttributes(
            &self,
            pixel_buffer_attributes: Option<&NSDictionary<NSString, AnyObject>>,
        );

        #[cfg(feature = "objc2-core-video")]
        #[method(copyDisplayedPixelBuffer)]
        pub unsafe fn copyDisplayedPixelBuffer(&self) -> CVPixelBufferRef;
    }
);

extern_methods!(
    /// Methods declared on superclass `CALayer`
    #[cfg(feature = "objc2-quartz-core")]
    #[cfg(not(target_os = "watchos"))]
    unsafe impl AVPlayerLayer {
        #[method_id(@__retain_semantics Other layer)]
        pub unsafe fn layer() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn initWithLayer(this: Allocated<Self>, layer: &AnyObject) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "objc2-quartz-core")]
    #[cfg(not(target_os = "watchos"))]
    unsafe impl AVPlayerLayer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
