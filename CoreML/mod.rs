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

#[link(name = "CoreML", kind = "framework")]
extern "C" {}

#[cfg(feature = "MLAllComputeDevices")]
#[path = "MLAllComputeDevices.rs"]
mod __MLAllComputeDevices;
#[cfg(feature = "MLArrayBatchProvider")]
#[path = "MLArrayBatchProvider.rs"]
mod __MLArrayBatchProvider;
#[cfg(feature = "MLBatchProvider")]
#[path = "MLBatchProvider.rs"]
mod __MLBatchProvider;
#[cfg(feature = "MLCPUComputeDevice")]
#[path = "MLCPUComputeDevice.rs"]
mod __MLCPUComputeDevice;
#[cfg(feature = "MLComputeDeviceProtocol")]
#[path = "MLComputeDeviceProtocol.rs"]
mod __MLComputeDeviceProtocol;
#[cfg(feature = "MLComputePlan")]
#[path = "MLComputePlan.rs"]
mod __MLComputePlan;
#[cfg(feature = "MLComputePlanCost")]
#[path = "MLComputePlanCost.rs"]
mod __MLComputePlanCost;
#[cfg(feature = "MLComputePlanDeviceUsage")]
#[path = "MLComputePlanDeviceUsage.rs"]
mod __MLComputePlanDeviceUsage;
#[cfg(feature = "MLCustomLayer")]
#[path = "MLCustomLayer.rs"]
mod __MLCustomLayer;
#[cfg(feature = "MLCustomModel")]
#[path = "MLCustomModel.rs"]
mod __MLCustomModel;
#[cfg(feature = "MLDictionaryConstraint")]
#[path = "MLDictionaryConstraint.rs"]
mod __MLDictionaryConstraint;
#[cfg(feature = "MLDictionaryFeatureProvider")]
#[path = "MLDictionaryFeatureProvider.rs"]
mod __MLDictionaryFeatureProvider;
#[cfg(feature = "MLExport")]
#[path = "MLExport.rs"]
mod __MLExport;
#[cfg(feature = "MLFeatureDescription")]
#[path = "MLFeatureDescription.rs"]
mod __MLFeatureDescription;
#[cfg(feature = "MLFeatureProvider")]
#[path = "MLFeatureProvider.rs"]
mod __MLFeatureProvider;
#[cfg(feature = "MLFeatureType")]
#[path = "MLFeatureType.rs"]
mod __MLFeatureType;
#[cfg(feature = "MLFeatureValue")]
#[path = "MLFeatureValue.rs"]
mod __MLFeatureValue;
#[cfg(feature = "MLFeatureValue_MLImageConversion")]
#[path = "MLFeatureValue_MLImageConversion.rs"]
mod __MLFeatureValue_MLImageConversion;
#[cfg(feature = "MLGPUComputeDevice")]
#[path = "MLGPUComputeDevice.rs"]
mod __MLGPUComputeDevice;
#[cfg(feature = "MLImageConstraint")]
#[path = "MLImageConstraint.rs"]
mod __MLImageConstraint;
#[cfg(feature = "MLImageSize")]
#[path = "MLImageSize.rs"]
mod __MLImageSize;
#[cfg(feature = "MLImageSizeConstraint")]
#[path = "MLImageSizeConstraint.rs"]
mod __MLImageSizeConstraint;
#[cfg(feature = "MLImageSizeConstraintType")]
#[path = "MLImageSizeConstraintType.rs"]
mod __MLImageSizeConstraintType;
#[cfg(feature = "MLKey")]
#[path = "MLKey.rs"]
mod __MLKey;
#[cfg(feature = "MLMetricKey")]
#[path = "MLMetricKey.rs"]
mod __MLMetricKey;
#[cfg(feature = "MLModel")]
#[path = "MLModel.rs"]
mod __MLModel;
#[cfg(feature = "MLModelAsset")]
#[path = "MLModelAsset.rs"]
mod __MLModelAsset;
#[cfg(feature = "MLModelCollection")]
#[path = "MLModelCollection.rs"]
mod __MLModelCollection;
#[cfg(feature = "MLModelCollectionEntry")]
#[path = "MLModelCollectionEntry.rs"]
mod __MLModelCollectionEntry;
#[cfg(feature = "MLModelConfiguration")]
#[path = "MLModelConfiguration.rs"]
mod __MLModelConfiguration;
#[cfg(feature = "MLModelDescription")]
#[path = "MLModelDescription.rs"]
mod __MLModelDescription;
#[cfg(feature = "MLModelError")]
#[path = "MLModelError.rs"]
mod __MLModelError;
#[cfg(feature = "MLModelMetadataKeys")]
#[path = "MLModelMetadataKeys.rs"]
mod __MLModelMetadataKeys;
#[cfg(feature = "MLModelStructure")]
#[path = "MLModelStructure.rs"]
mod __MLModelStructure;
#[cfg(feature = "MLModelStructureNeuralNetwork")]
#[path = "MLModelStructureNeuralNetwork.rs"]
mod __MLModelStructureNeuralNetwork;
#[cfg(feature = "MLModelStructureNeuralNetworkLayer")]
#[path = "MLModelStructureNeuralNetworkLayer.rs"]
mod __MLModelStructureNeuralNetworkLayer;
#[cfg(feature = "MLModelStructurePipeline")]
#[path = "MLModelStructurePipeline.rs"]
mod __MLModelStructurePipeline;
#[cfg(feature = "MLModelStructureProgram")]
#[path = "MLModelStructureProgram.rs"]
mod __MLModelStructureProgram;
#[cfg(feature = "MLModelStructureProgramArgument")]
#[path = "MLModelStructureProgramArgument.rs"]
mod __MLModelStructureProgramArgument;
#[cfg(feature = "MLModelStructureProgramBinding")]
#[path = "MLModelStructureProgramBinding.rs"]
mod __MLModelStructureProgramBinding;
#[cfg(feature = "MLModelStructureProgramBlock")]
#[path = "MLModelStructureProgramBlock.rs"]
mod __MLModelStructureProgramBlock;
#[cfg(feature = "MLModelStructureProgramFunction")]
#[path = "MLModelStructureProgramFunction.rs"]
mod __MLModelStructureProgramFunction;
#[cfg(feature = "MLModelStructureProgramNamedValueType")]
#[path = "MLModelStructureProgramNamedValueType.rs"]
mod __MLModelStructureProgramNamedValueType;
#[cfg(feature = "MLModelStructureProgramOperation")]
#[path = "MLModelStructureProgramOperation.rs"]
mod __MLModelStructureProgramOperation;
#[cfg(feature = "MLModelStructureProgramValue")]
#[path = "MLModelStructureProgramValue.rs"]
mod __MLModelStructureProgramValue;
#[cfg(feature = "MLModelStructureProgramValueType")]
#[path = "MLModelStructureProgramValueType.rs"]
mod __MLModelStructureProgramValueType;
#[cfg(feature = "MLModel_MLComputeDevice")]
#[path = "MLModel_MLComputeDevice.rs"]
mod __MLModel_MLComputeDevice;
#[cfg(feature = "MLModel_MLModelCompilation")]
#[path = "MLModel_MLModelCompilation.rs"]
mod __MLModel_MLModelCompilation;
#[cfg(feature = "MLModel_MLState")]
#[path = "MLModel_MLState.rs"]
mod __MLModel_MLState;
#[cfg(feature = "MLMultiArray")]
#[path = "MLMultiArray.rs"]
mod __MLMultiArray;
#[cfg(feature = "MLMultiArrayConstraint")]
#[path = "MLMultiArrayConstraint.rs"]
mod __MLMultiArrayConstraint;
#[cfg(feature = "MLMultiArrayShapeConstraint")]
#[path = "MLMultiArrayShapeConstraint.rs"]
mod __MLMultiArrayShapeConstraint;
#[cfg(feature = "MLMultiArrayShapeConstraintType")]
#[path = "MLMultiArrayShapeConstraintType.rs"]
mod __MLMultiArrayShapeConstraintType;
#[cfg(feature = "MLNeuralEngineComputeDevice")]
#[path = "MLNeuralEngineComputeDevice.rs"]
mod __MLNeuralEngineComputeDevice;
#[cfg(feature = "MLNumericConstraint")]
#[path = "MLNumericConstraint.rs"]
mod __MLNumericConstraint;
#[cfg(feature = "MLOptimizationHints")]
#[path = "MLOptimizationHints.rs"]
mod __MLOptimizationHints;
#[cfg(feature = "MLParameterDescription")]
#[path = "MLParameterDescription.rs"]
mod __MLParameterDescription;
#[cfg(feature = "MLParameterKey")]
#[path = "MLParameterKey.rs"]
mod __MLParameterKey;
#[cfg(feature = "MLPredictionOptions")]
#[path = "MLPredictionOptions.rs"]
mod __MLPredictionOptions;
#[cfg(feature = "MLReshapeFrequencyHint")]
#[path = "MLReshapeFrequencyHint.rs"]
mod __MLReshapeFrequencyHint;
#[cfg(feature = "MLSequence")]
#[path = "MLSequence.rs"]
mod __MLSequence;
#[cfg(feature = "MLSequenceConstraint")]
#[path = "MLSequenceConstraint.rs"]
mod __MLSequenceConstraint;
#[cfg(feature = "MLSpecializationStrategy")]
#[path = "MLSpecializationStrategy.rs"]
mod __MLSpecializationStrategy;
#[cfg(feature = "MLState")]
#[path = "MLState.rs"]
mod __MLState;
#[cfg(feature = "MLStateConstraint")]
#[path = "MLStateConstraint.rs"]
mod __MLStateConstraint;
#[cfg(feature = "MLTask")]
#[path = "MLTask.rs"]
mod __MLTask;
#[cfg(feature = "MLUpdateContext")]
#[path = "MLUpdateContext.rs"]
mod __MLUpdateContext;
#[cfg(feature = "MLUpdateProgressEvent")]
#[path = "MLUpdateProgressEvent.rs"]
mod __MLUpdateProgressEvent;
#[cfg(feature = "MLUpdateProgressHandlers")]
#[path = "MLUpdateProgressHandlers.rs"]
mod __MLUpdateProgressHandlers;
#[cfg(feature = "MLUpdateTask")]
#[path = "MLUpdateTask.rs"]
mod __MLUpdateTask;
#[cfg(feature = "MLWritable")]
#[path = "MLWritable.rs"]
mod __MLWritable;

