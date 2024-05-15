//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZMacHardwareModel;

    unsafe impl ClassType for VZMacHardwareModel {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for VZMacHardwareModel {}

unsafe impl NSObjectProtocol for VZMacHardwareModel {}

extern_methods!(
    unsafe impl VZMacHardwareModel {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithDataRepresentation:)]
        pub unsafe fn initWithDataRepresentation(
            this: Allocated<Self>,
            data_representation: &NSData,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other dataRepresentation)]
        pub unsafe fn dataRepresentation(&self) -> Id<NSData>;

        #[method(isSupported)]
        pub unsafe fn isSupported(&self) -> bool;
    }
);