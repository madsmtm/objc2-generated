//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// An AVAssetVariant represents a bit rate variant.
    /// Each asset contains a collection of variants that represent a combination of audio, video, text, closed captions, and subtitles for a particular bit rate.
    /// Subclasses of this type that are used from Swift must fulfill the requirements of a Sendable type.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetvariant?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetVariant;
);

unsafe impl Send for AVAssetVariant {}

unsafe impl Sync for AVAssetVariant {}

unsafe impl NSObjectProtocol for AVAssetVariant {}

extern_methods!(
    unsafe impl AVAssetVariant {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        /// If it is not declared, the value will be negative.
        #[method(peakBitRate)]
        pub unsafe fn peakBitRate(&self) -> c_double;

        /// If it is not declared, the value will be negative.
        #[method(averageBitRate)]
        pub unsafe fn averageBitRate(&self) -> c_double;

        /// Provides  variant's video rendition attributes. If no video attributes are declared, it will be nil.
        #[method_id(@__retain_semantics Other videoAttributes)]
        pub unsafe fn videoAttributes(&self) -> Option<Retained<AVAssetVariantVideoAttributes>>;

        /// Provides  variant's audio rendition attributes. If no audio attributes are declared, it will be nil.
        #[method_id(@__retain_semantics Other audioAttributes)]
        pub unsafe fn audioAttributes(&self) -> Option<Retained<AVAssetVariantAudioAttributes>>;
    }
);

extern_class!(
    /// Video attributes for an asset variant.
    ///
    /// Subclasses of this type that are used from Swift must fulfill the requirements of a Sendable type.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetvariantvideoattributes?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetVariantVideoAttributes;
);

unsafe impl NSObjectProtocol for AVAssetVariantVideoAttributes {}

extern_methods!(
    unsafe impl AVAssetVariantVideoAttributes {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "AVMediaFormat")]
        /// Provides the video range of the variant. If it is not declared, it will be AVVideoRangeSDR.
        #[method_id(@__retain_semantics Other videoRange)]
        pub unsafe fn videoRange(&self) -> Retained<AVVideoRange>;

        /// Provides an array of video sample codec types present in the variant's renditions if any are declared. Each value in the array is a NSNumber representation of CMVideoCodecType.
        #[method_id(@__retain_semantics Other codecTypes)]
        pub unsafe fn codecTypes(&self) -> Retained<NSArray<NSNumber>>;

        #[cfg(feature = "objc2-core-foundation")]
        /// If it is not declared, it will be CGSizeZero.
        #[method(presentationSize)]
        pub unsafe fn presentationSize(&self) -> CGSize;

        /// If it is not declared, the value will be negative.
        #[method(nominalFrameRate)]
        pub unsafe fn nominalFrameRate(&self) -> c_double;

        /// Describes the video layout attributes.
        ///
        /// videoLayoutAttributes' count may be greater than one if this variant contains a collection of differing video layout media attributes over time.
        #[method_id(@__retain_semantics Other videoLayoutAttributes)]
        pub unsafe fn videoLayoutAttributes(
            &self,
        ) -> Retained<NSArray<AVAssetVariantVideoLayoutAttributes>>;
    }
);

extern_class!(
    /// Subclasses of this type that are used from Swift must fulfill the requirements of a Sendable type.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetvariantvideolayoutattributes?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetVariantVideoLayoutAttributes;
);

unsafe impl Send for AVAssetVariantVideoLayoutAttributes {}

unsafe impl Sync for AVAssetVariantVideoLayoutAttributes {}

unsafe impl NSObjectProtocol for AVAssetVariantVideoLayoutAttributes {}

extern_methods!(
    unsafe impl AVAssetVariantVideoLayoutAttributes {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "objc2-core-media")]
        /// Describes the stereo components. If not declared, the value will be `kCMStereoViewComponent_None`.
        /// In case of monoscopic content, the value will be `kCMStereoViewComponent_None` and incase of stereoscopic content, the value will be `(kCMStereoViewComponent_LeftEye | kCMStereoViewComponent_RightEye)`.
        #[method(stereoViewComponents)]
        pub unsafe fn stereoViewComponents(&self) -> CMStereoViewComponents;
    }
);

