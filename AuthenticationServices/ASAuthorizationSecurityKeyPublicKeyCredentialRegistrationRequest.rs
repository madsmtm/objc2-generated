//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationsecuritykeypublickeycredentialregistrationrequest?language=objc)
    #[unsafe(super(ASAuthorizationRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ASAuthorizationRequest")]
    pub struct ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest;
);

#[cfg(all(
    feature = "ASAuthorizationPublicKeyCredentialRegistrationRequest",
    feature = "ASAuthorizationRequest"
))]
extern_conformance!(
    unsafe impl ASAuthorizationPublicKeyCredentialRegistrationRequest
        for ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest
    {
    }
);

#[cfg(feature = "ASAuthorizationRequest")]
extern_conformance!(
    unsafe impl NSCoding for ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest {}
);

#[cfg(feature = "ASAuthorizationRequest")]
extern_conformance!(
    unsafe impl NSCopying for ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest {}
);

#[cfg(feature = "ASAuthorizationRequest")]
unsafe impl CopyingHelper for ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest {
    type Result = Self;
}

#[cfg(feature = "ASAuthorizationRequest")]
extern_conformance!(
    unsafe impl NSObjectProtocol for ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest {}
);

#[cfg(feature = "ASAuthorizationRequest")]
extern_conformance!(
    unsafe impl NSSecureCoding for ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest {}
);

#[cfg(feature = "ASAuthorizationRequest")]
impl ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest {
    extern_methods!(
        #[cfg(feature = "ASAuthorizationPublicKeyCredentialParameters")]
        /// A list of parameters for the new credential which are supported by the Relying Party. The authenticator should choose from these parameters when creating the credential.
        #[unsafe(method(credentialParameters))]
        #[unsafe(method_family = none)]
        pub unsafe fn credentialParameters(
            &self,
        ) -> Retained<NSArray<ASAuthorizationPublicKeyCredentialParameters>>;

        #[cfg(feature = "ASAuthorizationPublicKeyCredentialParameters")]
        /// Setter for [`credentialParameters`][Self::credentialParameters].
        #[unsafe(method(setCredentialParameters:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCredentialParameters(
            &self,
            credential_parameters: &NSArray<ASAuthorizationPublicKeyCredentialParameters>,
        );

        #[cfg(feature = "ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor")]
        /// A list of descriptors indicating credentials which must not already exist on the authenticator. If a credential already exists on the authenticator which matches one or more of these descriptors, a new credential will not be created and authentication will fail.
        #[unsafe(method(excludedCredentials))]
        #[unsafe(method_family = none)]
        pub unsafe fn excludedCredentials(
            &self,
        ) -> Retained<NSArray<ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor>>;

        #[cfg(feature = "ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor")]
        /// Setter for [`excludedCredentials`][Self::excludedCredentials].
        #[unsafe(method(setExcludedCredentials:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setExcludedCredentials(
            &self,
            excluded_credentials: &NSArray<ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor>,
        );

        #[cfg(feature = "ASAuthorizationPublicKeyCredentialConstants")]
        /// A preference whether the authenticator should store the private key of the newly created credential.
        #[unsafe(method(residentKeyPreference))]
        #[unsafe(method_family = none)]
        pub unsafe fn residentKeyPreference(
            &self,
        ) -> Retained<ASAuthorizationPublicKeyCredentialResidentKeyPreference>;

        #[cfg(feature = "ASAuthorizationPublicKeyCredentialConstants")]
        /// Setter for [`residentKeyPreference`][Self::residentKeyPreference].
        #[unsafe(method(setResidentKeyPreference:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setResidentKeyPreference(
            &self,
            resident_key_preference: &ASAuthorizationPublicKeyCredentialResidentKeyPreference,
        );

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

#[cfg(feature = "ASAuthorizationRequest")]
impl ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest {
    extern_methods!();
}

#[cfg(all(
    feature = "ASAuthorizationRequest",
    feature = "ASAuthorizationWebBrowserSecurityKeyPublicKeyCredentialRegistrationRequest"
))]
extern_conformance!(
    unsafe impl ASAuthorizationWebBrowserSecurityKeyPublicKeyCredentialRegistrationRequest
        for ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest
    {
    }
);
