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

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avmoviereferencerestrictionskey?language=objc)
    pub static AVMovieReferenceRestrictionsKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avmovieshouldsupportaliasdatareferenceskey?language=objc)
    pub static AVMovieShouldSupportAliasDataReferencesKey: &'static NSString;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avmovie?language=objc)
    #[unsafe(super(AVAsset, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AVAsset")]
    pub struct AVMovie;
);

#[cfg(all(feature = "AVAsset", feature = "AVAsynchronousKeyValueLoading"))]
unsafe impl AVAsynchronousKeyValueLoading for AVMovie {}

#[cfg(feature = "AVAsset")]
unsafe impl NSCopying for AVMovie {}

#[cfg(feature = "AVAsset")]
unsafe impl CopyingHelper for AVMovie {
    type Result = Self;
}

#[cfg(feature = "AVAsset")]
unsafe impl NSMutableCopying for AVMovie {}

#[cfg(feature = "AVAsset")]
unsafe impl MutableCopyingHelper for AVMovie {
    type Result = AVMutableMovie;
}

#[cfg(feature = "AVAsset")]
unsafe impl NSObjectProtocol for AVMovie {}

extern_methods!(
    #[cfg(feature = "AVAsset")]
    unsafe impl AVMovie {
        #[cfg(feature = "AVMediaFormat")]
        #[method_id(@__retain_semantics Other movieTypes)]
        pub unsafe fn movieTypes() -> Retained<NSArray<AVFileType>>;

        #[method_id(@__retain_semantics Other movieWithURL:options:)]
        pub unsafe fn movieWithURL_options(
            url: &NSURL,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithURL:options:)]
        pub unsafe fn initWithURL_options(
            this: Allocated<Self>,
            url: &NSURL,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other movieWithData:options:)]
        pub unsafe fn movieWithData_options(
            data: &NSData,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithData:options:)]
        pub unsafe fn initWithData_options(
            this: Allocated<Self>,
            data: &NSData,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Retained<NSURL>>;

        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data(&self) -> Option<Retained<NSData>>;

        #[method_id(@__retain_semantics Other defaultMediaDataStorage)]
        pub unsafe fn defaultMediaDataStorage(&self) -> Option<Retained<AVMediaDataStorage>>;

        #[cfg(all(feature = "AVAssetTrack", feature = "AVMovieTrack"))]
        #[method_id(@__retain_semantics Other tracks)]
        pub unsafe fn tracks(&self) -> Retained<NSArray<AVMovieTrack>>;

        #[method(canContainMovieFragments)]
        pub unsafe fn canContainMovieFragments(&self) -> bool;

        #[method(containsMovieFragments)]
        pub unsafe fn containsMovieFragments(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `AVAsset`
    #[cfg(feature = "AVAsset")]
    unsafe impl AVMovie {
        #[method_id(@__retain_semantics Other assetWithURL:)]
        pub unsafe fn assetWithURL(url: &NSURL) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AVAsset")]
    unsafe impl AVMovie {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avmoviewritingoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVMovieWritingOptions(pub NSUInteger);
bitflags::bitflags! {
    impl AVMovieWritingOptions: NSUInteger {
        const AVMovieWritingAddMovieHeaderToDestination = 0;
        const AVMovieWritingTruncateDestinationToMovieHeaderOnly = 1<<0;
    }
}

unsafe impl Encode for AVMovieWritingOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for AVMovieWritingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// AVMovieMovieHeaderSupport
    #[cfg(feature = "AVAsset")]
    unsafe impl AVMovie {
        #[cfg(feature = "AVMediaFormat")]
        #[method_id(@__retain_semantics Other movieHeaderWithFileType:error:_)]
        pub unsafe fn movieHeaderWithFileType_error(
            &self,
            file_type: &AVFileType,
        ) -> Result<Retained<NSData>, Retained<NSError>>;

        #[cfg(feature = "AVMediaFormat")]
        #[method(writeMovieHeaderToURL:fileType:options:error:_)]
        pub unsafe fn writeMovieHeaderToURL_fileType_options_error(
            &self,
            url: &NSURL,
            file_type: &AVFileType,
            options: AVMovieWritingOptions,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "AVMediaFormat")]
        #[method(isCompatibleWithFileType:)]
        pub unsafe fn isCompatibleWithFileType(&self, file_type: &AVFileType) -> bool;
    }
);

extern_methods!(
    /// AVMovieTrackInspection
    #[cfg(feature = "AVAsset")]
    unsafe impl AVMovie {
        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVMovieTrack",
            feature = "objc2-core-media"
        ))]
        #[deprecated = "Use loadTrackWithTrackID:completionHandler: instead"]
        #[method_id(@__retain_semantics Other trackWithTrackID:)]
        pub unsafe fn trackWithTrackID(
            &self,
            track_id: CMPersistentTrackID,
        ) -> Option<Retained<AVMovieTrack>>;

        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVMovieTrack",
            feature = "block2",
            feature = "objc2-core-media"
        ))]
        #[method(loadTrackWithTrackID:completionHandler:)]
        pub unsafe fn loadTrackWithTrackID_completionHandler(
            &self,
            track_id: CMPersistentTrackID,
            completion_handler: &block2::Block<dyn Fn(*mut AVMovieTrack, *mut NSError)>,
        );

        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVMediaFormat",
            feature = "AVMovieTrack"
        ))]
        #[deprecated = "Use loadTracksWithMediaType:completionHandler: instead"]
        #[method_id(@__retain_semantics Other tracksWithMediaType:)]
        pub unsafe fn tracksWithMediaType(
            &self,
            media_type: &AVMediaType,
        ) -> Retained<NSArray<AVMovieTrack>>;

        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVMediaFormat",
            feature = "AVMovieTrack",
            feature = "block2"
        ))]
        #[method(loadTracksWithMediaType:completionHandler:)]
        pub unsafe fn loadTracksWithMediaType_completionHandler(
            &self,
            media_type: &AVMediaType,
            completion_handler: &block2::Block<dyn Fn(*mut NSArray<AVMovieTrack>, *mut NSError)>,
        );

        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVMediaFormat",
            feature = "AVMovieTrack"
        ))]
        #[deprecated = "Use loadTracksWithMediaCharacteristic:completionHandler: instead"]
        #[method_id(@__retain_semantics Other tracksWithMediaCharacteristic:)]
        pub unsafe fn tracksWithMediaCharacteristic(
            &self,
            media_characteristic: &AVMediaCharacteristic,
        ) -> Retained<NSArray<AVMovieTrack>>;

        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVMediaFormat",
            feature = "AVMovieTrack",
            feature = "block2"
        ))]
        #[method(loadTracksWithMediaCharacteristic:completionHandler:)]
        pub unsafe fn loadTracksWithMediaCharacteristic_completionHandler(
            &self,
            media_characteristic: &AVMediaCharacteristic,
            completion_handler: &block2::Block<dyn Fn(*mut NSArray<AVMovieTrack>, *mut NSError)>,
        );
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avmutablemovie?language=objc)
    #[unsafe(super(AVMovie, AVAsset, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AVAsset")]
    pub struct AVMutableMovie;
);