extern_class!(
    /// Audio attributes for an asset variant.
    ///
    /// Subclasses of this type that are used from Swift must fulfill the requirements of a Sendable type.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetvariantaudioattributes?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetVariantAudioAttributes;
);

unsafe impl Send for AVAssetVariantAudioAttributes {}

unsafe impl Sync for AVAssetVariantAudioAttributes {}

unsafe impl NSObjectProtocol for AVAssetVariantAudioAttributes {}

extern_methods!(
    unsafe impl AVAssetVariantAudioAttributes {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        /// Provides an array of audio formats present in the variant's renditions if any are declared. Each value in the array is a NSNumber representation of AudioFormatID.
        #[method_id(@__retain_semantics Other formatIDs)]
        pub unsafe fn formatIDs(&self) -> Retained<NSArray<NSNumber>>;

        #[cfg(feature = "AVMediaSelectionGroup")]
        /// Provides attributes for a specific audio media selection option. If no rendition specific attributes are declared, it will be nil.
        ///
        /// Parameter `mediaSelectionOption`: The option to return rendition specific information for.
        #[method_id(@__retain_semantics Other renditionSpecificAttributesForMediaOption:)]
        pub unsafe fn renditionSpecificAttributesForMediaOption(
            &self,
            media_selection_option: &AVMediaSelectionOption,
        ) -> Option<Retained<AVAssetVariantAudioRenditionSpecificAttributes>>;
    }
);

extern_class!(
    /// Audio rendition attributes for an asset variant.
    ///
    /// Subclasses of this type that are used from Swift must fulfill the requirements of a Sendable type.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetvariantaudiorenditionspecificattributes?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetVariantAudioRenditionSpecificAttributes;
);

unsafe impl Send for AVAssetVariantAudioRenditionSpecificAttributes {}

unsafe impl Sync for AVAssetVariantAudioRenditionSpecificAttributes {}

unsafe impl NSObjectProtocol for AVAssetVariantAudioRenditionSpecificAttributes {}

