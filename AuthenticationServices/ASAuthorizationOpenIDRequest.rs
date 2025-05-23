//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationopenidoperation?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type ASAuthorizationOpenIDOperation = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationoperationimplicit?language=objc)
    pub static ASAuthorizationOperationImplicit: &'static ASAuthorizationOpenIDOperation;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationoperationlogin?language=objc)
    pub static ASAuthorizationOperationLogin: &'static ASAuthorizationOpenIDOperation;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationoperationrefresh?language=objc)
    pub static ASAuthorizationOperationRefresh: &'static ASAuthorizationOpenIDOperation;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationoperationlogout?language=objc)
    pub static ASAuthorizationOperationLogout: &'static ASAuthorizationOpenIDOperation;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationopenidrequest?language=objc)
    #[unsafe(super(ASAuthorizationRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ASAuthorizationRequest")]
    pub struct ASAuthorizationOpenIDRequest;
);

#[cfg(feature = "ASAuthorizationRequest")]
extern_conformance!(
    unsafe impl NSCoding for ASAuthorizationOpenIDRequest {}
);

#[cfg(feature = "ASAuthorizationRequest")]
extern_conformance!(
    unsafe impl NSCopying for ASAuthorizationOpenIDRequest {}
);

#[cfg(feature = "ASAuthorizationRequest")]
unsafe impl CopyingHelper for ASAuthorizationOpenIDRequest {
    type Result = Self;
}

#[cfg(feature = "ASAuthorizationRequest")]
extern_conformance!(
    unsafe impl NSObjectProtocol for ASAuthorizationOpenIDRequest {}
);

#[cfg(feature = "ASAuthorizationRequest")]
extern_conformance!(
    unsafe impl NSSecureCoding for ASAuthorizationOpenIDRequest {}
);

#[cfg(feature = "ASAuthorizationRequest")]
impl ASAuthorizationOpenIDRequest {
    extern_methods!(
        #[cfg(feature = "ASAuthorization")]
        /// The contact information to be requested from the user.  Only scopes for which this app was authorized for will be returned.
        #[unsafe(method(requestedScopes))]
        #[unsafe(method_family = none)]
        pub unsafe fn requestedScopes(&self) -> Option<Retained<NSArray<ASAuthorizationScope>>>;

        #[cfg(feature = "ASAuthorization")]
        /// Setter for [`requestedScopes`][Self::requestedScopes].
        #[unsafe(method(setRequestedScopes:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRequestedScopes(
            &self,
            requested_scopes: Option<&NSArray<ASAuthorizationScope>>,
        );

        /// State to be passed to the identity provider.  This value will be returned as a part of successful ASAuthorization response.
        ///
        /// Note: The state size may depend on the actual technology used and an error might be returned by the request execution.
        #[unsafe(method(state))]
        #[unsafe(method_family = none)]
        pub unsafe fn state(&self) -> Option<Retained<NSString>>;

        /// Setter for [`state`][Self::state].
        #[unsafe(method(setState:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setState(&self, state: Option<&NSString>);

        /// Nonce to be passed to the identity provider.  This value can be verified with the identity token provided as a part of successful ASAuthorization response.
        ///
        /// Note: The nonce size may depend on the actual technology used and an error might be returned by the request execution.
        #[unsafe(method(nonce))]
        #[unsafe(method_family = none)]
        pub unsafe fn nonce(&self) -> Option<Retained<NSString>>;

        /// Setter for [`nonce`][Self::nonce].
        #[unsafe(method(setNonce:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setNonce(&self, nonce: Option<&NSString>);

        /// Operation to be executed by the request. The ASAuthorizationOperationImplicit operation interpretation depends on the credential provider implementation.
        #[unsafe(method(requestedOperation))]
        #[unsafe(method_family = none)]
        pub unsafe fn requestedOperation(&self) -> Retained<ASAuthorizationOpenIDOperation>;

        /// Setter for [`requestedOperation`][Self::requestedOperation].
        #[unsafe(method(setRequestedOperation:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRequestedOperation(
            &self,
            requested_operation: &ASAuthorizationOpenIDOperation,
        );
    );
}

/// Methods declared on superclass `ASAuthorizationRequest`.
#[cfg(feature = "ASAuthorizationRequest")]
impl ASAuthorizationOpenIDRequest {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}
