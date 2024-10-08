//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLSequenceConstraint;

    unsafe impl ClassType for MLSequenceConstraint {
        type Super = NSObject;
    }
);

unsafe impl NSCoding for MLSequenceConstraint {}

unsafe impl NSObjectProtocol for MLSequenceConstraint {}

unsafe impl NSSecureCoding for MLSequenceConstraint {}

extern_methods!(
    unsafe impl MLSequenceConstraint {
        #[cfg(feature = "MLFeatureDescription")]
        #[method_id(@__retain_semantics Other valueDescription)]
        pub unsafe fn valueDescription(&self) -> Retained<MLFeatureDescription>;

        #[method(countRange)]
        pub unsafe fn countRange(&self) -> NSRange;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MLSequenceConstraint {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
