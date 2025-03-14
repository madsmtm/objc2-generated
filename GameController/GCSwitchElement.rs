//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_protocol!(
    /// An object conforming to
    /// `GCSwitchElement`represents a latching switch.
    /// A switch may be in one of several positions, and remains in its last position
    /// after the user stops interacting with it.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcswitchelement?language=objc)
    #[cfg(feature = "GCPhysicalInputElement")]
    pub unsafe trait GCSwitchElement: GCPhysicalInputElement {
        #[cfg(feature = "GCSwitchPositionInput")]
        /// Get the input containing the absolute position of the switch.
        #[unsafe(method(positionInput))]
        #[unsafe(method_family = none)]
        unsafe fn positionInput(&self) -> Retained<ProtocolObject<dyn GCSwitchPositionInput>>;
    }
);
