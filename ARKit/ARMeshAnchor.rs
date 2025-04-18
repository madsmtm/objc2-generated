//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-foundation")]
use objc2_foundation::*;

use crate::*;

#[cfg(feature = "objc2")]
extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/arkit/armeshanchor?language=objc)
    #[unsafe(super(ARAnchor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "ARAnchor", feature = "objc2"))]
    pub struct ARMeshAnchor;
);

#[cfg(all(feature = "ARAnchor", feature = "objc2"))]
unsafe impl Send for ARMeshAnchor {}

#[cfg(all(feature = "ARAnchor", feature = "objc2"))]
unsafe impl Sync for ARMeshAnchor {}

#[cfg(all(feature = "ARAnchor", feature = "objc2", feature = "objc2-foundation"))]
extern_conformance!(
    unsafe impl ARAnchorCopying for ARMeshAnchor {}
);

#[cfg(all(feature = "ARAnchor", feature = "objc2", feature = "objc2-foundation"))]
extern_conformance!(
    unsafe impl NSCoding for ARMeshAnchor {}
);

#[cfg(all(feature = "ARAnchor", feature = "objc2", feature = "objc2-foundation"))]
extern_conformance!(
    unsafe impl NSCopying for ARMeshAnchor {}
);

#[cfg(all(feature = "ARAnchor", feature = "objc2", feature = "objc2-foundation"))]
unsafe impl CopyingHelper for ARMeshAnchor {
    type Result = Self;
}

#[cfg(all(feature = "ARAnchor", feature = "objc2"))]
extern_conformance!(
    unsafe impl NSObjectProtocol for ARMeshAnchor {}
);

#[cfg(all(feature = "ARAnchor", feature = "objc2", feature = "objc2-foundation"))]
extern_conformance!(
    unsafe impl NSSecureCoding for ARMeshAnchor {}
);

#[cfg(all(feature = "ARAnchor", feature = "objc2"))]
impl ARMeshAnchor {
    extern_methods!(
        #[cfg(feature = "ARMeshGeometry")]
        /// Geometry of the mesh in anchor's coordinate system.
        #[unsafe(method(geometry))]
        #[unsafe(method_family = none)]
        pub unsafe fn geometry(&self) -> Retained<ARMeshGeometry>;
    );
}

/// Methods declared on superclass `ARAnchor`.
#[cfg(all(feature = "ARAnchor", feature = "objc2"))]
impl ARMeshAnchor {
    extern_methods!(
        /// Unavailable
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
