//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// WatchKit corollary to AVFoundation AVAsset abstract class
    ///
    ///
    /// This class provides the functionality of AVAsset for Watch OS apps. Only file-based assets are allowed.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkaudiofileasset?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use AVFoundation's AVPlayer and AVQueuePlayer instead"]
    pub struct WKAudioFileAsset;
);

unsafe impl NSObjectProtocol for WKAudioFileAsset {}

extern_methods!(
    unsafe impl WKAudioFileAsset {
        #[deprecated = "Use AVFoundation's AVPlayer and AVQueuePlayer instead"]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Returns an instance of WKAudioFileAsset for inspection of a media resource.
        ///
        /// Parameter `URL`: An instance of NSURL that references a file-based media resource.
        ///
        /// Returns: An instance of WKAudioFileAsset.
        ///
        /// Returns a newly allocated instance of a subclass of WKAudioFileAsset initialized with the specified URL.
        /// Title, ablumTitle, and artist properties are initialized to the values found in the common metadata of the media resource
        #[deprecated = "Use AVFoundation's AVPlayer and AVQueuePlayer instead"]
        #[method_id(@__retain_semantics Other assetWithURL:)]
        pub unsafe fn assetWithURL(url: &NSURL) -> Retained<Self>;

        /// Returns an instance of WKAudioFileAsset for inspection of a media resource.
        ///
        /// Parameter `URL`: An instance of NSURL that references a file-based media resource.
        ///
        /// Parameter `title`: Title string to use for the Now Playing controls. If nil, value in common metadata of the media resource will be used. If no value is found in common metadata, the file name will be used.
        ///
        /// Parameter `albumTitle`: Album Title string to use for the Now Playing controls. If nil, value in common metadata of the media resource will be used.
        ///
        /// Parameter `artist`: Artist string to use for the Now Playing controls. If nil, value in common metadata of the media resource will be used.
        ///
        /// Returns: An instance of WKAudioFileAsset.
        ///
        /// Returns a newly allocated instance of a subclass of WKAudioFileAsset initialized with the specified URL.
        #[deprecated = "Use AVFoundation's AVPlayer and AVQueuePlayer instead"]
        #[method_id(@__retain_semantics Other assetWithURL:title:albumTitle:artist:)]
        pub unsafe fn assetWithURL_title_albumTitle_artist(
            url: &NSURL,
            title: Option<&NSString>,
            album_title: Option<&NSString>,
            artist: Option<&NSString>,
        ) -> Retained<Self>;

        #[deprecated = "Use AVFoundation's AVPlayer and AVQueuePlayer instead"]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Retained<NSURL>;

        #[deprecated = "Use AVFoundation's AVPlayer and AVQueuePlayer instead"]
        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;

        #[deprecated = "Use AVFoundation's AVPlayer and AVQueuePlayer instead"]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Retained<NSString>>;

        #[deprecated = "Use AVFoundation's AVPlayer and AVQueuePlayer instead"]
        #[method_id(@__retain_semantics Other albumTitle)]
        pub unsafe fn albumTitle(&self) -> Option<Retained<NSString>>;

        #[deprecated = "Use AVFoundation's AVPlayer and AVQueuePlayer instead"]
        #[method_id(@__retain_semantics Other artist)]
        pub unsafe fn artist(&self) -> Option<Retained<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKAudioFileAsset {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);