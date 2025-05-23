//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-audio-types")]
use objc2_core_audio_types::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A description of the roles of a set of audio channels.
    ///
    /// This object is a thin wrapper for the AudioChannelLayout structure, described
    /// in
    /// <CoreAudio
    /// /CoreAudioTypes.h>.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiochannellayout?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAudioChannelLayout;
);

unsafe impl Send for AVAudioChannelLayout {}

unsafe impl Sync for AVAudioChannelLayout {}

extern_conformance!(
    unsafe impl NSCoding for AVAudioChannelLayout {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for AVAudioChannelLayout {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for AVAudioChannelLayout {}
);

impl AVAudioChannelLayout {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "objc2-core-audio-types")]
        /// Initialize from a layout tag.
        ///
        /// Parameter `layoutTag`: The tag.
        ///
        /// Returns nil if the tag is either kAudioChannelLayoutTag_UseChannelDescriptions or
        /// kAudioChannelLayoutTag_UseChannelBitmap.
        #[unsafe(method(initWithLayoutTag:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithLayoutTag(
            this: Allocated<Self>,
            layout_tag: AudioChannelLayoutTag,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "objc2-core-audio-types")]
        /// Initialize from an AudioChannelLayout.
        ///
        /// Parameter `layout`: The AudioChannelLayout.
        ///
        /// If the provided layout's tag is kAudioChannelLayoutTag_UseChannelDescriptions, this
        /// initializer attempts to convert it to a more specific tag.
        #[unsafe(method(initWithLayout:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithLayout(
            this: Allocated<Self>,
            layout: NonNull<AudioChannelLayout>,
        ) -> Retained<Self>;

        /// Determine whether another AVAudioChannelLayout is exactly equal to this layout.
        ///
        /// Parameter `object`: The AVAudioChannelLayout to compare against.
        ///
        /// The underlying AudioChannelLayoutTag and AudioChannelLayout are compared for equality.
        #[unsafe(method(isEqual:))]
        #[unsafe(method_family = none)]
        pub unsafe fn isEqual(&self, object: &AnyObject) -> bool;

        #[cfg(feature = "objc2-core-audio-types")]
        /// Create from a layout tag.
        #[unsafe(method(layoutWithLayoutTag:))]
        #[unsafe(method_family = none)]
        pub unsafe fn layoutWithLayoutTag(layout_tag: AudioChannelLayoutTag) -> Retained<Self>;

        #[cfg(feature = "objc2-core-audio-types")]
        /// Create from an AudioChannelLayout
        #[unsafe(method(layoutWithLayout:))]
        #[unsafe(method_family = none)]
        pub unsafe fn layoutWithLayout(layout: NonNull<AudioChannelLayout>) -> Retained<Self>;

        #[cfg(feature = "objc2-core-audio-types")]
        /// The layout's tag.
        #[unsafe(method(layoutTag))]
        #[unsafe(method_family = none)]
        pub unsafe fn layoutTag(&self) -> AudioChannelLayoutTag;

        #[cfg(feature = "objc2-core-audio-types")]
        /// The underlying AudioChannelLayout.
        #[unsafe(method(layout))]
        #[unsafe(method_family = none)]
        pub unsafe fn layout(&self) -> NonNull<AudioChannelLayout>;

        #[cfg(feature = "AVAudioTypes")]
        /// The number of channels of audio data.
        #[unsafe(method(channelCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn channelCount(&self) -> AVAudioChannelCount;
    );
}

/// Methods declared on superclass `NSObject`.
impl AVAudioChannelLayout {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
