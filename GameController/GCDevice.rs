//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "dispatch2")]
use dispatch2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcdevice?language=objc)
    pub unsafe trait GCDevice: NSObjectProtocol {
        #[cfg(feature = "dispatch2")]
        /// The dispatch queue that element value change handlers are submitted on. The default queue is main, and setting this to any
        /// other queue will make value change handlers dispatch async on the given queue. This is useful if the main game loop
        /// of the application is not on main, or if input logic is handled on another thread from the main game loop.
        ///
        ///
        /// See: GCControllerAxisInput.valueChangedHandler
        ///
        /// See: GCControllerButtonInput.valueChangedHandler
        ///
        /// See: GCControllerButtonInput.pressedChangedHandler
        ///
        /// See: GCControllerDirectionPad.valueChangedHandler
        ///
        /// See: GCMotion.valueChangedHandler
        #[unsafe(method(handlerQueue))]
        #[unsafe(method_family = none)]
        unsafe fn handlerQueue(&self) -> Retained<DispatchQueue>;

        #[cfg(feature = "dispatch2")]
        /// Setter for [`handlerQueue`][Self::handlerQueue].
        #[unsafe(method(setHandlerQueue:))]
        #[unsafe(method_family = none)]
        unsafe fn setHandlerQueue(&self, handler_queue: &DispatchQueue);

        /// A vendor supplied name. May be nil, and is not guaranteed to be unique. This should not be used as a key in a dictionary,
        /// but simply as a way to present some basic information about the device in testing or to the user.
        #[unsafe(method(vendorName))]
        #[unsafe(method_family = none)]
        unsafe fn vendorName(&self) -> Option<Retained<NSString>>;

        /// The product category the device belongs to. This is useful for setting appropriate UI elements based on what type of device is connected.
        ///
        ///
        /// See: GCProductCategories.h
        #[unsafe(method(productCategory))]
        #[unsafe(method_family = none)]
        unsafe fn productCategory(&self) -> Retained<NSString>;

        #[cfg(feature = "GCPhysicalInputProfile")]
        /// Gets the physical input profile for the device.
        ///
        ///
        /// Note: This is equivalent to the controller's gamepad, microGamepad, or extendedGamepad instance.
        ///
        /// See: GCController.microGamepad
        ///
        /// See: GCController.extendedGamepad
        #[deprecated = "Use the physicalInputProfile property on GCController instead.  For GCKeyboard, use the keyboardInput property.  For GCMouse, use the mouseInput property."]
        #[unsafe(method(physicalInputProfile))]
        #[unsafe(method_family = none)]
        unsafe fn physicalInputProfile(&self) -> Retained<GCPhysicalInputProfile>;
    }
);
