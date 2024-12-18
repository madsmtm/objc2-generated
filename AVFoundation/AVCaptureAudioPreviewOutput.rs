//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptureaudiopreviewoutput?language=objc)
    #[unsafe(super(AVCaptureOutput, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AVCaptureOutputBase")]
    pub struct AVCaptureAudioPreviewOutput;
);

#[cfg(feature = "AVCaptureOutputBase")]
unsafe impl NSObjectProtocol for AVCaptureAudioPreviewOutput {}

extern_methods!(
    #[cfg(feature = "AVCaptureOutputBase")]
    unsafe impl AVCaptureAudioPreviewOutput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other outputDeviceUniqueID)]
        pub unsafe fn outputDeviceUniqueID(&self) -> Option<Retained<NSString>>;

        #[method(setOutputDeviceUniqueID:)]
        pub unsafe fn setOutputDeviceUniqueID(&self, output_device_unique_id: Option<&NSString>);

        #[method(volume)]
        pub unsafe fn volume(&self) -> c_float;

        #[method(setVolume:)]
        pub unsafe fn setVolume(&self, volume: c_float);
    }
);
