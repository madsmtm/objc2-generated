//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/usernotifications/unerrordomain?language=objc)
    pub static UNErrorDomain: Option<&'static NSString>;
}

/// [Apple's documentation](https://developer.apple.com/documentation/usernotifications/unerrorcode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UNErrorCode(pub NSInteger);
impl UNErrorCode {
    #[doc(alias = "UNErrorCodeNotificationsNotAllowed")]
    pub const NotificationsNotAllowed: Self = Self(1);
    #[doc(alias = "UNErrorCodeAttachmentInvalidURL")]
    pub const AttachmentInvalidURL: Self = Self(100);
    #[doc(alias = "UNErrorCodeAttachmentUnrecognizedType")]
    pub const AttachmentUnrecognizedType: Self = Self(101);
    #[doc(alias = "UNErrorCodeAttachmentInvalidFileSize")]
    pub const AttachmentInvalidFileSize: Self = Self(102);
    #[doc(alias = "UNErrorCodeAttachmentNotInDataStore")]
    pub const AttachmentNotInDataStore: Self = Self(103);
    #[doc(alias = "UNErrorCodeAttachmentMoveIntoDataStoreFailed")]
    pub const AttachmentMoveIntoDataStoreFailed: Self = Self(104);
    #[doc(alias = "UNErrorCodeAttachmentCorrupt")]
    pub const AttachmentCorrupt: Self = Self(105);
    #[doc(alias = "UNErrorCodeNotificationInvalidNoDate")]
    pub const NotificationInvalidNoDate: Self = Self(1400);
    #[doc(alias = "UNErrorCodeNotificationInvalidNoContent")]
    pub const NotificationInvalidNoContent: Self = Self(1401);
    #[doc(alias = "UNErrorCodeContentProvidingObjectNotAllowed")]
    pub const ContentProvidingObjectNotAllowed: Self = Self(1500);
    #[doc(alias = "UNErrorCodeContentProvidingInvalid")]
    pub const ContentProvidingInvalid: Self = Self(1501);
    #[doc(alias = "UNErrorCodeBadgeInputInvalid")]
    pub const BadgeInputInvalid: Self = Self(1600);
}

unsafe impl Encode for UNErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UNErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
