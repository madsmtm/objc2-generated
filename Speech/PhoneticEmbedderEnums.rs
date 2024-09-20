//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PhoneticEncoderType(pub NSInteger);
impl PhoneticEncoderType {
    #[doc(alias = "PhoneticEncoderTypeGrapheme")]
    pub const Grapheme: Self = Self(0);
    #[doc(alias = "PhoneticEncoderTypePhoneme")]
    pub const Phoneme: Self = Self(1);
}

unsafe impl Encode for PhoneticEncoderType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for PhoneticEncoderType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PhoneticEmbedderInitFlag(pub NSInteger);
impl PhoneticEmbedderInitFlag {
    #[doc(alias = "PhoneticEmbedderInitFlagAll")]
    pub const All: Self = Self(0);
    #[doc(alias = "PhoneticEmbedderInitFlagEmbedder")]
    pub const Embedder: Self = Self(1);
}

unsafe impl Encode for PhoneticEmbedderInitFlag {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for PhoneticEmbedderInitFlag {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}