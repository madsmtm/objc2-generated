//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionnoerror?language=objc)
pub const kAudioSessionNoError: c_uint = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionnotinitialized?language=objc)
pub const kAudioSessionNotInitialized: c_uint = 0x21696e69;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionalreadyinitialized?language=objc)
pub const kAudioSessionAlreadyInitialized: c_uint = 0x696e6974;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessioninitializationerror?language=objc)
pub const kAudioSessionInitializationError: c_uint = 0x696e693f;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionunsupportedpropertyerror?language=objc)
pub const kAudioSessionUnsupportedPropertyError: c_uint = 0x7074793f;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionbadpropertysizeerror?language=objc)
pub const kAudioSessionBadPropertySizeError: c_uint = 0x2173697a;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionnotactiveerror?language=objc)
pub const kAudioSessionNotActiveError: c_uint = 0x21616374;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioservicesnohardwareerror?language=objc)
pub const kAudioServicesNoHardwareError: c_uint = 0x6e6f6877;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionnocategoryset?language=objc)
pub const kAudioSessionNoCategorySet: c_uint = 0x3f636174;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionincompatiblecategory?language=objc)
pub const kAudioSessionIncompatibleCategory: c_uint = 0x21636174;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionunspecifiederror?language=objc)
pub const kAudioSessionUnspecifiedError: c_uint = 0x77686174;

/// Type used for specifying an AudioSession property.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiosessionpropertyid?language=objc)
pub type AudioSessionPropertyID = u32;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionbegininterruption?language=objc)
pub const kAudioSessionBeginInterruption: c_uint = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionendinterruption?language=objc)
pub const kAudioSessionEndInterruption: c_uint = 0;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessioncategory_ambientsound?language=objc)
pub const kAudioSessionCategory_AmbientSound: c_uint = 0x616d6269;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessioncategory_soloambientsound?language=objc)
pub const kAudioSessionCategory_SoloAmbientSound: c_uint = 0x736f6c6f;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessioncategory_mediaplayback?language=objc)
pub const kAudioSessionCategory_MediaPlayback: c_uint = 0x6d656469;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessioncategory_recordaudio?language=objc)
pub const kAudioSessionCategory_RecordAudio: c_uint = 0x72656361;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessioncategory_playandrecord?language=objc)
pub const kAudioSessionCategory_PlayAndRecord: c_uint = 0x706c6172;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessioncategory_audioprocessing?language=objc)
pub const kAudioSessionCategory_AudioProcessing: c_uint = 0x70726f63;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionoverrideaudioroute_none?language=objc)
pub const kAudioSessionOverrideAudioRoute_None: c_uint = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionoverrideaudioroute_speaker?language=objc)
pub const kAudioSessionOverrideAudioRoute_Speaker: c_uint = 0x73706b72;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionroutechangereason_unknown?language=objc)
pub const kAudioSessionRouteChangeReason_Unknown: c_uint = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionroutechangereason_newdeviceavailable?language=objc)
pub const kAudioSessionRouteChangeReason_NewDeviceAvailable: c_uint = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionroutechangereason_olddeviceunavailable?language=objc)
pub const kAudioSessionRouteChangeReason_OldDeviceUnavailable: c_uint = 2;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionroutechangereason_categorychange?language=objc)
pub const kAudioSessionRouteChangeReason_CategoryChange: c_uint = 3;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionroutechangereason_override?language=objc)
pub const kAudioSessionRouteChangeReason_Override: c_uint = 4;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionroutechangereason_wakefromsleep?language=objc)
pub const kAudioSessionRouteChangeReason_WakeFromSleep: c_uint = 6;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionroutechangereason_nosuitablerouteforcategory?language=objc)
pub const kAudioSessionRouteChangeReason_NoSuitableRouteForCategory: c_uint = 7;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionroutechangereason_routeconfigurationchange?language=objc)
pub const kAudioSessionRouteChangeReason_RouteConfigurationChange: c_uint = 8;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosession_routechangekey_reason?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kAudioSession_RouteChangeKey_Reason: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosession_audioroutechangekey_previousroutedescription?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kAudioSession_AudioRouteChangeKey_PreviousRouteDescription:
        Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosession_audioroutechangekey_currentroutedescription?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kAudioSession_AudioRouteChangeKey_CurrentRouteDescription: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosession_audioroutekey_inputs?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kAudioSession_AudioRouteKey_Inputs: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosession_audioroutekey_outputs?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kAudioSession_AudioRouteKey_Outputs: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosession_audioroutekey_type?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kAudioSession_AudioRouteKey_Type: Option<&'static CFString>;
}

