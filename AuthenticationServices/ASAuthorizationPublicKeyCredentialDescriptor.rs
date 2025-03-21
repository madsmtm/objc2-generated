//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationpublickeycredentialdescriptor?language=objc)
    pub unsafe trait ASAuthorizationPublicKeyCredentialDescriptor:
        NSObjectProtocol + NSSecureCoding + NSCopying
    {
        /// An identifier that uniquely identifies a specific credential.
        #[unsafe(method(credentialID))]
        #[unsafe(method_family = none)]
        unsafe fn credentialID(&self) -> Retained<NSData>;

        /// Setter for [`credentialID`][Self::credentialID].
        #[unsafe(method(setCredentialID:))]
        #[unsafe(method_family = none)]
        unsafe fn setCredentialID(&self, credential_id: &NSData);
    }
);
