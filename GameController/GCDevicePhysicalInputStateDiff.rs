//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcdevicephysicalinputelementchange?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GCDevicePhysicalInputElementChange(pub NSInteger);
impl GCDevicePhysicalInputElementChange {
    /// Indicates that a change could not be determined.  This is typically
    /// because the input state queue filled up and older input state snapshots
    /// were dropped.
    #[doc(alias = "GCDevicePhysicalInputElementUnknownChange")]
    pub const UnknownChange: Self = Self(-1);
    /// Indicates that no value of the element changed.
    #[doc(alias = "GCDevicePhysicalInputElementNoChange")]
    pub const NoChange: Self = Self(0);
    /// Indicates that a value of the element changed.
    #[doc(alias = "GCDevicePhysicalInputElementChanged")]
    pub const Changed: Self = Self(1);
}

unsafe impl Encode for GCDevicePhysicalInputElementChange {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for GCDevicePhysicalInputElementChange {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// An object conforming to the
    /// `GCDevicePhysicalInputStateDiff`protocol
    /// contains the input state differences between the current and previous
    /// `GCDevicePhysicalInputState`objects returned from the
    /// `-nextInputState`method of
    /// `GCDevicePhysicalInput.`
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcdevicephysicalinputstatediff?language=objc)
    pub unsafe trait GCDevicePhysicalInputStateDiff: NSObjectProtocol {
        #[cfg(feature = "GCPhysicalInputElement")]
        /// Check if a value of
        /// _element_changed, compared the previous input state.
        ///
        ///
        /// Parameter `element`: The element to check.  This may be a pointer to either the "live" element
        /// from the device's physical input, or a pointer to an element from any input
        /// state "snapshot" of the device's physical input.
        #[unsafe(method(changeForElement:))]
        #[unsafe(method_family = none)]
        unsafe fn changeForElement(
            &self,
            element: &ProtocolObject<dyn GCPhysicalInputElement>,
        ) -> GCDevicePhysicalInputElementChange;

        #[cfg(feature = "GCPhysicalInputElement")]
        /// Gets an enumerator that iterates over the elements that have changed, compared
        /// the previous input state.
        ///
        /// This method returns
        /// `nil`if the changed elements could not be determined -
        /// typically because the input state queue filled up and older input state
        /// snapshots were dropped.
        #[unsafe(method(changedElements))]
        #[unsafe(method_family = none)]
        unsafe fn changedElements(
            &self,
        ) -> Option<Retained<NSEnumerator<ProtocolObject<dyn GCPhysicalInputElement>>>>;
    }
);
