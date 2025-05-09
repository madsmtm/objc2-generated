//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avsamplebufferaudiorenderer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVSampleBufferAudioRenderer;
);

#[cfg(feature = "AVQueuedSampleBufferRendering")]
extern_conformance!(
    unsafe impl AVQueuedSampleBufferRendering for AVSampleBufferAudioRenderer {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for AVSampleBufferAudioRenderer {}
);

impl AVSampleBufferAudioRenderer {
    extern_methods!(
        #[cfg(feature = "AVQueuedSampleBufferRendering")]
        #[unsafe(method(status))]
        #[unsafe(method_family = none)]
        pub unsafe fn status(&self) -> AVQueuedSampleBufferRenderingStatus;

        #[unsafe(method(error))]
        #[unsafe(method_family = none)]
        pub unsafe fn error(&self) -> Option<Retained<NSError>>;

        /// Specifies the unique ID of the Core Audio output device used to play audio.
        ///
        /// By default, the value of this property is nil, indicating that the default audio output device is used. Otherwise the value of this property is an NSString containing the unique ID of the Core Audio output device to be used for audio output.
        ///
        /// Core Audio's kAudioDevicePropertyDeviceUID is a suitable source of audio output device unique IDs.
        ///
        /// Modifying this property while the timebase's rate is not 0.0 may cause the rate to briefly change to 0.0.
        ///
        /// On macOS, the audio device clock may be used as the AVSampleBufferRenderSynchronizer's and all attached AVQueuedSampleBufferRendering's timebase's clocks.  If the audioOutputDeviceUniqueID is modified, the clocks of all these timebases may also change.
        ///
        /// If multiple AVSampleBufferAudioRenderers with different values for audioOutputDeviceUniqueID are attached to the same AVSampleBufferRenderSynchronizer, audio may not stay in sync during playback.  To avoid this, ensure that all synchronized AVSampleBufferAudioRenderers are using the same audio output device.
        #[unsafe(method(audioOutputDeviceUniqueID))]
        #[unsafe(method_family = none)]
        pub unsafe fn audioOutputDeviceUniqueID(&self) -> Option<Retained<NSString>>;

        /// Setter for [`audioOutputDeviceUniqueID`][Self::audioOutputDeviceUniqueID].
        #[unsafe(method(setAudioOutputDeviceUniqueID:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAudioOutputDeviceUniqueID(
            &self,
            audio_output_device_unique_id: Option<&NSString>,
        );

        #[cfg(feature = "AVAudioProcessingSettings")]
        /// Indicates the processing algorithm used to manage audio pitch at varying rates.
        ///
        /// Constants for various time pitch algorithms, e.g. AVAudioTimePitchSpectral, are defined in AVAudioProcessingSettings.h.
        ///
        /// The default value for applications linked on or after iOS 15.0 or macOS 12.0 is AVAudioTimePitchAlgorithmTimeDomain. For iOS versions prior to 15.0 the default value is AVAudioTimePitchAlgorithmLowQualityZeroLatency.
        /// For macOS versions prior to 12.0 the default value is AVAudioTimePitchAlgorithmSpectral.
        ///
        /// If the timebase's rate is not supported by the audioTimePitchAlgorithm, audio will be muted.
        ///
        /// Modifying this property while the timebase's rate is not 0.0 may cause the rate to briefly change to 0.0.
        #[unsafe(method(audioTimePitchAlgorithm))]
        #[unsafe(method_family = none)]
        pub unsafe fn audioTimePitchAlgorithm(&self) -> Retained<AVAudioTimePitchAlgorithm>;

        #[cfg(feature = "AVAudioProcessingSettings")]
        /// Setter for [`audioTimePitchAlgorithm`][Self::audioTimePitchAlgorithm].
        #[unsafe(method(setAudioTimePitchAlgorithm:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAudioTimePitchAlgorithm(
            &self,
            audio_time_pitch_algorithm: &AVAudioTimePitchAlgorithm,
        );

        #[cfg(feature = "AVAudioProcessingSettings")]
        /// Indicates the source audio channel layouts allowed by the receiver for spatialization.
        ///
        /// Spatialization uses psychoacoustic methods to create a more immersive audio rendering when the content is played on specialized headphones and speaker arrangements. When an  AVSampleBufferAudioRenderer's allowedAudioSpatializationFormats property is set to AVAudioSpatializationFormatMonoAndStereo the  AVSampleBufferAudioRenderer will attempt to spatialize content tagged with a stereo channel layout, two-channel content with no layout specified as well as mono. It is considered incorrect to render a binaural recording with spatialization. A binaural recording is captured using two carefully placed microphones at each ear where the intent, when played on headphones, is to reproduce a naturally occurring spatial effect. Content tagged with a binaural channel layout will ignore this property value. When an  AVSampleBufferAudioRenderer's allowedAudioSpatializationFormats property is set to AVAudioSpatializationFormatMultichannel the  AVSampleBufferAudioRenderer will attempt to spatialize any decodable multichannel layout. Setting this property to AVAudioSpatializationFormatMonoStereoAndMultichannel indicates that the sender allows the  AVSampleBufferAudioRenderer to spatialize any decodable mono, stereo or multichannel layout. This property is not observable. The default value for this property is AVAudioSpatializationFormatMultichannel.
        #[unsafe(method(allowedAudioSpatializationFormats))]
        #[unsafe(method_family = none)]
        pub unsafe fn allowedAudioSpatializationFormats(&self) -> AVAudioSpatializationFormats;

        #[cfg(feature = "AVAudioProcessingSettings")]
        /// Setter for [`allowedAudioSpatializationFormats`][Self::allowedAudioSpatializationFormats].
        #[unsafe(method(setAllowedAudioSpatializationFormats:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAllowedAudioSpatializationFormats(
            &self,
            allowed_audio_spatialization_formats: AVAudioSpatializationFormats,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl AVSampleBufferAudioRenderer {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

/// AVSampleBufferAudioRendererVolumeControl.
impl AVSampleBufferAudioRenderer {
    extern_methods!(
        #[unsafe(method(volume))]
        #[unsafe(method_family = none)]
        pub unsafe fn volume(&self) -> c_float;

        /// Setter for [`volume`][Self::volume].
        #[unsafe(method(setVolume:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setVolume(&self, volume: c_float);

        #[unsafe(method(isMuted))]
        #[unsafe(method_family = none)]
        pub unsafe fn isMuted(&self) -> bool;

        /// Setter for [`isMuted`][Self::isMuted].
        #[unsafe(method(setMuted:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMuted(&self, muted: bool);
    );
}

extern "C" {
    /// A notification that fires whenever the receiver's enqueued media data has been flushed for a reason other than a call to the -flush method.
    ///
    /// The renderer may flush enqueued media data when the user routes playback to a new destination.  The renderer may also flush enqueued media data when the playback rate of the attached AVSampleBufferRenderSynchronizer is changed (e.g. 1.0 -> 2.0 or 1.0 -> 0.0 -> 2.0), however no flush will occur for normal pauses (non-zero -> 0.0) and resumes (0.0 -> same non-zero rate as before).
    ///
    /// When an automatic flush occurs, the attached render synchronizer's timebase will remain running at its current rate.  It is typically best to respond to this notification by enqueueing media data with timestamps starting at the timebase's current time.  To the listener, this will sound similar to muting the audio for a short period of time.  If it is more desirable to ensure that all audio is played than to keep the timeline moving, you may also stop the synchronizer, set the synchronizer's current time to the value of AVSampleBufferAudioRendererFlushTimeKey, start reenqueueing sample buffers with timestamps starting at that time, and restart the synchronizer.  To the listener, this will sound similar to pausing the audio for a short period of time.
    ///
    /// This notification is delivered on an arbitrary thread.  If sample buffers are being enqueued with the renderer concurrently with the receipt of this notification, it is possible that one or more sample buffers will remain enqueued in the renderer.  This is generally undesirable, because the sample buffers that remain will likely have timestamps far ahead of the timebase's current time and so won't be rendered for some time.  The best practice is to invoke the -flush method, in a manner that is serialized with enqueueing sample buffers, after receiving this notification and before resuming the enqueueing of sample buffers.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avsamplebufferaudiorendererwasflushedautomaticallynotification?language=objc)
    pub static AVSampleBufferAudioRendererWasFlushedAutomaticallyNotification:
        &'static NSNotificationName;
}

extern "C" {
    /// A notification that indicates the hardware configuration does not match the enqueued data format.
    ///
    /// The output configuration of the playback hardware might change during the playback session if other clients play content with different format. In such cases, if the media content format does not match the hardware configuration it would produce suboptimal rendering of the enqueued media data. When the framework detects such mismatch it will issue this notification, so the client can flush the renderer and re-enqueue the sample buffers from the current media playhead, which will configure the hardware based on the format of newly enqueued sample buffers.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avsamplebufferaudiorendereroutputconfigurationdidchangenotification?language=objc)
    pub static AVSampleBufferAudioRendererOutputConfigurationDidChangeNotification:
        &'static NSNotificationName;
}

extern "C" {
    /// The presentation timestamp of the first enqueued sample that was flushed.
    ///
    /// The value of this key is an NSValue wrapping a CMTime.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avsamplebufferaudiorendererflushtimekey?language=objc)
    pub static AVSampleBufferAudioRendererFlushTimeKey: &'static NSString;
}

/// AVSampleBufferAudioRendererQueueManagement.
impl AVSampleBufferAudioRenderer {
    extern_methods!(
        #[cfg(all(feature = "block2", feature = "objc2-core-media"))]
        /// Flushes enqueued sample buffers with presentation time stamps later than or equal to the specified time.
        ///
        /// Parameter `completionHandler`: A block that is invoked, possibly asynchronously, after the flush operation completes or fails.
        ///
        /// This method can be used to replace media data scheduled to be rendered in the future, without interrupting playback.  One example of this is when the data that has already been enqueued is from a sequence of two songs and the second song is swapped for a new song.  In this case, this method would be called with the time stamp of the first sample buffer from the second song.  After the completion handler is executed with a YES parameter, media data may again be enqueued with timestamps at the specified time.
        ///
        /// If NO is provided to the completion handler, the flush did not succeed and the set of enqueued sample buffers remains unchanged.  A flush can fail becuse the source time was too close to (or earlier than) the current time or because the current configuration of the receiver does not support flushing at a particular time.  In these cases, the caller can choose to flush all enqueued media data by invoking the -flush method.
        #[unsafe(method(flushFromSourceTime:completionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn flushFromSourceTime_completionHandler(
            &self,
            time: CMTime,
            completion_handler: &block2::DynBlock<dyn Fn(Bool)>,
        );
    );
}
