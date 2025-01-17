//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// Virtio Console Port Array
    ///
    /// This array stores a collection of ports configured for use by a VZVirtioConsoleDevice. VZVirtioConsolePort objects may be retrieved by index.
    ///
    /// See: VZVirtioConsoleDevice
    ///
    /// See: VZVirtioConsolePort
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzvirtioconsoleportarray?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZVirtioConsolePortArray;
);

unsafe impl NSObjectProtocol for VZVirtioConsolePortArray {}

extern_methods!(
    unsafe impl VZVirtioConsolePortArray {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "VZVirtioConsolePort")]
        /// Get a port at the specified index.
        #[unsafe(method_family(none))]
        #[method_id(objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            port_index: NSUInteger,
        ) -> Option<Retained<VZVirtioConsolePort>>;

        /// The maximum number of ports allocated by this device.
        #[method(maximumPortCount)]
        pub unsafe fn maximumPortCount(&self) -> u32;
    }
);
