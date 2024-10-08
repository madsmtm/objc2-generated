//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKDeletedObject;

    unsafe impl ClassType for HKDeletedObject {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for HKDeletedObject {}

unsafe impl NSObjectProtocol for HKDeletedObject {}

unsafe impl NSSecureCoding for HKDeletedObject {}

extern_methods!(
    unsafe impl HKDeletedObject {
        #[method_id(@__retain_semantics Other UUID)]
        pub unsafe fn UUID(&self) -> Retained<NSUUID>;

        #[method_id(@__retain_semantics Other metadata)]
        pub unsafe fn metadata(&self) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKDeletedObject {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
