//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Class representing a Virtio console port in a virtual machine.
    ///
    /// VZVirtioConsolePort should not be instantiated directly. This object can be retrieved from the VZVirtioConsoleDevice ports property.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzvirtioconsoleport?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZVirtioConsolePort;
);

unsafe impl NSObjectProtocol for VZVirtioConsolePort {}

extern_methods!(
    unsafe impl VZVirtioConsolePort {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// The console port name currently being used by this port.
        ///
        /// This property may not change while the VM is running. A null value indicates no name has been set.
        #[unsafe(method_family(none))]
        #[method_id(name)]
        pub unsafe fn name(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "VZSerialPortAttachment")]
        /// The console port attachment that's currently connected to this console port.
        ///
        /// This property may change at any time while the VM is running.
        #[unsafe(method_family(none))]
        #[method_id(attachment)]
        pub unsafe fn attachment(&self) -> Option<Retained<VZSerialPortAttachment>>;

        #[cfg(feature = "VZSerialPortAttachment")]
        /// Setter for [`attachment`][Self::attachment].
        #[method(setAttachment:)]
        pub unsafe fn setAttachment(&self, attachment: Option<&VZSerialPortAttachment>);
    }
);
