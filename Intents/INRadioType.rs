//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/intents/inradiotype?language=objc)
// NS_ENUM
#[deprecated = "INRadioType is deprecated. There is no replacement."]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct INRadioType(pub NSInteger);
impl INRadioType {
    #[deprecated = "INRadioType is deprecated. There is no replacement."]
    #[doc(alias = "INRadioTypeUnknown")]
    pub const Unknown: Self = Self(0);
    #[deprecated = "INRadioType is deprecated. There is no replacement."]
    #[doc(alias = "INRadioTypeAM")]
    pub const AM: Self = Self(1);
    #[deprecated = "INRadioType is deprecated. There is no replacement."]
    #[doc(alias = "INRadioTypeFM")]
    pub const FM: Self = Self(2);
    #[deprecated = "INRadioType is deprecated. There is no replacement."]
    #[doc(alias = "INRadioTypeHD")]
    pub const HD: Self = Self(3);
    #[deprecated = "INRadioType is deprecated. There is no replacement."]
    #[doc(alias = "INRadioTypeSatellite")]
    pub const Satellite: Self = Self(4);
    #[deprecated = "INRadioType is deprecated. There is no replacement."]
    #[doc(alias = "INRadioTypeDAB")]
    pub const DAB: Self = Self(5);
}

unsafe impl Encode for INRadioType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for INRadioType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
