//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_protocol!(
    pub unsafe trait UISpringLoadedInteractionSupporting:
        NSObjectProtocol + MainThreadOnly
    {
        #[method(isSpringLoaded)]
        unsafe fn isSpringLoaded(&self) -> bool;

        #[method(setSpringLoaded:)]
        unsafe fn setSpringLoaded(&self, spring_loaded: bool);
    }

    unsafe impl ProtocolType for dyn UISpringLoadedInteractionSupporting {}
);
