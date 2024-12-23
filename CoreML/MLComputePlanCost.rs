//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// A class representing the estimated cost of executing a layer/operation.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coreml/mlcomputeplancost?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLComputePlanCost;
);

unsafe impl Send for MLComputePlanCost {}

unsafe impl Sync for MLComputePlanCost {}

unsafe impl NSObjectProtocol for MLComputePlanCost {}

extern_methods!(
    unsafe impl MLComputePlanCost {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        /// The estimated workload of executing the operation over the total model execution. The value is between [0.0, 1.0].
        #[method(weight)]
        pub unsafe fn weight(&self) -> c_double;
    }
);
