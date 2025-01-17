//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Base class for an entropy device configuration.
    ///
    /// VZEntropyDeviceConfiguration should not be instantiated directly.
    /// The subclass VZVirtioEntropyDeviceConfiguration should be used instead.
    ///
    ///
    /// See: VZVirtioEntropyDeviceConfiguration
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzentropydeviceconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZEntropyDeviceConfiguration;
);

unsafe impl NSCopying for VZEntropyDeviceConfiguration {}

unsafe impl CopyingHelper for VZEntropyDeviceConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for VZEntropyDeviceConfiguration {}

extern_methods!(
    unsafe impl VZEntropyDeviceConfiguration {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
