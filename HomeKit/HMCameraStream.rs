//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Represents a camera stream.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/homekit/hmcamerastream?language=objc)
    #[unsafe(super(HMCameraSource, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HMCameraSource")]
    pub struct HMCameraStream;
);

#[cfg(feature = "HMCameraSource")]
unsafe impl Send for HMCameraStream {}

#[cfg(feature = "HMCameraSource")]
unsafe impl Sync for HMCameraStream {}

#[cfg(feature = "HMCameraSource")]
extern_conformance!(
    unsafe impl NSObjectProtocol for HMCameraStream {}
);

#[cfg(feature = "HMCameraSource")]
impl HMCameraStream {
    extern_methods!(
        #[cfg(feature = "HMCameraDefines")]
        /// Represents the audio setting for the current stream.
        #[unsafe(method(audioStreamSetting))]
        #[unsafe(method_family = none)]
        pub unsafe fn audioStreamSetting(&self) -> HMCameraAudioStreamSetting;

        #[cfg(feature = "HMCameraDefines")]
        /// Sets the audio stream setting.
        ///
        ///
        /// Parameter `audioStreamSetting`: New audio stream setting.
        #[deprecated]
        #[unsafe(method(setAudioStreamSetting:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAudioStreamSetting(
            &self,
            audio_stream_setting: HMCameraAudioStreamSetting,
        );

        #[cfg(all(feature = "HMCameraDefines", feature = "block2"))]
        /// Updates the settings of the audio stream.
        ///
        ///
        /// Parameter `audioStreamSetting`: New audio stream setting. Bidirectional audio is not allowed on TVOS.
        ///
        ///
        /// Parameter `completion`: Block that is invoked once the request is processed.
        /// The NSError provides more information on the status of the request, error
        /// will be nil on success.
        #[unsafe(method(updateAudioStreamSetting:completionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn updateAudioStreamSetting_completionHandler(
            &self,
            audio_stream_setting: HMCameraAudioStreamSetting,
            completion: &block2::DynBlock<dyn Fn(*mut NSError)>,
        );

        #[deprecated = "HMCameraStream objects are created by their parent container objects. Directly creating them is not supported."]
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "HMCameraSource")]
impl HMCameraStream {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