#[cfg(all(feature = "AVAsset", feature = "AVAsynchronousKeyValueLoading"))]
unsafe impl AVAsynchronousKeyValueLoading for AVMutableMovie {}

#[cfg(feature = "AVAsset")]
unsafe impl NSCopying for AVMutableMovie {}

#[cfg(feature = "AVAsset")]
unsafe impl CopyingHelper for AVMutableMovie {
    type Result = AVMovie;
}

#[cfg(feature = "AVAsset")]
unsafe impl NSMutableCopying for AVMutableMovie {}

#[cfg(feature = "AVAsset")]
unsafe impl MutableCopyingHelper for AVMutableMovie {
    type Result = Self;
}

#[cfg(feature = "AVAsset")]
unsafe impl NSObjectProtocol for AVMutableMovie {}

extern_methods!(
    #[cfg(feature = "AVAsset")]
    unsafe impl AVMutableMovie {
        #[method_id(@__retain_semantics Other movieWithURL:options:error:_)]
        pub unsafe fn movieWithURL_options_error(
            url: &NSURL,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[method_id(@__retain_semantics Init initWithURL:options:error:_)]
        pub unsafe fn initWithURL_options_error(
            this: Allocated<Self>,
            url: &NSURL,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[method_id(@__retain_semantics Other movieWithData:options:error:_)]
        pub unsafe fn movieWithData_options_error(
            data: &NSData,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[method_id(@__retain_semantics Init initWithData:options:error:_)]
        pub unsafe fn initWithData_options_error(
            this: Allocated<Self>,
            data: &NSData,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[method_id(@__retain_semantics Other movieWithSettingsFromMovie:options:error:_)]
        pub unsafe fn movieWithSettingsFromMovie_options_error(
            movie: Option<&AVMovie>,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[method_id(@__retain_semantics Init initWithSettingsFromMovie:options:error:_)]
        pub unsafe fn initWithSettingsFromMovie_options_error(
            this: Allocated<Self>,
            movie: Option<&AVMovie>,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[method(preferredRate)]
        pub unsafe fn preferredRate(&self) -> c_float;

        #[method(setPreferredRate:)]
        pub unsafe fn setPreferredRate(&self, preferred_rate: c_float);

        #[method(preferredVolume)]
        pub unsafe fn preferredVolume(&self) -> c_float;

        #[method(setPreferredVolume:)]
        pub unsafe fn setPreferredVolume(&self, preferred_volume: c_float);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(preferredTransform)]
        pub unsafe fn preferredTransform(&self) -> CGAffineTransform;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setPreferredTransform:)]
        pub unsafe fn setPreferredTransform(&self, preferred_transform: CGAffineTransform);

        #[cfg(feature = "objc2-core-media")]
        #[method(timescale)]
        pub unsafe fn timescale(&self) -> CMTimeScale;

        #[cfg(feature = "objc2-core-media")]
        #[method(setTimescale:)]
        pub unsafe fn setTimescale(&self, timescale: CMTimeScale);

        #[cfg(all(feature = "AVAssetTrack", feature = "AVMovieTrack"))]
        #[method_id(@__retain_semantics Other tracks)]
        pub unsafe fn tracks(&self) -> Retained<NSArray<AVMutableMovieTrack>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `AVMovie`
    #[cfg(feature = "AVAsset")]
    unsafe impl AVMutableMovie {
        #[method_id(@__retain_semantics Other movieWithURL:options:)]
        pub unsafe fn movieWithURL_options(
            url: &NSURL,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithURL:options:)]
        pub unsafe fn initWithURL_options(
            this: Allocated<Self>,
            url: &NSURL,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other movieWithData:options:)]
        pub unsafe fn movieWithData_options(
            data: &NSData,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithData:options:)]
        pub unsafe fn initWithData_options(
            this: Allocated<Self>,
            data: &NSData,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `AVAsset`
    #[cfg(feature = "AVAsset")]
    unsafe impl AVMutableMovie {
        #[method_id(@__retain_semantics Other assetWithURL:)]
        pub unsafe fn assetWithURL(url: &NSURL) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AVAsset")]
    unsafe impl AVMutableMovie {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// AVMutableMovieMovieLevelEditing
    #[cfg(feature = "AVAsset")]
    unsafe impl AVMutableMovie {
        #[method(isModified)]
        pub unsafe fn isModified(&self) -> bool;

        #[method(setModified:)]
        pub unsafe fn setModified(&self, modified: bool);

        #[method_id(@__retain_semantics Other defaultMediaDataStorage)]
        pub unsafe fn defaultMediaDataStorage(&self) -> Option<Retained<AVMediaDataStorage>>;

        #[method(setDefaultMediaDataStorage:)]
        pub unsafe fn setDefaultMediaDataStorage(
            &self,
            default_media_data_storage: Option<&AVMediaDataStorage>,
        );

        #[cfg(feature = "objc2-core-media")]
        #[method(interleavingPeriod)]
        pub unsafe fn interleavingPeriod(&self) -> CMTime;

        #[cfg(feature = "objc2-core-media")]
        #[method(setInterleavingPeriod:)]
        pub unsafe fn setInterleavingPeriod(&self, interleaving_period: CMTime);

        #[cfg(feature = "objc2-core-media")]
        #[method(insertTimeRange:ofAsset:atTime:copySampleData:error:_)]
        pub unsafe fn insertTimeRange_ofAsset_atTime_copySampleData_error(
            &self,
            time_range: CMTimeRange,
            asset: &AVAsset,
            start_time: CMTime,
            copy_sample_data: bool,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "objc2-core-media")]
        #[method(insertEmptyTimeRange:)]
        pub unsafe fn insertEmptyTimeRange(&self, time_range: CMTimeRange);

        #[cfg(feature = "objc2-core-media")]
        #[method(removeTimeRange:)]
        pub unsafe fn removeTimeRange(&self, time_range: CMTimeRange);

        #[cfg(feature = "objc2-core-media")]
        #[method(scaleTimeRange:toDuration:)]
        pub unsafe fn scaleTimeRange_toDuration(&self, time_range: CMTimeRange, duration: CMTime);
    }
);

extern_methods!(
    /// AVMutableMovieTrackLevelEditing
    #[cfg(feature = "AVAsset")]
    unsafe impl AVMutableMovie {
        #[cfg(all(feature = "AVAssetTrack", feature = "AVMovieTrack"))]
        #[method_id(@__retain_semantics Other mutableTrackCompatibleWithTrack:)]
        pub unsafe fn mutableTrackCompatibleWithTrack(
            &self,
            track: &AVAssetTrack,
        ) -> Option<Retained<AVMutableMovieTrack>>;

        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVMediaFormat",
            feature = "AVMovieTrack"
        ))]
        #[method_id(@__retain_semantics Other addMutableTrackWithMediaType:copySettingsFromTrack:options:)]
        pub unsafe fn addMutableTrackWithMediaType_copySettingsFromTrack_options(
            &self,
            media_type: &AVMediaType,
            track: Option<&AVAssetTrack>,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Option<Retained<AVMutableMovieTrack>>;

        #[cfg(all(feature = "AVAssetTrack", feature = "AVMovieTrack"))]
        #[method_id(@__retain_semantics Other addMutableTracksCopyingSettingsFromTracks:options:)]
        pub unsafe fn addMutableTracksCopyingSettingsFromTracks_options(
            &self,
            existing_tracks: &NSArray<AVAssetTrack>,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<NSArray<AVMutableMovieTrack>>;

        #[cfg(all(feature = "AVAssetTrack", feature = "AVMovieTrack"))]
        #[method(removeTrack:)]
        pub unsafe fn removeTrack(&self, track: &AVMovieTrack);
    }
);

extern_methods!(
    /// AVMutableMovieMetadataEditing
    #[cfg(feature = "AVAsset")]
    unsafe impl AVMutableMovie {
        #[cfg(feature = "AVMetadataItem")]
        #[method_id(@__retain_semantics Other metadata)]
        pub unsafe fn metadata(&self) -> Retained<NSArray<AVMetadataItem>>;

        #[cfg(feature = "AVMetadataItem")]
        #[method(setMetadata:)]
        pub unsafe fn setMetadata(&self, metadata: &NSArray<AVMetadataItem>);
    }
);

extern_methods!(
    /// AVMutableMovieTrackInspection
    #[cfg(feature = "AVAsset")]
    unsafe impl AVMutableMovie {
        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVMovieTrack",
            feature = "objc2-core-media"
        ))]
        #[method_id(@__retain_semantics Other trackWithTrackID:)]
        pub unsafe fn trackWithTrackID(
            &self,
            track_id: CMPersistentTrackID,
        ) -> Option<Retained<AVMutableMovieTrack>>;

        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVMovieTrack",
            feature = "block2",
            feature = "objc2-core-media"
        ))]
        #[method(loadTrackWithTrackID:completionHandler:)]
        pub unsafe fn loadTrackWithTrackID_completionHandler(
            &self,
            track_id: CMPersistentTrackID,
            completion_handler: &block2::Block<dyn Fn(*mut AVMutableMovieTrack, *mut NSError)>,
        );

        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVMediaFormat",
            feature = "AVMovieTrack"
        ))]
        #[method_id(@__retain_semantics Other tracksWithMediaType:)]
        pub unsafe fn tracksWithMediaType(
            &self,
            media_type: &AVMediaType,
        ) -> Retained<NSArray<AVMutableMovieTrack>>;

        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVMediaFormat",
            feature = "AVMovieTrack",
            feature = "block2"
        ))]
        #[method(loadTracksWithMediaType:completionHandler:)]
        pub unsafe fn loadTracksWithMediaType_completionHandler(
            &self,
            media_type: &AVMediaType,
            completion_handler: &block2::Block<
                dyn Fn(*mut NSArray<AVMutableMovieTrack>, *mut NSError),
            >,
        );

        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVMediaFormat",
            feature = "AVMovieTrack"
        ))]
        #[method_id(@__retain_semantics Other tracksWithMediaCharacteristic:)]
        pub unsafe fn tracksWithMediaCharacteristic(
            &self,
            media_characteristic: &AVMediaCharacteristic,
        ) -> Retained<NSArray<AVMutableMovieTrack>>;

        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVMediaFormat",
            feature = "AVMovieTrack",
            feature = "block2"
        ))]
        #[method(loadTracksWithMediaCharacteristic:completionHandler:)]
        pub unsafe fn loadTracksWithMediaCharacteristic_completionHandler(
            &self,
            media_characteristic: &AVMediaCharacteristic,
            completion_handler: &block2::Block<
                dyn Fn(*mut NSArray<AVMutableMovieTrack>, *mut NSError),
            >,
        );
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avmediadatastorage?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVMediaDataStorage;
);

