//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcapturescope?language=objc)
    pub unsafe trait MTLCaptureScope: NSObjectProtocol {
        #[method(beginScope)]
        fn beginScope(&self);

        #[method(endScope)]
        fn endScope(&self);

        /// Scope label
        ///
        /// Created capture scopes are listed in Xcode when long-pressing the capture button, performing the capture over the selected scope
        #[unsafe(method_family(none))]
        #[method_id(label)]
        fn label(&self) -> Option<Retained<NSString>>;

        /// Setter for [`label`][Self::label].
        #[method(setLabel:)]
        unsafe fn setLabel(&self, label: Option<&NSString>);

        #[cfg(feature = "MTLDevice")]
        #[unsafe(method_family(none))]
        #[method_id(device)]
        unsafe fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        #[cfg(feature = "MTLCommandQueue")]
        /// If set, this scope will only capture Metal commands from the associated command queue. Defaults to nil (all command queues from the associated device are captured).
        #[unsafe(method_family(none))]
        #[method_id(commandQueue)]
        unsafe fn commandQueue(&self) -> Option<Retained<ProtocolObject<dyn MTLCommandQueue>>>;
    }
);
