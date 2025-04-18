//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A class representing an argument in the Program.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coreml/mlmodelstructureprogramargument?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLModelStructureProgramArgument;
);

unsafe impl Send for MLModelStructureProgramArgument {}

unsafe impl Sync for MLModelStructureProgramArgument {}

extern_conformance!(
    unsafe impl NSObjectProtocol for MLModelStructureProgramArgument {}
);

impl MLModelStructureProgramArgument {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "MLModelStructureProgramBinding")]
        /// The array of bindings.
        #[unsafe(method(bindings))]
        #[unsafe(method_family = none)]
        pub unsafe fn bindings(&self) -> Retained<NSArray<MLModelStructureProgramBinding>>;
    );
}
