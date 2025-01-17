//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Virtio Console Port Configuration Array
    ///
    /// This array stores a collection of port configurations for a VZVirtioConsoleConfiguration. The index in the array corresponds to the port index used in the virtual machine.
    ///
    /// A maximumPortCount value may be set but must be larger than the highest indexed port. If no maximumPortCount value is set, the highest indexed port will be used.
    ///
    /// See: VZVirtioConsoleConfiguration
    ///
    /// See: VZVirtioConsolePortConfiguration
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzvirtioconsoleportconfigurationarray?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZVirtioConsolePortConfigurationArray;
);

unsafe impl NSCopying for VZVirtioConsolePortConfigurationArray {}

unsafe impl CopyingHelper for VZVirtioConsolePortConfigurationArray {
    type Result = Self;
}

unsafe impl NSObjectProtocol for VZVirtioConsolePortConfigurationArray {}

extern_methods!(
    unsafe impl VZVirtioConsolePortConfigurationArray {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// The maximum number of ports allocated by this device. The default is the number of ports attached to this device.
        #[method(maximumPortCount)]
        pub unsafe fn maximumPortCount(&self) -> u32;

        /// Setter for [`maximumPortCount`][Self::maximumPortCount].
        #[method(setMaximumPortCount:)]
        pub unsafe fn setMaximumPortCount(&self, maximum_port_count: u32);

        #[cfg(all(
            feature = "VZConsolePortConfiguration",
            feature = "VZVirtioConsolePortConfiguration"
        ))]
        /// Get a port configuration at the specified index.
        #[unsafe(method_family(none))]
        #[method_id(objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            port_index: NSUInteger,
        ) -> Option<Retained<VZVirtioConsolePortConfiguration>>;

        #[cfg(all(
            feature = "VZConsolePortConfiguration",
            feature = "VZVirtioConsolePortConfiguration"
        ))]
        /// Set a port configuration at the specified index.
        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            configuration: Option<&VZVirtioConsolePortConfiguration>,
            port_index: NSUInteger,
        );
    }
);
