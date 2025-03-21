//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/cifilterconstructor?language=objc)
    pub unsafe trait CIFilterConstructor {
        #[cfg(feature = "CIFilter")]
        #[unsafe(method(filterWithName:))]
        #[unsafe(method_family = none)]
        unsafe fn filterWithName(&self, name: &NSString) -> Option<Retained<CIFilter>>;
    }
);
