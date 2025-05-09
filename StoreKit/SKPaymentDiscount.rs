//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/storekit/skpaymentdiscount?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Create a Product.PurchaseOption.promotionalOffer to use in Product.purchase(confirmIn:options:)"]
    pub struct SKPaymentDiscount;
);

unsafe impl Send for SKPaymentDiscount {}

unsafe impl Sync for SKPaymentDiscount {}

extern_conformance!(
    unsafe impl NSObjectProtocol for SKPaymentDiscount {}
);

impl SKPaymentDiscount {
    extern_methods!(
        #[deprecated = "Create a Product.PurchaseOption.promotionalOffer to use in Product.purchase(confirmIn:options:)"]
        #[unsafe(method(initWithIdentifier:keyIdentifier:nonce:signature:timestamp:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithIdentifier_keyIdentifier_nonce_signature_timestamp(
            this: Allocated<Self>,
            identifier: &NSString,
            key_identifier: &NSString,
            nonce: &NSUUID,
            signature: &NSString,
            timestamp: &NSNumber,
        ) -> Retained<Self>;

        #[deprecated = "Create a Product.PurchaseOption.promotionalOffer to use in Product.purchase(confirmIn:options:)"]
        #[unsafe(method(identifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[deprecated = "Create a Product.PurchaseOption.promotionalOffer to use in Product.purchase(confirmIn:options:)"]
        #[unsafe(method(keyIdentifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn keyIdentifier(&self) -> Retained<NSString>;

        #[deprecated = "Create a Product.PurchaseOption.promotionalOffer to use in Product.purchase(confirmIn:options:)"]
        #[unsafe(method(nonce))]
        #[unsafe(method_family = none)]
        pub unsafe fn nonce(&self) -> Retained<NSUUID>;

        #[deprecated = "Create a Product.PurchaseOption.promotionalOffer to use in Product.purchase(confirmIn:options:)"]
        #[unsafe(method(signature))]
        #[unsafe(method_family = none)]
        pub unsafe fn signature(&self) -> Retained<NSString>;

        #[deprecated = "Create a Product.PurchaseOption.promotionalOffer to use in Product.purchase(confirmIn:options:)"]
        #[unsafe(method(timestamp))]
        #[unsafe(method_family = none)]
        pub unsafe fn timestamp(&self) -> Retained<NSNumber>;
    );
}

/// Methods declared on superclass `NSObject`.
impl SKPaymentDiscount {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
