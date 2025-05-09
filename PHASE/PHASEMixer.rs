//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-avf-audio")]
use objc2_avf_audio::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// *************************************************************************************************
    ///
    ///
    ///
    /// The base class for a mixer definition.
    ///
    /// Mixer definitions control how audio will be rendered to the output in PHASE.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/phase/phasemixerdefinition?language=objc)
    #[unsafe(super(PHASEDefinition, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PHASEDefinition")]
    pub struct PHASEMixerDefinition;
);

#[cfg(feature = "PHASEDefinition")]
extern_conformance!(
    unsafe impl NSObjectProtocol for PHASEMixerDefinition {}
);

#[cfg(feature = "PHASEDefinition")]
impl PHASEMixerDefinition {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        /// Linear gain scalar.
        ///
        /// Note: Values are clamped to the range [0, 1]. Default value is 1.
        #[unsafe(method(gain))]
        #[unsafe(method_family = none)]
        pub unsafe fn gain(&self) -> c_double;

        /// Setter for [`gain`][Self::gain].
        #[unsafe(method(setGain:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setGain(&self, gain: c_double);

        #[cfg(feature = "PHASEMetaParameter")]
        /// Optionally attach a metaparameter definition here to enable real-time control of the gain during playback.
        #[unsafe(method(gainMetaParameterDefinition))]
        #[unsafe(method_family = none)]
        pub unsafe fn gainMetaParameterDefinition(
            &self,
        ) -> Option<Retained<PHASENumberMetaParameterDefinition>>;

        #[cfg(feature = "PHASEMetaParameter")]
        /// Setter for [`gainMetaParameterDefinition`][Self::gainMetaParameterDefinition].
        #[unsafe(method(setGainMetaParameterDefinition:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setGainMetaParameterDefinition(
            &self,
            gain_meta_parameter_definition: Option<&PHASENumberMetaParameterDefinition>,
        );
    );
}

extern_class!(
    /// *************************************************************************************************
    ///
    ///
    ///
    /// Spatial mixer definition.
    ///
    /// Spatial mixers render audio with spatialization and environmental effects.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/phase/phasespatialmixerdefinition?language=objc)
    #[unsafe(super(PHASEMixerDefinition, PHASEDefinition, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PHASEDefinition")]
    pub struct PHASESpatialMixerDefinition;
);

#[cfg(feature = "PHASEDefinition")]
extern_conformance!(
    unsafe impl NSObjectProtocol for PHASESpatialMixerDefinition {}
);