unsafe impl Send for AVMediaDataStorage {}

unsafe impl Sync for AVMediaDataStorage {}

unsafe impl NSObjectProtocol for AVMediaDataStorage {}

extern_methods!(
    unsafe impl AVMediaDataStorage {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithURL:options:)]
        pub unsafe fn initWithURL_options(
            this: Allocated<Self>,
            url: &NSURL,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Retained<NSURL>>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avfragmentedmoviecontainsmoviefragmentsdidchangenotification?language=objc)
    pub static AVFragmentedMovieContainsMovieFragmentsDidChangeNotification: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avfragmentedmoviedurationdidchangenotification?language=objc)
    pub static AVFragmentedMovieDurationDidChangeNotification: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avfragmentedmoviewasdefragmentednotification?language=objc)
    pub static AVFragmentedMovieWasDefragmentedNotification: &'static NSString;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avfragmentedmovie?language=objc)
    #[unsafe(super(AVMovie, AVAsset, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AVAsset")]
    pub struct AVFragmentedMovie;
);

#[cfg(feature = "AVAsset")]
unsafe impl Send for AVFragmentedMovie {}

#[cfg(feature = "AVAsset")]
unsafe impl Sync for AVFragmentedMovie {}

#[cfg(all(feature = "AVAsset", feature = "AVAsynchronousKeyValueLoading"))]
unsafe impl AVAsynchronousKeyValueLoading for AVFragmentedMovie {}

