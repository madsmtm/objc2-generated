//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput;

    unsafe impl ClassType for ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput {}

unsafe impl NSCopying for ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput {}

unsafe impl CopyingHelper for ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput {
    type Result = Self;
}

unsafe impl NSObjectProtocol for ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput {}

unsafe impl NSSecureCoding for ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput {}

extern_methods!(
    unsafe impl ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput {
        #[method(isSupported)]
        pub unsafe fn isSupported(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASAuthorizationPublicKeyCredentialLargeBlobRegistrationOutput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
