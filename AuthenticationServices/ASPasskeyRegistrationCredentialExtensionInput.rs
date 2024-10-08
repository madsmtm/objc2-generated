//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASPasskeyRegistrationCredentialExtensionInput;

    unsafe impl ClassType for ASPasskeyRegistrationCredentialExtensionInput {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for ASPasskeyRegistrationCredentialExtensionInput {}

unsafe impl NSCopying for ASPasskeyRegistrationCredentialExtensionInput {}

unsafe impl CopyingHelper for ASPasskeyRegistrationCredentialExtensionInput {
    type Result = Self;
}

unsafe impl NSObjectProtocol for ASPasskeyRegistrationCredentialExtensionInput {}

unsafe impl NSSecureCoding for ASPasskeyRegistrationCredentialExtensionInput {}

extern_methods!(
    unsafe impl ASPasskeyRegistrationCredentialExtensionInput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "ASAuthorizationPublicKeyCredentialLargeBlobRegistrationInput")]
        #[method_id(@__retain_semantics Other largeBlob)]
        pub unsafe fn largeBlob(
            &self,
        ) -> Option<Retained<ASAuthorizationPublicKeyCredentialLargeBlobRegistrationInput>>;
    }
);
