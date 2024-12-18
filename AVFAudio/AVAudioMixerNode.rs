//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiomixernode?language=objc)
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
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(outputVolume)]
        pub unsafe fn outputVolume(&self) -> c_float;

        #[method(setOutputVolume:)]
        pub unsafe fn setOutputVolume(&self, output_volume: c_float);

        #[cfg(feature = "AVAudioTypes")]
        #[method(nextAvailableInputBus)]
        pub unsafe fn nextAvailableInputBus(&self) -> AVAudioNodeBus;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AVAudioNode")]
    unsafe impl AVAudioMixerNode {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
