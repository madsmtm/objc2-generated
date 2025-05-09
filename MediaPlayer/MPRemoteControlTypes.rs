//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpshuffletype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPShuffleType(pub NSInteger);
impl MPShuffleType {
    #[doc(alias = "MPShuffleTypeOff")]
    pub const Off: Self = Self(0);
    /// Nothing is shuffled during playback.
    #[doc(alias = "MPShuffleTypeItems")]
    pub const Items: Self = Self(1);
    /// Individual items are shuffled during playback.
    #[doc(alias = "MPShuffleTypeCollections")]
    pub const Collections: Self = Self(2);
}

unsafe impl Encode for MPShuffleType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MPShuffleType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mprepeattype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPRepeatType(pub NSInteger);
impl MPRepeatType {
    #[doc(alias = "MPRepeatTypeOff")]
    pub const Off: Self = Self(0);
    /// Nothing is repeated during playback.
    #[doc(alias = "MPRepeatTypeOne")]
    pub const One: Self = Self(1);
    /// Repeat a single item indefinitely.
    #[doc(alias = "MPRepeatTypeAll")]
    pub const All: Self = Self(2);
}

unsafe impl Encode for MPRepeatType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MPRepeatType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpchangelanguageoptionsetting?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPChangeLanguageOptionSetting(pub NSInteger);
impl MPChangeLanguageOptionSetting {
    #[doc(alias = "MPChangeLanguageOptionSettingNone")]
    pub const None: Self = Self(0);
    /// No Language Option Change
    #[doc(alias = "MPChangeLanguageOptionSettingNowPlayingItemOnly")]
    pub const NowPlayingItemOnly: Self = Self(1);
    /// The Language Option change applies only the the now playing item
    #[doc(alias = "MPChangeLanguageOptionSettingPermanent")]
    pub const Permanent: Self = Self(2);
}

unsafe impl Encode for MPChangeLanguageOptionSetting {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MPChangeLanguageOptionSetting {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
