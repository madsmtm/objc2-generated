//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/shazamkit/shmediaitemproperty?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type SHMediaItemProperty = NSString;

extern "C" {
    /// The Shazam media ID
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/shazamkit/shmediaitemshazamid?language=objc)
    pub static SHMediaItemShazamID: &'static SHMediaItemProperty;
}

extern "C" {
    /// Title
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/shazamkit/shmediaitemtitle?language=objc)
    pub static SHMediaItemTitle: &'static SHMediaItemProperty;
}

extern "C" {
    /// Subtitle
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/shazamkit/shmediaitemsubtitle?language=objc)
    pub static SHMediaItemSubtitle: &'static SHMediaItemProperty;
}

extern "C" {
    /// Artist
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/shazamkit/shmediaitemartist?language=objc)
    pub static SHMediaItemArtist: &'static SHMediaItemProperty;
}

extern "C" {
    /// A web URL representing this result
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/shazamkit/shmediaitemweburl?language=objc)
    pub static SHMediaItemWebURL: &'static SHMediaItemProperty;
}

extern "C" {
    /// The AppleMusic ID
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/shazamkit/shmediaitemapplemusicid?language=objc)
    pub static SHMediaItemAppleMusicID: &'static SHMediaItemProperty;
}

extern "C" {
    /// A link to this media on Apple Music
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/shazamkit/shmediaitemapplemusicurl?language=objc)
    pub static SHMediaItemAppleMusicURL: &'static SHMediaItemProperty;
}

extern "C" {
    /// A URL to the artwork
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/shazamkit/shmediaitemartworkurl?language=objc)
    pub static SHMediaItemArtworkURL: &'static SHMediaItemProperty;
}

extern "C" {
    /// A URL for a Video associated with the media
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/shazamkit/shmediaitemvideourl?language=objc)
    pub static SHMediaItemVideoURL: &'static SHMediaItemProperty;
}

extern "C" {
    /// Is this content explicit
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/shazamkit/shmediaitemexplicitcontent?language=objc)
    pub static SHMediaItemExplicitContent: &'static SHMediaItemProperty;
}

extern "C" {
    /// An array of strings representing the genres of the media item
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/shazamkit/shmediaitemgenres?language=objc)
    pub static SHMediaItemGenres: &'static SHMediaItemProperty;
}

extern "C" {
    /// The International Standard Recording Code
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/shazamkit/shmediaitemisrc?language=objc)
    pub static SHMediaItemISRC: &'static SHMediaItemProperty;
}

extern "C" {
    /// The time ranges in the represented media that are described by this
    /// `SHMediaItem`
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/shazamkit/shmediaitemtimeranges?language=objc)
    pub static SHMediaItemTimeRanges: &'static SHMediaItemProperty;
}

extern "C" {
    /// The frequency skew ranges that are described by this
    /// `SHMediaItem`
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/shazamkit/shmediaitemfrequencyskewranges?language=objc)
    pub static SHMediaItemFrequencySkewRanges: &'static SHMediaItemProperty;
}

extern "C" {
    /// The date when the media item was created
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/shazamkit/shmediaitemcreationdate?language=objc)
    pub static SHMediaItemCreationDate: &'static SHMediaItemProperty;
}

extern_class!(
    /// `SHMediaItem`represents metadata associated with a
    /// `SHSignature`
    /// A
    /// `SHMediaItem`is used in two distinct ways
    /// 1. As the base class of a
    /// `SHMatchedMediaItem,`and therefore as the result of a match
    /// 2. As a way of associating metadata with reference signatures in a
    /// `SHCustomCatalog`
    /// A SHMediaItem contains no required fields and may be entirely blank, they can also contain custom data set with custom keys when making a
    /// `SHCustomCatalog.`
    /// Note: `SHMediaItem`is not intended to be subclassed further.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/shazamkit/shmediaitem?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SHMediaItem;
);

unsafe impl Send for SHMediaItem {}

unsafe impl Sync for SHMediaItem {}

extern_conformance!(
    unsafe impl NSCoding for SHMediaItem {}
);

extern_conformance!(
    unsafe impl NSCopying for SHMediaItem {}
);

