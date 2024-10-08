//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKLensSpecification;

    unsafe impl ClassType for HKLensSpecification {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for HKLensSpecification {}

extern_methods!(
    unsafe impl HKLensSpecification {
        #[cfg(feature = "HKQuantity")]
        #[method_id(@__retain_semantics Other sphere)]
        pub unsafe fn sphere(&self) -> Retained<HKQuantity>;

        #[cfg(feature = "HKQuantity")]
        #[method_id(@__retain_semantics Other cylinder)]
        pub unsafe fn cylinder(&self) -> Option<Retained<HKQuantity>>;

        #[cfg(feature = "HKQuantity")]
        #[method_id(@__retain_semantics Other axis)]
        pub unsafe fn axis(&self) -> Option<Retained<HKQuantity>>;

        #[cfg(feature = "HKQuantity")]
        #[method_id(@__retain_semantics Other addPower)]
        pub unsafe fn addPower(&self) -> Option<Retained<HKQuantity>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
