//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZSocketDeviceConfiguration;

    unsafe impl ClassType for VZSocketDeviceConfiguration {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for VZSocketDeviceConfiguration {}

unsafe impl CopyingHelper for VZSocketDeviceConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for VZSocketDeviceConfiguration {}

extern_methods!(
    unsafe impl VZSocketDeviceConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
