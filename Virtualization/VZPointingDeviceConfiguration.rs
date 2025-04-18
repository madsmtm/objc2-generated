//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Base class for a pointing device configuration.
    ///
    /// VZPointingDeviceConfiguration should not be instantiated directly.
    /// One of its subclasses like VZUSBScreenCoordinatePointingDeviceConfiguration or VZMacTrackpadConfiguration should be used instead.
    ///
    ///
    /// See: VZUSBScreenCoordinatePointingDeviceConfiguration
    ///
    /// See: VZMacTrackpadConfiguration
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzpointingdeviceconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZPointingDeviceConfiguration;
);

extern_conformance!(
    unsafe impl NSCopying for VZPointingDeviceConfiguration {}
);

unsafe impl CopyingHelper for VZPointingDeviceConfiguration {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for VZPointingDeviceConfiguration {}
);

impl VZPointingDeviceConfiguration {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}
