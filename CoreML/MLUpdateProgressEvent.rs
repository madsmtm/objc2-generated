//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MLUpdateProgressEvent(pub NSInteger);
bitflags::bitflags! {
    impl MLUpdateProgressEvent: NSInteger {
        #[doc(alias = "MLUpdateProgressEventTrainingBegin")]
        const TrainingBegin = 1<<0;
        #[doc(alias = "MLUpdateProgressEventEpochEnd")]
        const EpochEnd = 1<<1;
        #[doc(alias = "MLUpdateProgressEventMiniBatchEnd")]
        const MiniBatchEnd = 1<<2;
    }
}

unsafe impl Encode for MLUpdateProgressEvent {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MLUpdateProgressEvent {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
