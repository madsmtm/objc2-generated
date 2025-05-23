//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpmediaplayback?language=objc)
    pub unsafe trait MPMediaPlayback {
        #[unsafe(method(prepareToPlay))]
        #[unsafe(method_family = none)]
        unsafe fn prepareToPlay(&self);

        #[unsafe(method(isPreparedToPlay))]
        #[unsafe(method_family = none)]
        unsafe fn isPreparedToPlay(&self) -> bool;

        #[unsafe(method(play))]
        #[unsafe(method_family = none)]
        unsafe fn play(&self);

        #[unsafe(method(pause))]
        #[unsafe(method_family = none)]
        unsafe fn pause(&self);

        #[unsafe(method(stop))]
        #[unsafe(method_family = none)]
        unsafe fn stop(&self);

        #[unsafe(method(currentPlaybackTime))]
        #[unsafe(method_family = none)]
        unsafe fn currentPlaybackTime(&self) -> NSTimeInterval;

        /// Setter for [`currentPlaybackTime`][Self::currentPlaybackTime].
        #[unsafe(method(setCurrentPlaybackTime:))]
        #[unsafe(method_family = none)]
        unsafe fn setCurrentPlaybackTime(&self, current_playback_time: NSTimeInterval);

        #[unsafe(method(currentPlaybackRate))]
        #[unsafe(method_family = none)]
        unsafe fn currentPlaybackRate(&self) -> c_float;

        /// Setter for [`currentPlaybackRate`][Self::currentPlaybackRate].
        #[unsafe(method(setCurrentPlaybackRate:))]
        #[unsafe(method_family = none)]
        unsafe fn setCurrentPlaybackRate(&self, current_playback_rate: c_float);

        #[unsafe(method(beginSeekingForward))]
        #[unsafe(method_family = none)]
        unsafe fn beginSeekingForward(&self);

        #[unsafe(method(beginSeekingBackward))]
        #[unsafe(method_family = none)]
        unsafe fn beginSeekingBackward(&self);

        #[unsafe(method(endSeeking))]
        #[unsafe(method_family = none)]
        unsafe fn endSeeking(&self);
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpmediaplaybackispreparedtoplaydidchangenotification?language=objc)
    pub static MPMediaPlaybackIsPreparedToPlayDidChangeNotification: Option<&'static NSString>;
}
