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
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcomposition?language=objc)
    #[unsafe(super(AVAsset, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AVAsset")]
    pub struct AVComposition;
);

#[cfg(all(feature = "AVAsset", feature = "AVAsynchronousKeyValueLoading"))]
unsafe impl AVAsynchronousKeyValueLoading for AVComposition {}

#[cfg(feature = "AVAsset")]
unsafe impl NSCopying for AVComposition {}

#[cfg(feature = "AVAsset")]
unsafe impl CopyingHelper for AVComposition {
    type Result = Self;
}

#[cfg(feature = "AVAsset")]
unsafe impl NSMutableCopying for AVComposition {}

#[cfg(feature = "AVAsset")]
unsafe impl MutableCopyingHelper for AVComposition {
    type Result = AVMutableComposition;
}

#[cfg(feature = "AVAsset")]
unsafe impl NSObjectProtocol for AVComposition {}

extern_methods!(
    #[cfg(feature = "AVAsset")]
    unsafe impl AVComposition {
        #[cfg(all(feature = "AVAssetTrack", feature = "AVCompositionTrack"))]
        /// Provides the array of AVCompositionTracks contained by the composition.
        #[method_id(@__retain_semantics Other tracks)]
        pub unsafe fn tracks(&self) -> Retained<NSArray<AVCompositionTrack>>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Indicates the authored size of the visual portion of the composition.
        #[method(naturalSize)]
        pub unsafe fn naturalSize(&self) -> CGSize;

        /// Specifies the initialization options for the creation of AVURLAssets by the receiver, e.g. AVURLAssetPreferPreciseDurationAndTimingKey. The default behavior for creation of AVURLAssets by an AVComposition is equivalent to the behavior of +[AVURLAsset URLAssetWithURL:options:] when specifying no initialization options.
        ///
        /// AVCompositions create AVURLAssets internally for URLs specified by AVCompositionTrackSegments of AVCompositionTracks, as needed, whenever AVCompositionTrackSegments were originally added to a track via -[AVMutableCompositionTrack setSegments:] rather than by inserting timeranges of already existing AVAssets or AVAssetTracks.
        /// The value of URLAssetInitializationOptions can be specified at the time an AVMutableComposition is created via +compositionWithURLAssetInitializationOptions:.
        #[method_id(@__retain_semantics Other URLAssetInitializationOptions)]
        pub unsafe fn URLAssetInitializationOptions(
            &self,
        ) -> Retained<NSDictionary<NSString, AnyObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `AVAsset`
    #[cfg(feature = "AVAsset")]
    unsafe impl AVComposition {
        /// Returns an instance of AVAsset for inspection of a media resource.
        ///
        /// Parameter `URL`: An instance of NSURL that references a media resource.
        ///
        /// Returns: An instance of AVAsset.
        ///
        /// Returns a newly allocated instance of a subclass of AVAsset initialized with the specified URL.
        #[method_id(@__retain_semantics Other assetWithURL:)]
        pub unsafe fn assetWithURL(url: &NSURL) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AVAsset")]
    unsafe impl AVComposition {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// AVCompositionTrackInspection
    #[cfg(feature = "AVAsset")]
    unsafe impl AVComposition {
        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVCompositionTrack",
            feature = "objc2-core-media"
        ))]
        /// Provides an instance of AVCompositionTrack that represents the track of the specified trackID.
        ///
        /// Parameter `trackID`: The trackID of the requested AVCompositionTrack.
        ///
        /// Returns: An instance of AVCompositionTrack; may be nil if no track of the specified trackID is available.
        ///
        /// Becomes callable without blocking when the key
        /// "
        /// tracks" has been loaded
        #[method_id(@__retain_semantics Other trackWithTrackID:)]
        pub unsafe fn trackWithTrackID(
            &self,
            track_id: CMPersistentTrackID,
        ) -> Option<Retained<AVCompositionTrack>>;

        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVCompositionTrack",
            feature = "block2",
            feature = "objc2-core-media"
        ))]
        /// Loads an instance of AVCompositionTrack that represents the track of the specified trackID.
        ///
        /// Parameter `trackID`: The trackID of the requested AVCompositionTrack.
        ///
        /// Parameter `completionHandler`: A block that is called when the loading is finished, with either the loaded track (which may be nil if no track of the specified trackID is available) or an error.
        #[method(loadTrackWithTrackID:completionHandler:)]
        pub unsafe fn loadTrackWithTrackID_completionHandler(
            &self,
            track_id: CMPersistentTrackID,
            completion_handler: &block2::Block<dyn Fn(*mut AVCompositionTrack, *mut NSError)>,
        );

        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVCompositionTrack",
            feature = "AVMediaFormat"
        ))]
        /// Provides an array of AVCompositionTracks of the asset that present media of the specified media type.
        ///
        /// Parameter `mediaType`: The media type according to which the receiver filters its AVCompositionTracks. (Media types are defined in AVMediaFormat.h)
        ///
        /// Returns: An NSArray of AVCompositionTracks; may be empty if no tracks of the specified media type are available.
        ///
        /// Becomes callable without blocking when the key
        /// "
        /// tracks" has been loaded
        #[method_id(@__retain_semantics Other tracksWithMediaType:)]
        pub unsafe fn tracksWithMediaType(
            &self,
            media_type: &AVMediaType,
        ) -> Retained<NSArray<AVCompositionTrack>>;

        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVCompositionTrack",
            feature = "AVMediaFormat",
            feature = "block2"
        ))]
        /// Loads an array of AVCompositionTracks of the asset that present media of the specified media type.
        ///
        /// Parameter `mediaType`: The media type according to which AVAsset filters its AVCompositionTracks. (Media types are defined in AVMediaFormat.h.)
        ///
        /// Parameter `completionHandler`: A block that is called when the loading is finished, with either the loaded tracks (which may be empty if no tracks of the specified media type are available) or an error.
        #[method(loadTracksWithMediaType:completionHandler:)]
        pub unsafe fn loadTracksWithMediaType_completionHandler(
            &self,
            media_type: &AVMediaType,
            completion_handler: &block2::Block<
                dyn Fn(*mut NSArray<AVCompositionTrack>, *mut NSError),
            >,
        );

        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVCompositionTrack",
            feature = "AVMediaFormat"
        ))]
        /// Provides an array of AVCompositionTracks of the asset that present media with the specified characteristic.
        ///
        /// Parameter `mediaCharacteristic`: The media characteristic according to which the receiver filters its AVCompositionTracks. (Media characteristics are defined in AVMediaFormat.h)
        ///
        /// Returns: An NSArray of AVCompositionTracks; may be empty if no tracks with the specified characteristic are available.
        ///
        /// Becomes callable without blocking when the key
        /// "
        /// tracks" has been loaded
        #[method_id(@__retain_semantics Other tracksWithMediaCharacteristic:)]
        pub unsafe fn tracksWithMediaCharacteristic(
            &self,
            media_characteristic: &AVMediaCharacteristic,
        ) -> Retained<NSArray<AVCompositionTrack>>;

        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVCompositionTrack",
            feature = "AVMediaFormat",
            feature = "block2"
        ))]
        /// Loads an array of AVCompositionTracks of the asset that present media with the specified characteristic.
        ///
        /// Parameter `mediaCharacteristic`: The media characteristic according to which AVAsset filters its AVCompositionTracks. (Media characteristics are defined in AVMediaFormat.h.)
        ///
        /// Parameter `completionHandler`: A block that is called when the loading is finished, with either the loaded tracks (which may be empty if no tracks with the specified characteristic are available) or an error.
        #[method(loadTracksWithMediaCharacteristic:completionHandler:)]
        pub unsafe fn loadTracksWithMediaCharacteristic_completionHandler(
            &self,
            media_characteristic: &AVMediaCharacteristic,
            completion_handler: &block2::Block<
                dyn Fn(*mut NSArray<AVCompositionTrack>, *mut NSError),
            >,
        );
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avmutablecomposition?language=objc)
    #[unsafe(super(AVComposition, AVAsset, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AVAsset")]
    pub struct AVMutableComposition;
);

