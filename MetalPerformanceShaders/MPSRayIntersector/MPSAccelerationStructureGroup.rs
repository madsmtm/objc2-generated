//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// A group of acceleration structures which may be used together in an instance acceleration
    /// structure.
    ///
    ///
    /// All acceleration structures in an instance acceleration structures must be created
    /// with the same group, although they do not all need to be used in the same instance acceleration
    /// structure. The acceleration structures in a group share internal GPU memory allocations, so
    /// the total number and size of acceleration structures that can be created with the same group is
    /// limited by the Metal device's buffer size limits. Therefore, do not group acceleration
    /// structures unless they are likely to be used in the same instance acceleration structure.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsaccelerationstructuregroup?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct MPSAccelerationStructureGroup;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MPSAccelerationStructureGroup {}
);

impl MPSAccelerationStructureGroup {
    extern_methods!(
        /// The Metal device this acceleration structure group was created with
        #[deprecated]
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        pub unsafe fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        #[deprecated]
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[deprecated]
        #[unsafe(method(initWithDevice:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl MPSAccelerationStructureGroup {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
