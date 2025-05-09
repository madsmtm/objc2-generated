//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-avf-audio")]
use objc2_avf_audio::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// ***********************************************************************************************
    ///
    ///
    ///
    /// An object that represents a registered asset in the asset registry.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/phase/phaseasset?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHASEAsset;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for PHASEAsset {}
);

impl PHASEAsset {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        /// The identifier that uniquely represents this asset.
        #[unsafe(method(identifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;
    );
}

extern_class!(
    /// *************************************************************************************************
    ///
    ///
    ///
    /// An object that represents a registered sound asset in the asset registry.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/phase/phasesoundasset?language=objc)
    #[unsafe(super(PHASEAsset, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHASESoundAsset;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for PHASESoundAsset {}
);

impl PHASESoundAsset {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        /// The URL of the sound asset, if applicable.
        #[unsafe(method(url))]
        #[unsafe(method_family = none)]
        pub unsafe fn url(&self) -> Option<Retained<NSURL>>;

        /// The buffer for the sound asset, if applicable.
        #[unsafe(method(data))]
        #[unsafe(method_family = none)]
        pub unsafe fn data(&self) -> Option<Retained<NSData>>;

        #[cfg(feature = "PHASETypes")]
        /// The sound asset type.
        #[unsafe(method(type))]
        #[unsafe(method_family = none)]
        pub unsafe fn r#type(&self) -> PHASEAssetType;
    );
}

extern_class!(
    /// *************************************************************************************************
    ///
    ///
    ///
    /// An object that represents a registered sound event asset in the asset registry.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/phase/phasesoundeventnodeasset?language=objc)
    #[unsafe(super(PHASEAsset, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHASESoundEventNodeAsset;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for PHASESoundEventNodeAsset {}
);

impl PHASESoundEventNodeAsset {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// *************************************************************************************************
    ///
    ///
    ///
    /// An object that represents a registered global metaparameter asset in the asset registry.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/phase/phaseglobalmetaparameterasset?language=objc)
    #[unsafe(super(PHASEAsset, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHASEGlobalMetaParameterAsset;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for PHASEGlobalMetaParameterAsset {}
);

impl PHASEGlobalMetaParameterAsset {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// *************************************************************************************************
    ///
    ///
    ///
    /// Asset registry
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/phase/phaseassetregistry?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHASEAssetRegistry;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for PHASEAssetRegistry {}
);

