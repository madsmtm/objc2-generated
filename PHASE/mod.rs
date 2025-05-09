// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::unportable_markdown)]
#![allow(rustdoc::invalid_html_tags)]

#[link(name = "PHASE", kind = "framework")]
extern "C" {}

#[cfg(feature = "PHASEAssetRegistry")]
#[path = "PHASEAssetRegistry.rs"]
mod __PHASEAssetRegistry;
#[cfg(feature = "PHASEDefinition")]
#[path = "PHASEDefinition.rs"]
mod __PHASEDefinition;
#[cfg(feature = "PHASEDirectivityModel")]
#[path = "PHASEDirectivityModel.rs"]
mod __PHASEDirectivityModel;
#[cfg(feature = "PHASEDistanceModel")]
#[path = "PHASEDistanceModel.rs"]
mod __PHASEDistanceModel;
#[cfg(feature = "PHASEDucker")]
#[path = "PHASEDucker.rs"]
mod __PHASEDucker;
#[cfg(feature = "PHASEEngine")]
#[path = "PHASEEngine.rs"]
mod __PHASEEngine;
#[cfg(feature = "PHASEEnvelope")]
#[path = "PHASEEnvelope.rs"]
mod __PHASEEnvelope;
#[cfg(feature = "PHASEGroup")]
#[path = "PHASEGroup.rs"]
mod __PHASEGroup;
#[cfg(feature = "PHASEGroupPreset")]
#[path = "PHASEGroupPreset.rs"]
mod __PHASEGroupPreset;
#[cfg(feature = "PHASEListener")]
#[path = "PHASEListener.rs"]
mod __PHASEListener;
#[cfg(feature = "PHASEMaterial")]
#[path = "PHASEMaterial.rs"]
mod __PHASEMaterial;
#[cfg(feature = "PHASEMedium")]
#[path = "PHASEMedium.rs"]
mod __PHASEMedium;
#[cfg(feature = "PHASEMetaParameter")]
#[path = "PHASEMetaParameter.rs"]
mod __PHASEMetaParameter;
#[cfg(feature = "PHASEMixer")]
#[path = "PHASEMixer.rs"]
mod __PHASEMixer;
#[cfg(feature = "PHASEObject")]
#[path = "PHASEObject.rs"]
mod __PHASEObject;
#[cfg(feature = "PHASEOccluder")]
#[path = "PHASEOccluder.rs"]
mod __PHASEOccluder;
#[cfg(feature = "PHASEShape")]
#[path = "PHASEShape.rs"]
mod __PHASEShape;
#[cfg(feature = "PHASESoundEvent")]
#[path = "PHASESoundEvent.rs"]
mod __PHASESoundEvent;
#[cfg(feature = "PHASESoundEventNodes")]
#[path = "PHASESoundEventNodes.rs"]
mod __PHASESoundEventNodes;
#[cfg(feature = "PHASESource")]
#[path = "PHASESource.rs"]
mod __PHASESource;
#[cfg(feature = "PHASESpatialPipeline")]
#[path = "PHASESpatialPipeline.rs"]
mod __PHASESpatialPipeline;
#[cfg(feature = "PHASETypes")]
#[path = "PHASETypes.rs"]
mod __PHASETypes;