#[cfg(all(feature = "AVAsset", feature = "AVAsynchronousKeyValueLoading"))]
unsafe impl AVAsynchronousKeyValueLoading for AVMutableComposition {}

#[cfg(feature = "AVAsset")]
unsafe impl NSCopying for AVMutableComposition {}

#[cfg(feature = "AVAsset")]
unsafe impl CopyingHelper for AVMutableComposition {
    type Result = AVComposition;
}

#[cfg(feature = "AVAsset")]
unsafe impl NSMutableCopying for AVMutableComposition {}

#[cfg(feature = "AVAsset")]
unsafe impl MutableCopyingHelper for AVMutableComposition {
    type Result = Self;
}

#[cfg(feature = "AVAsset")]
unsafe impl NSObjectProtocol for AVMutableComposition {}

extern_methods!(
    #[cfg(feature = "AVAsset")]
    unsafe impl AVMutableComposition {
        #[cfg(all(feature = "AVAssetTrack", feature = "AVCompositionTrack"))]
        /// Provides the array of AVMutableCompositionTracks contained by the composition.
        #[method_id(@__retain_semantics Other tracks)]
        pub unsafe fn tracks(&self) -> Retained<NSArray<AVMutableCompositionTrack>>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Indicates the authored size of the visual portion of the asset.
        ///
        /// If not set, the value is the size of the composition's first video track. Set to CGSizeZero to revert to default behavior.
        #[method(naturalSize)]
        pub unsafe fn naturalSize(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`naturalSize`][Self::naturalSize].
        #[method(setNaturalSize:)]
        pub unsafe fn setNaturalSize(&self, natural_size: CGSize);

        /// Returns an empty AVMutableComposition.
        #[method_id(@__retain_semantics Other composition)]
        pub unsafe fn composition() -> Retained<Self>;

        /// Returns an empty AVMutableComposition.
        ///
        /// Parameter `URLAssetInitializationOptions`: Specifies the initialization options that the receiver should use when creating AVURLAssets internally, e.g. AVURLAssetPreferPreciseDurationAndTimingKey. The default behavior for creation of AVURLAssets by an AVMutableComposition is equivalent to the behavior of +[AVURLAsset URLAssetWithURL:options:] when specifying no initialization options.
        ///
        /// AVMutableCompositions create AVURLAssets internally for URLs specified by AVCompositionTrackSegments of AVMutableCompositionTracks, as needed, whenever AVCompositionTrackSegments are added to tracks via -[AVMutableCompositionTrack setSegments:] rather than by inserting timeranges of already existing AVAssets or AVAssetTracks.
        #[method_id(@__retain_semantics Other compositionWithURLAssetInitializationOptions:)]
        pub unsafe fn compositionWithURLAssetInitializationOptions(
            url_asset_initialization_options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `AVAsset`
    #[cfg(feature = "AVAsset")]
    unsafe impl AVMutableComposition {
        /// Returns an instance of AVAsset for inspection of a media resource.
        ///
        /// Parameter `URL`: An instance of NSURL that references a media resource.
        ///
        /// Returns: An instance of AVAsset.
        ///
        /// Returns a newly allocated instance of a subclass of AVAsset initialized with the specified URL.
        #[method_id(@__retain_semantics Other assetWithURL:)]
        pub unsafe fn assetWithURL(url: &NSURL) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AVAsset")]
    unsafe impl AVMutableComposition {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// AVMutableCompositionCompositionLevelEditing
    #[cfg(feature = "AVAsset")]
    unsafe impl AVMutableComposition {
        #[cfg(feature = "objc2-core-media")]
        /// Inserts all the tracks of a timeRange of an asset into a composition.
        ///
        /// Parameter `timeRange`: Specifies the timeRange of the asset to be inserted.
        ///
        /// Parameter `asset`: Specifies the asset that contains the tracks that are to be inserted. Only instances of AVURLAsset and AVComposition are supported (AVComposition starting in macOS 10.10 and iOS 8.0). The asset should have its tracks loaded, and the tracks should have their formatDescriptions loaded before invoking this method to avoid blocking.
        ///
        /// Parameter `startTime`: Specifies the time at which the inserted tracks are to be presented by the composition.
        ///
        /// Parameter `outError`: Describes failures that may be reported to the user, e.g. the asset that was selected for insertion in the composition is restricted by copy-protection.
        ///
        /// Returns: A BOOL value indicating the success of the insertion.
        ///
        /// You provide a reference to an AVAsset and the timeRange within it that you want to insert. You specify the start time in the destination composition at which the timeRange should be inserted.
        /// This method may add new tracks to ensure that all tracks of the asset are represented in the inserted timeRange.
        /// Note that the media data for the inserted timeRange will be presented at its natural duration and rate. It can be scaled to a different duration and presented at a different rate via -scaleTimeRange:toDuration:.
        /// Existing content at the specified startTime will be pushed out by the duration of timeRange.
        /// Note that this operation only inserts one or more track segments into affected AVMutableCompositionTracks; it does not affect the values of other track properties, either to match the corresponding values of tracks in the source asset or for any other purpose.
        #[deprecated]
        #[method(insertTimeRange:ofAsset:atTime:error:_)]
        pub unsafe fn insertTimeRange_ofAsset_atTime_error(
            &self,
            time_range: CMTimeRange,
            asset: &AVAsset,
            start_time: CMTime,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(all(feature = "block2", feature = "objc2-core-media"))]
        /// Inserts all the tracks of a timeRange of an asset into a composition.
        ///
        /// Parameter `timeRange`: Specifies the timeRange of the asset to be inserted.
        ///
        /// Parameter `asset`: Specifies the asset that contains the tracks that are to be inserted. Only instances of AVURLAsset and AVComposition are supported (AVComposition starting in macOS 10.10 and iOS 8.0).
        ///
        /// Parameter `startTime`: Specifies the time at which the inserted tracks are to be presented by the composition.
        ///
        /// Parameter `completionHandler`: A block that is invoked when the insertion is complete.  If the error parameter is non-nil, it describes a failure that may be reported to the user, e.g. the asset that was selected for insertion in the composition is restricted by copy-protection.
        ///
        /// You provide a reference to an AVAsset and the timeRange within it that you want to insert. You specify the start time in the destination composition at which the timeRange should be inserted.
        /// This method may add new tracks to ensure that all tracks of the asset are represented in the inserted timeRange.
        /// Note that the media data for the inserted timeRange will be presented at its natural duration and rate. It can be scaled to a different duration and presented at a different rate via -scaleTimeRange:toDuration:.
        /// Existing content at the specified startTime will be pushed out by the duration of timeRange.
        /// Note that this operation only inserts one or more track segments into affected AVMutableCompositionTracks; it does not affect the values of other track properties, either to match the corresponding values of tracks in the source asset or for any other purpose.
        #[method(insertTimeRange:ofAsset:atTime:completionHandler:)]
        pub unsafe fn insertTimeRange_ofAsset_atTime_completionHandler(
            &self,
            time_range: CMTimeRange,
            asset: &AVAsset,
            start_time: CMTime,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "objc2-core-media")]
        /// Adds or extends an empty timeRange within all tracks of the composition.
        ///
        /// Parameter `timeRange`: Specifies the empty timeRange to be inserted.
        ///
        /// If you insert an empty timeRange into the composition, any media that was presented during that interval prior to the insertion will be presented instead immediately afterward. You can use this method to reserve an interval in which you want a subsequently created track to present its media.
        /// Note that you cannot add empty time ranges to the end of a composition.
        #[method(insertEmptyTimeRange:)]
        pub unsafe fn insertEmptyTimeRange(&self, time_range: CMTimeRange);

        #[cfg(feature = "objc2-core-media")]
        /// Removes a specified timeRange from all tracks of the composition.
        ///
        /// Parameter `timeRange`: Specifies the timeRange to be removed.
        ///
        /// Removal of a time range does not cause any existing tracks to be removed from the composition, even if removing timeRange results in an empty track. Instead, it removes or truncates track segments that intersect with the timeRange.
        ///
        /// After removing, existing content after timeRange will be pulled in.
        #[method(removeTimeRange:)]
        pub unsafe fn removeTimeRange(&self, time_range: CMTimeRange);

        #[cfg(feature = "objc2-core-media")]
        /// Changes the duration of a timeRange of all tracks.
        ///
        /// Parameter `timeRange`: Specifies the timeRange of the composition to be scaled.
        ///
        /// Parameter `duration`: Specifies the new duration of the timeRange.
        ///
        /// Each trackSegment affected by the scaling operation will be presented at a rate equal to source.duration / target.duration of its resulting timeMapping.
        #[method(scaleTimeRange:toDuration:)]
        pub unsafe fn scaleTimeRange_toDuration(&self, time_range: CMTimeRange, duration: CMTime);
    }
);

extern_methods!(
    /// AVMutableCompositionTrackLevelEditing
    #[cfg(feature = "AVAsset")]
    unsafe impl AVMutableComposition {
        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVCompositionTrack",
            feature = "AVMediaFormat",
            feature = "objc2-core-media"
        ))]
        /// Adds an empty track to a mutable composition.
        ///
        /// Parameter `mediaType`: The media type of the new track.
        ///
        /// Parameter `preferredTrackID`: Specifies the preferred track ID for the new track. If you do not need to specify a preferred track ID, pass kCMPersistentTrackID_Invalid. Otherwise the preferred track ID will be used for the new track, provided that it is not currently in use and has not previously been used.
        ///
        /// Returns: An instance of AVMutableCompositionTrack representing the new track. Its actual trackID is available via its
        /// "
        /// trackID" key.
        ///
        /// If the specified preferred track ID is not available, or kCMPersistentTrackID_Invalid was passed in, a unique track ID will be generated.
        #[method_id(@__retain_semantics Other addMutableTrackWithMediaType:preferredTrackID:)]
        pub unsafe fn addMutableTrackWithMediaType_preferredTrackID(
            &self,
            media_type: &AVMediaType,
            preferred_track_id: CMPersistentTrackID,
        ) -> Option<Retained<AVMutableCompositionTrack>>;

        #[cfg(all(feature = "AVAssetTrack", feature = "AVCompositionTrack"))]
        /// Removes a track of a mutable composition.
        ///
        /// Parameter `track`: A reference to the AVCompositionTrack to be removed.
        ///
        /// If you retain a reference to the removed track, note that its
        /// "
        /// composition" key will have the value nil, and the values of its other properties are undefined.
        #[method(removeTrack:)]
        pub unsafe fn removeTrack(&self, track: &AVCompositionTrack);

        #[cfg(all(feature = "AVAssetTrack", feature = "AVCompositionTrack"))]
        /// Provides a reference to a track of a mutable composition into which any timeRange of an AVAssetTrack can be inserted (via -[AVMutableCompositionTrack insertTimeRange:ofTrack:atTime:error:]).
        ///
        /// Parameter `track`: A reference to the AVAssetTrack from which a timeRange may be inserted.
        ///
        /// Returns: An AVMutableCompositionTrack that can accommodate the insertion, or, if no such track is available, nil.
        ///
        /// If a compatible track is desired but the result of this method is nil, a new track of the same mediaType as the AVAssetTrack can be created via -addMutableTrackWithMediaType:preferredTrackID:, and this new track will be compatible.
        ///
        /// For best performance, the number of tracks of a composition should be kept to a minimum, corresponding to the number for which media data must be presented in parallel. If media data of the same type is to be presented serially, even from multiple assets, a single track of that media type should be used. This method, -mutableTrackCompatibleWithTrack:, can help the client to identify an existing target track for an insertion.
        ///
        /// Similar to -[AVAsset compatibleTrackForCompositionTrack:].
        #[method_id(@__retain_semantics Other mutableTrackCompatibleWithTrack:)]
        pub unsafe fn mutableTrackCompatibleWithTrack(
            &self,
            track: &AVAssetTrack,
        ) -> Option<Retained<AVMutableCompositionTrack>>;
    }
);

