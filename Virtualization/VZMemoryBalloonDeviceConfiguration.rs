//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Base class for a memory balloon device configuration.
    ///
    /// VZMemoryBalloonDeviceConfiguration should not be instantiated directly.
    /// One of its subclasses like VZVirtioTraditionalMemoryBalloonDeviceConfiguration should be used instead.
    ///
    ///
    /// See: VZVirtioTraditionalMemoryBalloonDeviceConfiguration
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzmemoryballoondeviceconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZMemoryBalloonDeviceConfiguration;
);

extern_conformance!(
    unsafe impl NSCopying for VZMemoryBalloonDeviceConfiguration {}
);

unsafe impl CopyingHelper for VZMemoryBalloonDeviceConfiguration {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for VZMemoryBalloonDeviceConfiguration {}
);

impl VZMemoryBalloonDeviceConfiguration {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}
