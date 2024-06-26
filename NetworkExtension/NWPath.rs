//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NWPathStatus(pub NSInteger);
impl NWPathStatus {
    #[doc(alias = "NWPathStatusInvalid")]
    pub const Invalid: Self = Self(0);
    #[doc(alias = "NWPathStatusSatisfied")]
    pub const Satisfied: Self = Self(1);
    #[doc(alias = "NWPathStatusUnsatisfied")]
    pub const Unsatisfied: Self = Self(2);
    #[doc(alias = "NWPathStatusSatisfiable")]
    pub const Satisfiable: Self = Self(3);
}

unsafe impl Encode for NWPathStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NWPathStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NWPath;

    unsafe impl ClassType for NWPath {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NWPath {}

extern_methods!(
    unsafe impl NWPath {
        #[method(status)]
        pub unsafe fn status(&self) -> NWPathStatus;

        #[method(isExpensive)]
        pub unsafe fn isExpensive(&self) -> bool;

        #[method(isConstrained)]
        pub unsafe fn isConstrained(&self) -> bool;

        #[method(isEqualToPath:)]
        pub unsafe fn isEqualToPath(&self, path: &NWPath) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NWPath {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