extern_methods!(
    /// AVMutableCompositionTrackInspection
    #[cfg(feature = "AVAsset")]
    unsafe impl AVMutableComposition {
        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVCompositionTrack",
            feature = "objc2-core-media"
        ))]
        /// Provides an instance of AVMutableCompositionTrack that represents the track of the specified trackID.
        ///
        /// Parameter `trackID`: The trackID of the requested AVMutableCompositionTrack.
        ///
        /// Returns: An instance of AVMutableCompositionTrack; may be nil if no track of the specified trackID is available.
        ///
        /// Becomes callable without blocking when the key
        /// "
        /// tracks" has been loaded
        #[method_id(@__retain_semantics Other trackWithTrackID:)]
        pub unsafe fn trackWithTrackID(
            &self,
            track_id: CMPersistentTrackID,
        ) -> Option<Retained<AVMutableCompositionTrack>>;

        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVCompositionTrack",
            feature = "block2",
            feature = "objc2-core-media"
        ))]
        /// Loads an instance of AVMutableCompositionTrack that represents the track of the specified trackID.
        ///
        /// Parameter `trackID`: The trackID of the requested AVMutableCompositionTrack.
        ///
        /// Parameter `completionHandler`: A block that is called when the loading is finished, with either the loaded track (which may be nil if no track of the specified trackID is available) or an error.
        #[method(loadTrackWithTrackID:completionHandler:)]
        pub unsafe fn loadTrackWithTrackID_completionHandler(
            &self,
            track_id: CMPersistentTrackID,
            completion_handler: &block2::Block<
                dyn Fn(*mut AVMutableCompositionTrack, *mut NSError),
            >,
        );

        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVCompositionTrack",
            feature = "AVMediaFormat"
        ))]
        /// Provides an array of AVMutableCompositionTracks of the asset that present media of the specified media type.
        ///
        /// Parameter `mediaType`: The media type according to which the receiver filters its AVMutableCompositionTracks. (Media types are defined in AVMediaFormat.h)
        ///
        /// Returns: An NSArray of AVMutableCompositionTracks; may be empty if no tracks of the specified media type are available.
        ///
        /// Becomes callable without blocking when the key
        /// "
        /// tracks" has been loaded
        #[method_id(@__retain_semantics Other tracksWithMediaType:)]
        pub unsafe fn tracksWithMediaType(
            &self,
            media_type: &AVMediaType,
        ) -> Retained<NSArray<AVMutableCompositionTrack>>;

        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVCompositionTrack",
            feature = "AVMediaFormat",
            feature = "block2"
        ))]
        /// Loads an array of AVMutableCompositionTracks of the asset that present media of the specified media type.
        ///
        /// Parameter `mediaType`: The media type according to which AVAsset filters its AVMutableCompositionTracks. (Media types are defined in AVMediaFormat.h.)
        ///
        /// Parameter `completionHandler`: A block that is called when the loading is finished, with either the loaded tracks (which may be empty if no tracks of the specified media type are available) or an error.
        #[method(loadTracksWithMediaType:completionHandler:)]
        pub unsafe fn loadTracksWithMediaType_completionHandler(
            &self,
            media_type: &AVMediaType,
            completion_handler: &block2::Block<
                dyn Fn(*mut NSArray<AVMutableCompositionTrack>, *mut NSError),
            >,
        );

        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVCompositionTrack",
            feature = "AVMediaFormat"
        ))]
        /// Provides an array of AVMutableCompositionTracks of the asset that present media with the specified characteristic.
        ///
        /// Parameter `mediaCharacteristic`: The media characteristic according to which the receiver filters its AVMutableCompositionTracks. (Media characteristics are defined in AVMediaFormat.h)
        ///
        /// Returns: An NSArray of AVMutableCompositionTracks; may be empty if no tracks with the specified characteristic are available.
        ///
        /// Becomes callable without blocking when the key
        /// "
        /// tracks" has been loaded
        #[method_id(@__retain_semantics Other tracksWithMediaCharacteristic:)]
        pub unsafe fn tracksWithMediaCharacteristic(
            &self,
            media_characteristic: &AVMediaCharacteristic,
        ) -> Retained<NSArray<AVMutableCompositionTrack>>;

        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVCompositionTrack",
            feature = "AVMediaFormat",
            feature = "block2"
        ))]
        /// Loads an array of AVMutableCompositionTracks of the asset that present media with the specified characteristic.
        ///
        /// Parameter `mediaCharacteristic`: The media characteristic according to which AVAsset filters its AVMutableCompositionTracks. (Media characteristics are defined in AVMediaFormat.h.)
        ///
        /// Parameter `completionHandler`: A block that is called when the loading is finished, with either the loaded tracks (which may be empty if no tracks with the specified characteristic are available) or an error.
        #[method(loadTracksWithMediaCharacteristic:completionHandler:)]
        pub unsafe fn loadTracksWithMediaCharacteristic_completionHandler(
            &self,
            media_characteristic: &AVMediaCharacteristic,
            completion_handler: &block2::Block<
                dyn Fn(*mut NSArray<AVMutableCompositionTrack>, *mut NSError),
            >,
        );
    }
);

