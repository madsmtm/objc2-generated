//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkmultipolyline?language=objc)
    #[unsafe(super(MKShape, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MKShape")]
    pub struct MKMultiPolyline;
);

#[cfg(all(feature = "MKAnnotation", feature = "MKShape"))]
extern_conformance!(
    unsafe impl MKAnnotation for MKMultiPolyline {}
);

#[cfg(all(feature = "MKAnnotation", feature = "MKOverlay", feature = "MKShape"))]
extern_conformance!(
    unsafe impl MKOverlay for MKMultiPolyline {}
);

#[cfg(feature = "MKShape")]
extern_conformance!(
    unsafe impl NSObjectProtocol for MKMultiPolyline {}
);

#[cfg(feature = "MKShape")]
impl MKMultiPolyline {
    extern_methods!(
        #[cfg(all(feature = "MKMultiPoint", feature = "MKPolyline"))]
        #[unsafe(method(initWithPolylines:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithPolylines(
            this: Allocated<Self>,
            polylines: &NSArray<MKPolyline>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MKMultiPoint", feature = "MKPolyline"))]
        #[unsafe(method(polylines))]
        #[unsafe(method_family = none)]
        pub unsafe fn polylines(&self) -> Retained<NSArray<MKPolyline>>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "MKShape")]
impl MKMultiPolyline {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
