//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ILMessageFilterExtension;

    unsafe impl ClassType for ILMessageFilterExtension {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for ILMessageFilterExtension {}

extern_methods!(
    unsafe impl ILMessageFilterExtension {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ILMessageFilterExtension {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
