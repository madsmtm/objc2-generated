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
/// See also [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcextendedgamepadvaluechangedhandler?language=objc)
#[cfg(all(
    feature = "GCControllerElement",
    feature = "GCPhysicalInputProfile",
    feature = "block2"
))]
pub type GCExtendedGamepadValueChangedHandler =
    *mut block2::Block<dyn Fn(NonNull<GCExtendedGamepad>, NonNull<GCControllerElement>)>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcextendedgamepad?language=objc)
    #[unsafe(super(GCPhysicalInputProfile, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GCPhysicalInputProfile")]
    pub struct GCExtendedGamepad;
);

#[cfg(feature = "GCPhysicalInputProfile")]
unsafe impl NSObjectProtocol for GCExtendedGamepad {}

extern_methods!(
    #[cfg(feature = "GCPhysicalInputProfile")]
    unsafe impl GCExtendedGamepad {
        #[cfg(feature = "GCController")]
        #[unsafe(method_family(none))]
        #[method_id(controller)]
        pub unsafe fn controller(&self) -> Option<Retained<GCController>>;

        #[cfg(all(feature = "GCControllerElement", feature = "block2"))]
        #[method(valueChangedHandler)]
        pub unsafe fn valueChangedHandler(&self) -> GCExtendedGamepadValueChangedHandler;

        #[cfg(all(feature = "GCControllerElement", feature = "block2"))]
        /// Setter for [`valueChangedHandler`][Self::valueChangedHandler].
        #[method(setValueChangedHandler:)]
        pub unsafe fn setValueChangedHandler(
            &self,
            value_changed_handler: GCExtendedGamepadValueChangedHandler,
        );

        #[cfg(feature = "GCExtendedGamepadSnapshot")]
        /// Polls the state vector of the controller and saves it to a snapshot. The snapshot is stored in a device independent
        /// format that can be serialized and used at a later date. This is useful for features such as quality assurance,
        /// save game or replay functionality among many.
        ///
        /// If your application is heavily multithreaded this may also be useful to guarantee atomicity of input handling as
        /// a snapshot will not change based on user input once it is taken.
        #[deprecated = "GCExtendedGamepadSnapshot has been deprecated, use [GCController capture] instead"]
        #[unsafe(method_family(none))]
        #[method_id(saveSnapshot)]
        pub unsafe fn saveSnapshot(&self) -> Retained<GCExtendedGamepadSnapshot>;

        #[cfg(all(feature = "GCControllerDirectionPad", feature = "GCControllerElement"))]
        /// Required to be analog in the Extended profile. All the elements of this directional input are thus analog.
        #[unsafe(method_family(none))]
        #[method_id(dpad)]
        pub unsafe fn dpad(&self) -> Retained<GCControllerDirectionPad>;

        #[cfg(all(feature = "GCControllerButtonInput", feature = "GCControllerElement"))]
        /// All face buttons are required to be analog in the Extended profile. These must be arranged
        /// in the diamond pattern given below:
        ///
        /// Y
        /// /
        /// \
        /// X   B
        /// \
        /// /
        /// A
        #[unsafe(method_family(none))]
        #[method_id(buttonA)]
        pub unsafe fn buttonA(&self) -> Retained<GCControllerButtonInput>;

        #[cfg(all(feature = "GCControllerButtonInput", feature = "GCControllerElement"))]
        #[unsafe(method_family(none))]
        #[method_id(buttonB)]
        pub unsafe fn buttonB(&self) -> Retained<GCControllerButtonInput>;

        #[cfg(all(feature = "GCControllerButtonInput", feature = "GCControllerElement"))]
        #[unsafe(method_family(none))]
        #[method_id(buttonX)]
        pub unsafe fn buttonX(&self) -> Retained<GCControllerButtonInput>;

        #[cfg(all(feature = "GCControllerButtonInput", feature = "GCControllerElement"))]
        #[unsafe(method_family(none))]
        #[method_id(buttonY)]
        pub unsafe fn buttonY(&self) -> Retained<GCControllerButtonInput>;

        #[cfg(all(feature = "GCControllerButtonInput", feature = "GCControllerElement"))]
        /// Button menu is the primary menu button, and should be used to enter the main menu and pause the game.
        #[unsafe(method_family(none))]
        #[method_id(buttonMenu)]
        pub unsafe fn buttonMenu(&self) -> Retained<GCControllerButtonInput>;

        #[cfg(all(feature = "GCControllerButtonInput", feature = "GCControllerElement"))]
        /// Button options is the secondary menu button. It should be used to enter a secondary menu, such as graphics and sound configuration, and pause the game.
        #[unsafe(method_family(none))]
        #[method_id(buttonOptions)]
        pub unsafe fn buttonOptions(&self) -> Option<Retained<GCControllerButtonInput>>;

        #[cfg(all(feature = "GCControllerButtonInput", feature = "GCControllerElement"))]
        /// Button home is a special menu button. If the system does not consume button home events, they will be passed to your application and should be used to enter a secondary menu, and pause the game.
        #[unsafe(method_family(none))]
        #[method_id(buttonHome)]
        pub unsafe fn buttonHome(&self) -> Option<Retained<GCControllerButtonInput>>;

        #[cfg(all(feature = "GCControllerDirectionPad", feature = "GCControllerElement"))]
        /// A thumbstick is a 2-axis control that is physically required to be analog. All the elements of this directional input are thus analog.
        #[unsafe(method_family(none))]
        #[method_id(leftThumbstick)]
        pub unsafe fn leftThumbstick(&self) -> Retained<GCControllerDirectionPad>;

        #[cfg(all(feature = "GCControllerDirectionPad", feature = "GCControllerElement"))]
        /// A thumbstick is a 2-axis control that is physically required to be analog. All the elements of this directional input are thus analog.
        #[unsafe(method_family(none))]
        #[method_id(rightThumbstick)]
        pub unsafe fn rightThumbstick(&self) -> Retained<GCControllerDirectionPad>;

        #[cfg(all(feature = "GCControllerButtonInput", feature = "GCControllerElement"))]
        /// Shoulder buttons are required to be analog inputs.
        #[unsafe(method_family(none))]
        #[method_id(leftShoulder)]
        pub unsafe fn leftShoulder(&self) -> Retained<GCControllerButtonInput>;

        #[cfg(all(feature = "GCControllerButtonInput", feature = "GCControllerElement"))]
        /// Shoulder buttons are required to be analog inputs.
        #[unsafe(method_family(none))]
        #[method_id(rightShoulder)]
        pub unsafe fn rightShoulder(&self) -> Retained<GCControllerButtonInput>;

        #[cfg(all(feature = "GCControllerButtonInput", feature = "GCControllerElement"))]
        /// Triggers are required to be analog inputs. Common uses would be acceleration and decelleration in a driving game for example.
        #[unsafe(method_family(none))]
        #[method_id(leftTrigger)]
        pub unsafe fn leftTrigger(&self) -> Retained<GCControllerButtonInput>;

        #[cfg(all(feature = "GCControllerButtonInput", feature = "GCControllerElement"))]
        #[unsafe(method_family(none))]
        #[method_id(rightTrigger)]
        pub unsafe fn rightTrigger(&self) -> Retained<GCControllerButtonInput>;

        #[cfg(all(feature = "GCControllerButtonInput", feature = "GCControllerElement"))]
        /// A thumbstick may also have a clickable component, which is treated as a non-analog button.
        #[unsafe(method_family(none))]
        #[method_id(leftThumbstickButton)]
        pub unsafe fn leftThumbstickButton(&self) -> Option<Retained<GCControllerButtonInput>>;

        #[cfg(all(feature = "GCControllerButtonInput", feature = "GCControllerElement"))]
        #[unsafe(method_family(none))]
        #[method_id(rightThumbstickButton)]
        pub unsafe fn rightThumbstickButton(&self) -> Option<Retained<GCControllerButtonInput>>;

        /// Sets the state vector of the extended gamepad to a copy of the input extended gamepad's state vector.
        ///
        ///
        /// Note: If the controller's snapshot flag is set to NO, this method has no effect.
        ///
        /// See: GCController.snapshot
        #[method(setStateFromExtendedGamepad:)]
        pub unsafe fn setStateFromExtendedGamepad(&self, extended_gamepad: &GCExtendedGamepad);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GCPhysicalInputProfile")]
    unsafe impl GCExtendedGamepad {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
