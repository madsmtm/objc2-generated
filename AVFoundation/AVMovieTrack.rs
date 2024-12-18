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
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avmovietrack?language=objc)
    #[unsafe(super(AVAssetTrack, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AVAssetTrack")]
    pub struct AVMovieTrack;
);

#[cfg(all(feature = "AVAssetTrack", feature = "AVAsynchronousKeyValueLoading"))]
unsafe impl AVAsynchronousKeyValueLoading for AVMovieTrack {}

#[cfg(feature = "AVAssetTrack")]
unsafe impl NSCopying for AVMovieTrack {}

#[cfg(feature = "AVAssetTrack")]
unsafe impl CopyingHelper for AVMovieTrack {
    type Result = Self;
}

#[cfg(feature = "AVAssetTrack")]
unsafe impl NSObjectProtocol for AVMovieTrack {}

extern_methods!(
    #[cfg(feature = "AVAssetTrack")]
    unsafe impl AVMovieTrack {
        #[cfg(feature = "objc2-core-media")]
        #[method(mediaPresentationTimeRange)]
        pub unsafe fn mediaPresentationTimeRange(&self) -> CMTimeRange;

        #[cfg(feature = "objc2-core-media")]
        #[method(mediaDecodeTimeRange)]
        pub unsafe fn mediaDecodeTimeRange(&self) -> CMTimeRange;

        #[method(alternateGroupID)]
        pub unsafe fn alternateGroupID(&self) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `AVAssetTrack`
    #[cfg(feature = "AVAssetTrack")]
    unsafe impl AVMovieTrack {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// AVMovieTrackMediaDataStorage
    #[cfg(feature = "AVAssetTrack")]
    unsafe impl AVMovieTrack {
        #[cfg(feature = "AVMovie")]
        #[method_id(@__retain_semantics Other mediaDataStorage)]
        pub unsafe fn mediaDataStorage(&self) -> Option<Retained<AVMediaDataStorage>>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avmutablemovietrack?language=objc)
    #[unsafe(super(AVMovieTrack, AVAssetTrack, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AVAssetTrack")]
    pub struct AVMutableMovieTrack;
);

#[cfg(all(feature = "AVAssetTrack", feature = "AVAsynchronousKeyValueLoading"))]
unsafe impl AVAsynchronousKeyValueLoading for AVMutableMovieTrack {}

#[cfg(feature = "AVAssetTrack")]
unsafe impl NSCopying for AVMutableMovieTrack {}

#[cfg(feature = "AVAssetTrack")]
unsafe impl CopyingHelper for AVMutableMovieTrack {
    type Result = Self;
}

#[cfg(feature = "AVAssetTrack")]
unsafe impl NSObjectProtocol for AVMutableMovieTrack {}

extern_methods!(
    #[cfg(feature = "AVAssetTrack")]
    unsafe impl AVMutableMovieTrack {
        #[cfg(feature = "AVMovie")]
        #[method_id(@__retain_semantics Other mediaDataStorage)]
        pub unsafe fn mediaDataStorage(&self) -> Option<Retained<AVMediaDataStorage>>;

        #[cfg(feature = "AVMovie")]
        #[method(setMediaDataStorage:)]
        pub unsafe fn setMediaDataStorage(&self, media_data_storage: Option<&AVMediaDataStorage>);

        #[method_id(@__retain_semantics Other sampleReferenceBaseURL)]
        pub unsafe fn sampleReferenceBaseURL(&self) -> Option<Retained<NSURL>>;

        #[method(setSampleReferenceBaseURL:)]
        pub unsafe fn setSampleReferenceBaseURL(&self, sample_reference_base_url: Option<&NSURL>);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(alternateGroupID)]
        pub unsafe fn alternateGroupID(&self) -> NSInteger;

        #[method(setAlternateGroupID:)]
        pub unsafe fn setAlternateGroupID(&self, alternate_group_id: NSInteger);

        #[method(isModified)]
        pub unsafe fn isModified(&self) -> bool;

        #[method(setModified:)]
        pub unsafe fn setModified(&self, modified: bool);

        #[method(hasProtectedContent)]
        pub unsafe fn hasProtectedContent(&self) -> bool;

        #[cfg(feature = "objc2-core-media")]
        #[method(timescale)]
        pub unsafe fn timescale(&self) -> CMTimeScale;

        #[cfg(feature = "objc2-core-media")]
        #[method(setTimescale:)]
        pub unsafe fn setTimescale(&self, timescale: CMTimeScale);
    }
);

extern_methods!(
    /// Methods declared on superclass `AVAssetTrack`
    #[cfg(feature = "AVAssetTrack")]
    unsafe impl AVMutableMovieTrack {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// AVMutableMovieTrackLanguageProperties
    #[cfg(feature = "AVAssetTrack")]
    unsafe impl AVMutableMovieTrack {
        #[method_id(@__retain_semantics Other languageCode)]
        pub unsafe fn languageCode(&self) -> Option<Retained<NSString>>;

        #[method(setLanguageCode:)]
        pub unsafe fn setLanguageCode(&self, language_code: Option<&NSString>);

        #[method_id(@__retain_semantics Other extendedLanguageTag)]
        pub unsafe fn extendedLanguageTag(&self) -> Option<Retained<NSString>>;

        #[method(setExtendedLanguageTag:)]
        pub unsafe fn setExtendedLanguageTag(&self, extended_language_tag: Option<&NSString>);
    }
);

extern_methods!(
    /// AVMutableMovieTrackVisualProperties
    #[cfg(feature = "AVAssetTrack")]
    unsafe impl AVMutableMovieTrack {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(naturalSize)]
        pub unsafe fn naturalSize(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setNaturalSize:)]
        pub unsafe fn setNaturalSize(&self, natural_size: CGSize);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(preferredTransform)]
        pub unsafe fn preferredTransform(&self) -> CGAffineTransform;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setPreferredTransform:)]
        pub unsafe fn setPreferredTransform(&self, preferred_transform: CGAffineTransform);

        #[method(layer)]
        pub unsafe fn layer(&self) -> NSInteger;

        #[method(setLayer:)]
        pub unsafe fn setLayer(&self, layer: NSInteger);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(cleanApertureDimensions)]
        pub unsafe fn cleanApertureDimensions(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setCleanApertureDimensions:)]
        pub unsafe fn setCleanApertureDimensions(&self, clean_aperture_dimensions: CGSize);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(productionApertureDimensions)]
        pub unsafe fn productionApertureDimensions(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setProductionApertureDimensions:)]
        pub unsafe fn setProductionApertureDimensions(
            &self,
            production_aperture_dimensions: CGSize,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[method(encodedPixelsDimensions)]
        pub unsafe fn encodedPixelsDimensions(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setEncodedPixelsDimensions:)]
        pub unsafe fn setEncodedPixelsDimensions(&self, encoded_pixels_dimensions: CGSize);
    }
);

