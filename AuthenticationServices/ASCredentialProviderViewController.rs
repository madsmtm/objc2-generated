//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

#[cfg(target_os = "macos")]
unsafe impl NSCoding for ASCredentialProviderViewController {}

#[cfg(target_os = "macos")]
unsafe impl NSEditor for ASCredentialProviderViewController {}

#[cfg(target_os = "macos")]
unsafe impl NSObjectProtocol for ASCredentialProviderViewController {}

#[cfg(target_os = "macos")]
unsafe impl NSSeguePerforming for ASCredentialProviderViewController {}

#[cfg(target_os = "macos")]
unsafe impl NSUserInterfaceItemIdentification for ASCredentialProviderViewController {}

extern_methods!(
    #[cfg(target_os = "macos")]
    unsafe impl ASCredentialProviderViewController {
        #[cfg(feature = "ASCredentialProviderExtensionContext")]
        #[method_id(@__retain_semantics Other extensionContext)]
        pub unsafe fn extensionContext(&self) -> Retained<ASCredentialProviderExtensionContext>;

        #[cfg(feature = "ASCredentialServiceIdentifier")]
        #[method(prepareCredentialListForServiceIdentifiers:)]
        pub unsafe fn prepareCredentialListForServiceIdentifiers(
            &self,
            service_identifiers: &NSArray<ASCredentialServiceIdentifier>,
        );

        #[cfg(all(
            feature = "ASCredentialServiceIdentifier",
            feature = "ASPasskeyCredentialRequestParameters"
        ))]
        #[method(prepareCredentialListForServiceIdentifiers:requestParameters:)]
        pub unsafe fn prepareCredentialListForServiceIdentifiers_requestParameters(
            &self,
            service_identifiers: &NSArray<ASCredentialServiceIdentifier>,
            request_parameters: &ASPasskeyCredentialRequestParameters,
        );

        #[cfg(feature = "ASCredentialServiceIdentifier")]
        #[method(prepareOneTimeCodeCredentialListForServiceIdentifiers:)]
        pub unsafe fn prepareOneTimeCodeCredentialListForServiceIdentifiers(
            &self,
            service_identifiers: &NSArray<ASCredentialServiceIdentifier>,
        );

        #[cfg(feature = "ASPasswordCredentialIdentity")]
        #[deprecated]
        #[method(provideCredentialWithoutUserInteractionForIdentity:)]
        pub unsafe fn provideCredentialWithoutUserInteractionForIdentity(
            &self,
            credential_identity: &ASPasswordCredentialIdentity,
        );

        #[cfg(feature = "ASCredentialRequest")]
        #[method(provideCredentialWithoutUserInteractionForRequest:)]
        pub unsafe fn provideCredentialWithoutUserInteractionForRequest(
            &self,
            credential_request: &ProtocolObject<dyn ASCredentialRequest>,
        );

        #[cfg(feature = "ASPasswordCredentialIdentity")]
        #[deprecated]
        #[method(prepareInterfaceToProvideCredentialForIdentity:)]
        pub unsafe fn prepareInterfaceToProvideCredentialForIdentity(
            &self,
            credential_identity: &ASPasswordCredentialIdentity,
        );

        #[cfg(feature = "ASCredentialRequest")]
        #[method(prepareInterfaceToProvideCredentialForRequest:)]
        pub unsafe fn prepareInterfaceToProvideCredentialForRequest(
            &self,
            credential_request: &ProtocolObject<dyn ASCredentialRequest>,
        );

        #[method(prepareInterfaceForExtensionConfiguration)]
        pub unsafe fn prepareInterfaceForExtensionConfiguration(&self);

        #[cfg(feature = "ASCredentialRequest")]
        #[method(prepareInterfaceForPasskeyRegistration:)]
        pub unsafe fn prepareInterfaceForPasskeyRegistration(
            &self,
            registration_request: &ProtocolObject<dyn ASCredentialRequest>,
        );

        #[cfg(feature = "ASPasskeyCredentialRequest")]
        #[method(performPasskeyRegistrationWithoutUserInteractionIfPossible:)]
        pub unsafe fn performPasskeyRegistrationWithoutUserInteractionIfPossible(
            &self,
            registration_request: &ASPasskeyCredentialRequest,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(target_os = "macos")]
    unsafe impl ASCredentialProviderViewController {
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(target_os = "macos")]
    unsafe impl ASCredentialProviderViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(target_os = "macos")]
    unsafe impl ASCredentialProviderViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
