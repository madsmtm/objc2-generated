//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_methods!(
    /// MLModelCompilation
    #[cfg(feature = "MLModel")]
    unsafe impl MLModel {
        #[deprecated = "Use the asynchronous interface compileModelAtURL:completionHandler:error: instead."]
        #[method_id(@__retain_semantics Other compileModelAtURL:error:_)]
        pub unsafe fn compileModelAtURL_error(
            model_url: &NSURL,
        ) -> Result<Retained<NSURL>, Retained<NSError>>;

        #[cfg(feature = "block2")]
        #[method(compileModelAtURL:completionHandler:)]
        pub unsafe fn compileModelAtURL_completionHandler(
            model_url: &NSURL,
            handler: &block2::Block<dyn Fn(*mut NSURL, *mut NSError)>,
        );
    }
);
