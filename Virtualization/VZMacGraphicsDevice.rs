//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZGraphicsDevice")]
    pub struct VZMacGraphicsDevice;

    #[cfg(feature = "VZGraphicsDevice")]
    unsafe impl ClassType for VZMacGraphicsDevice {
        #[inherits(NSObject)]
        type Super = VZGraphicsDevice;
    }
);

#[cfg(feature = "VZGraphicsDevice")]
unsafe impl NSObjectProtocol for VZMacGraphicsDevice {}

extern_methods!(
    #[cfg(feature = "VZGraphicsDevice")]
    unsafe impl VZMacGraphicsDevice {}
);

extern_methods!(
    /// Methods declared on superclass `VZGraphicsDevice`
    #[cfg(feature = "VZGraphicsDevice")]
    unsafe impl VZMacGraphicsDevice {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
