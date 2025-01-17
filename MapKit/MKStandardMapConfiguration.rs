//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkstandardmapemphasisstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MKStandardMapEmphasisStyle(pub NSInteger);
impl MKStandardMapEmphasisStyle {
    #[doc(alias = "MKStandardMapEmphasisStyleDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "MKStandardMapEmphasisStyleMuted")]
    pub const Muted: Self = Self(1);
}

unsafe impl Encode for MKStandardMapEmphasisStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MKStandardMapEmphasisStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkstandardmapconfiguration?language=objc)
    #[unsafe(super(MKMapConfiguration, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MKMapConfiguration")]
    pub struct MKStandardMapConfiguration;
);

#[cfg(feature = "MKMapConfiguration")]
unsafe impl NSCoding for MKStandardMapConfiguration {}

#[cfg(feature = "MKMapConfiguration")]
unsafe impl NSCopying for MKStandardMapConfiguration {}

#[cfg(feature = "MKMapConfiguration")]
unsafe impl CopyingHelper for MKStandardMapConfiguration {
    type Result = Self;
}

#[cfg(feature = "MKMapConfiguration")]
unsafe impl NSObjectProtocol for MKStandardMapConfiguration {}

#[cfg(feature = "MKMapConfiguration")]
unsafe impl NSSecureCoding for MKStandardMapConfiguration {}

extern_methods!(
    #[cfg(feature = "MKMapConfiguration")]
    unsafe impl MKStandardMapConfiguration {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithElevationStyle:)]
        pub unsafe fn initWithElevationStyle(
            this: Allocated<Self>,
            elevation_style: MKMapElevationStyle,
        ) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithElevationStyle:emphasisStyle:)]
        pub unsafe fn initWithElevationStyle_emphasisStyle(
            this: Allocated<Self>,
            elevation_style: MKMapElevationStyle,
            emphasis_style: MKStandardMapEmphasisStyle,
        ) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithEmphasisStyle:)]
        pub unsafe fn initWithEmphasisStyle(
            this: Allocated<Self>,
            emphasis_style: MKStandardMapEmphasisStyle,
        ) -> Retained<Self>;

        #[method(emphasisStyle)]
        pub unsafe fn emphasisStyle(&self) -> MKStandardMapEmphasisStyle;

        /// Setter for [`emphasisStyle`][Self::emphasisStyle].
        #[method(setEmphasisStyle:)]
        pub unsafe fn setEmphasisStyle(&self, emphasis_style: MKStandardMapEmphasisStyle);

        #[cfg(feature = "MKPointOfInterestFilter")]
        #[unsafe(method_family(none))]
        #[method_id(pointOfInterestFilter)]
        pub unsafe fn pointOfInterestFilter(&self) -> Option<Retained<MKPointOfInterestFilter>>;

        #[cfg(feature = "MKPointOfInterestFilter")]
        /// Setter for [`pointOfInterestFilter`][Self::pointOfInterestFilter].
        #[method(setPointOfInterestFilter:)]
        pub unsafe fn setPointOfInterestFilter(
            &self,
            point_of_interest_filter: Option<&MKPointOfInterestFilter>,
        );

        #[method(showsTraffic)]
        pub unsafe fn showsTraffic(&self) -> bool;

        /// Setter for [`showsTraffic`][Self::showsTraffic].
        #[method(setShowsTraffic:)]
        pub unsafe fn setShowsTraffic(&self, shows_traffic: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `MKMapConfiguration`
    #[cfg(feature = "MKMapConfiguration")]
    unsafe impl MKStandardMapConfiguration {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
