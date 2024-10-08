//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPNowPlayingInfoMediaType(pub NSUInteger);
impl MPNowPlayingInfoMediaType {
    #[doc(alias = "MPNowPlayingInfoMediaTypeNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "MPNowPlayingInfoMediaTypeAudio")]
    pub const Audio: Self = Self(1);
    #[doc(alias = "MPNowPlayingInfoMediaTypeVideo")]
    pub const Video: Self = Self(2);
}

unsafe impl Encode for MPNowPlayingInfoMediaType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MPNowPlayingInfoMediaType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPNowPlayingPlaybackState(pub NSUInteger);
impl MPNowPlayingPlaybackState {
    #[doc(alias = "MPNowPlayingPlaybackStateUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "MPNowPlayingPlaybackStatePlaying")]
    pub const Playing: Self = Self(1);
    #[doc(alias = "MPNowPlayingPlaybackStatePaused")]
    pub const Paused: Self = Self(2);
    #[doc(alias = "MPNowPlayingPlaybackStateStopped")]
    pub const Stopped: Self = Self(3);
    #[doc(alias = "MPNowPlayingPlaybackStateInterrupted")]
    pub const Interrupted: Self = Self(4);
}

unsafe impl Encode for MPNowPlayingPlaybackState {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MPNowPlayingPlaybackState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPNowPlayingInfoCenter;

    unsafe impl ClassType for MPNowPlayingInfoCenter {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for MPNowPlayingInfoCenter {}

extern_methods!(
    unsafe impl MPNowPlayingInfoCenter {
        #[method_id(@__retain_semantics Other defaultCenter)]
        pub unsafe fn defaultCenter() -> Retained<MPNowPlayingInfoCenter>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other nowPlayingInfo)]
        pub unsafe fn nowPlayingInfo(&self) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[method(setNowPlayingInfo:)]
        pub unsafe fn setNowPlayingInfo(
            &self,
            now_playing_info: Option<&NSDictionary<NSString, AnyObject>>,
        );

        #[method(playbackState)]
        pub unsafe fn playbackState(&self) -> MPNowPlayingPlaybackState;

        #[method(setPlaybackState:)]
        pub unsafe fn setPlaybackState(&self, playback_state: MPNowPlayingPlaybackState);
    }
);

extern "C" {
    pub static MPNowPlayingInfoPropertyElapsedPlaybackTime: &'static NSString;
}

extern "C" {
    pub static MPNowPlayingInfoPropertyPlaybackRate: &'static NSString;
}

extern "C" {
    pub static MPNowPlayingInfoPropertyDefaultPlaybackRate: &'static NSString;
}

extern "C" {
    pub static MPNowPlayingInfoPropertyPlaybackQueueIndex: &'static NSString;
}

extern "C" {
    pub static MPNowPlayingInfoPropertyPlaybackQueueCount: &'static NSString;
}

extern "C" {
    pub static MPNowPlayingInfoPropertyChapterNumber: &'static NSString;
}

extern "C" {
    pub static MPNowPlayingInfoPropertyChapterCount: &'static NSString;
}

extern "C" {
    pub static MPNowPlayingInfoPropertyIsLiveStream: &'static NSString;
}

extern "C" {
    pub static MPNowPlayingInfoPropertyAvailableLanguageOptions: &'static NSString;
}

extern "C" {
    pub static MPNowPlayingInfoPropertyCurrentLanguageOptions: &'static NSString;
}

extern "C" {
    pub static MPNowPlayingInfoCollectionIdentifier: &'static NSString;
}

extern "C" {
    pub static MPNowPlayingInfoPropertyExternalContentIdentifier: &'static NSString;
}

extern "C" {
    pub static MPNowPlayingInfoPropertyExternalUserProfileIdentifier: &'static NSString;
}

extern "C" {
    pub static MPNowPlayingInfoPropertyServiceIdentifier: &'static NSString;
}

extern "C" {
    pub static MPNowPlayingInfoPropertyPlaybackProgress: &'static NSString;
}

extern "C" {
    pub static MPNowPlayingInfoPropertyMediaType: &'static NSString;
}

extern "C" {
    pub static MPNowPlayingInfoPropertyAssetURL: &'static NSString;
}

extern "C" {
    pub static MPNowPlayingInfoPropertyCurrentPlaybackDate: &'static NSString;
}

extern "C" {
    pub static MPNowPlayingInfoPropertyAdTimeRanges: &'static NSString;
}

extern "C" {
    pub static MPNowPlayingInfoPropertyCreditsStartTime: &'static NSString;
}

extern "C" {
    pub static MPNowPlayingInfoPropertyInternationalStandardRecordingCode: &'static NSString;
}

extern "C" {
    pub static MPNowPlayingInfoPropertyExcludeFromSuggestions: &'static NSString;
}
