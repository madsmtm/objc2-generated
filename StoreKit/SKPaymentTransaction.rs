//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SKPaymentTransactionState(pub NSInteger);
impl SKPaymentTransactionState {
    #[doc(alias = "SKPaymentTransactionStatePurchasing")]
    pub const Purchasing: Self = Self(0);
    #[doc(alias = "SKPaymentTransactionStatePurchased")]
    pub const Purchased: Self = Self(1);
    #[doc(alias = "SKPaymentTransactionStateFailed")]
    pub const Failed: Self = Self(2);
    #[doc(alias = "SKPaymentTransactionStateRestored")]
    pub const Restored: Self = Self(3);
    #[doc(alias = "SKPaymentTransactionStateDeferred")]
    pub const Deferred: Self = Self(4);
}

unsafe impl Encode for SKPaymentTransactionState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SKPaymentTransactionState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SKPaymentTransaction;

    unsafe impl ClassType for SKPaymentTransaction {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for SKPaymentTransaction {}

unsafe impl Sync for SKPaymentTransaction {}

unsafe impl NSObjectProtocol for SKPaymentTransaction {}

extern_methods!(
    unsafe impl SKPaymentTransaction {
        #[method_id(@__retain_semantics Other error)]
        pub unsafe fn error(&self) -> Option<Retained<NSError>>;

        #[method_id(@__retain_semantics Other originalTransaction)]
        pub unsafe fn originalTransaction(&self) -> Option<Retained<SKPaymentTransaction>>;

        #[cfg(feature = "SKPayment")]
        #[method_id(@__retain_semantics Other payment)]
        pub unsafe fn payment(&self) -> Retained<SKPayment>;

        #[cfg(feature = "SKDownload")]
        #[deprecated = "Hosted content is no longer supported"]
        #[method_id(@__retain_semantics Other downloads)]
        pub unsafe fn downloads(&self) -> Retained<NSArray<SKDownload>>;

        #[method_id(@__retain_semantics Other transactionDate)]
        pub unsafe fn transactionDate(&self) -> Option<Retained<NSDate>>;

        #[method_id(@__retain_semantics Other transactionIdentifier)]
        pub unsafe fn transactionIdentifier(&self) -> Option<Retained<NSString>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other transactionReceipt)]
        pub unsafe fn transactionReceipt(&self) -> Option<Retained<NSData>>;

        #[method(transactionState)]
        pub unsafe fn transactionState(&self) -> SKPaymentTransactionState;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SKPaymentTransaction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
