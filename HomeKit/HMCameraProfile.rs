//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// Represents a camera profile the accessory implements.
    ///
    ///
    /// Provides an interface to interact with a Camera in an Accessory.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/homekit/hmcameraprofile?language=objc)
    #[unsafe(super(HMAccessoryProfile, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HMAccessoryProfile")]
    pub struct HMCameraProfile;
);

#[cfg(feature = "HMAccessoryProfile")]
unsafe impl Send for HMCameraProfile {}

#[cfg(feature = "HMAccessoryProfile")]
unsafe impl Sync for HMCameraProfile {}

#[cfg(feature = "HMAccessoryProfile")]
extern_conformance!(
    unsafe impl NSObjectProtocol for HMCameraProfile {}
);

#[cfg(feature = "HMAccessoryProfile")]
impl HMCameraProfile {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(feature = "HMCameraControl", feature = "HMCameraStreamControl"))]
        /// Object that can be used to control the camera stream.
        #[unsafe(method(streamControl))]
        #[unsafe(method_family = none)]
        pub unsafe fn streamControl(&self) -> Option<Retained<HMCameraStreamControl>>;

        #[cfg(all(feature = "HMCameraControl", feature = "HMCameraSnapshotControl"))]
        /// Object that can be used to take image snapshots from the camera.
        #[unsafe(method(snapshotControl))]
        #[unsafe(method_family = none)]
        pub unsafe fn snapshotControl(&self) -> Option<Retained<HMCameraSnapshotControl>>;

        #[cfg(all(feature = "HMCameraControl", feature = "HMCameraSettingsControl"))]
        /// Object that can be used to control the settings on the camera.
        #[unsafe(method(settingsControl))]
        #[unsafe(method_family = none)]
        pub unsafe fn settingsControl(&self) -> Option<Retained<HMCameraSettingsControl>>;

        #[cfg(all(feature = "HMCameraAudioControl", feature = "HMCameraControl"))]
        /// Object that can be used to control the speaker settings on the camera.
        #[unsafe(method(speakerControl))]
        #[unsafe(method_family = none)]
        pub unsafe fn speakerControl(&self) -> Option<Retained<HMCameraAudioControl>>;

        #[cfg(all(feature = "HMCameraAudioControl", feature = "HMCameraControl"))]
        /// Object that can be used to control the microphone settings on the camera.
        #[unsafe(method(microphoneControl))]
        #[unsafe(method_family = none)]
        pub unsafe fn microphoneControl(&self) -> Option<Retained<HMCameraAudioControl>>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "HMAccessoryProfile")]
impl HMCameraProfile {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