#[cfg(feature = "PHASEAssetRegistry")]
pub use self::__PHASEAssetRegistry::PHASEAsset;
#[cfg(feature = "PHASEAssetRegistry")]
pub use self::__PHASEAssetRegistry::PHASEAssetRegistry;
#[cfg(feature = "PHASEAssetRegistry")]
pub use self::__PHASEAssetRegistry::PHASEGlobalMetaParameterAsset;
#[cfg(feature = "PHASEAssetRegistry")]
pub use self::__PHASEAssetRegistry::PHASESoundAsset;
#[cfg(feature = "PHASEAssetRegistry")]
pub use self::__PHASEAssetRegistry::PHASESoundEventNodeAsset;
#[cfg(feature = "PHASEDefinition")]
pub use self::__PHASEDefinition::PHASEDefinition;
#[cfg(feature = "PHASEDirectivityModel")]
pub use self::__PHASEDirectivityModel::PHASECardioidDirectivityModelParameters;
#[cfg(feature = "PHASEDirectivityModel")]
pub use self::__PHASEDirectivityModel::PHASECardioidDirectivityModelSubbandParameters;
#[cfg(feature = "PHASEDirectivityModel")]
pub use self::__PHASEDirectivityModel::PHASEConeDirectivityModelParameters;
#[cfg(feature = "PHASEDirectivityModel")]
pub use self::__PHASEDirectivityModel::PHASEConeDirectivityModelSubbandParameters;
#[cfg(feature = "PHASEDirectivityModel")]
pub use self::__PHASEDirectivityModel::PHASEDirectivityModelParameters;
#[cfg(feature = "PHASEDistanceModel")]
pub use self::__PHASEDistanceModel::PHASEDistanceModelFadeOutParameters;
#[cfg(feature = "PHASEDistanceModel")]
pub use self::__PHASEDistanceModel::PHASEDistanceModelParameters;
#[cfg(feature = "PHASEDistanceModel")]
pub use self::__PHASEDistanceModel::PHASEEnvelopeDistanceModelParameters;
#[cfg(feature = "PHASEDistanceModel")]
pub use self::__PHASEDistanceModel::PHASEGeometricSpreadingDistanceModelParameters;
#[cfg(feature = "PHASEDucker")]
pub use self::__PHASEDucker::PHASEDucker;
#[cfg(feature = "PHASEEngine")]
pub use self::__PHASEEngine::PHASEEngine;
#[cfg(feature = "PHASEEnvelope")]
pub use self::__PHASEEnvelope::PHASEEnvelope;
#[cfg(feature = "PHASEEnvelope")]
pub use self::__PHASEEnvelope::PHASEEnvelopeSegment;
#[cfg(feature = "PHASEEnvelope")]
pub use self::__PHASEEnvelope::PHASENumericPair;
#[cfg(feature = "PHASEGroup")]
pub use self::__PHASEGroup::PHASEGroup;
#[cfg(feature = "PHASEGroupPreset")]
pub use self::__PHASEGroupPreset::PHASEGroupPreset;
#[cfg(feature = "PHASEGroupPreset")]
pub use self::__PHASEGroupPreset::PHASEGroupPresetSetting;
#[cfg(all(feature = "PHASEListener", feature = "PHASEObject"))]
pub use self::__PHASEListener::PHASEListener;
#[cfg(feature = "PHASEMaterial")]
pub use self::__PHASEMaterial::PHASEMaterial;
#[cfg(feature = "PHASEMaterial")]
pub use self::__PHASEMaterial::PHASEMaterialPreset;
#[cfg(feature = "PHASEMedium")]
pub use self::__PHASEMedium::PHASEMedium;
#[cfg(feature = "PHASEMedium")]
pub use self::__PHASEMedium::PHASEMediumPreset;
#[cfg(all(feature = "PHASEDefinition", feature = "PHASEMetaParameter"))]
pub use self::__PHASEMetaParameter::PHASEMappedMetaParameterDefinition;
#[cfg(feature = "PHASEMetaParameter")]
pub use self::__PHASEMetaParameter::PHASEMetaParameter;
#[cfg(all(feature = "PHASEDefinition", feature = "PHASEMetaParameter"))]
pub use self::__PHASEMetaParameter::PHASEMetaParameterDefinition;
#[cfg(feature = "PHASEMetaParameter")]
pub use self::__PHASEMetaParameter::PHASENumberMetaParameter;
#[cfg(all(feature = "PHASEDefinition", feature = "PHASEMetaParameter"))]
pub use self::__PHASEMetaParameter::PHASENumberMetaParameterDefinition;
#[cfg(feature = "PHASEMetaParameter")]
pub use self::__PHASEMetaParameter::PHASEStringMetaParameter;
#[cfg(all(feature = "PHASEDefinition", feature = "PHASEMetaParameter"))]
pub use self::__PHASEMetaParameter::PHASEStringMetaParameterDefinition;
#[cfg(all(feature = "PHASEDefinition", feature = "PHASEMixer"))]
pub use self::__PHASEMixer::PHASEAmbientMixerDefinition;
#[cfg(all(feature = "PHASEDefinition", feature = "PHASEMixer"))]
pub use self::__PHASEMixer::PHASEChannelMixerDefinition;
#[cfg(feature = "PHASEMixer")]
pub use self::__PHASEMixer::PHASEMixer;
#[cfg(all(feature = "PHASEDefinition", feature = "PHASEMixer"))]
pub use self::__PHASEMixer::PHASEMixerDefinition;
#[cfg(feature = "PHASEMixer")]
pub use self::__PHASEMixer::PHASEMixerParameters;
#[cfg(all(feature = "PHASEDefinition", feature = "PHASEMixer"))]
pub use self::__PHASEMixer::PHASESpatialMixerDefinition;
#[cfg(feature = "PHASEObject")]
pub use self::__PHASEObject::PHASEObject;
#[cfg(all(feature = "PHASEObject", feature = "PHASEOccluder"))]
pub use self::__PHASEOccluder::PHASEOccluder;
#[cfg(feature = "PHASEShape")]
pub use self::__PHASEShape::PHASEShape;
#[cfg(feature = "PHASEShape")]
pub use self::__PHASEShape::PHASEShapeElement;
#[cfg(feature = "PHASESoundEvent")]
pub use self::__PHASESoundEvent::PHASESoundEvent;
#[cfg(all(feature = "PHASEDefinition", feature = "PHASESoundEventNodes"))]
pub use self::__PHASESoundEventNodes::PHASEBlendNodeDefinition;
#[cfg(all(feature = "PHASEDefinition", feature = "PHASESoundEventNodes"))]
pub use self::__PHASESoundEventNodes::PHASEContainerNodeDefinition;
#[cfg(all(feature = "PHASEDefinition", feature = "PHASESoundEventNodes"))]
pub use self::__PHASESoundEventNodes::PHASEGeneratorNodeDefinition;
#[cfg(feature = "PHASESoundEventNodes")]
pub use self::__PHASESoundEventNodes::PHASEPullStreamNode;
#[cfg(all(feature = "PHASEDefinition", feature = "PHASESoundEventNodes"))]
pub use self::__PHASESoundEventNodes::PHASEPullStreamNodeDefinition;
#[cfg(all(
    feature = "PHASESoundEventNodes",
    feature = "block2",
    feature = "objc2-avf-audio",
    feature = "objc2-core-audio-types"
))]
pub use self::__PHASESoundEventNodes::PHASEPullStreamRenderBlock;
#[cfg(feature = "PHASESoundEventNodes")]
pub use self::__PHASESoundEventNodes::PHASEPushStreamBufferOptions;
#[cfg(feature = "PHASESoundEventNodes")]
pub use self::__PHASESoundEventNodes::PHASEPushStreamCompletionCallbackCondition;
#[cfg(feature = "PHASESoundEventNodes")]
pub use self::__PHASESoundEventNodes::PHASEPushStreamNode;
#[cfg(all(feature = "PHASEDefinition", feature = "PHASESoundEventNodes"))]
pub use self::__PHASESoundEventNodes::PHASEPushStreamNodeDefinition;
#[cfg(all(feature = "PHASEDefinition", feature = "PHASESoundEventNodes"))]
pub use self::__PHASESoundEventNodes::PHASERandomNodeDefinition;
#[cfg(all(feature = "PHASEDefinition", feature = "PHASESoundEventNodes"))]
pub use self::__PHASESoundEventNodes::PHASESamplerNodeDefinition;
#[cfg(all(feature = "PHASEDefinition", feature = "PHASESoundEventNodes"))]
pub use self::__PHASESoundEventNodes::PHASESoundEventNodeDefinition;
#[cfg(feature = "PHASESoundEventNodes")]
pub use self::__PHASESoundEventNodes::PHASEStreamNode;
#[cfg(all(feature = "PHASEDefinition", feature = "PHASESoundEventNodes"))]
pub use self::__PHASESoundEventNodes::PHASESwitchNodeDefinition;
#[cfg(all(feature = "PHASEObject", feature = "PHASESource"))]
pub use self::__PHASESource::PHASESource;
#[cfg(feature = "PHASESpatialPipeline")]
pub use self::__PHASESpatialPipeline::PHASESpatialCategory;
#[cfg(feature = "PHASESpatialPipeline")]
pub use self::__PHASESpatialPipeline::PHASESpatialCategoryDirectPathTransmission;
#[cfg(feature = "PHASESpatialPipeline")]
pub use self::__PHASESpatialPipeline::PHASESpatialCategoryEarlyReflections;
#[cfg(feature = "PHASESpatialPipeline")]
pub use self::__PHASESpatialPipeline::PHASESpatialCategoryLateReverb;
#[cfg(feature = "PHASESpatialPipeline")]
pub use self::__PHASESpatialPipeline::PHASESpatialPipeline;
#[cfg(feature = "PHASESpatialPipeline")]
pub use self::__PHASESpatialPipeline::PHASESpatialPipelineEntry;
#[cfg(feature = "PHASESpatialPipeline")]
pub use self::__PHASESpatialPipeline::PHASESpatialPipelineFlags;
#[cfg(feature = "PHASETypes")]
pub use self::__PHASETypes::PHASEAssetError;
#[cfg(feature = "PHASETypes")]
pub use self::__PHASETypes::PHASEAssetErrorDomain;
#[cfg(feature = "PHASETypes")]
pub use self::__PHASETypes::PHASEAssetType;
#[cfg(feature = "PHASETypes")]
pub use self::__PHASETypes::PHASEAutomaticHeadTrackingFlags;
#[cfg(feature = "PHASETypes")]
pub use self::__PHASETypes::PHASECalibrationMode;
#[cfg(feature = "PHASETypes")]
pub use self::__PHASETypes::PHASECullOption;
#[cfg(feature = "PHASETypes")]
pub use self::__PHASETypes::PHASECurveType;
#[cfg(feature = "PHASETypes")]
pub use self::__PHASETypes::PHASEError;
#[cfg(feature = "PHASETypes")]
pub use self::__PHASETypes::PHASEErrorDomain;
#[cfg(feature = "PHASETypes")]
pub use self::__PHASETypes::PHASENormalizationMode;
#[cfg(feature = "PHASETypes")]
pub use self::__PHASETypes::PHASEPlaybackMode;
#[cfg(feature = "PHASETypes")]
pub use self::__PHASETypes::PHASERenderingState;
#[cfg(feature = "PHASETypes")]
pub use self::__PHASETypes::PHASEReverbPreset;
#[cfg(feature = "PHASETypes")]
pub use self::__PHASETypes::PHASESoundEventError;
#[cfg(feature = "PHASETypes")]
pub use self::__PHASETypes::PHASESoundEventErrorDomain;
#[cfg(feature = "PHASETypes")]
pub use self::__PHASETypes::PHASESoundEventPrepareHandlerReason;
#[cfg(feature = "PHASETypes")]
pub use self::__PHASETypes::PHASESoundEventPrepareState;
#[cfg(feature = "PHASETypes")]
pub use self::__PHASETypes::PHASESoundEventSeekHandlerReason;
#[cfg(feature = "PHASETypes")]
pub use self::__PHASETypes::PHASESoundEventStartHandlerReason;
#[cfg(feature = "PHASETypes")]
pub use self::__PHASETypes::PHASESpatializationMode;
#[cfg(feature = "PHASETypes")]
pub use self::__PHASETypes::PHASEUpdateMode;
