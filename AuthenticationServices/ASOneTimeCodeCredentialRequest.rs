//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asonetimecodecredentialrequest?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASOneTimeCodeCredentialRequest;
);

#[cfg(feature = "ASCredentialRequest")]
unsafe impl ASCredentialRequest for ASOneTimeCodeCredentialRequest {}

unsafe impl NSCoding for ASOneTimeCodeCredentialRequest {}

unsafe impl NSCopying for ASOneTimeCodeCredentialRequest {}

unsafe impl CopyingHelper for ASOneTimeCodeCredentialRequest {
    type Result = Self;
}

unsafe impl NSObjectProtocol for ASOneTimeCodeCredentialRequest {}

unsafe impl NSSecureCoding for ASOneTimeCodeCredentialRequest {}

extern_methods!(
    unsafe impl ASOneTimeCodeCredentialRequest {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "ASOneTimeCodeCredentialIdentity")]
        /// Initializes an instance of ASOneTimeCodeCredentialRequest.
        ///
        /// Parameter `credentialIdentity`: the credential identity to use for this request.
        #[unsafe(method_family(init))]
        #[method_id(initWithCredentialIdentity:)]
        pub unsafe fn initWithCredentialIdentity(
            this: Allocated<Self>,
            credential_identity: &ASOneTimeCodeCredentialIdentity,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASOneTimeCodeCredentialRequest {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
