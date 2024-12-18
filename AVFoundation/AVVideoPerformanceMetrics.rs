//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avvideoperformancemetrics?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVVideoPerformanceMetrics;
);

unsafe impl Send for AVVideoPerformanceMetrics {}

unsafe impl Sync for AVVideoPerformanceMetrics {}

unsafe impl NSObjectProtocol for AVVideoPerformanceMetrics {}

extern_methods!(
    unsafe impl AVVideoPerformanceMetrics {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method(totalNumberOfFrames)]
        pub unsafe fn totalNumberOfFrames(&self) -> NSInteger;

        #[method(numberOfDroppedFrames)]
        pub unsafe fn numberOfDroppedFrames(&self) -> NSInteger;

        #[method(numberOfCorruptedFrames)]
        pub unsafe fn numberOfCorruptedFrames(&self) -> NSInteger;

        #[method(numberOfFramesDisplayedUsingOptimizedCompositing)]
        pub unsafe fn numberOfFramesDisplayedUsingOptimizedCompositing(&self) -> NSInteger;

        #[method(totalAccumulatedFrameDelay)]
        pub unsafe fn totalAccumulatedFrameDelay(&self) -> NSTimeInterval;

        #[method(totalNumberOfVideoFrames)]
        pub unsafe fn totalNumberOfVideoFrames(&self) -> c_ulong;

        #[method(numberOfDroppedVideoFrames)]
        pub unsafe fn numberOfDroppedVideoFrames(&self) -> c_ulong;

        #[method(numberOfCorruptedVideoFrames)]
        pub unsafe fn numberOfCorruptedVideoFrames(&self) -> c_ulong;

        #[method(numberOfDisplayCompositedVideoFrames)]
        pub unsafe fn numberOfDisplayCompositedVideoFrames(&self) -> c_ulong;

        #[method(numberOfNonDisplayCompositedVideoFrames)]
        pub unsafe fn numberOfNonDisplayCompositedVideoFrames(&self) -> c_ulong;

        #[method(totalFrameDelay)]
        pub unsafe fn totalFrameDelay(&self) -> c_double;
    }
);
