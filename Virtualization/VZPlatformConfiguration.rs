//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZPlatformConfiguration;

    unsafe impl ClassType for VZPlatformConfiguration {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for VZPlatformConfiguration {}

unsafe impl CopyingHelper for VZPlatformConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for VZPlatformConfiguration {}

extern_methods!(
    unsafe impl VZPlatformConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
