//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;

use crate::*;

/// A value describing the camera’s tracking state.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/arkit/artrackingstate?language=objc)
// NS_ENUM
#[cfg(feature = "objc2")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ARTrackingState(pub NSInteger);
#[cfg(feature = "objc2")]
impl ARTrackingState {
    /// Tracking is not available.
    #[doc(alias = "ARTrackingStateNotAvailable")]
    pub const NotAvailable: Self = Self(0);
    /// Tracking is limited. See tracking reason for details.
    #[doc(alias = "ARTrackingStateLimited")]
    pub const Limited: Self = Self(1);
    /// Tracking is Normal.
    #[doc(alias = "ARTrackingStateNormal")]
    pub const Normal: Self = Self(2);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for ARTrackingState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for ARTrackingState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// A reason describing why the camera’s tracking state is limited.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/arkit/artrackingstatereason?language=objc)
// NS_ENUM
#[cfg(feature = "objc2")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ARTrackingStateReason(pub NSInteger);
#[cfg(feature = "objc2")]
impl ARTrackingStateReason {
    /// Tracking is not limited.
    #[doc(alias = "ARTrackingStateReasonNone")]
    pub const None: Self = Self(0);
    /// Tracking is limited due to initialization in progress.
    #[doc(alias = "ARTrackingStateReasonInitializing")]
    pub const Initializing: Self = Self(1);
    /// Tracking is limited due to a excessive motion of the camera.
    #[doc(alias = "ARTrackingStateReasonExcessiveMotion")]
    pub const ExcessiveMotion: Self = Self(2);
    /// Tracking is limited due to a lack of features visible to the camera.
    #[doc(alias = "ARTrackingStateReasonInsufficientFeatures")]
    pub const InsufficientFeatures: Self = Self(3);
    /// Tracking is limited due to a relocalization in progress.
    #[doc(alias = "ARTrackingStateReasonRelocalizing")]
    pub const Relocalizing: Self = Self(4);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for ARTrackingStateReason {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for ARTrackingStateReason {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}