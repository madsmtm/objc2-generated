//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nspressgesturerecognizer?language=objc)
    #[unsafe(super(NSGestureRecognizer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSGestureRecognizer")]
    pub struct NSPressGestureRecognizer;
);

#[cfg(feature = "NSGestureRecognizer")]
extern_conformance!(
    unsafe impl NSCoding for NSPressGestureRecognizer {}
);

#[cfg(feature = "NSGestureRecognizer")]
extern_conformance!(
    unsafe impl NSObjectProtocol for NSPressGestureRecognizer {}
);

#[cfg(feature = "NSGestureRecognizer")]
impl NSPressGestureRecognizer {
    extern_methods!(
        #[unsafe(method(buttonMask))]
        #[unsafe(method_family = none)]
        pub unsafe fn buttonMask(&self) -> NSUInteger;

        /// Setter for [`buttonMask`][Self::buttonMask].
        #[unsafe(method(setButtonMask:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setButtonMask(&self, button_mask: NSUInteger);

        #[unsafe(method(minimumPressDuration))]
        #[unsafe(method_family = none)]
        pub unsafe fn minimumPressDuration(&self) -> NSTimeInterval;

        /// Setter for [`minimumPressDuration`][Self::minimumPressDuration].
        #[unsafe(method(setMinimumPressDuration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMinimumPressDuration(&self, minimum_press_duration: NSTimeInterval);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(allowableMovement))]
        #[unsafe(method_family = none)]
        pub unsafe fn allowableMovement(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`allowableMovement`][Self::allowableMovement].
        #[unsafe(method(setAllowableMovement:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAllowableMovement(&self, allowable_movement: CGFloat);

        #[unsafe(method(numberOfTouchesRequired))]
        #[unsafe(method_family = none)]
        pub unsafe fn numberOfTouchesRequired(&self) -> NSInteger;

        /// Setter for [`numberOfTouchesRequired`][Self::numberOfTouchesRequired].
        #[unsafe(method(setNumberOfTouchesRequired:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setNumberOfTouchesRequired(&self, number_of_touches_required: NSInteger);
    );
}

/// Methods declared on superclass `NSGestureRecognizer`.
#[cfg(feature = "NSGestureRecognizer")]
impl NSPressGestureRecognizer {
    extern_methods!(
        #[unsafe(method(initWithTarget:action:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithTarget_action(
            this: Allocated<Self>,
            target: Option<&AnyObject>,
            action: Option<Sel>,
        ) -> Retained<Self>;

        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "NSGestureRecognizer")]
impl NSPressGestureRecognizer {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
