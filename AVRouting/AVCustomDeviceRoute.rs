//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// An object that represents a custom device route.
    ///
    /// Use the value of a route’s ``AVCustomDeviceRoute/networkEndpoint`` or
    /// ``AVCustomDeviceRoute/bluetoothIdentifier`` property to establish a
    /// connection to a device. Typically, only one of the properties provides a
    /// valid value, depending on the type of device. In certain cases, both
    /// properties might provide valid values, in which case your app determines which
    /// one to use.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avrouting/avcustomdeviceroute?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCustomDeviceRoute;
);

unsafe impl Send for AVCustomDeviceRoute {}

unsafe impl Sync for AVCustomDeviceRoute {}

unsafe impl NSObjectProtocol for AVCustomDeviceRoute {}

extern_methods!(
    unsafe impl AVCustomDeviceRoute {
        /// An identifier to use to establish a connection to a Bluetooth device.
        #[unsafe(method_family(none))]
        #[method_id(bluetoothIdentifier)]
        pub unsafe fn bluetoothIdentifier(&self) -> Option<Retained<NSUUID>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVCustomDeviceRoute {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
