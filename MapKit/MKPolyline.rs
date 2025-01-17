//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkpolyline?language=objc)
    #[unsafe(super(MKMultiPoint, MKShape, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MKMultiPoint", feature = "MKShape"))]
    pub struct MKPolyline;
);

#[cfg(all(
    feature = "MKAnnotation",
    feature = "MKMultiPoint",
    feature = "MKShape"
))]
unsafe impl MKAnnotation for MKPolyline {}

#[cfg(all(
    feature = "MKAnnotation",
    feature = "MKMultiPoint",
    feature = "MKOverlay",
    feature = "MKShape"
))]
unsafe impl MKOverlay for MKPolyline {}

#[cfg(all(feature = "MKMultiPoint", feature = "MKShape"))]
unsafe impl NSObjectProtocol for MKPolyline {}

extern_methods!(
    #[cfg(all(feature = "MKMultiPoint", feature = "MKShape"))]
    unsafe impl MKPolyline {
        #[cfg(feature = "MKGeometry")]
        #[unsafe(method_family(none))]
        #[method_id(polylineWithPoints:count:)]
        pub unsafe fn polylineWithPoints_count(
            points: NonNull<MKMapPoint>,
            count: NSUInteger,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-location")]
        #[unsafe(method_family(none))]
        #[method_id(polylineWithCoordinates:count:)]
        pub unsafe fn polylineWithCoordinates_count(
            coords: NonNull<CLLocationCoordinate2D>,
            count: NSUInteger,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MKMultiPoint", feature = "MKShape"))]
    unsafe impl MKPolyline {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
