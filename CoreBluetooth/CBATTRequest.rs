//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CBATTRequest;

    unsafe impl ClassType for CBATTRequest {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for CBATTRequest {}

extern_methods!(
    unsafe impl CBATTRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(all(feature = "CBCentral", feature = "CBPeer"))]
        #[method_id(@__retain_semantics Other central)]
        pub unsafe fn central(&self) -> Id<CBCentral>;

        #[cfg(all(feature = "CBAttribute", feature = "CBCharacteristic"))]
        #[method_id(@__retain_semantics Other characteristic)]
        pub unsafe fn characteristic(&self) -> Id<CBCharacteristic>;

        #[method(offset)]
        pub unsafe fn offset(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Option<Id<NSData>>;

        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: Option<&NSData>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CBATTRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);