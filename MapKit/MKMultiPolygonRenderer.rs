//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkmultipolygonrenderer?language=objc)
    #[unsafe(super(MKOverlayPathRenderer, MKOverlayRenderer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MKOverlayPathRenderer", feature = "MKOverlayRenderer"))]
    pub struct MKMultiPolygonRenderer;
);

#[cfg(all(feature = "MKOverlayPathRenderer", feature = "MKOverlayRenderer"))]
unsafe impl NSObjectProtocol for MKMultiPolygonRenderer {}

extern_methods!(
    #[cfg(all(feature = "MKOverlayPathRenderer", feature = "MKOverlayRenderer"))]
    unsafe impl MKMultiPolygonRenderer {
        #[cfg(all(feature = "MKMultiPolygon", feature = "MKShape"))]
        #[unsafe(method_family(init))]
        #[method_id(initWithMultiPolygon:)]
        pub unsafe fn initWithMultiPolygon(
            this: Allocated<Self>,
            multi_polygon: &MKMultiPolygon,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MKMultiPolygon", feature = "MKShape"))]
        #[unsafe(method_family(none))]
        #[method_id(multiPolygon)]
        pub unsafe fn multiPolygon(&self) -> Retained<MKMultiPolygon>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MKOverlayRenderer`
    #[cfg(all(feature = "MKOverlayPathRenderer", feature = "MKOverlayRenderer"))]
    unsafe impl MKMultiPolygonRenderer {
        #[cfg(all(feature = "MKAnnotation", feature = "MKOverlay"))]
        #[unsafe(method_family(init))]
        #[method_id(initWithOverlay:)]
        pub unsafe fn initWithOverlay(
            this: Allocated<Self>,
            overlay: &ProtocolObject<dyn MKOverlay>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MKOverlayPathRenderer", feature = "MKOverlayRenderer"))]
    unsafe impl MKMultiPolygonRenderer {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
