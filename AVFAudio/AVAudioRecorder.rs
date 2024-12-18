//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiorecorder?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAudioRecorder;
);

unsafe impl Send for AVAudioRecorder {}

unsafe impl Sync for AVAudioRecorder {}

unsafe impl NSObjectProtocol for AVAudioRecorder {}

extern_methods!(
    unsafe impl AVAudioRecorder {
        #[method_id(@__retain_semantics Init initWithURL:settings:error:_)]
        pub unsafe fn initWithURL_settings_error(
            this: Allocated<Self>,
            url: &NSURL,
            settings: &NSDictionary<NSString, AnyObject>,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "AVAudioFormat")]
        #[method_id(@__retain_semantics Init initWithURL:format:error:_)]
        pub unsafe fn initWithURL_format_error(
            this: Allocated<Self>,
            url: &NSURL,
            format: &AVAudioFormat,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[method(prepareToRecord)]
        pub unsafe fn prepareToRecord(&self) -> bool;

        #[method(record)]
        pub unsafe fn record(&self) -> bool;

        #[method(recordAtTime:)]
        pub unsafe fn recordAtTime(&self, time: NSTimeInterval) -> bool;

        #[method(recordForDuration:)]
        pub unsafe fn recordForDuration(&self, duration: NSTimeInterval) -> bool;

        #[method(recordAtTime:forDuration:)]
        pub unsafe fn recordAtTime_forDuration(
            &self,
            time: NSTimeInterval,
            duration: NSTimeInterval,
        ) -> bool;

        #[method(pause)]
        pub unsafe fn pause(&self);

        #[method(stop)]
        pub unsafe fn stop(&self);

        #[method(deleteRecording)]
        pub unsafe fn deleteRecording(&self) -> bool;

        #[method(isRecording)]
        pub unsafe fn isRecording(&self) -> bool;

        #[method_id(@__retain_semantics Other url)]
        pub unsafe fn url(&self) -> Retained<NSURL>;

        #[method_id(@__retain_semantics Other settings)]
        pub unsafe fn settings(&self) -> Retained<NSDictionary<NSString, AnyObject>>;

        #[cfg(feature = "AVAudioFormat")]
        #[method_id(@__retain_semantics Other format)]
        pub unsafe fn format(&self) -> Retained<AVAudioFormat>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn AVAudioRecorderDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn AVAudioRecorderDelegate>>,
        );

        #[method(currentTime)]
        pub unsafe fn currentTime(&self) -> NSTimeInterval;

        #[method(deviceCurrentTime)]
        pub unsafe fn deviceCurrentTime(&self) -> NSTimeInterval;

        #[method(isMeteringEnabled)]
        pub unsafe fn isMeteringEnabled(&self) -> bool;

        #[method(setMeteringEnabled:)]
        pub unsafe fn setMeteringEnabled(&self, metering_enabled: bool);

        #[method(updateMeters)]
        pub unsafe fn updateMeters(&self);

        #[method(peakPowerForChannel:)]
        pub unsafe fn peakPowerForChannel(&self, channel_number: NSUInteger) -> c_float;

        #[method(averagePowerForChannel:)]
        pub unsafe fn averagePowerForChannel(&self, channel_number: NSUInteger) -> c_float;

        #[cfg(feature = "AVAudioSessionRoute")]
        #[method_id(@__retain_semantics Other channelAssignments)]
        pub unsafe fn channelAssignments(
            &self,
        ) -> Option<Retained<NSArray<AVAudioSessionChannelDescription>>>;

        #[cfg(feature = "AVAudioSessionRoute")]
        #[method(setChannelAssignments:)]
        pub unsafe fn setChannelAssignments(
            &self,
            channel_assignments: Option<&NSArray<AVAudioSessionChannelDescription>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVAudioRecorder {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiorecorderdelegate?language=objc)
    pub unsafe trait AVAudioRecorderDelegate: NSObjectProtocol {
        #[optional]
        #[method(audioRecorderDidFinishRecording:successfully:)]
        unsafe fn audioRecorderDidFinishRecording_successfully(
            &self,
            recorder: &AVAudioRecorder,
            flag: bool,
        );

        #[optional]
        #[method(audioRecorderEncodeErrorDidOccur:error:)]
        unsafe fn audioRecorderEncodeErrorDidOccur_error(
            &self,
            recorder: &AVAudioRecorder,
            error: Option<&NSError>,
        );
    }

    unsafe impl ProtocolType for dyn AVAudioRecorderDelegate {}
);
