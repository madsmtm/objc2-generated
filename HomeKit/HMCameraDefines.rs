//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

/// This enumeration describes the different states of a camera stream.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/homekit/hmcamerastreamstate?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HMCameraStreamState(pub NSUInteger);
impl HMCameraStreamState {
    /// Start stream request is in progress.
    #[doc(alias = "HMCameraStreamStateStarting")]
    pub const Starting: Self = Self(1);
    /// Streaming is in progress.
    #[doc(alias = "HMCameraStreamStateStreaming")]
    pub const Streaming: Self = Self(2);
    /// Stop stream request is in progress.
    #[doc(alias = "HMCameraStreamStateStopping")]
    pub const Stopping: Self = Self(3);
    /// No streaming is in progress.
    #[doc(alias = "HMCameraStreamStateNotStreaming")]
    pub const NotStreaming: Self = Self(4);
}

unsafe impl Encode for HMCameraStreamState {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for HMCameraStreamState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// This enumeration describes the setting for audio on the recipient of the camera stream.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/homekit/hmcameraaudiostreamsetting?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HMCameraAudioStreamSetting(pub NSUInteger);
impl HMCameraAudioStreamSetting {
    /// Muted for incoming and outgoing audio.
    #[doc(alias = "HMCameraAudioStreamSettingMuted")]
    pub const Muted: Self = Self(1);
    /// Only incoming audio is allowed.
    #[doc(alias = "HMCameraAudioStreamSettingIncomingAudioAllowed")]
    pub const IncomingAudioAllowed: Self = Self(2);
    /// Bidirectional audio is allowed.
    #[doc(alias = "HMCameraAudioStreamSettingBidirectionalAudioAllowed")]
    pub const BidirectionalAudioAllowed: Self = Self(3);
}

unsafe impl Encode for HMCameraAudioStreamSetting {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for HMCameraAudioStreamSetting {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
