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
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uirotationgesturerecognizer?language=objc)
    #[unsafe(super(UIGestureRecognizer, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIGestureRecognizer")]
    pub struct UIRotationGestureRecognizer;
);

#[cfg(feature = "UIGestureRecognizer")]
extern_conformance!(
    unsafe impl NSObjectProtocol for UIRotationGestureRecognizer {}
);

#[cfg(feature = "UIGestureRecognizer")]
impl UIRotationGestureRecognizer {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(rotation))]
        #[unsafe(method_family = none)]
        pub fn rotation(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`rotation`][Self::rotation].
        #[unsafe(method(setRotation:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRotation(&self, rotation: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(velocity))]
        #[unsafe(method_family = none)]
        pub fn velocity(&self) -> CGFloat;
    );
}

/// Methods declared on superclass `UIGestureRecognizer`.
#[cfg(feature = "UIGestureRecognizer")]
impl UIRotationGestureRecognizer {
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
impl UIRotationGestureRecognizer {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
