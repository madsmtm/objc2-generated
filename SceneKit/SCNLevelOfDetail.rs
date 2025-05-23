//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// SCNLevelOfDetail represents a level of detail of a geometry.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/scenekit/scnlevelofdetail?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SCNLevelOfDetail;
);

extern_conformance!(
    unsafe impl NSCoding for SCNLevelOfDetail {}
);

extern_conformance!(
    unsafe impl NSCopying for SCNLevelOfDetail {}
);

unsafe impl CopyingHelper for SCNLevelOfDetail {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for SCNLevelOfDetail {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for SCNLevelOfDetail {}
);

impl SCNLevelOfDetail {
    extern_methods!(
        #[cfg(all(feature = "SCNGeometry", feature = "objc2-core-foundation"))]
        /// This is a convenience method to create a level of detail with a coverage radius threshold mode.
        ///
        /// Parameter `geometry`: The geometry for this level of detail. nil is supported and indicates that no geometry should be rendered for this level of detail.
        ///
        /// Parameter `radius`: The maximum radius in screen-space that this level of detail is suitable for. The coverage radius is calculated from the projected bounding sphere and expressed in pixels.
        #[unsafe(method(levelOfDetailWithGeometry:screenSpaceRadius:))]
        #[unsafe(method_family = none)]
        pub unsafe fn levelOfDetailWithGeometry_screenSpaceRadius(
            geometry: Option<&SCNGeometry>,
            radius: CGFloat,
        ) -> Retained<Self>;

        #[cfg(all(feature = "SCNGeometry", feature = "objc2-core-foundation"))]
        /// This is a convenience method to create a level of detail with a distance threshold mode.
        ///
        /// Parameter `geometry`: The geometry for this level of detail. nil is supported and indicates that no geometry should be rendered for this level of detail.
        ///
        /// Parameter `distance`: The minimum distance to the current point of view that this level of detail is suitable for.
        #[unsafe(method(levelOfDetailWithGeometry:worldSpaceDistance:))]
        #[unsafe(method_family = none)]
        pub unsafe fn levelOfDetailWithGeometry_worldSpaceDistance(
            geometry: Option<&SCNGeometry>,
            distance: CGFloat,
        ) -> Retained<Self>;

        #[cfg(feature = "SCNGeometry")]
        /// Returns the geometry of the receiver.
        #[unsafe(method(geometry))]
        #[unsafe(method_family = none)]
        pub unsafe fn geometry(&self) -> Option<Retained<SCNGeometry>>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Returns the screen space radius of the receiver if any, 0 otherwise.
        #[unsafe(method(screenSpaceRadius))]
        #[unsafe(method_family = none)]
        pub unsafe fn screenSpaceRadius(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Returns the world space distance of the receiver if any, 0 otherwise.
        #[unsafe(method(worldSpaceDistance))]
        #[unsafe(method_family = none)]
        pub unsafe fn worldSpaceDistance(&self) -> CGFloat;
    );
}

/// Methods declared on superclass `NSObject`.
impl SCNLevelOfDetail {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
