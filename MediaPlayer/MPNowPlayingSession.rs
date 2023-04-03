//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPAdTimeRange")]
    #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
    pub struct MPAdTimeRange;

    #[cfg(feature = "MediaPlayer_MPAdTimeRange")]
    unsafe impl ClassType for MPAdTimeRange {
        type Super = NSObject;
    }
);

#[cfg(feature = "MediaPlayer_MPAdTimeRange")]
unsafe impl NSObjectProtocol for MPAdTimeRange {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPAdTimeRange")]
    unsafe impl MPAdTimeRange {
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPNowPlayingSession")]
    #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
    pub struct MPNowPlayingSession;

    #[cfg(feature = "MediaPlayer_MPNowPlayingSession")]
    unsafe impl ClassType for MPNowPlayingSession {
        type Super = NSObject;
    }
);

#[cfg(feature = "MediaPlayer_MPNowPlayingSession")]
unsafe impl NSObjectProtocol for MPNowPlayingSession {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPNowPlayingSession")]
    unsafe impl MPNowPlayingSession {
        #[cfg(all(feature = "AVFoundation_AVPlayer", feature = "Foundation_NSArray"))]
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Init initWithPlayers:)]
        pub unsafe fn initWithPlayers(
            this: Option<Allocated<Self>>,
            players: &NSArray<AVPlayer>,
        ) -> Id<Self>;

        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(all(feature = "AVFoundation_AVPlayer", feature = "Foundation_NSArray"))]
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other players)]
        pub unsafe fn players(&self) -> Id<NSArray<AVPlayer>>;

        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn MPNowPlayingSessionDelegate>>>;

        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn MPNowPlayingSessionDelegate>>,
        );

        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[method(automaticallyPublishesNowPlayingInfo)]
        pub unsafe fn automaticallyPublishesNowPlayingInfo(&self) -> bool;

        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[method(setAutomaticallyPublishesNowPlayingInfo:)]
        pub unsafe fn setAutomaticallyPublishesNowPlayingInfo(
            &self,
            automatically_publishes_now_playing_info: bool,
        );

        #[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other nowPlayingInfoCenter)]
        pub unsafe fn nowPlayingInfoCenter(&self) -> Id<MPNowPlayingInfoCenter>;

        #[cfg(feature = "MediaPlayer_MPRemoteCommandCenter")]
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other remoteCommandCenter)]
        pub unsafe fn remoteCommandCenter(&self) -> Id<MPRemoteCommandCenter>;

        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[method(canBecomeActive)]
        pub unsafe fn canBecomeActive(&self) -> bool;

        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[method(isActive)]
        pub unsafe fn isActive(&self) -> bool;

        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[method(becomeActiveIfPossibleWithCompletion:)]
        pub unsafe fn becomeActiveIfPossibleWithCompletion(
            &self,
            completion: Option<&Block<(Bool,), ()>>,
        );

        #[cfg(feature = "AVFoundation_AVPlayer")]
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[method(addPlayer:)]
        pub unsafe fn addPlayer(&self, player: &AVPlayer);

        #[cfg(feature = "AVFoundation_AVPlayer")]
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[method(removePlayer:)]
        pub unsafe fn removePlayer(&self, player: &AVPlayer);
    }
);

extern_protocol!(
    #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
    pub unsafe trait MPNowPlayingSessionDelegate: NSObjectProtocol {
        #[cfg(feature = "MediaPlayer_MPNowPlayingSession")]
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[optional]
        #[method(nowPlayingSessionDidChangeActive:)]
        unsafe fn nowPlayingSessionDidChangeActive(
            &self,
            now_playing_session: &MPNowPlayingSession,
        );

        #[cfg(feature = "MediaPlayer_MPNowPlayingSession")]
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[optional]
        #[method(nowPlayingSessionDidChangeCanBecomeActive:)]
        unsafe fn nowPlayingSessionDidChangeCanBecomeActive(
            &self,
            now_playing_session: &MPNowPlayingSession,
        );
    }

    unsafe impl ProtocolType for dyn MPNowPlayingSessionDelegate {}
);