//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Configuration of a USB Mass Storage storage device.
    ///
    /// This device configuration creates a storage device that conforms to the USB Mass Storage specification.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzusbmassstoragedeviceconfiguration?language=objc)
    #[unsafe(super(VZStorageDeviceConfiguration, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZStorageDeviceConfiguration")]
    pub struct VZUSBMassStorageDeviceConfiguration;
);

#[cfg(feature = "VZStorageDeviceConfiguration")]
extern_conformance!(
    unsafe impl NSCopying for VZUSBMassStorageDeviceConfiguration {}
);

#[cfg(feature = "VZStorageDeviceConfiguration")]
unsafe impl CopyingHelper for VZUSBMassStorageDeviceConfiguration {
    type Result = Self;
}

#[cfg(feature = "VZStorageDeviceConfiguration")]
extern_conformance!(
    unsafe impl NSObjectProtocol for VZUSBMassStorageDeviceConfiguration {}
);

#[cfg(all(
    feature = "VZStorageDeviceConfiguration",
    feature = "VZUSBDeviceConfiguration"
))]
extern_conformance!(
    unsafe impl VZUSBDeviceConfiguration for VZUSBMassStorageDeviceConfiguration {}
);

#[cfg(feature = "VZStorageDeviceConfiguration")]
impl VZUSBMassStorageDeviceConfiguration {
    extern_methods!(
        #[cfg(feature = "VZStorageDeviceAttachment")]
        /// Initialize a VZUSBMassStorageDeviceConfiguration with a device attachment.
        ///
        /// Parameter `attachment`: The storage device attachment. This defines how the virtualized device operates on the host side.
        ///
        /// See: VZDiskImageStorageDeviceAttachment
        #[unsafe(method(initWithAttachment:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithAttachment(
            this: Allocated<Self>,
            attachment: &VZStorageDeviceAttachment,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `VZStorageDeviceConfiguration`.
#[cfg(feature = "VZStorageDeviceConfiguration")]
impl VZUSBMassStorageDeviceConfiguration {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}
