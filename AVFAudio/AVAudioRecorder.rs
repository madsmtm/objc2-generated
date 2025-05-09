//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// An object that records audio data to a file.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiorecorder?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAudioRecorder;
);

unsafe impl Send for AVAudioRecorder {}

unsafe impl Sync for AVAudioRecorder {}

extern_conformance!(
    unsafe impl NSObjectProtocol for AVAudioRecorder {}
);

impl AVAudioRecorder {
    extern_methods!(
        /// Init the AudioRecorder with a specified url and settings.
        ///
        /// The file type to create can be set through the corresponding settings key. If not set, it will be inferred from the file extension. Will overwrite a file at the specified url if a file exists.
        #[unsafe(method(initWithURL:settings:error:_))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithURL_settings_error(
            this: Allocated<Self>,
            url: &NSURL,
            settings: &NSDictionary<NSString, AnyObject>,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "AVAudioFormat")]
        /// Init the AudioRecorder with a specified url and format.
        ///
        /// The file type to create can be set through the corresponding settings key. If not set, it will be inferred from the file extension. Will overwrite a file at the specified url if a file exists.
        #[unsafe(method(initWithURL:format:error:_))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithURL_format_error(
            this: Allocated<Self>,
            url: &NSURL,
            format: &AVAudioFormat,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        /// Creates the output file and gets ready to record.
        ///
        /// This method is called automatically on record. Returns YES on success and NO on failure.
        #[unsafe(method(prepareToRecord))]
        #[unsafe(method_family = none)]
        pub unsafe fn prepareToRecord(&self) -> bool;

        /// Start or resume recording to file.
        ///
        /// Returns YES on success and NO on failure.
        #[unsafe(method(record))]
        #[unsafe(method_family = none)]
        pub unsafe fn record(&self) -> bool;

        /// Start recording at specified time in the future.
        ///
        /// Time is an absolute time based on and greater than deviceCurrentTime. Returns YES on success and NO on failure.
        #[unsafe(method(recordAtTime:))]
        #[unsafe(method_family = none)]
        pub unsafe fn recordAtTime(&self, time: NSTimeInterval) -> bool;

        /// Record for a specified duration.
        ///
        /// The recorder will stop when it has recorded this length of audio. Returns YES on success and NO on failure.
        #[unsafe(method(recordForDuration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn recordForDuration(&self, duration: NSTimeInterval) -> bool;

        /// Record for a specified duration at a specified time in the future.
        ///
        /// Time is an absolute time based on and greater than deviceCurrentTime. Returns YES on success and NO on failure.
        #[unsafe(method(recordAtTime:forDuration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn recordAtTime_forDuration(
            &self,
            time: NSTimeInterval,
            duration: NSTimeInterval,
        ) -> bool;

        /// Pause recording.
        #[unsafe(method(pause))]
        #[unsafe(method_family = none)]
        pub unsafe fn pause(&self);

        /// Stop recording.
        ///
        /// This method also closes the output file.
        #[unsafe(method(stop))]
        #[unsafe(method_family = none)]
        pub unsafe fn stop(&self);

        /// Delete the recorded file.
        ///
        /// AudioRecorder must be stopped. Returns YES on success and NO on failure.
        #[unsafe(method(deleteRecording))]
        #[unsafe(method_family = none)]
        pub unsafe fn deleteRecording(&self) -> bool;

        /// Returns YES if the AudioRecorder is currently recording.
        #[unsafe(method(isRecording))]
        #[unsafe(method_family = none)]
        pub unsafe fn isRecording(&self) -> bool;

        /// URL of the recorded file.
        #[unsafe(method(url))]
        #[unsafe(method_family = none)]
        pub unsafe fn url(&self) -> Retained<NSURL>;

        /// A dictionary of settings for the AudioRecorder.
        ///
        /// These settings are fully valid only when prepareToRecord has been called. For supported key-value pairs, see https://developer.apple.com/documentation/avfaudio/avaudiorecorder/1388386-initwithurl?language=objc
        #[unsafe(method(settings))]
        #[unsafe(method_family = none)]
        pub unsafe fn settings(&self) -> Retained<NSDictionary<NSString, AnyObject>>;

        #[cfg(feature = "AVAudioFormat")]
        /// The audio format of the AudioRecorder.
        ///
        /// This property is fully valid only when prepareToRecord has been called.
        #[unsafe(method(format))]
        #[unsafe(method_family = none)]
        pub unsafe fn format(&self) -> Retained<AVAudioFormat>;

        /// A delegate object to the AudioRecorder that conforms to the AVAudioRecorderDelegate protocol.
        #[unsafe(method(delegate))]
        #[unsafe(method_family = none)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn AVAudioRecorderDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[unsafe(method(setDelegate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn AVAudioRecorderDelegate>>,
        );

        /// Get the current time of the recording.
        ///
        /// This method is only vaild while recording.
        #[unsafe(method(currentTime))]
        #[unsafe(method_family = none)]
        pub unsafe fn currentTime(&self) -> NSTimeInterval;

        /// Get the device current time.
        ///
        /// This method is always valid.
        #[unsafe(method(deviceCurrentTime))]
        #[unsafe(method_family = none)]
        pub unsafe fn deviceCurrentTime(&self) -> NSTimeInterval;

        /// Turns level metering on or off.
        ///
        /// Default is off.
        #[unsafe(method(isMeteringEnabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn isMeteringEnabled(&self) -> bool;

        /// Setter for [`isMeteringEnabled`][Self::isMeteringEnabled].
        #[unsafe(method(setMeteringEnabled:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMeteringEnabled(&self, metering_enabled: bool);

        /// Call this method to refresh meter values.
        #[unsafe(method(updateMeters))]
        #[unsafe(method_family = none)]
        pub unsafe fn updateMeters(&self);

        /// Returns peak power in decibels for a given channel.
        #[unsafe(method(peakPowerForChannel:))]
        #[unsafe(method_family = none)]
        pub unsafe fn peakPowerForChannel(&self, channel_number: NSUInteger) -> c_float;

        /// Returns average power in decibels for a given channel.
        #[unsafe(method(averagePowerForChannel:))]
        #[unsafe(method_family = none)]
        pub unsafe fn averagePowerForChannel(&self, channel_number: NSUInteger) -> c_float;

        #[cfg(feature = "AVAudioSessionRoute")]
        /// Array of AVAudioSessionChannelDescription objects
        ///
        /// The channels property lets you assign the output to record specific channels as described by AVAudioSessionPortDescription's channels property. This property is nil valued until set. The array must have the same number of channels as returned by the numberOfChannels property.
        #[unsafe(method(channelAssignments))]
        #[unsafe(method_family = none)]
        pub unsafe fn channelAssignments(
            &self,
        ) -> Option<Retained<NSArray<AVAudioSessionChannelDescription>>>;

        #[cfg(feature = "AVAudioSessionRoute")]
        /// Setter for [`channelAssignments`][Self::channelAssignments].
        #[unsafe(method(setChannelAssignments:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setChannelAssignments(
            &self,
            channel_assignments: Option<&NSArray<AVAudioSessionChannelDescription>>,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl AVAudioRecorder {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_protocol!(
    /// A protocol for delegates of AVAudioRecorder.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiorecorderdelegate?language=objc)
    pub unsafe trait AVAudioRecorderDelegate: NSObjectProtocol {
        /// This callback method is called when a recording has been finished or stopped.
        ///
        /// This method is NOT called if the recorder is stopped due to an interruption.
        #[optional]
        #[unsafe(method(audioRecorderDidFinishRecording:successfully:))]
        #[unsafe(method_family = none)]
        unsafe fn audioRecorderDidFinishRecording_successfully(
            &self,
            recorder: &AVAudioRecorder,
            flag: bool,
        );

        /// This callback method is called when an error occurs while encoding.
        ///
        /// If an error occurs while encoding it will be reported to the delegate.
        #[optional]
        #[unsafe(method(audioRecorderEncodeErrorDidOccur:error:))]
        #[unsafe(method_family = none)]
        unsafe fn audioRecorderEncodeErrorDidOccur_error(
            &self,
            recorder: &AVAudioRecorder,
            error: Option<&NSError>,
        );
    }
);
