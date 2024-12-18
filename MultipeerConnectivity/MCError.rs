//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/multipeerconnectivity/mcerrordomain?language=objc)
    pub static MCErrorDomain: Option<&'static NSString>;
}

/// [Apple's documentation](https://developer.apple.com/documentation/multipeerconnectivity/mcerrorcode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MCErrorCode(pub NSInteger);
impl MCErrorCode {
    pub const MCErrorUnknown: Self = Self(0);
    pub const MCErrorNotConnected: Self = Self(1);
    pub const MCErrorInvalidParameter: Self = Self(2);
    pub const MCErrorUnsupported: Self = Self(3);
    pub const MCErrorTimedOut: Self = Self(4);
    pub const MCErrorCancelled: Self = Self(5);
    pub const MCErrorUnavailable: Self = Self(6);
}

unsafe impl Encode for MCErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MCErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
