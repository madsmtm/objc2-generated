//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZSerialPortConfiguration")]
    pub struct VZVirtioConsoleDeviceSerialPortConfiguration;

    #[cfg(feature = "VZSerialPortConfiguration")]
    unsafe impl ClassType for VZVirtioConsoleDeviceSerialPortConfiguration {
        #[inherits(NSObject)]
        type Super = VZSerialPortConfiguration;
    }
);

#[cfg(feature = "VZSerialPortConfiguration")]
unsafe impl NSCopying for VZVirtioConsoleDeviceSerialPortConfiguration {}

#[cfg(feature = "VZSerialPortConfiguration")]
unsafe impl CopyingHelper for VZVirtioConsoleDeviceSerialPortConfiguration {
    type Result = Self;
}

#[cfg(feature = "VZSerialPortConfiguration")]
unsafe impl NSObjectProtocol for VZVirtioConsoleDeviceSerialPortConfiguration {}

extern_methods!(
    #[cfg(feature = "VZSerialPortConfiguration")]
    unsafe impl VZVirtioConsoleDeviceSerialPortConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VZSerialPortConfiguration`
    #[cfg(feature = "VZSerialPortConfiguration")]
    unsafe impl VZVirtioConsoleDeviceSerialPortConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
