//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/corebluetooth/cbattribute?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CBAttribute;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for CBAttribute {}
);

impl CBAttribute {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "CBUUID")]
        /// The Bluetooth UUID of the attribute.
        #[unsafe(method(UUID))]
        #[unsafe(method_family = none)]
        pub unsafe fn UUID(&self) -> Retained<CBUUID>;
    );
}

/// Methods declared on superclass `NSObject`.
impl CBAttribute {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
