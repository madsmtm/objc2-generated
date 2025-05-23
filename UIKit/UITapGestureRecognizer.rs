//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitapgesturerecognizer?language=objc)
    #[unsafe(super(UIGestureRecognizer, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIGestureRecognizer")]
    pub struct UITapGestureRecognizer;
);

#[cfg(feature = "UIGestureRecognizer")]
extern_conformance!(
    unsafe impl NSObjectProtocol for UITapGestureRecognizer {}
);

#[cfg(feature = "UIGestureRecognizer")]
impl UITapGestureRecognizer {
    extern_methods!(
        #[unsafe(method(numberOfTapsRequired))]
        #[unsafe(method_family = none)]
        pub fn numberOfTapsRequired(&self) -> NSUInteger;

        /// Setter for [`numberOfTapsRequired`][Self::numberOfTapsRequired].
        #[unsafe(method(setNumberOfTapsRequired:))]
        #[unsafe(method_family = none)]
        pub fn setNumberOfTapsRequired(&self, number_of_taps_required: NSUInteger);

        #[unsafe(method(numberOfTouchesRequired))]
        #[unsafe(method_family = none)]
        pub fn numberOfTouchesRequired(&self) -> NSUInteger;

        /// Setter for [`numberOfTouchesRequired`][Self::numberOfTouchesRequired].
        #[unsafe(method(setNumberOfTouchesRequired:))]
        #[unsafe(method_family = none)]
        pub fn setNumberOfTouchesRequired(&self, number_of_touches_required: NSUInteger);

        #[cfg(feature = "UIEvent")]
        #[unsafe(method(buttonMaskRequired))]
        #[unsafe(method_family = none)]
        pub fn buttonMaskRequired(&self) -> UIEventButtonMask;

        #[cfg(feature = "UIEvent")]
        /// Setter for [`buttonMaskRequired`][Self::buttonMaskRequired].
        #[unsafe(method(setButtonMaskRequired:))]
        #[unsafe(method_family = none)]
        pub fn setButtonMaskRequired(&self, button_mask_required: UIEventButtonMask);
    );
}

/// Methods declared on superclass `UIGestureRecognizer`.
#[cfg(feature = "UIGestureRecognizer")]
impl UITapGestureRecognizer {
    extern_methods!(
        #[unsafe(method(initWithTarget:action:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithTarget_action(
            this: Allocated<Self>,
            target: Option<&AnyObject>,
            action: Option<Sel>,
        ) -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "UIGestureRecognizer")]
impl UITapGestureRecognizer {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
