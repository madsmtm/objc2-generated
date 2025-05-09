//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/corelocation/clcirculargeographiccondition?language=objc)
    #[unsafe(super(CLCondition, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CLCondition")]
    pub struct CLCircularGeographicCondition;
);

#[cfg(feature = "CLCondition")]
extern_conformance!(
    unsafe impl NSCoding for CLCircularGeographicCondition {}
);

#[cfg(feature = "CLCondition")]
extern_conformance!(
    unsafe impl NSCopying for CLCircularGeographicCondition {}
);

#[cfg(feature = "CLCondition")]
unsafe impl CopyingHelper for CLCircularGeographicCondition {
    type Result = Self;
}

#[cfg(feature = "CLCondition")]
extern_conformance!(
    unsafe impl NSObjectProtocol for CLCircularGeographicCondition {}
);

#[cfg(feature = "CLCondition")]
extern_conformance!(
    unsafe impl NSSecureCoding for CLCircularGeographicCondition {}
);

#[cfg(feature = "CLCondition")]
impl CLCircularGeographicCondition {
    extern_methods!(
        #[cfg(feature = "CLLocation")]
        #[unsafe(method(center))]
        #[unsafe(method_family = none)]
        pub unsafe fn center(&self) -> CLLocationCoordinate2D;

        #[cfg(feature = "CLLocation")]
        #[unsafe(method(radius))]
        #[unsafe(method_family = none)]
        pub unsafe fn radius(&self) -> CLLocationDistance;

        #[cfg(feature = "CLLocation")]
        #[unsafe(method(initWithCenter:radius:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCenter_radius(
            this: Allocated<Self>,
            center: CLLocationCoordinate2D,
            radius: CLLocationDistance,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `CLCondition`.
#[cfg(feature = "CLCondition")]
impl CLCircularGeographicCondition {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
