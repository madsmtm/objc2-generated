//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkcrownsequencer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKCrownSequencer;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for WKCrownSequencer {}
);

impl WKCrownSequencer {
    extern_methods!(
        #[unsafe(method(delegate))]
        #[unsafe(method_family = none)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn WKCrownDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[unsafe(method(setDelegate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn WKCrownDelegate>>);

        #[unsafe(method(rotationsPerSecond))]
        #[unsafe(method_family = none)]
        pub unsafe fn rotationsPerSecond(&self) -> c_double;

        #[unsafe(method(isIdle))]
        #[unsafe(method_family = none)]
        pub unsafe fn isIdle(&self) -> bool;

        #[unsafe(method(isHapticFeedbackEnabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn isHapticFeedbackEnabled(&self) -> bool;

        /// Setter for [`isHapticFeedbackEnabled`][Self::isHapticFeedbackEnabled].
        #[unsafe(method(setHapticFeedbackEnabled:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setHapticFeedbackEnabled(&self, haptic_feedback_enabled: bool);

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(focus))]
        #[unsafe(method_family = none)]
        pub unsafe fn focus(&self);

        #[unsafe(method(resignFocus))]
        #[unsafe(method_family = none)]
        pub unsafe fn resignFocus(&self);
    );
}

/// Methods declared on superclass `NSObject`.
impl WKCrownSequencer {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkcrowndelegate?language=objc)
    pub unsafe trait WKCrownDelegate: NSObjectProtocol {
        #[optional]
        #[unsafe(method(crownDidRotate:rotationalDelta:))]
        #[unsafe(method_family = none)]
        unsafe fn crownDidRotate_rotationalDelta(
            &self,
            crown_sequencer: Option<&WKCrownSequencer>,
            rotational_delta: c_double,
        );

        #[optional]
        #[unsafe(method(crownDidBecomeIdle:))]
        #[unsafe(method_family = none)]
        unsafe fn crownDidBecomeIdle(&self, crown_sequencer: Option<&WKCrownSequencer>);
    }
);