extern_methods!(
    /// AVMutableMovieTrackAudibleProperties
    #[cfg(feature = "AVAssetTrack")]
    unsafe impl AVMutableMovieTrack {
        #[method(preferredVolume)]
        pub unsafe fn preferredVolume(&self) -> c_float;

        #[method(setPreferredVolume:)]
        pub unsafe fn setPreferredVolume(&self, preferred_volume: c_float);
    }
);

extern_methods!(
    /// AVMutableMovieTrackChunkProperties
    #[cfg(feature = "AVAssetTrack")]
    unsafe impl AVMutableMovieTrack {
        #[method(preferredMediaChunkSize)]
        pub unsafe fn preferredMediaChunkSize(&self) -> NSInteger;

        #[method(setPreferredMediaChunkSize:)]
        pub unsafe fn setPreferredMediaChunkSize(&self, preferred_media_chunk_size: NSInteger);

        #[cfg(feature = "objc2-core-media")]
        #[method(preferredMediaChunkDuration)]
        pub unsafe fn preferredMediaChunkDuration(&self) -> CMTime;

        #[cfg(feature = "objc2-core-media")]
        #[method(setPreferredMediaChunkDuration:)]
        pub unsafe fn setPreferredMediaChunkDuration(&self, preferred_media_chunk_duration: CMTime);

        #[method(preferredMediaChunkAlignment)]
        pub unsafe fn preferredMediaChunkAlignment(&self) -> NSInteger;

        #[method(setPreferredMediaChunkAlignment:)]
        pub unsafe fn setPreferredMediaChunkAlignment(
            &self,
            preferred_media_chunk_alignment: NSInteger,
        );
    }
);

