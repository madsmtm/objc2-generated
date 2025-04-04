//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationerrordomain?language=objc)
    pub static ASAuthorizationErrorDomain: &'static NSErrorDomain;
}

/// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationerror?language=objc)
// NS_ERROR_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ASAuthorizationError(pub NSInteger);
impl ASAuthorizationError {
    #[doc(alias = "ASAuthorizationErrorUnknown")]
    pub const Unknown: Self = Self(1000);
    #[doc(alias = "ASAuthorizationErrorCanceled")]
    pub const Canceled: Self = Self(1001);
    #[doc(alias = "ASAuthorizationErrorInvalidResponse")]
    pub const InvalidResponse: Self = Self(1002);
    #[doc(alias = "ASAuthorizationErrorNotHandled")]
    pub const NotHandled: Self = Self(1003);
    #[doc(alias = "ASAuthorizationErrorFailed")]
    pub const Failed: Self = Self(1004);
    #[doc(alias = "ASAuthorizationErrorNotInteractive")]
    pub const NotInteractive: Self = Self(1005);
    /// This error should only be returned when specifying
    /// `excludedCredentials`on a public key credential registration request.
    #[doc(alias = "ASAuthorizationErrorMatchedExcludedCredential")]
    pub const MatchedExcludedCredential: Self = Self(1006);
    /// This error signals that the import request failed. Details will be available in the `userInfo` of the NSError.
    #[doc(alias = "ASAuthorizationErrorCredentialImport")]
    pub const CredentialImport: Self = Self(1007);
    /// This error signals that the export request failed. Details will be available in the `userInfo` of the NSError.
    #[doc(alias = "ASAuthorizationErrorCredentialExport")]
    pub const CredentialExport: Self = Self(1008);
}

unsafe impl Encode for ASAuthorizationError {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for ASAuthorizationError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
