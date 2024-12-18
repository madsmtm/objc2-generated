//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_category!(
    /// Category "UTAdditions" on [`NSString`].
    #[doc(alias = "UTAdditions")]
    pub unsafe trait NSStringUTAdditions {
        #[cfg(feature = "UTType")]
        #[method_id(@__retain_semantics Other stringByAppendingPathComponent:conformingToType:)]
        unsafe fn stringByAppendingPathComponent_conformingToType(
            &self,
            partial_name: &NSString,
            content_type: &UTType,
        ) -> Retained<NSString>;

        #[cfg(feature = "UTType")]
        #[method_id(@__retain_semantics Other stringByAppendingPathExtensionForType:)]
        unsafe fn stringByAppendingPathExtensionForType(
            &self,
            content_type: &UTType,
        ) -> Retained<NSString>;
    }

    unsafe impl NSStringUTAdditions for NSString {}
);

extern_category!(
    /// Category "UTAdditions" on [`NSURL`].
    #[doc(alias = "UTAdditions")]
    pub unsafe trait NSURLUTAdditions {
        #[cfg(feature = "UTType")]
        #[method_id(@__retain_semantics Other URLByAppendingPathComponent:conformingToType:)]
        unsafe fn URLByAppendingPathComponent_conformingToType(
            &self,
            partial_name: &NSString,
            content_type: &UTType,
        ) -> Retained<NSURL>;

        #[cfg(feature = "UTType")]
        #[method_id(@__retain_semantics Other URLByAppendingPathExtensionForType:)]
        unsafe fn URLByAppendingPathExtensionForType(
            &self,
            content_type: &UTType,
        ) -> Retained<NSURL>;
    }

    unsafe impl NSURLUTAdditions for NSURL {}
);
