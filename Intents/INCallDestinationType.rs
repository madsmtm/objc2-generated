//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/intents/incalldestinationtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct INCallDestinationType(pub NSInteger);
impl INCallDestinationType {
    #[doc(alias = "INCallDestinationTypeUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "INCallDestinationTypeNormal")]
    pub const Normal: Self = Self(1);
    #[doc(alias = "INCallDestinationTypeEmergency")]
    pub const Emergency: Self = Self(2);
    #[doc(alias = "INCallDestinationTypeVoicemail")]
    pub const Voicemail: Self = Self(3);
    #[doc(alias = "INCallDestinationTypeRedial")]
    pub const Redial: Self = Self(4);
    #[doc(alias = "INCallDestinationTypeCallBack")]
    pub const CallBack: Self = Self(5);
    #[deprecated = "Use INCallDestinationTypeNormal instead"]
    #[doc(alias = "INCallDestinationTypeNormalDestination")]
    pub const NormalDestination: Self = Self(1);
    #[deprecated = "Use INCallDestinationTypeEmergency instead"]
    #[doc(alias = "INCallDestinationTypeEmergencyDestination")]
    pub const EmergencyDestination: Self = Self(2);
    #[deprecated = "Use INCallDestinationTypeVoicemail instead"]
    #[doc(alias = "INCallDestinationTypeVoicemailDestination")]
    pub const VoicemailDestination: Self = Self(3);
    #[deprecated = "Use INCallDestinationTypeRedial instead"]
    #[doc(alias = "INCallDestinationTypeRedialDestination")]
    pub const RedialDestination: Self = Self(4);
}

unsafe impl Encode for INCallDestinationType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for INCallDestinationType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
