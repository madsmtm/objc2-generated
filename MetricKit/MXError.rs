//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// Error domain for NSError values stemming from the MetricKit Framework API.
    ///
    /// This error domain is used as the domain for all NSError instances stemming from the MetricKit Framework.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metrickit/mxerrordomain?language=objc)
    pub static MXErrorDomain: Option<&'static NSErrorDomain>;
}

/// Error codes for NSError values stemming from the MetricKit Framework.
///
/// These error codes are used as the codes for all NSError instances stemmming from the MetricKit Framework.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metrickit/mxerrorcode?language=objc)
// NS_ERROR_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MXErrorCode(pub NSInteger);
impl MXErrorCode {
    #[doc(alias = "MXErrorLaunchTaskInvalidID")]
    pub const LaunchTaskInvalidID: Self = Self(0);
    #[doc(alias = "MXErrorLaunchTaskMaxCount")]
    pub const LaunchTaskMaxCount: Self = Self(1);
    #[doc(alias = "MXErrorLaunchTaskPastDeadline")]
    pub const LaunchTaskPastDeadline: Self = Self(2);
    #[doc(alias = "MXErrorLaunchTaskDuplicated")]
    pub const LaunchTaskDuplicated: Self = Self(3);
    #[doc(alias = "MXErrorLaunchTaskUnknown")]
    pub const LaunchTaskUnknown: Self = Self(4);
    #[doc(alias = "MXErrorLaunchTaskInternalFailure")]
    pub const LaunchTaskInternalFailure: Self = Self(5);
}

unsafe impl Encode for MXErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MXErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
