//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkmapsnapshotoptions?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKMapSnapshotOptions;
);

extern_conformance!(
    unsafe impl NSCopying for MKMapSnapshotOptions {}
);

unsafe impl CopyingHelper for MKMapSnapshotOptions {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MKMapSnapshotOptions {}
);

impl MKMapSnapshotOptions {
    extern_methods!(
        #[cfg(feature = "MKMapConfiguration")]
        #[unsafe(method(preferredConfiguration))]
        #[unsafe(method_family = none)]
        pub unsafe fn preferredConfiguration(&self) -> Retained<MKMapConfiguration>;

        #[cfg(feature = "MKMapConfiguration")]
        /// Setter for [`preferredConfiguration`][Self::preferredConfiguration].
        #[unsafe(method(setPreferredConfiguration:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPreferredConfiguration(
            &self,
            preferred_configuration: &MKMapConfiguration,
        );

        #[cfg(feature = "MKMapCamera")]
        #[unsafe(method(camera))]
        #[unsafe(method_family = none)]
        pub unsafe fn camera(&self) -> Retained<MKMapCamera>;

        #[cfg(feature = "MKMapCamera")]
        /// Setter for [`camera`][Self::camera].
        #[unsafe(method(setCamera:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCamera(&self, camera: &MKMapCamera);

        #[cfg(feature = "MKGeometry")]
        #[unsafe(method(mapRect))]
        #[unsafe(method_family = none)]
        pub unsafe fn mapRect(&self) -> MKMapRect;

        #[cfg(feature = "MKGeometry")]
        /// Setter for [`mapRect`][Self::mapRect].
        #[unsafe(method(setMapRect:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMapRect(&self, map_rect: MKMapRect);

        #[cfg(all(feature = "MKGeometry", feature = "objc2-core-location"))]
        #[unsafe(method(region))]
        #[unsafe(method_family = none)]
        pub unsafe fn region(&self) -> MKCoordinateRegion;

        #[cfg(all(feature = "MKGeometry", feature = "objc2-core-location"))]
        /// Setter for [`region`][Self::region].
        #[unsafe(method(setRegion:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRegion(&self, region: MKCoordinateRegion);

        #[cfg(feature = "MKTypes")]
        #[deprecated = "Use preferredConfiguration"]
        #[unsafe(method(mapType))]
        #[unsafe(method_family = none)]
        pub unsafe fn mapType(&self) -> MKMapType;

        #[cfg(feature = "MKTypes")]
        /// Setter for [`mapType`][Self::mapType].
        #[deprecated = "Use preferredConfiguration"]
        #[unsafe(method(setMapType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMapType(&self, map_type: MKMapType);

        #[cfg(feature = "MKPointOfInterestFilter")]
        #[deprecated = "Use preferredConfiguration"]
        #[unsafe(method(pointOfInterestFilter))]
        #[unsafe(method_family = none)]
        pub unsafe fn pointOfInterestFilter(&self) -> Option<Retained<MKPointOfInterestFilter>>;

        #[cfg(feature = "MKPointOfInterestFilter")]
        /// Setter for [`pointOfInterestFilter`][Self::pointOfInterestFilter].
        #[deprecated = "Use preferredConfiguration"]
        #[unsafe(method(setPointOfInterestFilter:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPointOfInterestFilter(
            &self,
            point_of_interest_filter: Option<&MKPointOfInterestFilter>,
        );

        #[deprecated = "Use preferredConfiguration"]
        #[unsafe(method(showsPointsOfInterest))]
        #[unsafe(method_family = none)]
        pub unsafe fn showsPointsOfInterest(&self) -> bool;

        /// Setter for [`showsPointsOfInterest`][Self::showsPointsOfInterest].
        #[deprecated = "Use preferredConfiguration"]
        #[unsafe(method(setShowsPointsOfInterest:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setShowsPointsOfInterest(&self, shows_points_of_interest: bool);

        #[deprecated = "No longer supported."]
        #[unsafe(method(showsBuildings))]
        #[unsafe(method_family = none)]
        pub unsafe fn showsBuildings(&self) -> bool;

        /// Setter for [`showsBuildings`][Self::showsBuildings].
        #[deprecated = "No longer supported."]
        #[unsafe(method(setShowsBuildings:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setShowsBuildings(&self, shows_buildings: bool);

        #[unsafe(method(size))]
        #[unsafe(method_family = none)]
        pub unsafe fn size(&self) -> NSSize;

        /// Setter for [`size`][Self::size].
        #[unsafe(method(setSize:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSize(&self, size: NSSize);

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[unsafe(method(appearance))]
        #[unsafe(method_family = none)]
        pub unsafe fn appearance(&self) -> Option<Retained<NSAppearance>>;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        /// Setter for [`appearance`][Self::appearance].
        #[unsafe(method(setAppearance:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAppearance(&self, appearance: Option<&NSAppearance>);
    );
}

/// Methods declared on superclass `NSObject`.
impl MKMapSnapshotOptions {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