impl PHASEAssetRegistry {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(all(feature = "PHASEDefinition", feature = "PHASEMetaParameter"))]
        /// Register a global metaparameter with the asset registry.
        ///
        /// Note: This function is synchronous and thread-safe.
        /// Clients can safely run this function to register multiple global metaparameters from multiple threads, if required.
        ///
        /// Parameter `metaParameterDefinition`: The metaparameter object to register.
        ///
        /// Parameter `error`: The error object in case of an error.
        ///
        /// Returns: A PHASEGlobalMetaParameterAsset object.
        #[unsafe(method(registerGlobalMetaParameter:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn registerGlobalMetaParameter_error(
            &self,
            meta_parameter_definition: &PHASEMetaParameterDefinition,
        ) -> Result<Retained<PHASEGlobalMetaParameterAsset>, Retained<NSError>>;

        #[cfg(all(feature = "PHASEDefinition", feature = "PHASESoundEventNodes"))]
        /// Register a sound event asset with the asset registry.
        ///
        /// Note: This function is synchronous and thread-safe.
        /// Clients can safely run this function to register multiple sound event assets from multiple threads, if required.
        ///
        /// Parameter `rootNode`: The root node of the sound event asset to register.
        ///
        /// Parameter `identifier`: An identifier that uniquely represents this sound event asset. Nil generates an automatic identifier.
        ///
        /// Parameter `error`: The error object in case of an error
        ///
        /// Returns: A PHASESoundEventNodeAsset object
        #[unsafe(method(registerSoundEventAssetWithRootNode:identifier:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn registerSoundEventAssetWithRootNode_identifier_error(
            &self,
            root_node: &PHASESoundEventNodeDefinition,
            identifier: Option<&NSString>,
        ) -> Result<Retained<PHASESoundEventNodeAsset>, Retained<NSError>>;

        #[cfg(all(feature = "PHASETypes", feature = "objc2-avf-audio"))]
        /// Register an audio file as a sound asset in the system.
        ///
        /// Note: This function is synchronous and thread-safe.
        /// Clients can safely run this function to register multiple sound assets from multiple threads, if required.
        ///
        /// Parameter `url`: The URL of the audio file.
        ///
        /// Parameter `identifier`: An identifier that uniquely represents this sound event asset. Nil generates an automatic identifier.
        ///
        /// Parameter `assetType`: The asset type for this sound asset.
        ///
        /// Parameter `channelLayout`: The audio channel layout for this sound asset.
        /// If a valid channel layout definition is read from the file being registered, this will override it.
        /// If nil is passed as a value for this property, the file must either be mono or stereo, or already contain a vaild channel layout definition.
        /// This channel layout must have the same channel count as the audio file being loaded.
        ///
        /// Parameter `normalizationMode`: The normalization mode.
        ///
        /// Parameter `error`: The error object in case of an error
        ///
        /// Returns: A PHASESoundAsset object
        #[unsafe(method(registerSoundAssetAtURL:identifier:assetType:channelLayout:normalizationMode:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn registerSoundAssetAtURL_identifier_assetType_channelLayout_normalizationMode_error(
            &self,
            url: &NSURL,
            identifier: Option<&NSString>,
            asset_type: PHASEAssetType,
            channel_layout: Option<&AVAudioChannelLayout>,
            normalization_mode: PHASENormalizationMode,
        ) -> Result<Retained<PHASESoundAsset>, Retained<NSError>>;

        #[cfg(all(feature = "PHASETypes", feature = "objc2-avf-audio"))]
        /// Register audio data as a sound asset in the system.
        ///
        /// Note: This function is synchronous and thread-safe.
        /// Clients can safely run this function to register multiple sound assets from multiple threads, if required.
        ///
        /// Parameter `data`: A buffer containing the audio data to register as a sound asset.
        /// Audio data must either be a single PCM buffer of interleaved channels or multiple deinterleaved PCM buffers per channel packed back to back.
        ///
        /// Parameter `identifier`: The identifier to assign to this sound asset. Nil generates an automatic identifier.
        ///
        /// Parameter `format`: The AVAudioFormat object that describes the audio data in the buffer.
        ///
        /// Parameter `normalizationMode`: The normalization mode.
        ///
        /// Parameter `error`: The error object in case of an error.
        ///
        /// Returns: A PHASESoundAsset object.
        #[unsafe(method(registerSoundAssetWithData:identifier:format:normalizationMode:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn registerSoundAssetWithData_identifier_format_normalizationMode_error(
            &self,
            data: &NSData,
            identifier: Option<&NSString>,
            format: &AVAudioFormat,
            normalization_mode: PHASENormalizationMode,
        ) -> Result<Retained<PHASESoundAsset>, Retained<NSError>>;

        /// Finds an asset in the asset registry, given an identifier.
        ///
        /// Parameter `identifier`: The identifier of this asset
        ///
        /// Returns: A PHASEAsset object, or nil if one could not be found.
        #[unsafe(method(assetForIdentifier:))]
        #[unsafe(method_family = none)]
        pub unsafe fn assetForIdentifier(
            &self,
            identifier: &NSString,
        ) -> Option<Retained<PHASEAsset>>;

        #[cfg(feature = "PHASEMetaParameter")]
        /// A dictionary of global metaparameters
        #[unsafe(method(globalMetaParameters))]
        #[unsafe(method_family = none)]
        pub unsafe fn globalMetaParameters(
            &self,
        ) -> Retained<NSDictionary<NSString, PHASEMetaParameter>>;
    );
}
