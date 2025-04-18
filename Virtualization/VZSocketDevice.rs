//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// Base class representing a socket device in a virtual machine.
    ///
    /// VZSocketDevice should not be instantiated directly.
    ///
    /// Socket devices are first configured on the VZVirtualMachineConfiguration through a subclass of VZSocketDeviceConfiguration.
    /// When a VZVirtualMachine is created from the configuration, the socket devices are available through the VZVirtualMachine.socketDevices property.
    ///
    /// The real type of VZSocketDevice corresponds to the type used by the configuration.
    /// For example, a VZVirtioSocketDeviceConfiguration leads to a device of type VZVirtioSocketDevice.
    ///
    /// See: VZVirtioSocketDevice
    ///
    /// See: VZVirtioSocketDeviceConfiguration
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzsocketdevice?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZSocketDevice;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for VZSocketDevice {}
);

impl VZSocketDevice {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}
