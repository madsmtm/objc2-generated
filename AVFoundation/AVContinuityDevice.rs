//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-avf-audio")]
use objc2_avf_audio::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcontinuitydevice?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVContinuityDevice;
);

unsafe impl NSObjectProtocol for AVContinuityDevice {}

extern_methods!(
    unsafe impl AVContinuityDevice {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other connectionID)]
        pub unsafe fn connectionID(&self) -> Retained<NSUUID>;

        #[method(isConnected)]
        pub unsafe fn isConnected(&self) -> bool;

        #[cfg(feature = "AVCaptureDevice")]
        #[method_id(@__retain_semantics Other videoDevices)]
        pub unsafe fn videoDevices(&self) -> Retained<NSArray<AVCaptureDevice>>;

        #[cfg(feature = "objc2-avf-audio")]
        #[method_id(@__retain_semantics Other audioSessionInputs)]
        pub unsafe fn audioSessionInputs(&self)
            -> Retained<NSArray<AVAudioSessionPortDescription>>;
    }
);
