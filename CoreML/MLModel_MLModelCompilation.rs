//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_methods!(
    /// MLModelCompilation
    /// MLModel (MLModelCompilation)
    ///
    /// Class with utilties for performing .mlmodel compilation
    #[cfg(feature = "MLModel")]
    unsafe impl MLModel {
        /// Compile a .mlmodel for this device
        ///
        ///
        /// Parameter `modelURL`: URL file path to .mlmodel file you wish to compile
        ///
        /// Parameter `error`: Any errors are surfaced here
        ///
        ///
        /// Returns: a URL to the compiled .mlmodelc directory if successful
        /// The model is compiled to a temporary location on disk
        /// You must move the compiled model to a permanent location if you wish to keep it
        ///
        ///
        /// The returned model can be loaded using:
        ///
        /// ```text
        ///  [MLModel modelWithContentsOfURL:error:]
        /// ```
        #[deprecated = "Use the asynchronous interface compileModelAtURL:completionHandler:error: instead."]
        #[unsafe(method_family(none))]
        #[method_id(compileModelAtURL:error:_)]
        pub unsafe fn compileModelAtURL_error(
            model_url: &NSURL,
        ) -> Result<Retained<NSURL>, Retained<NSError>>;

        #[cfg(feature = "block2")]
        /// Compile a .mlmodel or .mlpackage for this device. Perform the compilation asynchronously.
        ///
        ///
        /// Parameter `modelURL`: URL file path to .mlmodel file you wish to compile
        ///
        /// Parameter `handler`: When the model compilation completes successfully  the completion handler is invoked with a valid URL to the compiled .mlmodelc directory.
        /// On failure, signified by nil  compiledModelURL, the NSError object is populated.
        ///
        ///
        /// The model is compiled to a temporary location in the file system. You must move the compiled model to a permanent location if you wish to keep it. Then the model can be loaded using the returned URL:
        ///
        /// ```text
        ///  [MLModel modelWithContentsOfURL:error:]
        /// ```
        #[method(compileModelAtURL:completionHandler:)]
        pub unsafe fn compileModelAtURL_completionHandler(
            model_url: &NSURL,
            handler: &block2::Block<dyn Fn(*mut NSURL, *mut NSError)>,
        );
    }
);
