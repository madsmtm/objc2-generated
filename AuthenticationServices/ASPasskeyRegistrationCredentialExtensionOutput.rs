//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// This class encapsulates output for various WebAuthn extensions used during passkey registration.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/aspasskeyregistrationcredentialextensionoutput?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASPasskeyRegistrationCredentialExtensionOutput;
);

extern_conformance!(
    unsafe impl NSCoding for ASPasskeyRegistrationCredentialExtensionOutput {}
);

extern_conformance!(
    unsafe impl NSCopying for ASPasskeyRegistrationCredentialExtensionOutput {}
);

unsafe impl CopyingHelper for ASPasskeyRegistrationCredentialExtensionOutput {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for ASPasskeyRegistrationCredentialExtensionOutput {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for ASPasskeyRegistrationCredentialExtensionOutput {}
);

impl ASPasskeyRegistrationCredentialExtensionOutput {
    extern_methods!(
        #[cfg(feature = "ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput")]
        #[unsafe(method(initWithLargeBlobOutput:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithLargeBlobOutput(
            this: Allocated<Self>,
            large_blob: Option<&ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput>,
        ) -> Retained<Self>;

        #[cfg(feature = "ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput")]
        /// Output for `largeBlob` operation during passkey registration.
        #[unsafe(method(largeBlobRegistrationOutput))]
        #[unsafe(method_family = none)]
        pub unsafe fn largeBlobRegistrationOutput(
            &self,
        ) -> Option<Retained<ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl ASPasskeyRegistrationCredentialExtensionOutput {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