extern "C" {
    /// These are the strings used with the kAudioSession_AudioRouteKey_Type key for the CFDictionary associated
    /// with kAudioSession_AudioRouteKey_Inputs.
    /// Available in iOS 5.0 or greater
    ///
    /// A line in input
    ///
    /// A built-in microphone input.  (Note that some devices like early iPods do not have this input)
    ///
    /// A microphone that is part of a headset (combined microphone and headphones)
    ///
    /// A microphone that is part of a Bluetooth Hands-Free Profile device
    ///
    /// A Universal Serial Bus input
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessioninputroute_linein?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kAudioSessionInputRoute_LineIn: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessioninputroute_builtinmic?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kAudioSessionInputRoute_BuiltInMic: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessioninputroute_headsetmic?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kAudioSessionInputRoute_HeadsetMic: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessioninputroute_bluetoothhfp?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kAudioSessionInputRoute_BluetoothHFP: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessioninputroute_usbaudio?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kAudioSessionInputRoute_USBAudio: Option<&'static CFString>;
}

extern "C" {
    /// These are strings used with the kAudioSession_AudioRouteKey_Type key for the CFDictionary associated
    /// with kAudioSession_AudioRouteKey_Outputs.
    /// Available in iOS 5.0 or greater
    ///
    /// A line out output
    ///
    /// Speakers in a headset (mic and headphones) or simple headphones
    ///
    /// Speakers that are part of a Bluetooth Hands-Free Profile device
    ///
    /// Speakers in a Bluetooth A2DP device
    ///
    /// The speaker you hold to your ear when on a phone call
    ///
    /// The built-in speaker
    ///
    /// Speaker(s) in a Universal Serial Bus device
    ///
    /// Output via High-Definition Multimedia Interface
    ///
    /// Output on a remote Air Play device
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionoutputroute_lineout?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kAudioSessionOutputRoute_LineOut: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionoutputroute_headphones?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kAudioSessionOutputRoute_Headphones: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionoutputroute_bluetoothhfp?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kAudioSessionOutputRoute_BluetoothHFP: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionoutputroute_bluetootha2dp?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kAudioSessionOutputRoute_BluetoothA2DP: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionoutputroute_builtinreceiver?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kAudioSessionOutputRoute_BuiltInReceiver: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionoutputroute_builtinspeaker?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kAudioSessionOutputRoute_BuiltInSpeaker: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionoutputroute_usbaudio?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kAudioSessionOutputRoute_USBAudio: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionoutputroute_hdmi?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kAudioSessionOutputRoute_HDMI: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionoutputroute_airplay?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kAudioSessionOutputRoute_AirPlay: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosession_inputsourcekey_id?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kAudioSession_InputSourceKey_ID: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosession_inputsourcekey_description?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kAudioSession_InputSourceKey_Description: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosession_outputdestinationkey_id?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kAudioSession_OutputDestinationKey_ID: Option<&'static CFString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosession_outputdestinationkey_description?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kAudioSession_OutputDestinationKey_Description: Option<&'static CFString>;
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessioninterruptiontype_shouldresume?language=objc)
pub const kAudioSessionInterruptionType_ShouldResume: c_uint = 0x6972736d;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessioninterruptiontype_shouldnotresume?language=objc)
pub const kAudioSessionInterruptionType_ShouldNotResume: c_uint = 0x2172736d;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiosessioninterruptiontype?language=objc)
pub type AudioSessionInterruptionType = u32;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionmode_default?language=objc)
pub const kAudioSessionMode_Default: c_uint = 0x64666c74;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionmode_voicechat?language=objc)
pub const kAudioSessionMode_VoiceChat: c_uint = 0x76636374;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionmode_videorecording?language=objc)
pub const kAudioSessionMode_VideoRecording: c_uint = 0x76726364;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionmode_measurement?language=objc)
pub const kAudioSessionMode_Measurement: c_uint = 0x6d736d74;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionmode_gamechat?language=objc)
pub const kAudioSessionMode_GameChat: c_uint = 0x676d6374;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_preferredhardwaresamplerate?language=objc)
pub const kAudioSessionProperty_PreferredHardwareSampleRate: c_uint = 0x68777372;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_preferredhardwareiobufferduration?language=objc)
pub const kAudioSessionProperty_PreferredHardwareIOBufferDuration: c_uint = 0x696f6264;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_audiocategory?language=objc)
pub const kAudioSessionProperty_AudioCategory: c_uint = 0x61636174;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_audioroutechange?language=objc)
pub const kAudioSessionProperty_AudioRouteChange: c_uint = 0x726f6368;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_currenthardwaresamplerate?language=objc)
pub const kAudioSessionProperty_CurrentHardwareSampleRate: c_uint = 0x63687372;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_currenthardwareinputnumberchannels?language=objc)
pub const kAudioSessionProperty_CurrentHardwareInputNumberChannels: c_uint = 0x63686963;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_currenthardwareoutputnumberchannels?language=objc)
pub const kAudioSessionProperty_CurrentHardwareOutputNumberChannels: c_uint = 0x63686f63;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_currenthardwareoutputvolume?language=objc)
pub const kAudioSessionProperty_CurrentHardwareOutputVolume: c_uint = 0x63686f76;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_currenthardwareinputlatency?language=objc)
pub const kAudioSessionProperty_CurrentHardwareInputLatency: c_uint = 0x63696c74;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_currenthardwareoutputlatency?language=objc)
pub const kAudioSessionProperty_CurrentHardwareOutputLatency: c_uint = 0x636f6c74;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_currenthardwareiobufferduration?language=objc)
pub const kAudioSessionProperty_CurrentHardwareIOBufferDuration: c_uint = 0x63686264;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_otheraudioisplaying?language=objc)
pub const kAudioSessionProperty_OtherAudioIsPlaying: c_uint = 0x6f746872;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_overrideaudioroute?language=objc)
pub const kAudioSessionProperty_OverrideAudioRoute: c_uint = 0x6f767264;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_audioinputavailable?language=objc)
pub const kAudioSessionProperty_AudioInputAvailable: c_uint = 0x61696176;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_serverdied?language=objc)
pub const kAudioSessionProperty_ServerDied: c_uint = 0x64696564;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_othermixableaudioshouldduck?language=objc)
pub const kAudioSessionProperty_OtherMixableAudioShouldDuck: c_uint = 0x6475636b;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_overridecategorymixwithothers?language=objc)
pub const kAudioSessionProperty_OverrideCategoryMixWithOthers: c_uint = 0x636d6978;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_overridecategorydefaulttospeaker?language=objc)
pub const kAudioSessionProperty_OverrideCategoryDefaultToSpeaker: c_uint = 0x6373706b;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_overridecategoryenablebluetoothinput?language=objc)
pub const kAudioSessionProperty_OverrideCategoryEnableBluetoothInput: c_uint = 0x63626c75;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_interruptiontype?language=objc)
pub const kAudioSessionProperty_InterruptionType: c_uint = 0x74797065;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_mode?language=objc)
pub const kAudioSessionProperty_Mode: c_uint = 0x6d6f6465;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_inputsources?language=objc)
pub const kAudioSessionProperty_InputSources: c_uint = 0x73726373;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_outputdestinations?language=objc)
pub const kAudioSessionProperty_OutputDestinations: c_uint = 0x64737473;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_inputsource?language=objc)
pub const kAudioSessionProperty_InputSource: c_uint = 0x69737263;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_outputdestination?language=objc)
pub const kAudioSessionProperty_OutputDestination: c_uint = 0x6f647374;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_inputgainavailable?language=objc)
pub const kAudioSessionProperty_InputGainAvailable: c_uint = 0x69676176;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_inputgainscalar?language=objc)
pub const kAudioSessionProperty_InputGainScalar: c_uint = 0x69677363;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_audioroutedescription?language=objc)
pub const kAudioSessionProperty_AudioRouteDescription: c_uint = 0x63726172;