extern_methods!(
    unsafe impl AVAssetVariantAudioRenditionSpecificAttributes {
        /// If it is not declared, the value will be negative.
        ///
        /// A channel count greater than two indicates that the variant offers a rich multichannel authoring.
        #[method(channelCount)]
        pub unsafe fn channelCount(&self) -> NSInteger;

        /// Indicates that the variant is best suited for delivery to headphones.
        ///
        /// A binaural variant may originate from a direct binaural recording or from the processing of a multichannel audio source.
        #[method(isBinaural)]
        pub unsafe fn isBinaural(&self) -> bool;

        /// Indicates that this variant contains virtualized or otherwise pre-processed audio content that is suitable for a variety of purposes.
        ///
        /// If a variant audio redition is immersive it is eligible for rendering either to headphones or speakers.
        #[method(isImmersive)]
        pub unsafe fn isImmersive(&self) -> bool;

        /// Indicates that this variant is declared as a downmix derivative of other media of greater channel count.
        ///
        /// If one or more multichannel variants are also provided, the dowmix is assumed to be compatible in its internal timing and other attributes with those variants. Typically this is because it has been derived from the same source. A downmix can be used as a suitable substitute for a multichannel variant under some conditions.
        #[method(isDownmix)]
        pub unsafe fn isDownmix(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVAssetVariantAudioRenditionSpecificAttributes {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// The qualifier of an asset variant.
    ///
    /// Subclasses of this type that are used from Swift must fulfill the requirements of a Sendable type.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetvariantqualifier?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetVariantQualifier;
);

unsafe impl Send for AVAssetVariantQualifier {}

unsafe impl Sync for AVAssetVariantQualifier {}

unsafe impl NSCopying for AVAssetVariantQualifier {}

unsafe impl CopyingHelper for AVAssetVariantQualifier {
    type Result = Self;
}

unsafe impl NSObjectProtocol for AVAssetVariantQualifier {}

extern_methods!(
    unsafe impl AVAssetVariantQualifier {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        /// Returns a qualifer for a predicate.
        ///
        /// Parameter `predicate`: The variant predicate. Must be a valid, non-nil NSPredicate.
        #[method_id(@__retain_semantics Other assetVariantQualifierWithPredicate:)]
        pub unsafe fn assetVariantQualifierWithPredicate(predicate: &NSPredicate)
            -> Retained<Self>;

        /// Returns a qualifer for a particular asset variant.
        ///
        /// Parameter `variant`: A variant obtained from the -[AVAsset variants] or -[AVAssetDownloadConfiguration playableVariants]. Must be a valid, non-nil AVAssetVariant.
        #[method_id(@__retain_semantics Other assetVariantQualifierWithVariant:)]
        pub unsafe fn assetVariantQualifierWithVariant(variant: &AVAssetVariant) -> Retained<Self>;

        /// Returns a qualifer for finding variant with minimum value in the input key path.
        ///
        /// Parameter `keyPath`: AVAssetVariant keyPath. Allowed keyPath values are peakBitRate, averageBitRate, videoAttributes.presentationSize. Must be a valid, non-nil NSString.
        #[method_id(@__retain_semantics Other assetVariantQualifierForMinimumValueInKeyPath:)]
        pub unsafe fn assetVariantQualifierForMinimumValueInKeyPath(
            key_path: &NSString,
        ) -> Retained<Self>;

        /// Returns a qualifer for finding variant with maximum value in the input key path
        ///
        /// Parameter `keyPath`: AVAssetVariant keyPath. Allowed keyPath values are peakBitRate, averageBitRate, videoAttributes.presentationSize. Must be a valid, non-nil NSString.
        #[method_id(@__retain_semantics Other assetVariantQualifierForMaximumValueInKeyPath:)]
        pub unsafe fn assetVariantQualifierForMaximumValueInKeyPath(
            key_path: &NSString,
        ) -> Retained<Self>;

        #[cfg(feature = "AVMediaSelectionGroup")]
        /// Creates a NSPredicate for audio channel count which can be used with other NSPredicates to express variant preferences.
        ///
        /// Parameter `channelCount`: The RHS value for the channel count in the predicate equation.
        ///
        /// Parameter `mediaSelectionOption`: The audio media selection option under consideration.
        ///
        /// Parameter `operatorType`: The valid values are NSLessThanPredicateOperatorType, NSLessThanOrEqualToPredicateOperatorType, NSGreaterThanPredicateOperatorType, NSGreaterThanOrEqualToPredicateOperatorType, NSEqualToPredicateOperatorType and NSNotEqualToPredicateOperatorType.
        #[method_id(@__retain_semantics Other predicateForChannelCount:mediaSelectionOption:operatorType:)]
        pub unsafe fn predicateForChannelCount_mediaSelectionOption_operatorType(
            channel_count: NSInteger,
            media_selection_option: &AVMediaSelectionOption,
            operator_type: NSPredicateOperatorType,
        ) -> Retained<NSPredicate>;

        #[cfg(feature = "AVMediaSelectionGroup")]
        /// Creates a NSPredicate for binaural which can be used with other NSPredicates to express variant preferences.
        ///
        /// Parameter `isBinaural`: The RHS value for the value of isBinauralAudio in the predicate equation.
        ///
        /// Parameter `mediaSelectionOption`: The audio media selection option under consideration.
        #[method_id(@__retain_semantics Other predicateForBinauralAudio:mediaSelectionOption:)]
        pub unsafe fn predicateForBinauralAudio_mediaSelectionOption(
            is_binaural_audio: bool,
            media_selection_option: &AVMediaSelectionOption,
        ) -> Retained<NSPredicate>;

        #[cfg(feature = "AVMediaSelectionGroup")]
        /// Creates a NSPredicate for immersive audio which can be used with other NSPredicates to express variant preferences.
        ///
        /// Parameter `isImmersiveAudio`: The RHS value for the value of isImmersiveAudio in the predicate equation.
        ///
        /// Parameter `mediaSelectionOption`: The audio media selection option under consideration.
        #[method_id(@__retain_semantics Other predicateForImmersiveAudio:mediaSelectionOption:)]
        pub unsafe fn predicateForImmersiveAudio_mediaSelectionOption(
            is_immersive_audio: bool,
            media_selection_option: &AVMediaSelectionOption,
        ) -> Retained<NSPredicate>;

        #[cfg(feature = "AVMediaSelectionGroup")]
        /// Creates a NSPredicate for immersive audio which can be used with other NSPredicates to express variant preferences.
        ///
        /// Parameter `isDownmixAudio`: The RHS value for the value of isDownmixAudio in the predicate equation.
        ///
        /// Parameter `mediaSelectionOption`: The audio media selection option under consideration.
        #[method_id(@__retain_semantics Other predicateForDownmixAudio:mediaSelectionOption:)]
        pub unsafe fn predicateForDownmixAudio_mediaSelectionOption(
            is_downmix_audio: bool,
            media_selection_option: &AVMediaSelectionOption,
        ) -> Retained<NSPredicate>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Creates a NSPredicate for presentation size width which can be used with other NSPredicates to express variant preferences.
        ///
        /// Parameter `width`: The RHS value for the presentation size width in the predicate equation.
        ///
        /// Parameter `operatorType`: The valid values are NSLessThanPredicateOperatorType, NSLessThanOrEqualToPredicateOperatorType, NSGreaterThanPredicateOperatorType, NSGreaterThanOrEqualToPredicateOperatorType, NSEqualToPredicateOperatorType and NSNotEqualToPredicateOperatorType.
        #[method_id(@__retain_semantics Other predicateForPresentationWidth:operatorType:)]
        pub unsafe fn predicateForPresentationWidth_operatorType(
            width: CGFloat,
            operator_type: NSPredicateOperatorType,
        ) -> Retained<NSPredicate>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Creates a NSPredicate for presentation size height which can be used with other NSPredicates to express variant preferences.
        ///
        /// Parameter `height`: The RHS value for the presentation size height in the predicate equation.
        ///
        /// Parameter `operatorType`: The valid values are NSLessThanPredicateOperatorType, NSLessThanOrEqualToPredicateOperatorType, NSGreaterThanPredicateOperatorType, NSGreaterThanOrEqualToPredicateOperatorType, NSEqualToPredicateOperatorType and NSNotEqualToPredicateOperatorType.
        #[method_id(@__retain_semantics Other predicateForPresentationHeight:operatorType:)]
        pub unsafe fn predicateForPresentationHeight_operatorType(
            height: CGFloat,
            operator_type: NSPredicateOperatorType,
        ) -> Retained<NSPredicate>;

        #[cfg(feature = "AVMediaSelectionGroup")]
        /// Creates a NSPredicate for audio sample rate which can be used with other NSPredicates to express variant preferences.
        ///
        /// Parameter `sampleRate`: The RHS value for the sample rate in the predicate equation.
        ///
        /// Parameter `mediaSelectionOption`: The audio media selection option under consideration.
        ///
        /// Parameter `operatorType`: The valid values are NSLessThanPredicateOperatorType, NSLessThanOrEqualToPredicateOperatorType, NSGreaterThanPredicateOperatorType, NSGreaterThanOrEqualToPredicateOperatorType, NSEqualToPredicateOperatorType and NSNotEqualToPredicateOperatorType.
        #[method_id(@__retain_semantics Other predicateForAudioSampleRate:mediaSelectionOption:operatorType:)]
        pub unsafe fn predicateForAudioSampleRate_mediaSelectionOption_operatorType(
            sample_rate: c_double,
            media_selection_option: &AVMediaSelectionOption,
            operator_type: NSPredicateOperatorType,
        ) -> Retained<NSPredicate>;
    }
);