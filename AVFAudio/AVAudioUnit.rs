//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-audio-toolbox")]
#[cfg(not(target_os = "watchos"))]
use objc2_audio_toolbox::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiounit?language=objc)
    #[unsafe(super(AVAudioNode, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AVAudioNode")]
    pub struct AVAudioUnit;
);

#[cfg(feature = "AVAudioNode")]
unsafe impl NSObjectProtocol for AVAudioUnit {}

extern_methods!(
    #[cfg(feature = "AVAudioNode")]
    unsafe impl AVAudioUnit {
        #[cfg(all(feature = "block2", feature = "objc2-audio-toolbox"))]
        #[cfg(not(target_os = "watchos"))]
        #[method(instantiateWithComponentDescription:options:completionHandler:)]
        pub unsafe fn instantiateWithComponentDescription_options_completionHandler(
            audio_component_description: AudioComponentDescription,
            options: AudioComponentInstantiationOptions,
            completion_handler: &block2::Block<dyn Fn(*mut AVAudioUnit, *mut NSError)>,
        );

        #[method(loadAudioUnitPresetAtURL:error:_)]
        pub unsafe fn loadAudioUnitPresetAtURL_error(
            &self,
            url: &NSURL,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "objc2-audio-toolbox")]
        #[cfg(not(target_os = "watchos"))]
        #[method(audioComponentDescription)]
        pub unsafe fn audioComponentDescription(&self) -> AudioComponentDescription;

        #[cfg(feature = "objc2-audio-toolbox")]
        #[cfg(not(target_os = "watchos"))]
        #[method(audioUnit)]
        pub unsafe fn audioUnit(&self) -> AudioUnit;

        #[cfg(feature = "objc2-audio-toolbox")]
        #[cfg(not(target_os = "watchos"))]
        #[method_id(@__retain_semantics Other AUAudioUnit)]
        pub unsafe fn AUAudioUnit(&self) -> Retained<AUAudioUnit>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other manufacturerName)]
        pub unsafe fn manufacturerName(&self) -> Retained<NSString>;

        #[method(version)]
        pub unsafe fn version(&self) -> NSUInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AVAudioNode")]
    unsafe impl AVAudioUnit {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