/// A function to be called when an interruption begins or ends.
///
/// AudioSessionInterruptionListener has to be provided by client applications in the
/// AudioSessionInitialize function.  It will be called when an interruption begins or ends.
///
/// Parameter `inClientData`: The client user data to use when calling the listener.
///
/// Parameter `inInterruptionState`: Indicates if the interruption begins (kAudioSessionBeginInterruption)
/// or ends (kAudioSessionEndInterruption)
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiosessioninterruptionlistener?language=objc)
pub type AudioSessionInterruptionListener = Option<unsafe extern "C-unwind" fn(*mut c_void, u32)>;

/// A function to be executed when a property changes.
///
/// AudioSessionPropertyListener may be provided by client application to be
/// called when a property changes.
///
/// Parameter `inClientData`: The client user data to use when calling the listener.
///
/// Parameter `inID`: The AudioSession property that changed
///
/// Parameter `inDataSize`: The size of the payload
///
/// Parameter `inData`: The payload of the property that changed (see data type for each property)
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiosessionpropertylistener?language=objc)
pub type AudioSessionPropertyListener =
    Option<unsafe extern "C-unwind" fn(*mut c_void, AudioSessionPropertyID, u32, *const c_void)>;

extern "C-unwind" {
    /// Initialize the AudioSession.
    ///
    /// This function has to be called once before calling any other
    /// AudioSession functions.
    ///
    /// Parameter `inRunLoop`: A CFRunLoopRef indicating the desired run loop the interruption routine should
    /// be run on. Pass NULL to use the main run loop.
    ///
    /// Parameter `inRunLoopMode`: A CFStringRef indicating the run loop mode for the runloop where the
    /// completion routine will be executed. Pass NULL to use kCFRunLoopDefaultMode.
    ///
    /// Parameter `inInterruptionListener`: An AudioSessionInterruptionListener to be called when the AudioSession
    /// is interrupted.
    ///
    /// Parameter `inClientData`: The client user data to use when calling the interruption listener.
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated = "no longer supported"]
    pub fn AudioSessionInitialize(
        in_run_loop: Option<&CFRunLoop>,
        in_run_loop_mode: Option<&CFString>,
        in_interruption_listener: AudioSessionInterruptionListener,
        in_client_data: *mut c_void,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Activate or deactivate the AudioSession.
    ///
    /// Call this function with active set to true to activate this AudioSession (interrupt
    /// the currently active AudioSession).
    /// Call this function with active set to false to deactivate this AudioSession (allow
    /// another interrupted AudioSession to resume).
    /// When active is true this call may fail if the currently active AudioSession has a higher priority.
    ///
    /// Parameter `active`: A Boolean indicating if you want to make this AudioSession active or inactive.
    #[deprecated = "no longer supported"]
    pub fn AudioSessionSetActive(active: Boolean) -> OSStatus;
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionsetactiveflag_notifyothersondeactivation?language=objc)
pub const kAudioSessionSetActiveFlag_NotifyOthersOnDeactivation: c_uint = 1 << 0;

