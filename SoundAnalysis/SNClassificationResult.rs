//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SNClassification;

    unsafe impl ClassType for SNClassification {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for SNClassification {}

extern_methods!(
    unsafe impl SNClassification {
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[method(confidence)]
        pub unsafe fn confidence(&self) -> c_double;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SNClassificationResult;

    unsafe impl ClassType for SNClassificationResult {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for SNClassificationResult {}

#[cfg(feature = "SNResult")]
unsafe impl SNResult for SNClassificationResult {}

extern_methods!(
    unsafe impl SNClassificationResult {
        #[method_id(@__retain_semantics Other classifications)]
        pub unsafe fn classifications(&self) -> Retained<NSArray<SNClassification>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other classificationForIdentifier:)]
        pub unsafe fn classificationForIdentifier(
            &self,
            identifier: &NSString,
        ) -> Option<Retained<SNClassification>>;
    }
);
