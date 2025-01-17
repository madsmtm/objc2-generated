//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use objc2::__framework_prelude::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/aswebauthenticationsessionwebbrowsersessionhandling?language=objc)
    pub unsafe trait ASWebAuthenticationSessionWebBrowserSessionHandling {
        #[cfg(feature = "ASWebAuthenticationSessionRequest")]
        #[method(beginHandlingWebAuthenticationSessionRequest:)]
        unsafe fn beginHandlingWebAuthenticationSessionRequest(
            &self,
            request: Option<&ASWebAuthenticationSessionRequest>,
        );

        #[cfg(feature = "ASWebAuthenticationSessionRequest")]
        #[method(cancelWebAuthenticationSessionRequest:)]
        unsafe fn cancelWebAuthenticationSessionRequest(
            &self,
            request: Option<&ASWebAuthenticationSessionRequest>,
        );
    }
);
