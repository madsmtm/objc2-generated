//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uistepper?language=objc)
    #[unsafe(super(UIControl, UIView, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    pub struct UIStepper;
);

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
unsafe impl CALayerDelegate for UIStepper {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl NSCoding for UIStepper {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl NSObjectProtocol for UIStepper {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIAppearance for UIStepper {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIAppearanceContainer for UIStepper {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl UICoordinateSpace for UIStepper {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIDynamicItem for UIStepper {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIFocusEnvironment for UIStepper {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIFocusItem for UIStepper {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIFocusItemContainer for UIStepper {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIResponderStandardEditActions for UIStepper {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
unsafe impl UITraitEnvironment for UIStepper {}

extern_methods!(
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIStepper {
        #[method(isContinuous)]
        pub unsafe fn isContinuous(&self) -> bool;

        /// Setter for [`isContinuous`][Self::isContinuous].
        #[method(setContinuous:)]
        pub unsafe fn setContinuous(&self, continuous: bool);

        #[method(autorepeat)]
        pub unsafe fn autorepeat(&self) -> bool;

        /// Setter for [`autorepeat`][Self::autorepeat].
        #[method(setAutorepeat:)]
        pub unsafe fn setAutorepeat(&self, autorepeat: bool);

        #[method(wraps)]
        pub unsafe fn wraps(&self) -> bool;

        /// Setter for [`wraps`][Self::wraps].
        #[method(setWraps:)]
        pub unsafe fn setWraps(&self, wraps: bool);

        #[method(value)]
        pub unsafe fn value(&self) -> c_double;

        /// Setter for [`value`][Self::value].
        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: c_double);

        #[method(minimumValue)]
        pub unsafe fn minimumValue(&self) -> c_double;

        /// Setter for [`minimumValue`][Self::minimumValue].
        #[method(setMinimumValue:)]
        pub unsafe fn setMinimumValue(&self, minimum_value: c_double);

        #[method(maximumValue)]
        pub unsafe fn maximumValue(&self) -> c_double;

        /// Setter for [`maximumValue`][Self::maximumValue].
        #[method(setMaximumValue:)]
        pub unsafe fn setMaximumValue(&self, maximum_value: c_double);

        #[method(stepValue)]
        pub unsafe fn stepValue(&self) -> c_double;

        /// Setter for [`stepValue`][Self::stepValue].
        #[method(setStepValue:)]
        pub unsafe fn setStepValue(&self, step_value: c_double);

        #[cfg(feature = "UIImage")]
        #[method(setBackgroundImage:forState:)]
        pub unsafe fn setBackgroundImage_forState(
            &self,
            image: Option<&UIImage>,
            state: UIControlState,
        );

        #[cfg(feature = "UIImage")]
        #[unsafe(method_family(none))]
        #[method_id(backgroundImageForState:)]
        pub unsafe fn backgroundImageForState(
            &self,
            state: UIControlState,
        ) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method(setDividerImage:forLeftSegmentState:rightSegmentState:)]
        pub unsafe fn setDividerImage_forLeftSegmentState_rightSegmentState(
            &self,
            image: Option<&UIImage>,
            left_state: UIControlState,
            right_state: UIControlState,
        );

        #[cfg(feature = "UIImage")]
        #[method(setIncrementImage:forState:)]
        pub unsafe fn setIncrementImage_forState(
            &self,
            image: Option<&UIImage>,
            state: UIControlState,
        );

        #[cfg(feature = "UIImage")]
        #[unsafe(method_family(none))]
        #[method_id(incrementImageForState:)]
        pub unsafe fn incrementImageForState(
            &self,
            state: UIControlState,
        ) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method(setDecrementImage:forState:)]
        pub unsafe fn setDecrementImage_forState(
            &self,
            image: Option<&UIImage>,
            state: UIControlState,
        );

        #[cfg(feature = "UIImage")]
        #[unsafe(method_family(none))]
        #[method_id(decrementImageForState:)]
        pub unsafe fn decrementImageForState(
            &self,
            state: UIControlState,
        ) -> Option<Retained<UIImage>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIControl`
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIStepper {
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method_family(init))]
        #[method_id(initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[cfg(all(
            feature = "UIAction",
            feature = "UIMenuElement",
            feature = "objc2-core-foundation"
        ))]
        /// Initializes the control and adds primaryAction for the UIControlEventPrimaryActionTriggered control event. Subclasses of UIControl may alter or add behaviors around the usage of primaryAction, see subclass documentation of this initializer for additional information.
        #[unsafe(method_family(init))]
        #[method_id(initWithFrame:primaryAction:)]
        pub unsafe fn initWithFrame_primaryAction(
            this: Allocated<Self>,
            frame: CGRect,
            primary_action: Option<&UIAction>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIStepper {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
