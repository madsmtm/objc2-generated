//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avmediaselectiongroup?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVMediaSelectionGroup;
);

unsafe impl NSCopying for AVMediaSelectionGroup {}

unsafe impl CopyingHelper for AVMediaSelectionGroup {
    type Result = Self;
}

unsafe impl NSObjectProtocol for AVMediaSelectionGroup {}

extern_methods!(
    unsafe impl AVMediaSelectionGroup {
        /// A collection of mutually exclusive media selection options.
        ///
        /// An NSArray of AVMediaSelectionOption*.
        #[method_id(@__retain_semantics Other options)]
        pub unsafe fn options(&self) -> Retained<NSArray<AVMediaSelectionOption>>;

        /// Indicates the default option in the group, i.e. the option that's intended for use in the absence of a specific end-user selection or preference.
        ///
        /// Can be nil, indicating that without a specific end-user selection or preference, no option in the group is intended to be selected.
        #[method_id(@__retain_semantics Other defaultOption)]
        pub unsafe fn defaultOption(&self) -> Option<Retained<AVMediaSelectionOption>>;

        /// Indicates whether it's possible to present none of the options in the group when an associated AVPlayerItem is played.
        ///
        /// If allowsEmptySelection is YES, all of the available media options in the group can be deselected by passing nil
        /// as the specified AVMediaSelectionOption to -[AVPlayerItem selectMediaOption:inMediaSelectionGroup:].
        #[method(allowsEmptySelection)]
        pub unsafe fn allowsEmptySelection(&self) -> bool;

        /// Returns the instance of AVMediaSelectionOption with properties that match the specified property list.
        ///
        /// Parameter `plist`: A property list previously obtained from an option in the group via -[AVMediaSelectionOption propertyList].
        ///
        /// Returns: If the specified properties match those of an option in the group, an instance of AVMediaSelectionOption. Otherwise nil.
        #[method_id(@__retain_semantics Other mediaSelectionOptionWithPropertyList:)]
        pub unsafe fn mediaSelectionOptionWithPropertyList(
            &self,
            plist: &AnyObject,
        ) -> Option<Retained<AVMediaSelectionOption>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVMediaSelectionGroup {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// AVMediaSelectionOptionFiltering
    /// Filtering of media selection options.
    ///
    /// The AVMediaSelectionOptionFiltering category is provided for convenience in filtering the media selection options in a group
    /// according to playability, locale, and media characteristics.
    /// Note that it's possible to implement additional filtering behaviors by using -[NSArray indexesOfObjectsPassingTest:].
    unsafe impl AVMediaSelectionGroup {
        /// Filters an array of AVMediaSelectionOptions according to whether they are playable.
        ///
        /// Parameter `mediaSelectionOptions`: An array of AVMediaSelectionOption to be filtered according to whether they are playable.
        ///
        /// Returns: An instance of NSArray containing the media selection options of the specified NSArray that are playable.
        #[method_id(@__retain_semantics Other playableMediaSelectionOptionsFromArray:)]
        pub unsafe fn playableMediaSelectionOptionsFromArray(
            media_selection_options: &NSArray<AVMediaSelectionOption>,
        ) -> Retained<NSArray<AVMediaSelectionOption>>;

        /// Filters an array of AVMediaSelectionOptions according to whether their locales match any language identifier in the specified array of preferred languages. The returned array is sorted according to the order of preference of the language each matches.
        ///
        /// Parameter `mediaSelectionOptions`: An array of AVMediaSelectionOptions to be filtered and sorted.
        ///
        /// Parameter `preferredLanguages`: An array of language identifiers in order of preference, each of which is an IETF BCP 47 (RFC 4646) language identifier. Use +[NSLocale preferredLanguages] to obtain the user's list of preferred languages.
        ///
        /// Returns: An instance of NSArray containing media selection options of the specified NSArray that match a preferred language, sorted according to the order of preference of the language each matches.
        #[method_id(@__retain_semantics Other mediaSelectionOptionsFromArray:filteredAndSortedAccordingToPreferredLanguages:)]
        pub unsafe fn mediaSelectionOptionsFromArray_filteredAndSortedAccordingToPreferredLanguages(
            media_selection_options: &NSArray<AVMediaSelectionOption>,
            preferred_languages: &NSArray<NSString>,
        ) -> Retained<NSArray<AVMediaSelectionOption>>;

        /// Filters an array of AVMediaSelectionOptions according to locale.
        ///
        /// Parameter `mediaSelectionOptions`: An array of AVMediaSelectionOption to be filtered by locale.
        ///
        /// Parameter `locale`: The NSLocale that must be matched for a media selection option to be copied to the output array.
        ///
        /// Returns: An instance of NSArray containing the media selection options of the specified NSArray that match the specified locale.
        #[method_id(@__retain_semantics Other mediaSelectionOptionsFromArray:withLocale:)]
        pub unsafe fn mediaSelectionOptionsFromArray_withLocale(
            media_selection_options: &NSArray<AVMediaSelectionOption>,
            locale: &NSLocale,
        ) -> Retained<NSArray<AVMediaSelectionOption>>;

        #[cfg(feature = "AVMediaFormat")]
        /// Filters an array of AVMediaSelectionOptions according to one or more media characteristics.
        ///
        /// Parameter `mediaSelectionOptions`: An array of AVMediaSelectionOptions to be filtered by media characteristic.
        ///
        /// Parameter `mediaCharacteristics`: The media characteristics that must be matched for a media selection option to be copied to the output array.
        ///
        /// Returns: An instance of NSArray containing the media selection options of the specified NSArray that match the specified
        /// media characteristics.
        #[method_id(@__retain_semantics Other mediaSelectionOptionsFromArray:withMediaCharacteristics:)]
        pub unsafe fn mediaSelectionOptionsFromArray_withMediaCharacteristics(
            media_selection_options: &NSArray<AVMediaSelectionOption>,
            media_characteristics: &NSArray<AVMediaCharacteristic>,
        ) -> Retained<NSArray<AVMediaSelectionOption>>;

        #[cfg(feature = "AVMediaFormat")]
        /// Filters an array of AVMediaSelectionOptions according to whether they lack one or more media characteristics.
        ///
        /// Parameter `mediaSelectionOptions`: An array of AVMediaSelectionOptions to be filtered by media characteristic.
        ///
        /// Parameter `mediaCharacteristics`: The media characteristics that must not be present for a media selection option to be copied to the output array.
        ///
        /// Returns: An instance of NSArray containing the media selection options of the specified NSArray that lack the specified
        /// media characteristics.
        #[method_id(@__retain_semantics Other mediaSelectionOptionsFromArray:withoutMediaCharacteristics:)]
        pub unsafe fn mediaSelectionOptionsFromArray_withoutMediaCharacteristics(
            media_selection_options: &NSArray<AVMediaSelectionOption>,
            media_characteristics: &NSArray<AVMediaCharacteristic>,
        ) -> Retained<NSArray<AVMediaSelectionOption>>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avmediaselectionoption?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVMediaSelectionOption;
);

unsafe impl NSCopying for AVMediaSelectionOption {}

unsafe impl CopyingHelper for AVMediaSelectionOption {
    type Result = Self;
}

unsafe impl NSObjectProtocol for AVMediaSelectionOption {}

extern_methods!(
    unsafe impl AVMediaSelectionOption {
        #[cfg(feature = "AVMediaFormat")]
        /// The media type of the media data, e.g. AVMediaTypeAudio, AVMediaTypeSubtitle, etc.
        #[method_id(@__retain_semantics Other mediaType)]
        pub unsafe fn mediaType(&self) -> Retained<AVMediaType>;

        /// The mediaSubTypes of the media data associated with the option.
        ///
        /// An NSArray of NSNumbers carrying four character codes (of type FourCharCode) as defined in CoreAudioTypes.h for audio media and in CMFormatDescription.h for video media.
        /// Also see CMFormatDescriptionGetMediaSubType in CMFormatDescription.h for more information about media subtypes.
        ///
        /// Note that if no information is available about the encoding of the media presented when a media option is selected, the value of mediaSubTypes will be an empty array. This can occur, for example, with streaming media. In these cases the value of mediaSubTypes should simply not be used as a criteria for selection.
        #[method_id(@__retain_semantics Other mediaSubTypes)]
        pub unsafe fn mediaSubTypes(&self) -> Retained<NSArray<NSNumber>>;

        #[cfg(feature = "AVMediaFormat")]
        /// Reports whether the media selection option includes media with the specified media characteristic.
        ///
        /// Parameter `mediaCharacteristic`: The media characteristic of interest, e.g. AVMediaCharacteristicVisual, AVMediaCharacteristicAudible, AVMediaCharacteristicLegible, etc.
        ///
        /// Returns: YES if the media selection option includes media with the specified characteristic, otherwise NO.
        #[method(hasMediaCharacteristic:)]
        pub unsafe fn hasMediaCharacteristic(
            &self,
            media_characteristic: &AVMediaCharacteristic,
        ) -> bool;

        /// Indicates whether a media selection option is playable.
        ///
        /// If the media data associated with the option cannot be decoded or otherwise rendered, playable is NO.
        #[method(isPlayable)]
        pub unsafe fn isPlayable(&self) -> bool;

        /// Indicates the RFC 4646 language tag associated with the option. May be nil.
        #[method_id(@__retain_semantics Other extendedLanguageTag)]
        pub unsafe fn extendedLanguageTag(&self) -> Option<Retained<NSString>>;

        /// Indicates the locale for which the media option was authored.
        ///
        /// Use -[NSLocale objectForKey:NSLocaleLanguageCode] to obtain the language code of the locale. See NSLocale.h for additional information.
        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Option<Retained<NSLocale>>;

        #[cfg(feature = "AVMetadataItem")]
        /// Provides an array of AVMetadataItems for each common metadata key for which a value is available.
        ///
        /// The array of AVMetadataItems can be filtered according to language via +[AVMetadataItem metadataItemsFromArray:filteredAndSortedAccordingToPreferredLanguages:], according to locale via +[AVMetadataItem metadataItemsFromArray:withLocale:],
        /// or according to key via +[AVMetadataItem metadataItemsFromArray:withKey:keySpace:].
        /// Example: to obtain the name (or title) of a media selection option in any of the user's preferred languages.
        ///
        /// NSString *title = nil;
        /// NSArray *titles = [AVMetadataItem metadataItemsFromArray:[mediaSelectionOption commonMetadata] withKey:AVMetadataCommonKeyTitle keySpace:AVMetadataKeySpaceCommon];
        /// if ([titles count] > 0)
        /// {
        /// // Try to get a title that matches one of the user's preferred languages.
        /// NSArray *titlesForPreferredLanguages = [AVMetadataItem metadataItemsFromArray:titles filteredAndSortedAccordingToPreferredLanguages:[NSLocale preferredLanguages]];
        /// if ([titlesForPreferredLanguages count] > 0)
        /// {
        /// title = [[titlesForPreferredLanguages objectAtIndex:0] stringValue];
        /// }
        ///
        /// // No matches in any of the preferred languages. Just use the primary title metadata we find.
        /// if (title == nil)
        /// {
        /// title = [[titles objectAtIndex:0] stringValue];
        /// }
        /// }
        #[method_id(@__retain_semantics Other commonMetadata)]
        pub unsafe fn commonMetadata(&self) -> Retained<NSArray<AVMetadataItem>>;

        /// Provides an NSArray of NSStrings, each representing a metadata format that contains metadata associated with the option (e.g. ID3, iTunes metadata, etc.).
        ///
        /// Metadata formats are defined in AVMetadataFormat.h.
        #[method_id(@__retain_semantics Other availableMetadataFormats)]
        pub unsafe fn availableMetadataFormats(&self) -> Retained<NSArray<NSString>>;

        #[cfg(feature = "AVMetadataItem")]
        /// Provides an NSArray of AVMetadataItems, one for each metadata item in the container of the specified format.
        ///
        /// Parameter `format`: The metadata format for which items are requested.
        ///
        /// Returns: An NSArray containing AVMetadataItems.
        #[method_id(@__retain_semantics Other metadataForFormat:)]
        pub unsafe fn metadataForFormat(
            &self,
            format: &NSString,
        ) -> Retained<NSArray<AVMetadataItem>>;

        /// If a media selection option in another group is associated with the specified option, returns a reference to the associated option.
        ///
        /// Parameter `mediaSelectionGroup`: A media selection group in which an associated option is to be sought.
        ///
        /// Returns: An instance of AVMediaSelectionOption.
        ///
        /// Audible media selection options often have associated legible media selection options; in particular, audible options are typically associated with forced-only subtitle options with the same locale. See AVMediaCharacteristicContainsOnlyForcedSubtitles in AVMediaFormat.h for a discussion of forced-only subtitles.
        #[method_id(@__retain_semantics Other associatedMediaSelectionOptionInMediaSelectionGroup:)]
        pub unsafe fn associatedMediaSelectionOptionInMediaSelectionGroup(
            &self,
            media_selection_group: &AVMediaSelectionGroup,
        ) -> Option<Retained<AVMediaSelectionOption>>;

        /// Returns a serializable property list that can be used to obtain an instance of AVMediaSelectionOption representing the same option as the receiver via -[AVMediaSelectionGroup mediaSelectionOptionWithPropertyList:].
        ///
        /// Returns: A serializable property list that's sufficient to identify the option within its group. For serialization utilities, see NSPropertyList.h.
        #[method_id(@__retain_semantics Other propertyList)]
        pub unsafe fn propertyList(&self) -> Retained<AnyObject>;

        /// Provides an NSString suitable for display.
        ///
        /// Parameter `locale`: Localize manufactured portions of the string using the specificed locale.
        ///
        /// May use this option's common metadata, media characteristics and locale properties in addition to the provided locale to formulate an NSString intended for display. Will only consider common metadata with the specified locale.
        #[method_id(@__retain_semantics Other displayNameWithLocale:)]
        pub unsafe fn displayNameWithLocale(&self, locale: &NSLocale) -> Retained<NSString>;

        /// Provides an NSString suitable for display using the current system locale.
        ///
        /// May use this option's common metadata, media characteristics and locale properties in addition to the current system locale to formulate an NSString intended for display.
        /// In the event that common metadata is not available in the specified locale, displayName will fall back to considering locales with the multilingual ("mul") then undetermined ("und") locale identifiers.
        /// For a display name strictly with the specified locale use displayNameWithLocale: instead.
        #[method_id(@__retain_semantics Other displayName)]
        pub unsafe fn displayName(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVMediaSelectionOption {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);