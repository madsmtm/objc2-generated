//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-audio-types")]
use objc2_core_audio_types::*;
use objc2_foundation::*;

use crate::*;

/// The location of a data source on an iOS device.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionlocation?language=objc)
// NS_TYPED_ENUM
pub type AVAudioSessionLocation = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionlocationupper?language=objc)
    pub static AVAudioSessionLocationUpper: &'static AVAudioSessionLocation;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionlocationlower?language=objc)
    pub static AVAudioSessionLocationLower: &'static AVAudioSessionLocation;
}

/// The orientation or directionality of a data source on an iOS device.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionorientation?language=objc)
// NS_TYPED_ENUM
pub type AVAudioSessionOrientation = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionorientationtop?language=objc)
    pub static AVAudioSessionOrientationTop: &'static AVAudioSessionOrientation;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionorientationbottom?language=objc)
    pub static AVAudioSessionOrientationBottom: &'static AVAudioSessionOrientation;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionorientationfront?language=objc)
    pub static AVAudioSessionOrientationFront: &'static AVAudioSessionOrientation;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionorientationback?language=objc)
    pub static AVAudioSessionOrientationBack: &'static AVAudioSessionOrientation;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionorientationleft?language=objc)
    pub static AVAudioSessionOrientationLeft: &'static AVAudioSessionOrientation;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionorientationright?language=objc)
    pub static AVAudioSessionOrientationRight: &'static AVAudioSessionOrientation;
}

/// The possible polar patterns for a data source on an iOS device.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionpolarpattern?language=objc)
// NS_TYPED_ENUM
pub type AVAudioSessionPolarPattern = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionpolarpatternomnidirectional?language=objc)
    pub static AVAudioSessionPolarPatternOmnidirectional: &'static AVAudioSessionPolarPattern;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionpolarpatterncardioid?language=objc)
    pub static AVAudioSessionPolarPatternCardioid: &'static AVAudioSessionPolarPattern;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionpolarpatternsubcardioid?language=objc)
    pub static AVAudioSessionPolarPatternSubcardioid: &'static AVAudioSessionPolarPattern;
}

extern "C" {
    /// If you select a data source with AVAudioSessionPolarPatternStereo, then you must call setPreferredInputOrientation:error: on your Audio Session so that left and right are presented from the correct directions.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionpolarpatternstereo?language=objc)
    pub static AVAudioSessionPolarPatternStereo: &'static AVAudioSessionPolarPattern;
}

extern_class!(
    /// Information about a port's audio channels.
    ///
    /// AudioQueue, AURemoteIO and AUVoiceIO instances can be assigned to communicate with specific
    /// hardware channels by setting an array of
    /// <port
    /// UID, channel index> pairs.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionchanneldescription?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAudioSessionChannelDescription;
);

unsafe impl Send for AVAudioSessionChannelDescription {}

unsafe impl Sync for AVAudioSessionChannelDescription {}

unsafe impl NSObjectProtocol for AVAudioSessionChannelDescription {}

