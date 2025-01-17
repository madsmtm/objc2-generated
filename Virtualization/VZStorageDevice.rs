//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// Class representing a storage device in a virtual machine.
    ///
    /// VZStorageDevice should not be instantiated directly.
    /// One of its subclasses like VZUSBMassStorageDevice should be used instead.
    ///
    /// See: VZUSBMassStorageDevice
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzstoragedevice?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZStorageDevice;
);

unsafe impl NSObjectProtocol for VZStorageDevice {}

extern_methods!(
    unsafe impl VZStorageDevice {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
