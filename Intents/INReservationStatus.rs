//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/intents/inreservationstatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct INReservationStatus(pub NSInteger);
impl INReservationStatus {
    #[doc(alias = "INReservationStatusUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "INReservationStatusCanceled")]
    pub const Canceled: Self = Self(1);
    #[doc(alias = "INReservationStatusPending")]
    pub const Pending: Self = Self(2);
    #[doc(alias = "INReservationStatusHold")]
    pub const Hold: Self = Self(3);
    #[doc(alias = "INReservationStatusConfirmed")]
    pub const Confirmed: Self = Self(4);
}

unsafe impl Encode for INReservationStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for INReservationStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
