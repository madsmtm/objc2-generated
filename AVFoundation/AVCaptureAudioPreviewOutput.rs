//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// AVCaptureAudioPreviewOutput is a concrete subclass of AVCaptureOutput that can be used to preview the audio being captured.
    ///
    ///
    /// Instances of AVCaptureAudioPreviewOutput have an associated Core Audio output device that can be used to play audio being captured by the capture session. The unique ID of a Core Audio device can be obtained from its kAudioDevicePropertyDeviceUID property.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptureaudiopreviewoutput?language=objc)
    #[unsafe(super(AVCaptureOutput, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AVCaptureOutputBase")]
    pub struct AVCaptureAudioPreviewOutput;
);

#[cfg(feature = "AVCaptureOutputBase")]
extern_conformance!(
    unsafe impl NSObjectProtocol for AVCaptureAudioPreviewOutput {}
);

#[cfg(feature = "AVCaptureOutputBase")]
impl AVCaptureAudioPreviewOutput {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        /// Specifies the unique ID of the Core Audio output device being used to play preview audio.
        ///
        ///
        /// The value of this property is an NSString containing the unique ID of the Core Audio device to be used for output, or nil if the default system output should be used.
        #[unsafe(method(outputDeviceUniqueID))]
        #[unsafe(method_family = none)]
        pub unsafe fn outputDeviceUniqueID(&self) -> Option<Retained<NSString>>;

        /// Setter for [`outputDeviceUniqueID`][Self::outputDeviceUniqueID].
        #[unsafe(method(setOutputDeviceUniqueID:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setOutputDeviceUniqueID(&self, output_device_unique_id: Option<&NSString>);

        /// Specifies the preview volume of the output.
        ///
        ///
        /// The value of this property is the preview volume of the receiver, where 1.0 is the maximum volume and 0.0 is muted.
        #[unsafe(method(volume))]
        #[unsafe(method_family = none)]
        pub unsafe fn volume(&self) -> c_float;

        /// Setter for [`volume`][Self::volume].
        #[unsafe(method(setVolume:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setVolume(&self, volume: c_float);
    );
}
