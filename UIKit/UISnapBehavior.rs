//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uisnapbehavior?language=objc)
    #[unsafe(super(UIDynamicBehavior, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIDynamicBehavior")]
    pub struct UISnapBehavior;
);

#[cfg(feature = "UIDynamicBehavior")]
extern_conformance!(
    unsafe impl NSObjectProtocol for UISnapBehavior {}
);

#[cfg(feature = "UIDynamicBehavior")]
impl UISnapBehavior {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(initWithItem:snapToPoint:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithItem_snapToPoint(
            this: Allocated<Self>,
            item: &ProtocolObject<dyn UIDynamicItem>,
            point: CGPoint,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(snapPoint))]
        #[unsafe(method_family = none)]
        pub unsafe fn snapPoint(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`snapPoint`][Self::snapPoint].
        #[unsafe(method(setSnapPoint:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSnapPoint(&self, snap_point: CGPoint);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(damping))]
        #[unsafe(method_family = none)]
        pub unsafe fn damping(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`damping`][Self::damping].
        #[unsafe(method(setDamping:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDamping(&self, damping: CGFloat);
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "UIDynamicBehavior")]
impl UISnapBehavior {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
