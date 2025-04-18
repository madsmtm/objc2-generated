//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uihovergesturerecognizer?language=objc)
    #[unsafe(super(UIGestureRecognizer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIGestureRecognizer")]
    pub struct UIHoverGestureRecognizer;
);

#[cfg(feature = "UIGestureRecognizer")]
extern_conformance!(
    unsafe impl NSObjectProtocol for UIHoverGestureRecognizer {}
);

#[cfg(feature = "UIGestureRecognizer")]
impl UIHoverGestureRecognizer {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        /// The normalized distance from the screen of the hovering device. This value will be 1 at the maximum distance
        /// from the screen and will approach 0 as the device gets closer to the screen. Will always return 0 for devices that
        /// don't support z offset.
        #[unsafe(method(zOffset))]
        #[unsafe(method_family = none)]
        pub unsafe fn zOffset(&self) -> CGFloat;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        /// The azimuth angle of the current device in the specified view, or the gesture recognizer's window if nil. 0 is
        /// returned for devices that don't support azimuth.
        #[unsafe(method(azimuthAngleInView:))]
        #[unsafe(method_family = none)]
        pub unsafe fn azimuthAngleInView(&self, view: Option<&UIView>) -> CGFloat;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        /// The azimuth unit vector of the current device in the specified view, or the gesture recognizer's window if nil.
        /// An empty vector is returned for devices that don't support azimuth.
        #[unsafe(method(azimuthUnitVectorInView:))]
        #[unsafe(method_family = none)]
        pub unsafe fn azimuthUnitVectorInView(&self, view: Option<&UIView>) -> CGVector;

        #[cfg(feature = "objc2-core-foundation")]
        /// The altitude angle of the current device. 0 is returned for devices that don't support altitude.
        #[unsafe(method(altitudeAngle))]
        #[unsafe(method_family = none)]
        pub unsafe fn altitudeAngle(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(rollAngle))]
        #[unsafe(method_family = none)]
        pub unsafe fn rollAngle(&self) -> CGFloat;
    );
}

/// Methods declared on superclass `UIGestureRecognizer`.
#[cfg(feature = "UIGestureRecognizer")]
impl UIHoverGestureRecognizer {
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
impl UIHoverGestureRecognizer {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
