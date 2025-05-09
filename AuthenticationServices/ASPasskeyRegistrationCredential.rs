//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/aspasskeyregistrationcredential?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASPasskeyRegistrationCredential;
);

#[cfg(feature = "ASAuthorizationCredential")]
extern_conformance!(
    unsafe impl ASAuthorizationCredential for ASPasskeyRegistrationCredential {}
);

extern_conformance!(
    unsafe impl NSCoding for ASPasskeyRegistrationCredential {}
);

extern_conformance!(
    unsafe impl NSCopying for ASPasskeyRegistrationCredential {}
);

unsafe impl CopyingHelper for ASPasskeyRegistrationCredential {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for ASPasskeyRegistrationCredential {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for ASPasskeyRegistrationCredential {}
);

impl ASPasskeyRegistrationCredential {
    extern_methods!(
        /// Initializes an ASPasskeyRegistrationCredential object.
        ///
        /// Parameter `relyingParty`: The relying party identifier associated with this passkey.
        ///
        /// Parameter `clientDataHash`: The JSON encoded client data for this registration result.
        ///
        /// Parameter `credentialID`: The unique identifier for this passkey.
        ///
        /// Parameter `attestationObject`: The attestation object for this passkey registration result.
        #[unsafe(method(initWithRelyingParty:clientDataHash:credentialID:attestationObject:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithRelyingParty_clientDataHash_credentialID_attestationObject(
            this: Allocated<Self>,
            relying_party: &NSString,
            client_data_hash: &NSData,
            credential_id: &NSData,
            attestation_object: &NSData,
        ) -> Retained<Self>;

        #[cfg(feature = "ASPasskeyRegistrationCredentialExtensionOutput")]
        /// Initializes an ASPasskeyRegistrationCredential object.
        ///
        /// Parameter `relyingParty`: The relying party identifier associated with this passkey.
        ///
        /// Parameter `clientDataHash`: The JSON encoded client data for this registration result.
        ///
        /// Parameter `credentialID`: The unique identifier for this passkey.
        ///
        /// Parameter `attestationObject`: The attestation object for this passkey registration result.
        ///
        /// Parameter `extensionOutput`: The output of WebAuthn extensions processed by the credential provider.
        #[unsafe(method(initWithRelyingParty:clientDataHash:credentialID:attestationObject:extensionOutput:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithRelyingParty_clientDataHash_credentialID_attestationObject_extensionOutput(
            this: Allocated<Self>,
            relying_party: &NSString,
            client_data_hash: &NSData,
            credential_id: &NSData,
            attestation_object: &NSData,
            extension_output: Option<&ASPasskeyRegistrationCredentialExtensionOutput>,
        ) -> Retained<Self>;

        /// Creates and initializes an ASPasskeyRegistrationCredential object.
        ///
        /// Parameter `relyingParty`: The relying party identifier associated with this passkey.
        ///
        /// Parameter `clientDataHash`: The JSON encoded client data for this registration result.
        ///
        /// Parameter `credentialID`: The unique identifier for this passkey.
        ///
        /// Parameter `attestationObject`: The attestation object for this passkey registration result.
        #[unsafe(method(credentialWithRelyingParty:clientDataHash:credentialID:attestationObject:))]
        #[unsafe(method_family = none)]
        pub unsafe fn credentialWithRelyingParty_clientDataHash_credentialID_attestationObject(
            relying_party: &NSString,
            client_data_hash: &NSData,
            credential_id: &NSData,
            attestation_object: &NSData,
        ) -> Retained<Self>;

        /// The relying party identifier associated with this passkey.
        #[unsafe(method(relyingParty))]
        #[unsafe(method_family = none)]
        pub unsafe fn relyingParty(&self) -> Retained<NSString>;

        /// The hash of the client data for this registration result.
        #[unsafe(method(clientDataHash))]
        #[unsafe(method_family = none)]
        pub unsafe fn clientDataHash(&self) -> Retained<NSData>;

        /// The raw credential identifier of this passkey.
        #[unsafe(method(credentialID))]
        #[unsafe(method_family = none)]
        pub unsafe fn credentialID(&self) -> Retained<NSData>;

        /// The attestation object for this passkey registration result.
        #[unsafe(method(attestationObject))]
        #[unsafe(method_family = none)]
        pub unsafe fn attestationObject(&self) -> Retained<NSData>;

        #[cfg(feature = "ASPasskeyRegistrationCredentialExtensionOutput")]
        /// The outputs for WebAuthn extensions processed by the credential provider.
        #[unsafe(method(extensionOutput))]
        #[unsafe(method_family = none)]
        pub unsafe fn extensionOutput(
            &self,
        ) -> Option<Retained<ASPasskeyRegistrationCredentialExtensionOutput>>;

        #[cfg(feature = "ASPasskeyRegistrationCredentialExtensionOutput")]
        /// Setter for [`extensionOutput`][Self::extensionOutput].
        #[unsafe(method(setExtensionOutput:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setExtensionOutput(
            &self,
            extension_output: Option<&ASPasskeyRegistrationCredentialExtensionOutput>,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl ASPasskeyRegistrationCredential {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
