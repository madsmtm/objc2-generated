//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Describes a model parameter along with a default value and any applicable constaint on the values.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coreml/mlparameterdescription?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLParameterDescription;
);

unsafe impl NSCoding for MLParameterDescription {}

unsafe impl NSObjectProtocol for MLParameterDescription {}

unsafe impl NSSecureCoding for MLParameterDescription {}

extern_methods!(
    unsafe impl MLParameterDescription {
        #[cfg(all(feature = "MLKey", feature = "MLParameterKey"))]
        #[unsafe(method_family(none))]
        #[method_id(key)]
        pub unsafe fn key(&self) -> Retained<MLParameterKey>;

        #[unsafe(method_family(none))]
        #[method_id(defaultValue)]
        pub unsafe fn defaultValue(&self) -> Retained<AnyObject>;

        #[cfg(feature = "MLNumericConstraint")]
        #[unsafe(method_family(none))]
        #[method_id(numericConstraint)]
        pub unsafe fn numericConstraint(&self) -> Option<Retained<MLNumericConstraint>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MLParameterDescription {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
