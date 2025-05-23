//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationpublickeycredentialprfassertioninputvalues?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationPublicKeyCredentialPRFAssertionInputValues;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for ASAuthorizationPublicKeyCredentialPRFAssertionInputValues {}
);

impl ASAuthorizationPublicKeyCredentialPRFAssertionInputValues {
    extern_methods!(
        #[unsafe(method(initWithSaltInput1:saltInput2:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithSaltInput1_saltInput2(
            this: Allocated<Self>,
            salt_input1: &NSData,
            salt_input2: Option<&NSData>,
        ) -> Retained<Self>;

        #[unsafe(method(saltInput1))]
        #[unsafe(method_family = none)]
        pub unsafe fn saltInput1(&self) -> Retained<NSData>;

        #[unsafe(method(saltInput2))]
        #[unsafe(method_family = none)]
        pub unsafe fn saltInput2(&self) -> Option<Retained<NSData>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl ASAuthorizationPublicKeyCredentialPRFAssertionInputValues {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/authenticationservices/asauthorizationpublickeycredentialprfassertioninput?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationPublicKeyCredentialPRFAssertionInput;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for ASAuthorizationPublicKeyCredentialPRFAssertionInput {}
);

impl ASAuthorizationPublicKeyCredentialPRFAssertionInput {
    extern_methods!(
        #[unsafe(method(initWithInputValues:perCredentialInputValues:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithInputValues_perCredentialInputValues(
            this: Allocated<Self>,
            input_values: Option<&ASAuthorizationPublicKeyCredentialPRFAssertionInputValues>,
            per_credential_input_values: Option<
                &NSDictionary<NSData, ASAuthorizationPublicKeyCredentialPRFAssertionInputValues>,
            >,
        ) -> Retained<Self>;

        #[unsafe(method(inputValues))]
        #[unsafe(method_family = none)]
        pub unsafe fn inputValues(
            &self,
        ) -> Option<Retained<ASAuthorizationPublicKeyCredentialPRFAssertionInputValues>>;

        #[unsafe(method(perCredentialInputValues))]
        #[unsafe(method_family = none)]
        pub unsafe fn perCredentialInputValues(
            &self,
        ) -> Option<
            Retained<
                NSDictionary<NSData, ASAuthorizationPublicKeyCredentialPRFAssertionInputValues>,
            >,
        >;
    );
}

/// Methods declared on superclass `NSObject`.
impl ASAuthorizationPublicKeyCredentialPRFAssertionInput {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
