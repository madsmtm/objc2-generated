//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
#[cfg(feature = "objc2-core-image")]
#[cfg(not(target_os = "watchos"))]
use objc2_core_image::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/spritekit/sktransitiondirection?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SKTransitionDirection(pub NSInteger);
impl SKTransitionDirection {
    #[doc(alias = "SKTransitionDirectionUp")]
    pub const Up: Self = Self(0);
    #[doc(alias = "SKTransitionDirectionDown")]
    pub const Down: Self = Self(1);
    #[doc(alias = "SKTransitionDirectionRight")]
    pub const Right: Self = Self(2);
    #[doc(alias = "SKTransitionDirectionLeft")]
    pub const Left: Self = Self(3);
}

unsafe impl Encode for SKTransitionDirection {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SKTransitionDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// A transition style from one scene to another.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/spritekit/sktransition?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SKTransition;
);

extern_conformance!(
    unsafe impl NSCopying for SKTransition {}
);

unsafe impl CopyingHelper for SKTransition {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for SKTransition {}
);

impl SKTransition {
    extern_methods!(
        #[unsafe(method(crossFadeWithDuration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn crossFadeWithDuration(sec: NSTimeInterval) -> Retained<SKTransition>;

        #[unsafe(method(fadeWithDuration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn fadeWithDuration(sec: NSTimeInterval) -> Retained<SKTransition>;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[unsafe(method(fadeWithColor:duration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn fadeWithColor_duration(
            color: &NSColor,
            sec: NSTimeInterval,
        ) -> Retained<SKTransition>;

        #[unsafe(method(flipHorizontalWithDuration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn flipHorizontalWithDuration(sec: NSTimeInterval) -> Retained<SKTransition>;

        #[unsafe(method(flipVerticalWithDuration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn flipVerticalWithDuration(sec: NSTimeInterval) -> Retained<SKTransition>;

        #[unsafe(method(revealWithDirection:duration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn revealWithDirection_duration(
            direction: SKTransitionDirection,
            sec: NSTimeInterval,
        ) -> Retained<SKTransition>;

        #[unsafe(method(moveInWithDirection:duration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn moveInWithDirection_duration(
            direction: SKTransitionDirection,
            sec: NSTimeInterval,
        ) -> Retained<SKTransition>;

        #[unsafe(method(pushWithDirection:duration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn pushWithDirection_duration(
            direction: SKTransitionDirection,
            sec: NSTimeInterval,
        ) -> Retained<SKTransition>;

        #[unsafe(method(doorsOpenHorizontalWithDuration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn doorsOpenHorizontalWithDuration(
            sec: NSTimeInterval,
        ) -> Retained<SKTransition>;

        #[unsafe(method(doorsOpenVerticalWithDuration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn doorsOpenVerticalWithDuration(sec: NSTimeInterval) -> Retained<SKTransition>;

        #[unsafe(method(doorsCloseHorizontalWithDuration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn doorsCloseHorizontalWithDuration(
            sec: NSTimeInterval,
        ) -> Retained<SKTransition>;

        #[unsafe(method(doorsCloseVerticalWithDuration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn doorsCloseVerticalWithDuration(sec: NSTimeInterval)
            -> Retained<SKTransition>;

        #[unsafe(method(doorwayWithDuration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn doorwayWithDuration(sec: NSTimeInterval) -> Retained<SKTransition>;

        #[cfg(feature = "objc2-core-image")]
        #[cfg(not(target_os = "watchos"))]
        #[unsafe(method(transitionWithCIFilter:duration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn transitionWithCIFilter_duration(
            filter: &CIFilter,
            sec: NSTimeInterval,
        ) -> Retained<SKTransition>;

        /// Pause the incoming Scene during the transition, defaults to YES.
        #[unsafe(method(pausesIncomingScene))]
        #[unsafe(method_family = none)]
        pub unsafe fn pausesIncomingScene(&self) -> bool;

        /// Setter for [`pausesIncomingScene`][Self::pausesIncomingScene].
        #[unsafe(method(setPausesIncomingScene:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPausesIncomingScene(&self, pauses_incoming_scene: bool);

        /// Pause the outgoing Scene during the transition, defaults to YES.
        #[unsafe(method(pausesOutgoingScene))]
        #[unsafe(method_family = none)]
        pub unsafe fn pausesOutgoingScene(&self) -> bool;

        /// Setter for [`pausesOutgoingScene`][Self::pausesOutgoingScene].
        #[unsafe(method(setPausesOutgoingScene:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPausesOutgoingScene(&self, pauses_outgoing_scene: bool);
    );
}

/// Methods declared on superclass `NSObject`.
impl SKTransition {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
