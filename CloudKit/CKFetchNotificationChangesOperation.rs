//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CKOperation")]
    pub struct CKFetchNotificationChangesOperation;

    #[cfg(feature = "CKOperation")]
    unsafe impl ClassType for CKFetchNotificationChangesOperation {
        #[inherits(NSOperation, NSObject)]
        type Super = CKOperation;
    }
);

#[cfg(feature = "CKOperation")]
unsafe impl NSObjectProtocol for CKFetchNotificationChangesOperation {}

extern_methods!(
    #[cfg(feature = "CKOperation")]
    unsafe impl CKFetchNotificationChangesOperation {}
);

extern_methods!(
    /// Methods declared on superclass `CKOperation`
    #[cfg(feature = "CKOperation")]
    unsafe impl CKFetchNotificationChangesOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CKOperation")]
    unsafe impl CKFetchNotificationChangesOperation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