extern "C-unwind" {
    /// Same functionality as AudioSessionSetActive, with an additional flags parameter for
    /// refining behavior.
    ///
    /// Call this function with active set to true to activate this AudioSession (interrupt
    /// the currently active AudioSession).
    /// Call this function with active set to false to deactivate this AudioSession (allow
    /// another interrupted AudioSession to resume).
    /// Pass in one or more flags to refine the behavior during activation or deactivation.
    /// When active is true this call may fail if the currently active AudioSession has a
    /// higher priority.
    ///
    /// Parameter `active`: A Boolean indicating if you want to make this AudioSession active or inactive.
    ///
    /// Parameter `inFlags`: A bitmap containing one or more flags from the AudioSessionActivationFlags enum.
    #[deprecated = "no longer supported"]
    pub fn AudioSessionSetActiveWithFlags(active: Boolean, in_flags: u32) -> OSStatus;
}

extern "C-unwind" {
    /// Get the value of a property.
    ///
    /// This function can be called to get the value for a property of the AudioSession.
    /// Valid properties are listed in an enum above.
    ///
    /// Parameter `inID`: The AudioSessionPropertyID for which we want to get the value.
    ///
    /// Parameter `ioDataSize`: The size of the data payload.
    /// On entry it should contain the size of the memory pointed to by outData.
    /// On exit it will contain the actual size of the data.
    ///
    /// Parameter `outData`: The data for the property will be copied here.
    ///
    /// Returns: kAudioSessionNoError if the operation was successful.  If the property is a
    /// write-only property or only available by way of property listeners,
    /// kAudioSessionUnsupportedPropertyError will be returned.  Other error codes
    /// listed under AudioSession Error Constants also apply to this function.
    #[deprecated = "no longer supported"]
    pub fn AudioSessionGetProperty(
        in_id: AudioSessionPropertyID,
        io_data_size: *mut u32,
        out_data: *mut c_void,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Set the value of a property.
    ///
    /// This function can be called to set the value for a property of the AudioSession.
    /// Valid properties are listed in an enum above.
    ///
    /// Parameter `inID`: The AudioSessionPropertyID for which we want to set the value.
    ///
    /// Parameter `inDataSize`: The size of the data payload.
    ///
    /// Parameter `inData`: The data for the property we want to set.
    ///
    /// Returns: kAudioSessionNoError if the operation was successful.  If the property is a
    /// read-only property or an invalid property value is passed in,
    /// kAudioSessionUnsupportedPropertyError will be returned.  Other error codes
    /// listed under AudioSession Error Constants also apply to
    /// this function.
    #[deprecated = "no longer supported"]
    pub fn AudioSessionSetProperty(
        in_id: AudioSessionPropertyID,
        in_data_size: u32,
        in_data: *const c_void,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Get the size of the payload for a property.
    ///
    /// This function can be called to get the size for the payload of a property.
    /// Valid properties are listed in an enum above.
    ///
    /// Parameter `inID`: The AudioSessionPropertyID for which we want to get the size of the payload.
    ///
    /// Parameter `outDataSize`: The size of the data payload will be copied here.
    #[deprecated = "no longer supported"]
    pub fn AudioSessionGetPropertySize(
        in_id: AudioSessionPropertyID,
        out_data_size: *mut u32,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Add a property listener.
    ///
    /// This function can be used to add a listener to be called when a property changes.
    /// If a listener and user data already exist for this property, they will be replaced.
    /// Valid properties are listed above.
    ///
    /// Parameter `inID`: The AudioSessionPropertyID for which we want to set a listener.
    ///
    /// Parameter `inProc`: The listener to be called when the property changes.
    ///
    /// Parameter `inClientData`: The client user data to use when calling the listener.
    ///
    /// Returns: kAudioSessionNoError if the operation was successful.  If the property does
    /// not support listeners, kAudioSessionUnsupportedPropertyError will be returned.
    /// Other error codes listed under AudioSession Error Constants also apply to
    /// this function.
    #[deprecated = "no longer supported"]
    pub fn AudioSessionAddPropertyListener(
        in_id: AudioSessionPropertyID,
        in_proc: AudioSessionPropertyListener,
        in_client_data: *mut c_void,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// see AudioSessionRemovePropertyListenerWithUserData
    ///
    /// see AudioSessionRemovePropertyListenerWithUserData
    #[deprecated = "no longer supported"]
    pub fn AudioSessionRemovePropertyListener(in_id: AudioSessionPropertyID) -> OSStatus;
}

extern "C-unwind" {
    /// Remove a property listener.
    ///
    /// This function can be called to remove the listener for a property. The caller
    /// provides the same proc and user data that was used to add the listener. This ensures
    /// that there can be more than one listener established for a given property ID,
    /// and each listener can be removed as requested.
    /// Valid properties are listed above.
    ///
    /// Parameter `inID`: The AudioSessionPropertyID for which we want to remove the listener.
    ///
    /// Parameter `inProc`: The proc that was used to add the listener that needs to be removed.
    ///
    /// Parameter `inClientData`: The client data that was used to add the listener that needs to be removed.
    ///
    /// Returns: kAudioSessionNoError if the operation was successful.  If the property does
    /// not support listeners, kAudioSessionUnsupportedPropertyError will be returned.
    /// Other error codes listed under AudioSession Error Constants also apply to
    /// this function.
    #[deprecated = "no longer supported"]
    pub fn AudioSessionRemovePropertyListenerWithUserData(
        in_id: AudioSessionPropertyID,
        in_proc: AudioSessionPropertyListener,
        in_client_data: *mut c_void,
    ) -> OSStatus;
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessioncategory_userinterfacesoundeffects?language=objc)
pub const kAudioSessionCategory_UserInterfaceSoundEffects: c_uint = 0x75696678;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessioncategory_liveaudio?language=objc)
pub const kAudioSessionCategory_LiveAudio: c_uint = 0x6c697665;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiosessionproperty_audioroute?language=objc)
pub const kAudioSessionProperty_AudioRoute: c_uint = 0x726f7574;