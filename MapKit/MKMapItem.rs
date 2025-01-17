//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkmapitem?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKMapItem;
);

unsafe impl NSObjectProtocol for MKMapItem {}

extern_methods!(
    unsafe impl MKMapItem {
        #[cfg(feature = "MKMapItemIdentifier")]
        #[unsafe(method_family(none))]
        #[method_id(identifier)]
        pub unsafe fn identifier(&self) -> Option<Retained<MKMapItemIdentifier>>;

        #[cfg(feature = "MKMapItemIdentifier")]
        #[unsafe(method_family(none))]
        #[method_id(alternateIdentifiers)]
        pub unsafe fn alternateIdentifiers(&self) -> Retained<NSSet<MKMapItemIdentifier>>;

        #[cfg(all(feature = "MKPlacemark", feature = "objc2-core-location"))]
        #[unsafe(method_family(none))]
        #[method_id(placemark)]
        pub unsafe fn placemark(&self) -> Retained<MKPlacemark>;

        #[method(isCurrentLocation)]
        pub unsafe fn isCurrentLocation(&self) -> bool;

        #[unsafe(method_family(none))]
        #[method_id(name)]
        pub unsafe fn name(&self) -> Option<Retained<NSString>>;

        /// Setter for [`name`][Self::name].
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[unsafe(method_family(none))]
        #[method_id(phoneNumber)]
        pub unsafe fn phoneNumber(&self) -> Option<Retained<NSString>>;

        /// Setter for [`phoneNumber`][Self::phoneNumber].
        #[method(setPhoneNumber:)]
        pub unsafe fn setPhoneNumber(&self, phone_number: Option<&NSString>);

        #[unsafe(method_family(none))]
        #[method_id(url)]
        pub unsafe fn url(&self) -> Option<Retained<NSURL>>;

        /// Setter for [`url`][Self::url].
        #[method(setUrl:)]
        pub unsafe fn setUrl(&self, url: Option<&NSURL>);

        #[unsafe(method_family(none))]
        #[method_id(timeZone)]
        pub unsafe fn timeZone(&self) -> Option<Retained<NSTimeZone>>;

        /// Setter for [`timeZone`][Self::timeZone].
        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, time_zone: Option<&NSTimeZone>);

        #[cfg(feature = "MKPointOfInterestCategory")]
        #[unsafe(method_family(none))]
        #[method_id(pointOfInterestCategory)]
        pub unsafe fn pointOfInterestCategory(&self)
            -> Option<Retained<MKPointOfInterestCategory>>;

        #[cfg(feature = "MKPointOfInterestCategory")]
        /// Setter for [`pointOfInterestCategory`][Self::pointOfInterestCategory].
        #[method(setPointOfInterestCategory:)]
        pub unsafe fn setPointOfInterestCategory(
            &self,
            point_of_interest_category: Option<&MKPointOfInterestCategory>,
        );

        #[unsafe(method_family(none))]
        #[method_id(mapItemForCurrentLocation)]
        pub unsafe fn mapItemForCurrentLocation() -> Retained<MKMapItem>;

        #[cfg(all(feature = "MKPlacemark", feature = "objc2-core-location"))]
        #[unsafe(method_family(init))]
        #[method_id(initWithPlacemark:)]
        pub unsafe fn initWithPlacemark(
            this: Allocated<Self>,
            placemark: &MKPlacemark,
        ) -> Retained<Self>;

        #[method(openInMapsWithLaunchOptions:)]
        pub unsafe fn openInMapsWithLaunchOptions(
            &self,
            launch_options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> bool;

        #[method(openMapsWithItems:launchOptions:)]
        pub unsafe fn openMapsWithItems_launchOptions(
            map_items: &NSArray<MKMapItem>,
            launch_options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> bool;

        #[cfg(feature = "block2")]
        #[method(openInMapsWithLaunchOptions:completionHandler:)]
        pub unsafe fn openInMapsWithLaunchOptions_completionHandler(
            &self,
            launch_options: Option<&NSDictionary<NSString, AnyObject>>,
            completion: Option<&block2::Block<dyn Fn(Bool)>>,
        );

        #[cfg(feature = "block2")]
        #[method(openMapsWithItems:launchOptions:completionHandler:)]
        pub unsafe fn openMapsWithItems_launchOptions_completionHandler(
            map_items: &NSArray<MKMapItem>,
            launch_options: Option<&NSDictionary<NSString, AnyObject>>,
            completion: Option<&block2::Block<dyn Fn(Bool)>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKMapItem {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mklaunchoptionsdirectionsmodekey?language=objc)
    pub static MKLaunchOptionsDirectionsModeKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mklaunchoptionsmaptypekey?language=objc)
    pub static MKLaunchOptionsMapTypeKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mklaunchoptionsshowstraffickey?language=objc)
    pub static MKLaunchOptionsShowsTrafficKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mklaunchoptionsdirectionsmodedefault?language=objc)
    pub static MKLaunchOptionsDirectionsModeDefault: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mklaunchoptionsdirectionsmodedriving?language=objc)
    pub static MKLaunchOptionsDirectionsModeDriving: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mklaunchoptionsdirectionsmodewalking?language=objc)
    pub static MKLaunchOptionsDirectionsModeWalking: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mklaunchoptionsdirectionsmodetransit?language=objc)
    pub static MKLaunchOptionsDirectionsModeTransit: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mklaunchoptionsmapcenterkey?language=objc)
    pub static MKLaunchOptionsMapCenterKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mklaunchoptionsmapspankey?language=objc)
    pub static MKLaunchOptionsMapSpanKey: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mklaunchoptionscamerakey?language=objc)
    pub static MKLaunchOptionsCameraKey: &'static NSString;
}

extern_methods!(
    /// MKMapItemSerialization
    unsafe impl MKMapItem {}
);

unsafe impl NSSecureCoding for MKMapItem {}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkmapitemtypeidentifier?language=objc)
    pub static MKMapItemTypeIdentifier: &'static NSString;
}
