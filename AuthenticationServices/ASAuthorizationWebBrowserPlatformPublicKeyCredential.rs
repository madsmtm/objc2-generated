//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationwebbrowserplatformpublickeycredential?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationWebBrowserPlatformPublicKeyCredential;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for ASAuthorizationWebBrowserPlatformPublicKeyCredential {}
);

impl ASAuthorizationWebBrowserPlatformPublicKeyCredential {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// The user name of the saved credential.
        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        /// A user-specified title for the credential.
        #[unsafe(method(customTitle))]
        #[unsafe(method_family = none)]
        pub unsafe fn customTitle(&self) -> Option<Retained<NSString>>;

        /// The "relying party" (generally website) the credential was saved for.
        #[unsafe(method(relyingParty))]
        #[unsafe(method_family = none)]
        pub unsafe fn relyingParty(&self) -> Retained<NSString>;

        /// A unique identifier for this credential.
        #[unsafe(method(credentialID))]
        #[unsafe(method_family = none)]
        pub unsafe fn credentialID(&self) -> Retained<NSData>;

        /// A unique identifier for the user account associated with this credential. One account may have multiple associated credentials.
        #[unsafe(method(userHandle))]
        #[unsafe(method_family = none)]
        pub unsafe fn userHandle(&self) -> Retained<NSData>;

        /// The localized name of the credential provider that provided this passkey, either the name of a third party app or "iCloud Keychain".
        #[unsafe(method(providerName))]
        #[unsafe(method_family = none)]
        pub unsafe fn providerName(&self) -> Retained<NSString>;
    );
}
