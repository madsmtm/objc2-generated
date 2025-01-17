//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// A node that mixes its inputs to a single output.
    ///
    /// Mixers may have any number of inputs.
    ///
    /// The mixer accepts input at any sample rate and efficiently combines sample rate
    /// conversions. It also accepts any channel count and will correctly upmix or downmix
    /// to the output channel count.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiomixernode?language=objc)
    #[unsafe(super(AVAudioNode, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AVAudioNode")]
    pub struct AVAudioMixerNode;
);

#[cfg(all(feature = "AVAudioMixing", feature = "AVAudioNode"))]
unsafe impl AVAudio3DMixing for AVAudioMixerNode {}

#[cfg(all(feature = "AVAudioMixing", feature = "AVAudioNode"))]
unsafe impl AVAudioMixing for AVAudioMixerNode {}

#[cfg(all(feature = "AVAudioMixing", feature = "AVAudioNode"))]
unsafe impl AVAudioStereoMixing for AVAudioMixerNode {}

#[cfg(feature = "AVAudioNode")]
unsafe impl NSObjectProtocol for AVAudioMixerNode {}

extern_methods!(
    #[cfg(feature = "AVAudioNode")]
    unsafe impl AVAudioMixerNode {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// The mixer's output volume.
        ///
        /// This accesses the mixer's output volume (0.0-1.0, inclusive).
        #[method(outputVolume)]
        pub unsafe fn outputVolume(&self) -> c_float;

        /// Setter for [`outputVolume`][Self::outputVolume].
        #[method(setOutputVolume:)]
        pub unsafe fn setOutputVolume(&self, output_volume: c_float);

        #[cfg(feature = "AVAudioTypes")]
        /// Find an unused input bus.
        ///
        /// This will find and return the first input bus to which no other node is connected.
        #[method(nextAvailableInputBus)]
        pub unsafe fn nextAvailableInputBus(&self) -> AVAudioNodeBus;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AVAudioNode")]
    unsafe impl AVAudioMixerNode {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
