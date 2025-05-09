//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corelocation/clheadingcomponentvalue?language=objc)
pub type CLHeadingComponentValue = c_double;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corelocation/kclheadingfilternone?language=objc)
    #[cfg(feature = "CLLocation")]
    pub static kCLHeadingFilterNone: CLLocationDegrees;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/corelocation/clheading?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CLHeading;
);

extern_conformance!(
    unsafe impl NSCoding for CLHeading {}
);

extern_conformance!(
    unsafe impl NSCopying for CLHeading {}
);

unsafe impl CopyingHelper for CLHeading {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for CLHeading {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for CLHeading {}
);

impl CLHeading {
    extern_methods!(
        #[cfg(feature = "CLLocation")]
        #[unsafe(method(magneticHeading))]
        #[unsafe(method_family = none)]
        pub unsafe fn magneticHeading(&self) -> CLLocationDirection;

        #[cfg(feature = "CLLocation")]
        #[unsafe(method(trueHeading))]
        #[unsafe(method_family = none)]
        pub unsafe fn trueHeading(&self) -> CLLocationDirection;

        #[cfg(feature = "CLLocation")]
        #[unsafe(method(headingAccuracy))]
        #[unsafe(method_family = none)]
        pub unsafe fn headingAccuracy(&self) -> CLLocationDirection;

        #[unsafe(method(x))]
        #[unsafe(method_family = none)]
        pub unsafe fn x(&self) -> CLHeadingComponentValue;

        #[unsafe(method(y))]
        #[unsafe(method_family = none)]
        pub unsafe fn y(&self) -> CLHeadingComponentValue;

        #[unsafe(method(z))]
        #[unsafe(method_family = none)]
        pub unsafe fn z(&self) -> CLHeadingComponentValue;

        #[unsafe(method(timestamp))]
        #[unsafe(method_family = none)]
        pub unsafe fn timestamp(&self) -> Retained<NSDate>;
    );
}

/// Methods declared on superclass `NSObject`.
impl CLHeading {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
