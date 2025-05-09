//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/passkit/pkissuerprovisioningextensionpaymentpassentry?language=objc)
    #[unsafe(super(PKIssuerProvisioningExtensionPassEntry, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PKIssuerProvisioningExtensionPassEntry")]
    pub struct PKIssuerProvisioningExtensionPaymentPassEntry;
);

#[cfg(feature = "PKIssuerProvisioningExtensionPassEntry")]
extern_conformance!(
    unsafe impl NSObjectProtocol for PKIssuerProvisioningExtensionPaymentPassEntry {}
);

#[cfg(feature = "PKIssuerProvisioningExtensionPassEntry")]
impl PKIssuerProvisioningExtensionPaymentPassEntry {
    extern_methods!(
        #[cfg(all(feature = "PKAddPaymentPassRequest", feature = "objc2-core-graphics"))]
        #[unsafe(method(initWithIdentifier:title:art:addRequestConfiguration:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithIdentifier_title_art_addRequestConfiguration(
            this: Allocated<Self>,
            identifier: &NSString,
            title: &NSString,
            art: &CGImage,
            configuration: &PKAddPaymentPassRequestConfiguration,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "PKAddPaymentPassRequest")]
        #[unsafe(method(addRequestConfiguration))]
        #[unsafe(method_family = none)]
        pub unsafe fn addRequestConfiguration(
            &self,
        ) -> Retained<PKAddPaymentPassRequestConfiguration>;
    );
}

/// Methods declared on superclass `PKIssuerProvisioningExtensionPassEntry`.
#[cfg(feature = "PKIssuerProvisioningExtensionPassEntry")]
impl PKIssuerProvisioningExtensionPaymentPassEntry {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "PKIssuerProvisioningExtensionPassEntry")]
impl PKIssuerProvisioningExtensionPaymentPassEntry {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
