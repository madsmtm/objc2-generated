//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLState;

    unsafe impl ClassType for MLState {
        type Super = NSObject;
    }
);

unsafe impl Send for MLState {}

unsafe impl Sync for MLState {}

unsafe impl NSObjectProtocol for MLState {}

extern_methods!(
    unsafe impl MLState {
        #[cfg(all(feature = "MLMultiArray", feature = "block2"))]
        #[method(getMultiArrayForStateNamed:handler:)]
        pub unsafe fn getMultiArrayForStateNamed_handler(
            &self,
            state_name: &NSString,
            handler: &block2::Block<dyn Fn(NonNull<MLMultiArray>) + '_>,
        );

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);