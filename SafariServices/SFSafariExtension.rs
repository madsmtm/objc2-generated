//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/safariservices/sfsafariextension?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SFSafariExtension;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for SFSafariExtension {}
);

impl SFSafariExtension {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        /// Calls the completion handler with the base URI of the extension.
        #[unsafe(method(getBaseURIWithCompletionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn getBaseURIWithCompletionHandler(
            completion_handler: &block2::DynBlock<dyn Fn(*mut NSURL)>,
        );
    );
}
