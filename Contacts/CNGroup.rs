//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNGroup;

    unsafe impl ClassType for CNGroup {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for CNGroup {}

unsafe impl NSCopying for CNGroup {}

unsafe impl CopyingHelper for CNGroup {
    type Result = Self;
}

unsafe impl NSMutableCopying for CNGroup {}

#[cfg(feature = "CNMutableGroup")]
unsafe impl MutableCopyingHelper for CNGroup {
    type Result = CNMutableGroup;
}

unsafe impl NSObjectProtocol for CNGroup {}

unsafe impl NSSecureCoding for CNGroup {}

extern_methods!(
    unsafe impl CNGroup {
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CNGroup {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    pub static CNGroupIdentifierKey: &'static NSString;
}

extern "C" {
    pub static CNGroupNameKey: &'static NSString;
}