unsafe impl CopyingHelper for SHMediaItem {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for SHMediaItem {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for SHMediaItem {}
);

impl SHMediaItem {
    extern_methods!(
        /// The Shazam Media ID
        ///
        /// Note: This may be fetched using the key
        /// `SHMediaItemShazamID`
        #[unsafe(method(shazamID))]
        #[unsafe(method_family = none)]
        pub unsafe fn shazamID(&self) -> Option<Retained<NSString>>;

        /// The Title
        ///
        /// Note: This may be fetched using the key
        /// `SHMediaItemTitle`
        #[unsafe(method(title))]
        #[unsafe(method_family = none)]
        pub unsafe fn title(&self) -> Option<Retained<NSString>>;

        /// The Subtitle
        ///
        /// Note: This may be fetched using the key
        /// `SHMediaItemSubtitle`
        #[unsafe(method(subtitle))]
        #[unsafe(method_family = none)]
        pub unsafe fn subtitle(&self) -> Option<Retained<NSString>>;

        /// The Artist
        ///
        /// Note: This may be fetched using the key
        /// `SHMediaItemArtist`
        #[unsafe(method(artist))]
        #[unsafe(method_family = none)]
        pub unsafe fn artist(&self) -> Option<Retained<NSString>>;

        /// The Genre Names
        ///
        /// Note: This may be fetched using the key
        /// `SHMediaItemGenres`
        /// An array of strings representing the genres of the media item. Will return an empty array if there are no genres.
        #[unsafe(method(genres))]
        #[unsafe(method_family = none)]
        pub unsafe fn genres(&self) -> Retained<NSArray<NSString>>;

        /// The Apple Music ID
        ///
        /// Note: This may be fetched using the key
        /// `SHMediaItemAppleMusicID`
        #[unsafe(method(appleMusicID))]
        #[unsafe(method_family = none)]
        pub unsafe fn appleMusicID(&self) -> Option<Retained<NSString>>;

        /// The Apple Music URL
        ///
        /// Note: This may be fetched using the key
        /// `SHMediaItemAppleMusicURL`
        #[unsafe(method(appleMusicURL))]
        #[unsafe(method_family = none)]
        pub unsafe fn appleMusicURL(&self) -> Option<Retained<NSURL>>;

        /// The Web URL
        ///
        /// The URL will point to a page that displays the current object in its entirety
        ///
        /// Note: This may be fetched using the key
        /// `SHMediaItemWebURL`
        #[unsafe(method(webURL))]
        #[unsafe(method_family = none)]
        pub unsafe fn webURL(&self) -> Option<Retained<NSURL>>;

        /// The Artwork URL
        ///
        /// Note: This may be fetched using the key
        /// `SHMediaItemArtworkURL`
        #[unsafe(method(artworkURL))]
        #[unsafe(method_family = none)]
        pub unsafe fn artworkURL(&self) -> Option<Retained<NSURL>>;

        /// The VideoURL
        ///
        /// Note: This may be fetched using the key
        /// `SHMediaItemVideoURL`
        #[unsafe(method(videoURL))]
        #[unsafe(method_family = none)]
        pub unsafe fn videoURL(&self) -> Option<Retained<NSURL>>;

        /// Whether this object represents explicit material
        ///
        /// Note: This may be fetched using the key
        /// `SHMediaItemExplicitContent`
        #[unsafe(method(explicitContent))]
        #[unsafe(method_family = none)]
        pub unsafe fn explicitContent(&self) -> bool;

        /// The International Standard Recording Code
        ///
        /// Note: This may be fetched using the key
        /// `SHMediaItemISRC`
        #[unsafe(method(isrc))]
        #[unsafe(method_family = none)]
        pub unsafe fn isrc(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "SHRange")]
        /// An array of
        /// `SHRange`that indicate the offsets within the reference signature that this media item describes
        #[unsafe(method(timeRanges))]
        #[unsafe(method_family = none)]
        pub unsafe fn timeRanges(&self) -> Retained<NSArray<SHRange>>;

        #[cfg(feature = "SHRange")]
        /// An array of
        /// `SHRange`that indicate the frequency skews in the reference signature that this media item describes
        #[unsafe(method(frequencySkewRanges))]
        #[unsafe(method_family = none)]
        pub unsafe fn frequencySkewRanges(&self) -> Retained<NSArray<SHRange>>;

        /// The date when the
        /// `SHMediaItem`was created
        ///
        /// Note: This may be fetched using the key
        /// `SHMediaItemCreationDate`
        #[unsafe(method(creationDate))]
        #[unsafe(method_family = none)]
        pub unsafe fn creationDate(&self) -> Option<Retained<NSDate>>;

        /// Construct a new instance with the provided dictionary
        ///
        /// Parameter `properties`: A dictionary of
        /// `SHMediaItemProperty`and their values
        ///
        /// You may add your own keys here to return custom data, custom data should conform to NSCoding
        #[unsafe(method(mediaItemWithProperties:))]
        #[unsafe(method_family = none)]
        pub unsafe fn mediaItemWithProperties(
            properties: &NSDictionary<SHMediaItemProperty, AnyObject>,
        ) -> Retained<Self>;

        #[cfg(feature = "block2")]
        /// Fetch a
        /// `SHMediaItem`by Shazam ID
        ///
        /// The completionHandler will contain a
        /// `SHMediaItem`if the ShazamID is valid, otherwise nil and an error
        #[unsafe(method(fetchMediaItemWithShazamID:completionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn fetchMediaItemWithShazamID_completionHandler(
            shazam_id: &NSString,
            completion_handler: &block2::DynBlock<dyn Fn(*mut SHMediaItem, *mut NSError)>,
        );

        /// Retrieve a value using a known key
        ///
        /// Parameter `property`: The `SHMediaItemProperty` for a value
        #[unsafe(method(valueForProperty:))]
        #[unsafe(method_family = none)]
        pub unsafe fn valueForProperty(
            &self,
            property: &SHMediaItemProperty,
        ) -> Retained<AnyObject>;

        /// Use subscripting to retrieve values
        ///
        /// Parameter `key`: The `SHMediaItemProperty` or custom key for a value
        #[unsafe(method(objectForKeyedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn objectForKeyedSubscript(
            &self,
            key: &SHMediaItemProperty,
        ) -> Retained<AnyObject>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}
