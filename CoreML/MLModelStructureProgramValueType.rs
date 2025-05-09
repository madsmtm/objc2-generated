//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// A class representing the type of a value or a variable in the Program.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coreml/mlmodelstructureprogramvaluetype?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLModelStructureProgramValueType;
);

unsafe impl Send for MLModelStructureProgramValueType {}

unsafe impl Sync for MLModelStructureProgramValueType {}

extern_conformance!(
    unsafe impl NSObjectProtocol for MLModelStructureProgramValueType {}
);

impl MLModelStructureProgramValueType {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
