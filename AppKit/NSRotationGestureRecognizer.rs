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
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsrotationgesturerecognizer?language=objc)
    #[unsafe(super(NSGestureRecognizer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSGestureRecognizer")]
    pub struct NSRotationGestureRecognizer;
);

#[cfg(feature = "NSGestureRecognizer")]
extern_conformance!(
    unsafe impl NSCoding for NSRotationGestureRecognizer {}
);

#[cfg(feature = "NSGestureRecognizer")]
extern_conformance!(
    unsafe impl NSObjectProtocol for NSRotationGestureRecognizer {}
);

#[cfg(feature = "NSGestureRecognizer")]
impl NSRotationGestureRecognizer {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(rotation))]
        #[unsafe(method_family = none)]
        pub unsafe fn rotation(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`rotation`][Self::rotation].
        #[unsafe(method(setRotation:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRotation(&self, rotation: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(rotationInDegrees))]
        #[unsafe(method_family = none)]
        pub unsafe fn rotationInDegrees(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`rotationInDegrees`][Self::rotationInDegrees].
        #[unsafe(method(setRotationInDegrees:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRotationInDegrees(&self, rotation_in_degrees: CGFloat);
    );
}

/// Methods declared on superclass `NSGestureRecognizer`.
#[cfg(feature = "NSGestureRecognizer")]
impl NSRotationGestureRecognizer {
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
impl NSRotationGestureRecognizer {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