#[cfg(feature = "AVAsset")]
unsafe impl AVFragmentMinding for AVFragmentedMovie {}

#[cfg(feature = "AVAsset")]
unsafe impl NSObjectProtocol for AVFragmentedMovie {}

extern_methods!(
    #[cfg(feature = "AVAsset")]
    unsafe impl AVFragmentedMovie {
        #[cfg(all(feature = "AVAssetTrack", feature = "AVMovieTrack"))]
        #[method_id(@__retain_semantics Other tracks)]
        pub unsafe fn tracks(&self) -> Retained<NSArray<AVFragmentedMovieTrack>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `AVMovie`
    #[cfg(feature = "AVAsset")]
    unsafe impl AVFragmentedMovie {
        #[method_id(@__retain_semantics Other movieWithURL:options:)]
        pub unsafe fn movieWithURL_options(
            url: &NSURL,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithURL:options:)]
        pub unsafe fn initWithURL_options(
            this: Allocated<Self>,
            url: &NSURL,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other movieWithData:options:)]
        pub unsafe fn movieWithData_options(
            data: &NSData,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithData:options:)]
        pub unsafe fn initWithData_options(
            this: Allocated<Self>,
            data: &NSData,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `AVAsset`
    #[cfg(feature = "AVAsset")]
    unsafe impl AVFragmentedMovie {
        #[method_id(@__retain_semantics Other assetWithURL:)]
        pub unsafe fn assetWithURL(url: &NSURL) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AVAsset")]
    unsafe impl AVFragmentedMovie {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// AVFragmentedMovieTrackInspection
    #[cfg(feature = "AVAsset")]
    unsafe impl AVFragmentedMovie {
        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVMovieTrack",
            feature = "objc2-core-media"
        ))]
        #[deprecated = "Use loadTrackWithTrackID:completionHandler: instead"]
        #[method_id(@__retain_semantics Other trackWithTrackID:)]
        pub unsafe fn trackWithTrackID(
            &self,
            track_id: CMPersistentTrackID,
        ) -> Option<Retained<AVFragmentedMovieTrack>>;

        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVMovieTrack",
            feature = "block2",
            feature = "objc2-core-media"
        ))]
        #[method(loadTrackWithTrackID:completionHandler:)]
        pub unsafe fn loadTrackWithTrackID_completionHandler(
            &self,
            track_id: CMPersistentTrackID,
            completion_handler: &block2::Block<dyn Fn(*mut AVFragmentedMovieTrack, *mut NSError)>,
        );

        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVMediaFormat",
            feature = "AVMovieTrack"
        ))]
        #[deprecated = "Use loadTracksWithMediaType:completionHandler: instead"]
        #[method_id(@__retain_semantics Other tracksWithMediaType:)]
        pub unsafe fn tracksWithMediaType(
            &self,
            media_type: &AVMediaType,
        ) -> Retained<NSArray<AVFragmentedMovieTrack>>;

        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVMediaFormat",
            feature = "AVMovieTrack",
            feature = "block2"
        ))]
        #[method(loadTracksWithMediaType:completionHandler:)]
        pub unsafe fn loadTracksWithMediaType_completionHandler(
            &self,
            media_type: &AVMediaType,
            completion_handler: &block2::Block<
                dyn Fn(*mut NSArray<AVFragmentedMovieTrack>, *mut NSError),
            >,
        );

        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVMediaFormat",
            feature = "AVMovieTrack"
        ))]
        #[deprecated = "loadTracksWithMediaCharacteristic:completionHandler:"]
        #[method_id(@__retain_semantics Other tracksWithMediaCharacteristic:)]
        pub unsafe fn tracksWithMediaCharacteristic(
            &self,
            media_characteristic: &AVMediaCharacteristic,
        ) -> Retained<NSArray<AVFragmentedMovieTrack>>;

        #[cfg(all(
            feature = "AVAssetTrack",
            feature = "AVMediaFormat",
            feature = "AVMovieTrack",
            feature = "block2"
        ))]
        #[method(loadTracksWithMediaCharacteristic:completionHandler:)]
        pub unsafe fn loadTracksWithMediaCharacteristic_completionHandler(
            &self,
            media_characteristic: &AVMediaCharacteristic,
            completion_handler: &block2::Block<
                dyn Fn(*mut NSArray<AVFragmentedMovieTrack>, *mut NSError),
            >,
        );
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avfragmentedmovieminder?language=objc)
    #[unsafe(super(AVFragmentedAssetMinder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AVAsset")]
    pub struct AVFragmentedMovieMinder;
);

