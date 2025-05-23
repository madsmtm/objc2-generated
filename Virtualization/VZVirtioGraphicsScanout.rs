//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// Class representing a Virtio graphics device scanout.
    ///
    /// The VZVirtioGraphicsScanout is the runtime counterpart of VZVirtioGraphicsScanoutConfiguration.
    ///
    /// When a graphics device is configured with class VZVirtioGraphicsScanoutConfiguration,
    /// the VZGraphicsDevice's displays are in the same order as their configuration objects and they have the type VZVirtioGraphicsScanout.
    ///
    /// For example, if when setting up a virtual machine, its `VZVirtioGraphicsDeviceConfiguration.scanouts[0]` is a `VZVirtioGraphicsScanoutConfiguration`,
    /// then after creating a virtual machine from the configuration, the `VZVirtualMachine.graphicsDevices` is a `VZVirtioGraphicsDevice`.
    /// The `VZVirtioGraphicsDevice.displays[0]` is a `VZVirtioGraphicsScanout` corresponding to the `VZVirtioGraphicsScanoutConfiguration` in the configuration.
    ///
    ///
    /// See: VZVirtioGraphicsScanoutConfiguration
    ///
    /// See: VZGraphicsDisplay
    ///
    /// See: VZGraphicsDevice
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzvirtiographicsscanout?language=objc)
    #[unsafe(super(VZGraphicsDisplay, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZGraphicsDisplay")]
    pub struct VZVirtioGraphicsScanout;
);

#[cfg(feature = "VZGraphicsDisplay")]
extern_conformance!(
    unsafe impl NSObjectProtocol for VZVirtioGraphicsScanout {}
);

#[cfg(feature = "VZGraphicsDisplay")]
impl VZVirtioGraphicsScanout {
    extern_methods!();
}

/// Methods declared on superclass `VZGraphicsDisplay`.
#[cfg(feature = "VZGraphicsDisplay")]
impl VZVirtioGraphicsScanout {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}
