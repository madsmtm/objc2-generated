//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/speech/sfspeechrecognitionmetadata?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SFSpeechRecognitionMetadata;
);

unsafe impl NSCoding for SFSpeechRecognitionMetadata {}

unsafe impl NSCopying for SFSpeechRecognitionMetadata {}

unsafe impl CopyingHelper for SFSpeechRecognitionMetadata {
    type Result = Self;
}

unsafe impl NSObjectProtocol for SFSpeechRecognitionMetadata {}

unsafe impl NSSecureCoding for SFSpeechRecognitionMetadata {}

extern_methods!(
    unsafe impl SFSpeechRecognitionMetadata {
        #[method(speakingRate)]
        pub unsafe fn speakingRate(&self) -> c_double;

        #[method(averagePauseDuration)]
        pub unsafe fn averagePauseDuration(&self) -> NSTimeInterval;

        #[method(speechStartTimestamp)]
        pub unsafe fn speechStartTimestamp(&self) -> NSTimeInterval;

        #[method(speechDuration)]
        pub unsafe fn speechDuration(&self) -> NSTimeInterval;

        #[cfg(feature = "SFVoiceAnalytics")]
        #[unsafe(method_family(none))]
        #[method_id(voiceAnalytics)]
        pub unsafe fn voiceAnalytics(&self) -> Option<Retained<SFVoiceAnalytics>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SFSpeechRecognitionMetadata {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
