//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_EXTENSIBLE_ENUM
pub type ASAuthorizationProviderAuthorizationOperation = NSString;

extern "C" {
    pub static ASAuthorizationProviderAuthorizationOperationConfigurationRemoved:
        &'static ASAuthorizationProviderAuthorizationOperation;
}

extern "C" {
    pub static ASAuthorizationProviderAuthorizationOperationDirectRequest:
        &'static ASAuthorizationProviderAuthorizationOperation;
}

extern_protocol!(
    pub unsafe trait ASAuthorizationProviderExtensionAuthorizationRequestHandler:
        NSObjectProtocol + MainThreadOnly
    {
        #[method(beginAuthorizationWithRequest:)]
        unsafe fn beginAuthorizationWithRequest(
            &self,
            request: &ASAuthorizationProviderExtensionAuthorizationRequest,
        );

        #[optional]
        #[method(cancelAuthorizationWithRequest:)]
        unsafe fn cancelAuthorizationWithRequest(
            &self,
            request: &ASAuthorizationProviderExtensionAuthorizationRequest,
        );
    }

    unsafe impl ProtocolType for dyn ASAuthorizationProviderExtensionAuthorizationRequestHandler {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationProviderExtensionAuthorizationRequest;

    unsafe impl ClassType for ASAuthorizationProviderExtensionAuthorizationRequest {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for ASAuthorizationProviderExtensionAuthorizationRequest {}

extern_methods!(
    unsafe impl ASAuthorizationProviderExtensionAuthorizationRequest {
        #[method(doNotHandle)]
        pub unsafe fn doNotHandle(&self);

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method(complete)]
        pub unsafe fn complete(&self);

        #[method(completeWithHTTPAuthorizationHeaders:)]
        pub unsafe fn completeWithHTTPAuthorizationHeaders(
            &self,
            http_authorization_headers: &NSDictionary<NSString, NSString>,
        );

        #[method(completeWithHTTPResponse:httpBody:)]
        pub unsafe fn completeWithHTTPResponse_httpBody(
            &self,
            http_response: &NSHTTPURLResponse,
            http_body: Option<&NSData>,
        );

        #[cfg(feature = "ASAuthorizationProviderExtensionAuthorizationResult")]
        #[method(completeWithAuthorizationResult:)]
        pub unsafe fn completeWithAuthorizationResult(
            &self,
            authorization_result: &ASAuthorizationProviderExtensionAuthorizationResult,
        );

        #[method(completeWithError:)]
        pub unsafe fn completeWithError(&self, error: &NSError);

        #[cfg(feature = "block2")]
        #[method(presentAuthorizationViewControllerWithCompletion:)]
        pub unsafe fn presentAuthorizationViewControllerWithCompletion(
            &self,
            completion: &block2::Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[method_id(@__retain_semantics Other url)]
        pub unsafe fn url(&self) -> Retained<NSURL>;

        #[method_id(@__retain_semantics Other requestedOperation)]
        pub unsafe fn requestedOperation(
            &self,
        ) -> Retained<ASAuthorizationProviderAuthorizationOperation>;

        #[method_id(@__retain_semantics Other httpHeaders)]
        pub unsafe fn httpHeaders(&self) -> Retained<NSDictionary<NSString, NSString>>;

        #[method_id(@__retain_semantics Other httpBody)]
        pub unsafe fn httpBody(&self) -> Retained<NSData>;

        #[method_id(@__retain_semantics Other realm)]
        pub unsafe fn realm(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other extensionData)]
        pub unsafe fn extensionData(&self) -> Retained<NSDictionary>;

        #[method_id(@__retain_semantics Other callerBundleIdentifier)]
        pub unsafe fn callerBundleIdentifier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other authorizationOptions)]
        pub unsafe fn authorizationOptions(&self) -> Retained<NSDictionary>;

        #[method(isCallerManaged)]
        pub unsafe fn isCallerManaged(&self) -> bool;

        #[method_id(@__retain_semantics Other callerTeamIdentifier)]
        pub unsafe fn callerTeamIdentifier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other localizedCallerDisplayName)]
        pub unsafe fn localizedCallerDisplayName(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other callerAuditToken)]
        pub unsafe fn callerAuditToken(&self) -> Retained<NSData>;

        #[method(isUserInterfaceEnabled)]
        pub unsafe fn isUserInterfaceEnabled(&self) -> bool;

        #[cfg(feature = "ASAuthorizationProviderExtensionLoginManager")]
        #[method_id(@__retain_semantics Other loginManager)]
        pub unsafe fn loginManager(
            &self,
        ) -> Option<Retained<ASAuthorizationProviderExtensionLoginManager>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ASAuthorizationProviderExtensionAuthorizationRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