extern_methods!(
    /// SynchronousAssetInterface
    /// Redeclarations of async-only AVAsset interfaces to allow synchronous usage in the synchronous subclass.
    ///
    /// See AVAsset's interface for more information about these interfaces.
    #[cfg(feature = "AVAsset")]
    unsafe impl AVComposition {
        #[cfg(all(feature = "AVMetadataFormat", feature = "AVMetadataItem"))]
        #[method_id(@__retain_semantics Other metadataForFormat:)]
        pub unsafe fn metadataForFormat(
            &self,
            format: &AVMetadataFormat,
        ) -> Retained<NSArray<AVMetadataItem>>;

        #[cfg(all(feature = "AVMetadataFormat", feature = "AVTimedMetadataGroup"))]
        #[method_id(@__retain_semantics Other chapterMetadataGroupsWithTitleLocale:containingItemsWithCommonKeys:)]
        pub unsafe fn chapterMetadataGroupsWithTitleLocale_containingItemsWithCommonKeys(
            &self,
            locale: &NSLocale,
            common_keys: Option<&NSArray<AVMetadataKey>>,
        ) -> Retained<NSArray<AVTimedMetadataGroup>>;

        #[cfg(feature = "AVTimedMetadataGroup")]
        #[method_id(@__retain_semantics Other chapterMetadataGroupsBestMatchingPreferredLanguages:)]
        pub unsafe fn chapterMetadataGroupsBestMatchingPreferredLanguages(
            &self,
            preferred_languages: &NSArray<NSString>,
        ) -> Retained<NSArray<AVTimedMetadataGroup>>;

        #[cfg(all(feature = "AVMediaFormat", feature = "AVMediaSelectionGroup"))]
        #[method_id(@__retain_semantics Other mediaSelectionGroupForMediaCharacteristic:)]
        pub unsafe fn mediaSelectionGroupForMediaCharacteristic(
            &self,
            media_characteristic: &AVMediaCharacteristic,
        ) -> Option<Retained<AVMediaSelectionGroup>>;

        #[cfg(feature = "objc2-core-media")]
        #[method(unusedTrackID)]
        pub unsafe fn unusedTrackID(&self) -> CMPersistentTrackID;
    }
);