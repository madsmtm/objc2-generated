//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcsteeringwheelelement?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GCSteeringWheelElement;
);

#[cfg(all(feature = "GCAxisElement", feature = "GCPhysicalInputElement"))]
extern_conformance!(
    unsafe impl GCAxisElement for GCSteeringWheelElement {}
);

#[cfg(feature = "GCPhysicalInputElement")]
extern_conformance!(
    unsafe impl GCPhysicalInputElement for GCSteeringWheelElement {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for GCSteeringWheelElement {}
);

impl GCSteeringWheelElement {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(maximumDegreesOfRotation))]
        #[unsafe(method_family = none)]
        pub unsafe fn maximumDegreesOfRotation(&self) -> c_float;
    );
}

/// Methods declared on superclass `NSObject`.
impl GCSteeringWheelElement {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
