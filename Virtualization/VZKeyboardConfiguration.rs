//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Base class for a keyboard configuration.
    ///
    /// VZKeyboardConfiguration should not be instantiated directly.
    /// One of its subclasses like VZUSBKeyboardConfiguration or VZMacKeyboardConfiguration should be used instead.
    ///
    ///
    /// See: VZUSBKeyboardConfiguration
    ///
    /// See: VZMacKeyboardConfiguration
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzkeyboardconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZKeyboardConfiguration;
);

extern_conformance!(
    unsafe impl NSCopying for VZKeyboardConfiguration {}
);

unsafe impl CopyingHelper for VZKeyboardConfiguration {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for VZKeyboardConfiguration {}
);

impl VZKeyboardConfiguration {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}