#[cfg(feature = "AVAsset")]
unsafe impl NSObjectProtocol for AVFragmentedMovieMinder {}

extern_methods!(
    #[cfg(feature = "AVAsset")]
    unsafe impl AVFragmentedMovieMinder {
        #[method_id(@__retain_semantics Other fragmentedMovieMinderWithMovie:mindingInterval:)]
        pub unsafe fn fragmentedMovieMinderWithMovie_mindingInterval(
            movie: &AVFragmentedMovie,
            minding_interval: NSTimeInterval,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithMovie:mindingInterval:)]
        pub unsafe fn initWithMovie_mindingInterval(
            this: Allocated<Self>,
            movie: &AVFragmentedMovie,
            minding_interval: NSTimeInterval,
        ) -> Retained<Self>;

        #[method(mindingInterval)]
        pub unsafe fn mindingInterval(&self) -> NSTimeInterval;

        #[method(setMindingInterval:)]
        pub unsafe fn setMindingInterval(&self, minding_interval: NSTimeInterval);

        #[method_id(@__retain_semantics Other movies)]
        pub unsafe fn movies(&self) -> Retained<NSArray<AVFragmentedMovie>>;

        #[method(addFragmentedMovie:)]
        pub unsafe fn addFragmentedMovie(&self, movie: &AVFragmentedMovie);

        #[method(removeFragmentedMovie:)]
        pub unsafe fn removeFragmentedMovie(&self, movie: &AVFragmentedMovie);
    }
);

extern_methods!(
    /// Methods declared on superclass `AVFragmentedAssetMinder`
    #[cfg(feature = "AVAsset")]
    unsafe impl AVFragmentedMovieMinder {
        #[method_id(@__retain_semantics Other fragmentedAssetMinderWithAsset:mindingInterval:)]
        pub unsafe fn fragmentedAssetMinderWithAsset_mindingInterval(
            asset: &AVAsset,
            minding_interval: NSTimeInterval,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithAsset:mindingInterval:)]
        pub unsafe fn initWithAsset_mindingInterval(
            this: Allocated<Self>,
            asset: &AVAsset,
            minding_interval: NSTimeInterval,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AVAsset")]
    unsafe impl AVFragmentedMovieMinder {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// SynchronousAssetInterface
    #[cfg(feature = "AVAsset")]
    unsafe impl AVMutableMovie {
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
