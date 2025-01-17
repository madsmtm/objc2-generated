//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nslocalizednumberformatrule?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSLocalizedNumberFormatRule;
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSLocalizedNumberFormatRule {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSLocalizedNumberFormatRule {}

#[cfg(feature = "NSObject")]
unsafe impl CopyingHelper for NSLocalizedNumberFormatRule {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSLocalizedNumberFormatRule {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSLocalizedNumberFormatRule {}

extern_methods!(
    unsafe impl NSLocalizedNumberFormatRule {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(none))]
        #[method_id(automatic)]
        pub unsafe fn automatic() -> Retained<NSLocalizedNumberFormatRule>;
    }
);