#[cfg(all(feature = "MLAllComputeDevices", feature = "MLComputeDeviceProtocol"))]
pub use self::__MLAllComputeDevices::MLAllComputeDevices;
#[cfg(feature = "MLArrayBatchProvider")]
pub use self::__MLArrayBatchProvider::MLArrayBatchProvider;
#[cfg(feature = "MLBatchProvider")]
pub use self::__MLBatchProvider::MLBatchProvider;
#[cfg(feature = "MLCPUComputeDevice")]
pub use self::__MLCPUComputeDevice::MLCPUComputeDevice;
#[cfg(feature = "MLComputeDeviceProtocol")]
pub use self::__MLComputeDeviceProtocol::MLComputeDeviceProtocol;
#[cfg(feature = "MLComputePlan")]
pub use self::__MLComputePlan::MLComputePlan;
#[cfg(feature = "MLComputePlanCost")]
pub use self::__MLComputePlanCost::MLComputePlanCost;
#[cfg(feature = "MLComputePlanDeviceUsage")]
pub use self::__MLComputePlanDeviceUsage::MLComputePlanDeviceUsage;
#[cfg(feature = "MLCustomLayer")]
pub use self::__MLCustomLayer::MLCustomLayer;
#[cfg(feature = "MLCustomModel")]
pub use self::__MLCustomModel::MLCustomModel;
#[cfg(feature = "MLDictionaryConstraint")]
pub use self::__MLDictionaryConstraint::MLDictionaryConstraint;
#[cfg(feature = "MLDictionaryFeatureProvider")]
pub use self::__MLDictionaryFeatureProvider::MLDictionaryFeatureProvider;
#[cfg(feature = "MLFeatureDescription")]
pub use self::__MLFeatureDescription::MLFeatureDescription;
#[cfg(feature = "MLFeatureProvider")]
pub use self::__MLFeatureProvider::MLFeatureProvider;
#[cfg(feature = "MLFeatureType")]
pub use self::__MLFeatureType::MLFeatureType;
#[cfg(feature = "MLFeatureValue")]
pub use self::__MLFeatureValue::MLFeatureValue;
#[cfg(feature = "MLFeatureValue_MLImageConversion")]
pub use self::__MLFeatureValue_MLImageConversion::MLFeatureValueImageOption;
#[cfg(feature = "MLFeatureValue_MLImageConversion")]
pub use self::__MLFeatureValue_MLImageConversion::MLFeatureValueImageOptionCropAndScale;
#[cfg(feature = "MLFeatureValue_MLImageConversion")]
pub use self::__MLFeatureValue_MLImageConversion::MLFeatureValueImageOptionCropRect;
#[cfg(feature = "MLGPUComputeDevice")]
pub use self::__MLGPUComputeDevice::MLGPUComputeDevice;
#[cfg(feature = "MLImageConstraint")]
pub use self::__MLImageConstraint::MLImageConstraint;
#[cfg(feature = "MLImageSize")]
pub use self::__MLImageSize::MLImageSize;
#[cfg(feature = "MLImageSizeConstraint")]
pub use self::__MLImageSizeConstraint::MLImageSizeConstraint;
#[cfg(feature = "MLImageSizeConstraintType")]
pub use self::__MLImageSizeConstraintType::MLImageSizeConstraintType;
#[cfg(feature = "MLKey")]
pub use self::__MLKey::MLKey;
#[cfg(all(feature = "MLKey", feature = "MLMetricKey"))]
pub use self::__MLMetricKey::MLMetricKey;
#[cfg(feature = "MLModel")]
pub use self::__MLModel::MLModel;
#[cfg(feature = "MLModelAsset")]
pub use self::__MLModelAsset::MLModelAsset;
#[cfg(feature = "MLModelCollection")]
pub use self::__MLModelCollection::MLModelCollection;
#[cfg(feature = "MLModelCollection")]
pub use self::__MLModelCollection::MLModelCollectionDidChangeNotification;
#[cfg(feature = "MLModelCollectionEntry")]
pub use self::__MLModelCollectionEntry::MLModelCollectionEntry;
#[cfg(feature = "MLModelConfiguration")]
pub use self::__MLModelConfiguration::MLComputeUnits;
#[cfg(feature = "MLModelConfiguration")]
pub use self::__MLModelConfiguration::MLModelConfiguration;
#[cfg(feature = "MLModelDescription")]
pub use self::__MLModelDescription::MLModelDescription;
#[cfg(feature = "MLModelError")]
pub use self::__MLModelError::MLModelError;
#[cfg(feature = "MLModelError")]
pub use self::__MLModelError::MLModelErrorDomain;
#[cfg(feature = "MLModelMetadataKeys")]
pub use self::__MLModelMetadataKeys::MLModelAuthorKey;
#[cfg(feature = "MLModelMetadataKeys")]
pub use self::__MLModelMetadataKeys::MLModelCreatorDefinedKey;
#[cfg(feature = "MLModelMetadataKeys")]
pub use self::__MLModelMetadataKeys::MLModelDescriptionKey;
#[cfg(feature = "MLModelMetadataKeys")]
pub use self::__MLModelMetadataKeys::MLModelLicenseKey;
#[cfg(feature = "MLModelMetadataKeys")]
pub use self::__MLModelMetadataKeys::MLModelMetadataKey;
#[cfg(feature = "MLModelMetadataKeys")]
pub use self::__MLModelMetadataKeys::MLModelVersionStringKey;
#[cfg(feature = "MLModelStructure")]
pub use self::__MLModelStructure::MLModelStructure;
#[cfg(feature = "MLModelStructureNeuralNetwork")]
pub use self::__MLModelStructureNeuralNetwork::MLModelStructureNeuralNetwork;
#[cfg(feature = "MLModelStructureNeuralNetworkLayer")]
pub use self::__MLModelStructureNeuralNetworkLayer::MLModelStructureNeuralNetworkLayer;
#[cfg(feature = "MLModelStructurePipeline")]
pub use self::__MLModelStructurePipeline::MLModelStructurePipeline;
#[cfg(feature = "MLModelStructureProgram")]
pub use self::__MLModelStructureProgram::MLModelStructureProgram;
#[cfg(feature = "MLModelStructureProgramArgument")]
pub use self::__MLModelStructureProgramArgument::MLModelStructureProgramArgument;
#[cfg(feature = "MLModelStructureProgramBinding")]
pub use self::__MLModelStructureProgramBinding::MLModelStructureProgramBinding;
#[cfg(feature = "MLModelStructureProgramBlock")]
pub use self::__MLModelStructureProgramBlock::MLModelStructureProgramBlock;
#[cfg(feature = "MLModelStructureProgramFunction")]
pub use self::__MLModelStructureProgramFunction::MLModelStructureProgramFunction;
#[cfg(feature = "MLModelStructureProgramNamedValueType")]
pub use self::__MLModelStructureProgramNamedValueType::MLModelStructureProgramNamedValueType;
#[cfg(feature = "MLModelStructureProgramOperation")]
pub use self::__MLModelStructureProgramOperation::MLModelStructureProgramOperation;
#[cfg(feature = "MLModelStructureProgramValue")]
pub use self::__MLModelStructureProgramValue::MLModelStructureProgramValue;
#[cfg(feature = "MLModelStructureProgramValueType")]
pub use self::__MLModelStructureProgramValueType::MLModelStructureProgramValueType;
#[cfg(feature = "MLMultiArray")]
pub use self::__MLMultiArray::MLMultiArray;
#[cfg(feature = "MLMultiArray")]
pub use self::__MLMultiArray::MLMultiArrayDataType;
#[cfg(feature = "MLMultiArrayConstraint")]
pub use self::__MLMultiArrayConstraint::MLMultiArrayConstraint;
#[cfg(feature = "MLMultiArrayShapeConstraint")]
pub use self::__MLMultiArrayShapeConstraint::MLMultiArrayShapeConstraint;
#[cfg(feature = "MLMultiArrayShapeConstraintType")]
pub use self::__MLMultiArrayShapeConstraintType::MLMultiArrayShapeConstraintType;
#[cfg(feature = "MLNeuralEngineComputeDevice")]
pub use self::__MLNeuralEngineComputeDevice::MLNeuralEngineComputeDevice;
#[cfg(feature = "MLNumericConstraint")]
pub use self::__MLNumericConstraint::MLNumericConstraint;
#[cfg(feature = "MLOptimizationHints")]
pub use self::__MLOptimizationHints::MLOptimizationHints;
#[cfg(feature = "MLParameterDescription")]
pub use self::__MLParameterDescription::MLParameterDescription;
#[cfg(all(feature = "MLKey", feature = "MLParameterKey"))]
pub use self::__MLParameterKey::MLParameterKey;
#[cfg(feature = "MLPredictionOptions")]
pub use self::__MLPredictionOptions::MLPredictionOptions;
#[cfg(feature = "MLReshapeFrequencyHint")]
pub use self::__MLReshapeFrequencyHint::MLReshapeFrequencyHint;
#[cfg(feature = "MLSequence")]
pub use self::__MLSequence::MLSequence;
#[cfg(feature = "MLSequenceConstraint")]
pub use self::__MLSequenceConstraint::MLSequenceConstraint;
#[cfg(feature = "MLSpecializationStrategy")]
pub use self::__MLSpecializationStrategy::MLSpecializationStrategy;
#[cfg(feature = "MLState")]
pub use self::__MLState::MLState;
#[cfg(feature = "MLStateConstraint")]
pub use self::__MLStateConstraint::MLStateConstraint;
#[cfg(feature = "MLTask")]
pub use self::__MLTask::MLTask;
#[cfg(feature = "MLTask")]
pub use self::__MLTask::MLTaskState;
#[cfg(feature = "MLUpdateContext")]
pub use self::__MLUpdateContext::MLUpdateContext;
#[cfg(feature = "MLUpdateProgressEvent")]
pub use self::__MLUpdateProgressEvent::MLUpdateProgressEvent;
#[cfg(feature = "MLUpdateProgressHandlers")]
pub use self::__MLUpdateProgressHandlers::MLUpdateProgressHandlers;
#[cfg(all(feature = "MLTask", feature = "MLUpdateTask"))]
pub use self::__MLUpdateTask::MLUpdateTask;
#[cfg(feature = "MLWritable")]
pub use self::__MLWritable::MLWritable;