extern_methods!(
    unsafe impl AVAudioSessionChannelDescription {
        /// A human-readable name for the channel.
        #[method_id(@__retain_semantics Other channelName)]
        pub unsafe fn channelName(&self) -> Retained<NSString>;

        /// The UID (unique identifier) of the port owning the channel.
        #[method_id(@__retain_semantics Other owningPortUID)]
        pub unsafe fn owningPortUID(&self) -> Retained<NSString>;

        /// The index of this channel in its owning port's array of channels.
        #[method(channelNumber)]
        pub unsafe fn channelNumber(&self) -> NSUInteger;

        #[cfg(feature = "objc2-core-audio-types")]
        /// Description of the physical location of this channel.
        #[method(channelLabel)]
        pub unsafe fn channelLabel(&self) -> AudioChannelLabel;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVAudioSessionChannelDescription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// Information about one of potentially multiple data sources associated with a port.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessiondatasourcedescription?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAudioSessionDataSourceDescription;
);

unsafe impl Send for AVAudioSessionDataSourceDescription {}

unsafe impl Sync for AVAudioSessionDataSourceDescription {}

unsafe impl NSObjectProtocol for AVAudioSessionDataSourceDescription {}

extern_methods!(
    unsafe impl AVAudioSessionDataSourceDescription {
        /// System-assigned ID for the data source.
        #[method_id(@__retain_semantics Other dataSourceID)]
        pub unsafe fn dataSourceID(&self) -> Retained<NSNumber>;

        /// Human-readable name for the data source.
        #[method_id(@__retain_semantics Other dataSourceName)]
        pub unsafe fn dataSourceName(&self) -> Retained<NSString>;

        /// Describes the general location of a data source. Will be nil for data sources for which the
        /// location is not known.
        #[method_id(@__retain_semantics Other location)]
        pub unsafe fn location(&self) -> Option<Retained<AVAudioSessionLocation>>;

        /// Describes the orientation of a data source.  Will be nil for data sources for which the
        /// orientation is not known.
        #[method_id(@__retain_semantics Other orientation)]
        pub unsafe fn orientation(&self) -> Option<Retained<AVAudioSessionOrientation>>;

        /// Array of one or more AVAudioSessionPolarPatterns describing the supported polar patterns for a
        /// data source.  Will be nil for data sources that have no selectable patterns.
        #[method_id(@__retain_semantics Other supportedPolarPatterns)]
        pub unsafe fn supportedPolarPatterns(
            &self,
        ) -> Option<Retained<NSArray<AVAudioSessionPolarPattern>>>;

        /// Describes the currently selected polar pattern.  Will be nil for data sources that have no
        /// selectable patterns.
        #[method_id(@__retain_semantics Other selectedPolarPattern)]
        pub unsafe fn selectedPolarPattern(&self) -> Option<Retained<AVAudioSessionPolarPattern>>;

        /// Describes the preferred polar pattern.  Will be nil for data sources that have no selectable
        /// patterns or if no preference has been set.
        #[method_id(@__retain_semantics Other preferredPolarPattern)]
        pub unsafe fn preferredPolarPattern(&self) -> Option<Retained<AVAudioSessionPolarPattern>>;

        /// Select the desired polar pattern from the set of available patterns. Setting a nil value
        /// will clear the preference.
        ///
        ///
        /// Note: If the owning port and data source are part of the active audio route, changing the polar
        /// pattern will likely result in a route reconfiguration. If the owning port and data source are
        /// not part of the active route, selecting a polar pattern will not result in an immediate route
        /// reconfiguration.  Use AVAudioSession's setPreferredInput:error: method to activate the port. Use
        /// setPreferredDataSource:error: to active the data source on the port.
        /// You must call setPreferredInputOrientation:error: on the AVAudioSession if you chose the
        /// AVAudioSessionPolarPatternStereo polar pattern.
        #[method(setPreferredPolarPattern:error:_)]
        pub unsafe fn setPreferredPolarPattern_error(
            &self,
            pattern: Option<&AVAudioSessionPolarPattern>,
        ) -> Result<(), Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVAudioSessionDataSourceDescription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// Information about a port, a physical connector or audio device.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionportdescription?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAudioSessionPortDescription;
);

unsafe impl Send for AVAudioSessionPortDescription {}

unsafe impl Sync for AVAudioSessionPortDescription {}

unsafe impl NSObjectProtocol for AVAudioSessionPortDescription {}

extern_methods!(
    unsafe impl AVAudioSessionPortDescription {
        #[cfg(feature = "AVAudioSessionTypes")]
        #[method_id(@__retain_semantics Other portType)]
        pub unsafe fn portType(&self) -> Retained<AVAudioSessionPort>;

        /// A descriptive name for the associated hardware port
        #[method_id(@__retain_semantics Other portName)]
        pub unsafe fn portName(&self) -> Retained<NSString>;

        /// A system-assigned unique identifier for the associated hardware port
        #[method_id(@__retain_semantics Other UID)]
        pub unsafe fn UID(&self) -> Retained<NSString>;

        /// This property's value will be true if the associated hardware port has built-in
        /// processing for two-way voice communication.
        ///
        /// Applications that use their own proprietary voice processing algorithms should use this property
        /// to decide when to disable processing.  On the other hand, if using Apple's Voice Processing I/O
        /// unit (subtype kAudioUnitSubType_VoiceProcessingIO), the system will automatically manage this
        /// for the application. In particular, ports of type AVAudioSessionPortBluetoothHFP and
        /// AVAudioSessionPortCarAudio often have hardware voice processing.
        #[method(hasHardwareVoiceCallProcessing)]
        pub unsafe fn hasHardwareVoiceCallProcessing(&self) -> bool;

        /// This property's value will be true if the port supports spatial audio playback and the feature is
        /// enabled.
        ///
        /// 'Now Playing' apps should also inform the system if they support multichannel audio content using
        /// -setSupportsMultichannelContent:error: method. Apps may also register to receive the
        /// AVAudioSessionSpatialPlaybackCapabilitiesChanged notification to detect changes in user preferences that
        /// affect spatial audio playback.
        ///
        /// This property is only relevant in the context of ports that have a small number of hardware channels
        /// (typically 2), but have enhanced capabilities for rendering multi-channel content. Note that some port
        /// types such as USB and HDMI may support multi-channel playback because they have hardware formats supporting
        /// more than 2 channels. For example, many HDMI receivers are connected to multiple speakers and are capable of
        /// rendering 5.1, 7.1, or other popular surround sound formats. Applications interested in utilizing multi-channel
        /// formats should also query AVAudioSession's maximumOutputNumberOfChannels property and make use of
        /// -setPreferredOutputNumberOfChannels:error: to set the preferred number of hardware channels.
        #[method(isSpatialAudioEnabled)]
        pub unsafe fn isSpatialAudioEnabled(&self) -> bool;

        #[method_id(@__retain_semantics Other channels)]
        pub unsafe fn channels(
            &self,
        ) -> Option<Retained<NSArray<AVAudioSessionChannelDescription>>>;

        /// Will be nil if there are no selectable data sources.
        #[method_id(@__retain_semantics Other dataSources)]
        pub unsafe fn dataSources(
            &self,
        ) -> Option<Retained<NSArray<AVAudioSessionDataSourceDescription>>>;

        /// Will be nil if there are no selectable data sources. In all other cases, this property reflects
        /// the currently selected data source.
        #[method_id(@__retain_semantics Other selectedDataSource)]
        pub unsafe fn selectedDataSource(
            &self,
        ) -> Option<Retained<AVAudioSessionDataSourceDescription>>;

        /// This property reflects the application's preferred data source for the Port. Will be nil if
        /// there are no selectable data sources or if no preference has been set.
        #[method_id(@__retain_semantics Other preferredDataSource)]
        pub unsafe fn preferredDataSource(
            &self,
        ) -> Option<Retained<AVAudioSessionDataSourceDescription>>;

        /// Select the preferred data source for this port. The input dataSource parameter must be
        /// one of the dataSources exposed by the dataSources property. Setting a nil value will clear the
        /// preference. Note: if the port is part of the active audio route, changing the data source will
        /// likely result in a route reconfiguration.  If the port is not part of the active route,
        /// selecting a new data source will not result in an immediate route reconfiguration.  Use
        /// AVAudioSession's -setPreferredInput:error: method to activate the port.
        #[method(setPreferredDataSource:error:_)]
        pub unsafe fn setPreferredDataSource_error(
            &self,
            data_source: Option<&AVAudioSessionDataSourceDescription>,
        ) -> Result<(), Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVAudioSessionPortDescription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// A description of the input and output ports which comprise a route.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiosessionroutedescription?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAudioSessionRouteDescription;
);

unsafe impl Send for AVAudioSessionRouteDescription {}

unsafe impl Sync for AVAudioSessionRouteDescription {}

unsafe impl NSObjectProtocol for AVAudioSessionRouteDescription {}

extern_methods!(
    unsafe impl AVAudioSessionRouteDescription {
        /// Flattened list of all input port descriptions associated with all the streams as part of the route.
        #[method_id(@__retain_semantics Other inputs)]
        pub unsafe fn inputs(&self) -> Retained<NSArray<AVAudioSessionPortDescription>>;

        /// Flattened list of all output port descriptions associated with all the streams as part of the route.
        #[method_id(@__retain_semantics Other outputs)]
        pub unsafe fn outputs(&self) -> Retained<NSArray<AVAudioSessionPortDescription>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVAudioSessionRouteDescription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);