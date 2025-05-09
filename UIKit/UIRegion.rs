//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiregion?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIRegion;
);

extern_conformance!(
    unsafe impl NSCoding for UIRegion {}
);

extern_conformance!(
    unsafe impl NSCopying for UIRegion {}
);

unsafe impl CopyingHelper for UIRegion {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for UIRegion {}
);

impl UIRegion {
    extern_methods!(
        /// A shared infinite region
        #[unsafe(method(infiniteRegion))]
        #[unsafe(method_family = none)]
        pub unsafe fn infiniteRegion(mtm: MainThreadMarker) -> Retained<UIRegion>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Create a circular region with radius
        #[unsafe(method(initWithRadius:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithRadius(this: Allocated<Self>, radius: CGFloat) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Create a rectangular region of size.
        #[unsafe(method(initWithSize:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithSize(this: Allocated<Self>, size: CGSize) -> Retained<Self>;

        /// Create a new region that is the inverse of the current region.
        /// The inverse of the infiniteRegion is an empty region.
        /// Subclasses of UIRegion need to provide an implementation of inverseRegion.
        #[unsafe(method(inverseRegion))]
        #[unsafe(method_family = none)]
        pub unsafe fn inverseRegion(&self) -> Retained<Self>;

        /// Create a new region that is the original region plus the supplied region
        #[unsafe(method(regionByUnionWithRegion:))]
        #[unsafe(method_family = none)]
        pub unsafe fn regionByUnionWithRegion(&self, region: &UIRegion) -> Retained<Self>;

        /// Create a new region that is the original region minus the supplied region
        #[unsafe(method(regionByDifferenceFromRegion:))]
        #[unsafe(method_family = none)]
        pub unsafe fn regionByDifferenceFromRegion(&self, region: &UIRegion) -> Retained<Self>;

        /// Create a new region that is the region covered by the original region and the supplied region
        #[unsafe(method(regionByIntersectionWithRegion:))]
        #[unsafe(method_family = none)]
        pub unsafe fn regionByIntersectionWithRegion(&self, region: &UIRegion) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Test for containment
        #[unsafe(method(containsPoint:))]
        #[unsafe(method_family = none)]
        pub unsafe fn containsPoint(&self, point: CGPoint) -> bool;
    );
}

/// Methods declared on superclass `NSObject`.
impl UIRegion {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