#[cfg(feature = "PHASEDefinition")]
impl PHASESpatialMixerDefinition {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "PHASESpatialPipeline")]
        /// Create a new PHASESpatialMixerDefinition
        ///
        /// Parameter `spatialPipeline`: A spatial pipeline.
        ///
        /// Returns: A new PHASESpatialMixerDefinition object
        #[unsafe(method(initWithSpatialPipeline:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithSpatialPipeline(
            this: Allocated<Self>,
            spatial_pipeline: &PHASESpatialPipeline,
        ) -> Retained<Self>;

        #[cfg(feature = "PHASESpatialPipeline")]
        /// Create a new PHASESpatialMixerDefinition
        ///
        /// Parameter `spatialPipeline`: A spatial pipeline.
        ///
        /// Parameter `identifier`: An optional custom identifier to give to this object
        ///
        /// Returns: A new PHASESpatialMixerDefinition object
        #[unsafe(method(initWithSpatialPipeline:identifier:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithSpatialPipeline_identifier(
            this: Allocated<Self>,
            spatial_pipeline: &PHASESpatialPipeline,
            identifier: &NSString,
        ) -> Retained<Self>;

        #[cfg(feature = "PHASESpatialPipeline")]
        /// Spatial Pipeline.
        #[unsafe(method(spatialPipeline))]
        #[unsafe(method_family = none)]
        pub unsafe fn spatialPipeline(&self) -> Retained<PHASESpatialPipeline>;

        #[cfg(feature = "PHASEDistanceModel")]
        /// Distance model parameters (optional).
        #[unsafe(method(distanceModelParameters))]
        #[unsafe(method_family = none)]
        pub unsafe fn distanceModelParameters(
            &self,
        ) -> Option<Retained<PHASEDistanceModelParameters>>;

        #[cfg(feature = "PHASEDistanceModel")]
        /// Setter for [`distanceModelParameters`][Self::distanceModelParameters].
        #[unsafe(method(setDistanceModelParameters:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDistanceModelParameters(
            &self,
            distance_model_parameters: Option<&PHASEDistanceModelParameters>,
        );

        #[cfg(feature = "PHASEDirectivityModel")]
        /// Listener directivity model parameters (optional).
        #[unsafe(method(listenerDirectivityModelParameters))]
        #[unsafe(method_family = none)]
        pub unsafe fn listenerDirectivityModelParameters(
            &self,
        ) -> Option<Retained<PHASEDirectivityModelParameters>>;

        #[cfg(feature = "PHASEDirectivityModel")]
        /// Setter for [`listenerDirectivityModelParameters`][Self::listenerDirectivityModelParameters].
        #[unsafe(method(setListenerDirectivityModelParameters:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setListenerDirectivityModelParameters(
            &self,
            listener_directivity_model_parameters: Option<&PHASEDirectivityModelParameters>,
        );

        #[cfg(feature = "PHASEDirectivityModel")]
        /// Source directivity model parameters (optional).
        #[unsafe(method(sourceDirectivityModelParameters))]
        #[unsafe(method_family = none)]
        pub unsafe fn sourceDirectivityModelParameters(
            &self,
        ) -> Option<Retained<PHASEDirectivityModelParameters>>;

        #[cfg(feature = "PHASEDirectivityModel")]
        /// Setter for [`sourceDirectivityModelParameters`][Self::sourceDirectivityModelParameters].
        #[unsafe(method(setSourceDirectivityModelParameters:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSourceDirectivityModelParameters(
            &self,
            source_directivity_model_parameters: Option<&PHASEDirectivityModelParameters>,
        );
    );
}

extern_class!(
    /// *************************************************************************************************
    ///
    ///
    ///
    /// Ambient mixer definition.
    ///
    /// Ambient mixers render audio with spatialization but without environmental effects.
    /// Use ambient mixers for content that isn't being simulated in the environment,
    /// but should still sound like it's coming from somewhere out in space.
    ///
    /// Note: Ambient mixers do not support distance modeling or directivity modeling.
    /// Clients can however set the orientation at initialization time.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/phase/phaseambientmixerdefinition?language=objc)
    #[unsafe(super(PHASEMixerDefinition, PHASEDefinition, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PHASEDefinition")]
    pub struct PHASEAmbientMixerDefinition;
);

#[cfg(feature = "PHASEDefinition")]
extern_conformance!(
    unsafe impl NSObjectProtocol for PHASEAmbientMixerDefinition {}
);

#[cfg(feature = "PHASEDefinition")]
impl PHASEAmbientMixerDefinition {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "objc2-avf-audio")]
        /// A readonly value of the input channel layout this mixer was initialized with.
        #[unsafe(method(inputChannelLayout))]
        #[unsafe(method_family = none)]
        pub unsafe fn inputChannelLayout(&self) -> Retained<AVAudioChannelLayout>;
    );
}

extern_class!(
    /// *************************************************************************************************
    ///
    ///
    ///
    /// Channel mixer definition.
    ///
    /// Channel mixers render audio without spatialization or environmental effects.
    /// Use channel mixers for regular stem-based content that needs be rendered directly to the output device, such as stereo music
    /// or center channel narrative dialogue.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/phase/phasechannelmixerdefinition?language=objc)
    #[unsafe(super(PHASEMixerDefinition, PHASEDefinition, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PHASEDefinition")]
    pub struct PHASEChannelMixerDefinition;
);

#[cfg(feature = "PHASEDefinition")]
extern_conformance!(
    unsafe impl NSObjectProtocol for PHASEChannelMixerDefinition {}
);

