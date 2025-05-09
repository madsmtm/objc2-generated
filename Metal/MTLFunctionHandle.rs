//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlfunctionhandle?language=objc)
    pub unsafe trait MTLFunctionHandle: NSObjectProtocol {
        #[cfg(feature = "MTLLibrary")]
        #[unsafe(method(functionType))]
        #[unsafe(method_family = none)]
        fn functionType(&self) -> MTLFunctionType;

        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        fn name(&self) -> Retained<NSString>;

        #[cfg(feature = "MTLDevice")]
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;
    }
);
