//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// An object conforming to
    /// `GCTouchedStateInput`represents the touched state of
    /// an element.
    ///
    /// Some buttons feature capacitive touch capabilities, where the user can touch
    /// the button without pressing it.  In such cases, a button can be touched without
    /// being pressed.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gctouchedstateinput?language=objc)
    pub unsafe trait GCTouchedStateInput: NSObjectProtocol {
        #[cfg(all(feature = "GCPhysicalInputElement", feature = "block2"))]
        /// Set this block if you want to be notified when the touched state changes.
        #[unsafe(method(touchedDidChangeHandler))]
        #[unsafe(method_family = none)]
        unsafe fn touchedDidChangeHandler(
            &self,
        ) -> *mut block2::DynBlock<
            dyn Fn(
                NonNull<ProtocolObject<dyn GCPhysicalInputElement>>,
                NonNull<ProtocolObject<dyn GCTouchedStateInput>>,
                Bool,
            ),
        >;

        #[cfg(all(feature = "GCPhysicalInputElement", feature = "block2"))]
        /// Setter for [`touchedDidChangeHandler`][Self::touchedDidChangeHandler].
        #[unsafe(method(setTouchedDidChangeHandler:))]
        #[unsafe(method_family = none)]
        unsafe fn setTouchedDidChangeHandler(
            &self,
            touched_did_change_handler: Option<
                &block2::DynBlock<
                    dyn Fn(
                        NonNull<ProtocolObject<dyn GCPhysicalInputElement>>,
                        NonNull<ProtocolObject<dyn GCTouchedStateInput>>,
                        Bool,
                    ),
                >,
            >,
        );

        /// Some buttons feature capacitive touch capabilities, where the user can touch
        /// the button without pressing it. In such cases, a button will be touched before
        /// it is pressed.
        ///
        ///
        /// See: touchedDidChangeHandler
        ///
        /// See: GCPressedStateInput
        #[unsafe(method(isTouched))]
        #[unsafe(method_family = none)]
        unsafe fn isTouched(&self) -> bool;

        /// The timestamp of the last touched state change.
        ///
        /// This time interval is not relative to any specific point in time.  You can
        /// subtract a previous timestamp from the returned timestamp to determine the time
        /// (in seconds) between changes to the value.
        #[unsafe(method(lastTouchedStateTimestamp))]
        #[unsafe(method_family = none)]
        unsafe fn lastTouchedStateTimestamp(&self) -> NSTimeInterval;

        /// The interval (in seconds) between the timestamp of the last touched state
        /// change and the current time.
        ///
        /// This should be treated as a lower bound of the event latency.  It may not
        /// include (wired or wireless) transmission latency, or latency accrued on
        /// the device before the event was transmitted to the host.
        #[unsafe(method(lastTouchedStateLatency))]
        #[unsafe(method_family = none)]
        unsafe fn lastTouchedStateLatency(&self) -> NSTimeInterval;

        #[cfg(feature = "GCPhysicalInputSource")]
        /// An object describing the physical action(s) the user performs to manipulate
        /// this input.
        #[unsafe(method(sources))]
        #[unsafe(method_family = none)]
        unsafe fn sources(&self) -> Retained<NSSet<ProtocolObject<dyn GCPhysicalInputSource>>>;
    }
);
