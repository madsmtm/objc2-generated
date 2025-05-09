//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationplatformpublickeycredentialprovider?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationPlatformPublicKeyCredentialProvider;
);

#[cfg(feature = "ASAuthorizationProvider")]
extern_conformance!(
    unsafe impl ASAuthorizationProvider for ASAuthorizationPlatformPublicKeyCredentialProvider {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for ASAuthorizationPlatformPublicKeyCredentialProvider {}
);

impl ASAuthorizationPlatformPublicKeyCredentialProvider {
    extern_methods!(
        #[unsafe(method(initWithRelyingPartyIdentifier:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithRelyingPartyIdentifier(
            this: Allocated<Self>,
            relying_party_identifier: &NSString,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest",
            feature = "ASAuthorizationRequest"
        ))]
        /// Create a request to register a new platform credential.
        ///
        /// Parameter `challenge`: The challenge to sign.
        ///
        /// Parameter `name`: The user name for the new credential.
        ///
        /// Parameter `userID`: An identifier to be stored alongside the credential, which will be returned with the credential when it is used to authenticate.
        #[unsafe(method(createCredentialRegistrationRequestWithChallenge:name:userID:))]
        #[unsafe(method_family = none)]
        pub unsafe fn createCredentialRegistrationRequestWithChallenge_name_userID(
            &self,
            challenge: &NSData,
            name: &NSString,
            user_id: &NSData,
        ) -> Retained<ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest>;

        #[cfg(all(
            feature = "ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest",
            feature = "ASAuthorizationRequest"
        ))]
        /// Create a request to register a new platform credential.
        ///
        /// Parameter `challenge`: The challenge to sign.
        ///
        /// Parameter `name`: The user name for the new credential.
        ///
        /// Parameter `userID`: An identifier to be stored alongside the credential, which will be returned with the credential when it is used to authenticate.
        ///
        /// Parameter `requestStyle`: The style for this request.
        #[unsafe(method(createCredentialRegistrationRequestWithChallenge:name:userID:requestStyle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn createCredentialRegistrationRequestWithChallenge_name_userID_requestStyle(
            &self,
            challenge: &NSData,
            name: &NSString,
            user_id: &NSData,
            request_style: ASAuthorizationPlatformPublicKeyCredentialRegistrationRequestStyle,
        ) -> Retained<ASAuthorizationPlatformPublicKeyCredentialRegistrationRequest>;

        #[cfg(all(
            feature = "ASAuthorizationPlatformPublicKeyCredentialAssertionRequest",
            feature = "ASAuthorizationRequest"
        ))]
        /// Create a request to authenticate using an existing credential.
        ///
        /// Parameter `challenge`: The challenge to sign.
        #[unsafe(method(createCredentialAssertionRequestWithChallenge:))]
        #[unsafe(method_family = none)]
        pub unsafe fn createCredentialAssertionRequestWithChallenge(
            &self,
            challenge: &NSData,
        ) -> Retained<ASAuthorizationPlatformPublicKeyCredentialAssertionRequest>;

        /// The Relying Party identifier used for all requests created by this object.
        #[unsafe(method(relyingPartyIdentifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn relyingPartyIdentifier(&self) -> Retained<NSString>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

impl ASAuthorizationPlatformPublicKeyCredentialProvider {
    extern_methods!();
}

#[cfg(feature = "ASAuthorizationWebBrowserPlatformPublicKeyCredentialProvider")]
extern_conformance!(
    unsafe impl ASAuthorizationWebBrowserPlatformPublicKeyCredentialProvider
        for ASAuthorizationPlatformPublicKeyCredentialProvider
    {
    }
);
