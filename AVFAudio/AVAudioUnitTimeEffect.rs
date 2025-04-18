//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-audio-toolbox")]
#[cfg(not(target_os = "watchos"))]
use objc2_audio_toolbox::*;

use crate::*;

extern_class!(
    /// an AVAudioUnit that processes audio in non real-time
    ///
    /// An AVAudioUnitTimeEffect represents an audio unit of type aufc.
    /// These effects do not process audio in real-time. The varispeed
    /// unit is an example of a time effect unit.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiounittimeeffect?language=objc)
    #[unsafe(super(AVAudioUnit, AVAudioNode, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "AVAudioNode", feature = "AVAudioUnit"))]
    pub struct AVAudioUnitTimeEffect;
);

#[cfg(all(feature = "AVAudioNode", feature = "AVAudioUnit"))]
extern_conformance!(
    unsafe impl NSObjectProtocol for AVAudioUnitTimeEffect {}
);

#[cfg(all(feature = "AVAudioNode", feature = "AVAudioUnit"))]
impl AVAudioUnitTimeEffect {
    extern_methods!(
        #[cfg(feature = "objc2-audio-toolbox")]
        #[cfg(not(target_os = "watchos"))]
        /// create an AVAudioUnitTimeEffect object
        ///
        ///
        /// Parameter `audioComponentDescription`: AudioComponentDescription of the audio unit to be initialized
        ///
        /// The componentType must be kAudioUnitType_FormatConverter
        #[unsafe(method(initWithAudioComponentDescription:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithAudioComponentDescription(
            this: Allocated<Self>,
            audio_component_description: AudioComponentDescription,
        ) -> Retained<Self>;

        /// bypass state of the audio unit
        #[unsafe(method(bypass))]
        #[unsafe(method_family = none)]
        pub unsafe fn bypass(&self) -> bool;

        /// Setter for [`bypass`][Self::bypass].
        #[unsafe(method(setBypass:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setBypass(&self, bypass: bool);
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(feature = "AVAudioNode", feature = "AVAudioUnit"))]
impl AVAudioUnitTimeEffect {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
