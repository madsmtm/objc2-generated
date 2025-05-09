//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

/// Set this block if you want to be notified when a value on a element changed. If multiple elements have changed this block will be called
/// for each element that changed. As elements in a collection, such as the axis in a dpad, tend to change at the same time and thus
/// will only call this once with the collection as the element.
///
///
/// Parameter `gamepad`: this gamepad that is being used to map the raw input data into logical values on controller elements such as the dpad or the buttons.
///
/// Parameter `element`: the element that has been modified.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcgamepadvaluechangedhandler?language=objc)
#[cfg(all(
    feature = "GCControllerElement",
    feature = "GCPhysicalInputProfile",
    feature = "block2"
))]
pub type GCGamepadValueChangedHandler =
    *mut block2::DynBlock<dyn Fn(NonNull<GCGamepad>, NonNull<GCControllerElement>)>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcgamepad?language=objc)
    #[unsafe(super(GCPhysicalInputProfile, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GCPhysicalInputProfile")]
    #[deprecated]
    pub struct GCGamepad;
);

#[cfg(feature = "GCPhysicalInputProfile")]
extern_conformance!(
    unsafe impl NSObjectProtocol for GCGamepad {}
);

#[cfg(feature = "GCPhysicalInputProfile")]
impl GCGamepad {
    extern_methods!(
        #[cfg(feature = "GCController")]
        /// A profile keeps a reference to the controller that this profile is mapping input from.
        #[deprecated]
        #[unsafe(method(controller))]
        #[unsafe(method_family = none)]
        pub unsafe fn controller(&self) -> Option<Retained<GCController>>;

        #[cfg(all(feature = "GCControllerElement", feature = "block2"))]
        #[deprecated]
        #[unsafe(method(valueChangedHandler))]
        #[unsafe(method_family = none)]
        pub unsafe fn valueChangedHandler(&self) -> GCGamepadValueChangedHandler;

        #[cfg(all(feature = "GCControllerElement", feature = "block2"))]
        /// Setter for [`valueChangedHandler`][Self::valueChangedHandler].
        #[deprecated]
        #[unsafe(method(setValueChangedHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setValueChangedHandler(
            &self,
            value_changed_handler: GCGamepadValueChangedHandler,
        );

        #[cfg(feature = "GCGamepadSnapshot")]
        /// Polls the state vector of the controller and saves it to a snapshot. The snapshot is stored in a device independent
        /// format that can be serialized and used at a later date. This is useful for features such as quality assurance,
        /// save game or replay functionality among many.
        ///
        /// If your application is heavily multithreaded this may also be useful to guarantee atomicity of input handling as
        /// a snapshot will not change based on user input once it is taken.
        #[deprecated]
        #[unsafe(method(saveSnapshot))]
        #[unsafe(method_family = none)]
        pub unsafe fn saveSnapshot(&self) -> Retained<GCGamepadSnapshot>;

        #[cfg(all(feature = "GCControllerDirectionPad", feature = "GCControllerElement"))]
        /// Required to be analog in the Standard profile. All the elements of this directional input are thus analog.
        #[deprecated]
        #[unsafe(method(dpad))]
        #[unsafe(method_family = none)]
        pub unsafe fn dpad(&self) -> Retained<GCControllerDirectionPad>;

        #[cfg(all(feature = "GCControllerButtonInput", feature = "GCControllerElement"))]
        /// All face buttons are required to be analog in the Standard profile. These must be arranged
        /// in the diamond pattern given below:
        ///
        /// Y
        /// /
        /// \
        /// X   B
        /// \
        /// /
        /// A
        #[deprecated]
        #[unsafe(method(buttonA))]
        #[unsafe(method_family = none)]
        pub unsafe fn buttonA(&self) -> Retained<GCControllerButtonInput>;

        #[cfg(all(feature = "GCControllerButtonInput", feature = "GCControllerElement"))]
        #[deprecated]
        #[unsafe(method(buttonB))]
        #[unsafe(method_family = none)]
        pub unsafe fn buttonB(&self) -> Retained<GCControllerButtonInput>;

        #[cfg(all(feature = "GCControllerButtonInput", feature = "GCControllerElement"))]
        #[deprecated]
        #[unsafe(method(buttonX))]
        #[unsafe(method_family = none)]
        pub unsafe fn buttonX(&self) -> Retained<GCControllerButtonInput>;

        #[cfg(all(feature = "GCControllerButtonInput", feature = "GCControllerElement"))]
        #[deprecated]
        #[unsafe(method(buttonY))]
        #[unsafe(method_family = none)]
        pub unsafe fn buttonY(&self) -> Retained<GCControllerButtonInput>;

        #[cfg(all(feature = "GCControllerButtonInput", feature = "GCControllerElement"))]
        /// Shoulder buttons are required to be analog inputs.
        #[deprecated]
        #[unsafe(method(leftShoulder))]
        #[unsafe(method_family = none)]
        pub unsafe fn leftShoulder(&self) -> Retained<GCControllerButtonInput>;

        #[cfg(all(feature = "GCControllerButtonInput", feature = "GCControllerElement"))]
        /// Shoulder buttons are required to be analog inputs.
        #[deprecated]
        #[unsafe(method(rightShoulder))]
        #[unsafe(method_family = none)]
        pub unsafe fn rightShoulder(&self) -> Retained<GCControllerButtonInput>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "GCPhysicalInputProfile")]
impl GCGamepad {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
