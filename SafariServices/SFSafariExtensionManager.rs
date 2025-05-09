//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/safariservices/sfsafariextensionmanager?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SFSafariExtensionManager;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for SFSafariExtensionManager {}
);

impl SFSafariExtensionManager {
    extern_methods!(
        #[cfg(all(feature = "SFSafariExtensionState", feature = "block2"))]
        #[unsafe(method(getStateOfSafariExtensionWithIdentifier:completionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn getStateOfSafariExtensionWithIdentifier_completionHandler(
            identifier: &NSString,
            completion_handler: &block2::DynBlock<
                dyn Fn(*mut SFSafariExtensionState, *mut NSError),
            >,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl SFSafariExtensionManager {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