#[cfg(feature = "PHASEDefinition")]
impl PHASEChannelMixerDefinition {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "objc2-avf-audio")]
        /// Create a new PHASEChannelMixerDefinition
        ///
        /// Note: Any connected sampler must match this channel layout.
        ///
        /// Parameter `layout`: The input channel layout.
        ///
        /// Parameter `identifier`: An optional custom identifier to give to this object
        ///
        /// Returns: A new PHASEChannelMixerDefinition object
        #[unsafe(method(initWithChannelLayout:identifier:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithChannelLayout_identifier(
            this: Allocated<Self>,
            layout: &AVAudioChannelLayout,
            identifier: &NSString,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-avf-audio")]
        /// Create a new PHASEChannelMixerDefinition
        ///
        /// Note: Any connected sampler must match this channel layout.
        ///
        /// Parameter `layout`: The input channel layout. Any connected sampler must match this channel layout.
        ///
        /// Returns: A new PHASEChannelMixerDefinition object
        #[unsafe(method(initWithChannelLayout:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithChannelLayout(
            this: Allocated<Self>,
            layout: &AVAudioChannelLayout,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-avf-audio")]
        /// A readonly value of the input channel layout this mixer was initialized with.
        #[unsafe(method(inputChannelLayout))]
        #[unsafe(method_family = none)]
        pub unsafe fn inputChannelLayout(&self) -> Retained<AVAudioChannelLayout>;
    );
}

extern_class!(
    /// *************************************************************************************************
    ///
    ///
    ///
    /// A generic object the represents an active mixer in the system
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/phase/phasemixer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHASEMixer;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for PHASEMixer {}
);

impl PHASEMixer {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        /// The identifier that uniquely represents this mixer.
        #[unsafe(method(identifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        /// Linear gain scalar.
        ///
        /// Note: Values are clamped to the range [0, 1]. Default value is 1.
        #[unsafe(method(gain))]
        #[unsafe(method_family = none)]
        pub unsafe fn gain(&self) -> c_double;

        #[cfg(feature = "PHASEMetaParameter")]
        /// The metaparameter that can be used to adjust the gain during playback
        #[unsafe(method(gainMetaParameter))]
        #[unsafe(method_family = none)]
        pub unsafe fn gainMetaParameter(&self) -> Option<Retained<PHASEMetaParameter>>;
    );
}

extern_class!(
    /// *************************************************************************************************
    ///
    ///
    ///
    /// An object that holds runtime parameters for mixers when creating PHASESoundEvents.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/phase/phasemixerparameters?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHASEMixerParameters;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for PHASEMixerParameters {}
);

impl PHASEMixerParameters {
    extern_methods!(
        #[cfg(all(
            feature = "PHASEListener",
            feature = "PHASEObject",
            feature = "PHASESource"
        ))]
        /// Adds runtime parameters for a spatial mixer
        ///
        /// Parameter `identifier`: The unique identifier assigned to a spatial submixer object.
        ///
        /// Parameter `source`: The PHASESource object that this mixer will use to spatialize sounds.
        ///
        /// Parameter `listener`: The PHASEListener object that this mixer will use to spatialize sounds.
        #[unsafe(method(addSpatialMixerParametersWithIdentifier:source:listener:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addSpatialMixerParametersWithIdentifier_source_listener(
            &self,
            identifier: &NSString,
            source: &PHASESource,
            listener: &PHASEListener,
        );

        #[cfg(all(feature = "PHASEListener", feature = "PHASEObject"))]
        /// Adds runtime parameters for an ambient mixer
        ///
        /// Parameter `identifier`: The unique identifier assigned to a spatial submixer object.
        ///
        /// Parameter `listener`: The PHASEListener object that this mixer will use to orient sounds.
        #[unsafe(method(addAmbientMixerParametersWithIdentifier:listener:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addAmbientMixerParametersWithIdentifier_listener(
            &self,
            identifier: &NSString,
            listener: &PHASEListener,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl PHASEMixerParameters {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
