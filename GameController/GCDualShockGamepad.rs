//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// The GCDualShockGamepad profile represents any supported DualShock 4 controller.
    ///
    ///
    /// See: GCExtendedGamepad
    ///
    /// See: GCMotion
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcdualshockgamepad?language=objc)
    #[unsafe(super(GCExtendedGamepad, GCPhysicalInputProfile, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "GCExtendedGamepad", feature = "GCPhysicalInputProfile"))]
    pub struct GCDualShockGamepad;
);

#[cfg(all(feature = "GCExtendedGamepad", feature = "GCPhysicalInputProfile"))]
unsafe impl NSObjectProtocol for GCDualShockGamepad {}

extern_methods!(
    #[cfg(all(feature = "GCExtendedGamepad", feature = "GCPhysicalInputProfile"))]
    unsafe impl GCDualShockGamepad {
        #[cfg(all(feature = "GCControllerButtonInput", feature = "GCControllerElement"))]
        /// DualShock controllers have a touchpad with a button and two-finger tracking.
        #[unsafe(method_family(none))]
        #[method_id(touchpadButton)]
        pub unsafe fn touchpadButton(&self) -> Option<Retained<GCControllerButtonInput>>;

        #[cfg(all(feature = "GCControllerDirectionPad", feature = "GCControllerElement"))]
        #[unsafe(method_family(none))]
        #[method_id(touchpadPrimary)]
        pub unsafe fn touchpadPrimary(&self) -> Option<Retained<GCControllerDirectionPad>>;

        #[cfg(all(feature = "GCControllerDirectionPad", feature = "GCControllerElement"))]
        #[unsafe(method_family(none))]
        #[method_id(touchpadSecondary)]
        pub unsafe fn touchpadSecondary(&self) -> Option<Retained<GCControllerDirectionPad>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "GCExtendedGamepad", feature = "GCPhysicalInputProfile"))]
    unsafe impl GCDualShockGamepad {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
