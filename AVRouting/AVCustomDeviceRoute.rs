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

extern_conformance!(
    unsafe impl NSObjectProtocol for AVCustomDeviceRoute {}
);

impl AVCustomDeviceRoute {
    extern_methods!(
        /// An identifier to use to establish a connection to a Bluetooth device.
        #[unsafe(method(bluetoothIdentifier))]
        #[unsafe(method_family = none)]
        pub unsafe fn bluetoothIdentifier(&self) -> Option<Retained<NSUUID>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl AVCustomDeviceRoute {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