extern_methods!(
    /// AVMutableMovieTrackTrackLevelEditing
    #[cfg(feature = "AVAssetTrack")]
    unsafe impl AVMutableMovieTrack {
        #[cfg(feature = "objc2-core-media")]
        #[method(insertTimeRange:ofTrack:atTime:copySampleData:error:_)]
        pub unsafe fn insertTimeRange_ofTrack_atTime_copySampleData_error(
            &self,
            time_range: CMTimeRange,
            track: &AVAssetTrack,
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
    /// AVMutableMovieTrackMetadataEditing
    #[cfg(feature = "AVAssetTrack")]
    unsafe impl AVMutableMovieTrack {
        #[cfg(feature = "AVMetadataItem")]
        #[method_id(@__retain_semantics Other metadata)]
        pub unsafe fn metadata(&self) -> Retained<NSArray<AVMetadataItem>>;

        #[cfg(feature = "AVMetadataItem")]
        #[method(setMetadata:)]
        pub unsafe fn setMetadata(&self, metadata: &NSArray<AVMetadataItem>);
    }
);

extern_methods!(
    /// AVMutableMovieTrackTrackAssociations
    #[cfg(feature = "AVAssetTrack")]
    unsafe impl AVMutableMovieTrack {
        #[method(addTrackAssociationToTrack:type:)]
        pub unsafe fn addTrackAssociationToTrack_type(
            &self,
            movie_track: &AVMovieTrack,
            track_association_type: &AVTrackAssociationType,
        );

        #[method(removeTrackAssociationToTrack:type:)]
        pub unsafe fn removeTrackAssociationToTrack_type(
            &self,
            movie_track: &AVMovieTrack,
            track_association_type: &AVTrackAssociationType,
        );
    }
);

extern_methods!(
    /// AVMutableMovieTrackFormatDescriptions
    #[cfg(feature = "AVAssetTrack")]
    unsafe impl AVMutableMovieTrack {
        #[cfg(feature = "objc2-core-media")]
        #[method(replaceFormatDescription:withFormatDescription:)]
        pub unsafe fn replaceFormatDescription_withFormatDescription(
            &self,
            format_description: CMFormatDescriptionRef,
            new_format_description: CMFormatDescriptionRef,
        );
    }
);

extern_methods!(
    /// AVMutableMovieTrackSampleLevelEditing
    #[cfg(feature = "AVAssetTrack")]
    unsafe impl AVMutableMovieTrack {
        #[cfg(feature = "objc2-core-media")]
        #[method(appendSampleBuffer:decodeTime:presentationTime:error:_)]
        pub unsafe fn appendSampleBuffer_decodeTime_presentationTime_error(
            &self,
            sample_buffer: CMSampleBufferRef,
            out_decode_time: *mut CMTime,
            out_presentation_time: *mut CMTime,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "objc2-core-media")]
        #[method(insertMediaTimeRange:intoTimeRange:)]
        pub unsafe fn insertMediaTimeRange_intoTimeRange(
            &self,
            media_time_range: CMTimeRange,
            track_time_range: CMTimeRange,
        ) -> bool;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avfragmentedmovietracktimerangedidchangenotification?language=objc)
    pub static AVFragmentedMovieTrackTimeRangeDidChangeNotification: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avfragmentedmovietracksegmentsdidchangenotification?language=objc)
    pub static AVFragmentedMovieTrackSegmentsDidChangeNotification: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avfragmentedmovietracktotalsampledatalengthdidchangenotification?language=objc)
    pub static AVFragmentedMovieTrackTotalSampleDataLengthDidChangeNotification: &'static NSString;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avfragmentedmovietrack?language=objc)
    #[unsafe(super(AVMovieTrack, AVAssetTrack, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AVAssetTrack")]
    pub struct AVFragmentedMovieTrack;
);

#[cfg(feature = "AVAssetTrack")]
unsafe impl Send for AVFragmentedMovieTrack {}

#[cfg(feature = "AVAssetTrack")]
unsafe impl Sync for AVFragmentedMovieTrack {}

#[cfg(all(feature = "AVAssetTrack", feature = "AVAsynchronousKeyValueLoading"))]
unsafe impl AVAsynchronousKeyValueLoading for AVFragmentedMovieTrack {}

#[cfg(feature = "AVAssetTrack")]
unsafe impl NSCopying for AVFragmentedMovieTrack {}

#[cfg(feature = "AVAssetTrack")]
unsafe impl CopyingHelper for AVFragmentedMovieTrack {
    type Result = Self;
}

#[cfg(feature = "AVAssetTrack")]
unsafe impl NSObjectProtocol for AVFragmentedMovieTrack {}

extern_methods!(
    #[cfg(feature = "AVAssetTrack")]
    unsafe impl AVFragmentedMovieTrack {}
);

extern_methods!(
    /// Methods declared on superclass `AVAssetTrack`
    #[cfg(feature = "AVAssetTrack")]
    unsafe impl AVFragmentedMovieTrack {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// SynchronousTrackInterface
    #[cfg(feature = "AVAssetTrack")]
    unsafe impl AVMutableMovieTrack {
        #[cfg(feature = "AVMediaFormat")]
        #[method(hasMediaCharacteristic:)]
        pub unsafe fn hasMediaCharacteristic(
            &self,
            media_characteristic: &AVMediaCharacteristic,
        ) -> bool;

        #[cfg(all(feature = "AVAssetTrackSegment", feature = "objc2-core-media"))]
        #[method_id(@__retain_semantics Other segmentForTrackTime:)]
        pub unsafe fn segmentForTrackTime(
            &self,
            track_time: CMTime,
        ) -> Option<Retained<AVAssetTrackSegment>>;

        #[cfg(feature = "objc2-core-media")]
        #[method(samplePresentationTimeForTrackTime:)]
        pub unsafe fn samplePresentationTimeForTrackTime(&self, track_time: CMTime) -> CMTime;

        #[cfg(all(feature = "AVMetadataFormat", feature = "AVMetadataItem"))]
        #[method_id(@__retain_semantics Other metadataForFormat:)]
        pub unsafe fn metadataForFormat(
            &self,
            format: &AVMetadataFormat,
        ) -> Retained<NSArray<AVMetadataItem>>;

        #[method_id(@__retain_semantics Other associatedTracksOfType:)]
        pub unsafe fn associatedTracksOfType(
            &self,
            track_association_type: &AVTrackAssociationType,
        ) -> Retained<NSArray<AVAssetTrack>>;
    }
);
