//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZPointingDeviceConfiguration;

    unsafe impl ClassType for VZPointingDeviceConfiguration {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for VZPointingDeviceConfiguration {}

unsafe impl CopyingHelper for VZPointingDeviceConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for VZPointingDeviceConfiguration {}

extern_methods!(
    unsafe impl VZPointingDeviceConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
