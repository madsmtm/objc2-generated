//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
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
    /// Provides information about the current state of hinting for the focused item.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/uikit/uifocusmovementhint?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIFocusMovementHint;
);

extern_conformance!(
    unsafe impl NSCopying for UIFocusMovementHint {}
);

unsafe impl CopyingHelper for UIFocusMovementHint {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for UIFocusMovementHint {}
);

impl UIFocusMovementHint {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        /// Value between {-1.0, -1.0} and {1.0, 1.0} representing how close focus is to moving in a particular direction.
        #[unsafe(method(movementDirection))]
        #[unsafe(method_family = none)]
        pub unsafe fn movementDirection(&self) -> CGVector;

        #[cfg(feature = "objc2-quartz-core")]
        #[cfg(not(target_os = "watchos"))]
        /// A 3D transform representing the perspective matrix that should be applied to match the system interaction hinting. Assumes a 0..1 near/far plane.
        #[unsafe(method(perspectiveTransform))]
        #[unsafe(method_family = none)]
        pub unsafe fn perspectiveTransform(&self) -> CATransform3D;

        #[cfg(feature = "objc2-core-foundation")]
        /// A vector representing the X and Y axis rotation expressed in radians that should be applied to match the system interaction hinting.
        #[unsafe(method(rotation))]
        #[unsafe(method_family = none)]
        pub unsafe fn rotation(&self) -> CGVector;

        #[cfg(feature = "objc2-core-foundation")]
        /// A vector representing the X and Y axis translation expressed in points that should be applied to match the system interaction hinting.
        #[unsafe(method(translation))]
        #[unsafe(method_family = none)]
        pub unsafe fn translation(&self) -> CGVector;

        #[cfg(feature = "objc2-quartz-core")]
        #[cfg(not(target_os = "watchos"))]
        /// A 3D transform that contains the combined transformations of perspective, rotation and translation.
        #[unsafe(method(interactionTransform))]
        #[unsafe(method_family = none)]
        pub unsafe fn interactionTransform(&self) -> CATransform3D;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
