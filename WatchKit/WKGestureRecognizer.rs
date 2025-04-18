//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkgesturerecognizerstate?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WKGestureRecognizerState(pub NSInteger);
impl WKGestureRecognizerState {
    #[doc(alias = "WKGestureRecognizerStatePossible")]
    pub const Possible: Self = Self(0);
    #[doc(alias = "WKGestureRecognizerStateBegan")]
    pub const Began: Self = Self(1);
    #[doc(alias = "WKGestureRecognizerStateChanged")]
    pub const Changed: Self = Self(2);
    #[doc(alias = "WKGestureRecognizerStateEnded")]
    pub const Ended: Self = Self(3);
    #[doc(alias = "WKGestureRecognizerStateCancelled")]
    pub const Cancelled: Self = Self(4);
    #[doc(alias = "WKGestureRecognizerStateFailed")]
    pub const Failed: Self = Self(5);
    #[doc(alias = "WKGestureRecognizerStateRecognized")]
    pub const Recognized: Self = Self(6);
}

unsafe impl Encode for WKGestureRecognizerState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for WKGestureRecognizerState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkswipegesturerecognizerdirection?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WKSwipeGestureRecognizerDirection(pub NSUInteger);
bitflags::bitflags! {
    impl WKSwipeGestureRecognizerDirection: NSUInteger {
        #[doc(alias = "WKSwipeGestureRecognizerDirectionRight")]
        const Right = 1<<0;
        #[doc(alias = "WKSwipeGestureRecognizerDirectionLeft")]
        const Left = 1<<1;
        #[doc(alias = "WKSwipeGestureRecognizerDirectionUp")]
        const Up = 1<<2;
        #[doc(alias = "WKSwipeGestureRecognizerDirectionDown")]
        const Down = 1<<3;
    }
}

unsafe impl Encode for WKSwipeGestureRecognizerDirection {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for WKSwipeGestureRecognizerDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkgesturerecognizer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKGestureRecognizer;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for WKGestureRecognizer {}
);

impl WKGestureRecognizer {
    extern_methods!(
        #[unsafe(method(state))]
        #[unsafe(method_family = none)]
        pub unsafe fn state(&self) -> WKGestureRecognizerState;

        #[unsafe(method(isEnabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn isEnabled(&self) -> bool;

        /// Setter for [`isEnabled`][Self::isEnabled].
        #[unsafe(method(setEnabled:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(locationInObject))]
        #[unsafe(method_family = none)]
        pub unsafe fn locationInObject(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(objectBounds))]
        #[unsafe(method_family = none)]
        pub unsafe fn objectBounds(&self) -> CGRect;
    );
}

/// Methods declared on superclass `NSObject`.
impl WKGestureRecognizer {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wktapgesturerecognizer?language=objc)
    #[unsafe(super(WKGestureRecognizer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKTapGestureRecognizer;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for WKTapGestureRecognizer {}
);

impl WKTapGestureRecognizer {
    extern_methods!(
        #[unsafe(method(numberOfTapsRequired))]
        #[unsafe(method_family = none)]
        pub unsafe fn numberOfTapsRequired(&self) -> NSUInteger;

        /// Setter for [`numberOfTapsRequired`][Self::numberOfTapsRequired].
        #[unsafe(method(setNumberOfTapsRequired:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setNumberOfTapsRequired(&self, number_of_taps_required: NSUInteger);
    );
}

/// Methods declared on superclass `NSObject`.
impl WKTapGestureRecognizer {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wklongpressgesturerecognizer?language=objc)
    #[unsafe(super(WKGestureRecognizer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKLongPressGestureRecognizer;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for WKLongPressGestureRecognizer {}
);

impl WKLongPressGestureRecognizer {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(minimumPressDuration))]
        #[unsafe(method_family = none)]
        pub unsafe fn minimumPressDuration(&self) -> CFTimeInterval;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`minimumPressDuration`][Self::minimumPressDuration].
        #[unsafe(method(setMinimumPressDuration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMinimumPressDuration(&self, minimum_press_duration: CFTimeInterval);

        #[unsafe(method(numberOfTapsRequired))]
        #[unsafe(method_family = none)]
        pub unsafe fn numberOfTapsRequired(&self) -> NSUInteger;

        /// Setter for [`numberOfTapsRequired`][Self::numberOfTapsRequired].
        #[unsafe(method(setNumberOfTapsRequired:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setNumberOfTapsRequired(&self, number_of_taps_required: NSUInteger);

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(allowableMovement))]
        #[unsafe(method_family = none)]
        pub unsafe fn allowableMovement(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`allowableMovement`][Self::allowableMovement].
        #[unsafe(method(setAllowableMovement:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAllowableMovement(&self, allowable_movement: CGFloat);
    );
}

/// Methods declared on superclass `NSObject`.
impl WKLongPressGestureRecognizer {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkswipegesturerecognizer?language=objc)
    #[unsafe(super(WKGestureRecognizer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKSwipeGestureRecognizer;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for WKSwipeGestureRecognizer {}
);

impl WKSwipeGestureRecognizer {
    extern_methods!(
        #[unsafe(method(direction))]
        #[unsafe(method_family = none)]
        pub unsafe fn direction(&self) -> WKSwipeGestureRecognizerDirection;

        /// Setter for [`direction`][Self::direction].
        #[unsafe(method(setDirection:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDirection(&self, direction: WKSwipeGestureRecognizerDirection);
    );
}

/// Methods declared on superclass `NSObject`.
impl WKSwipeGestureRecognizer {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkpangesturerecognizer?language=objc)
    #[unsafe(super(WKGestureRecognizer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKPanGestureRecognizer;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for WKPanGestureRecognizer {}
);

impl WKPanGestureRecognizer {
    extern_methods!(
        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(translationInObject))]
        #[unsafe(method_family = none)]
        pub unsafe fn translationInObject(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(velocityInObject))]
        #[unsafe(method_family = none)]
        pub unsafe fn velocityInObject(&self) -> CGPoint;
    );
}

/// Methods declared on superclass `NSObject`.
impl WKPanGestureRecognizer {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
