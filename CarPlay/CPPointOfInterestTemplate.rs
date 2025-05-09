//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-map-kit")]
use objc2_map_kit::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/carplay/cppointofinteresttemplatedelegate?language=objc)
    pub unsafe trait CPPointOfInterestTemplateDelegate: NSObjectProtocol {
        #[cfg(all(feature = "CPTemplate", feature = "objc2-map-kit"))]
        /// The user has changed the map region on the
        /// `CPPointOfInterestTemplate.`Your application
        /// should respond by updating
        /// `pointsOfInterest`to show new points of interest for the new region.
        #[unsafe(method(pointOfInterestTemplate:didChangeMapRegion:))]
        #[unsafe(method_family = none)]
        unsafe fn pointOfInterestTemplate_didChangeMapRegion(
            &self,
            point_of_interest_template: &CPPointOfInterestTemplate,
            region: MKCoordinateRegion,
        );

        #[cfg(all(feature = "CPPointOfInterest", feature = "CPTemplate"))]
        /// The user has selected the
        /// `pointOfInterest`and the details are being shown.
        #[optional]
        #[unsafe(method(pointOfInterestTemplate:didSelectPointOfInterest:))]
        #[unsafe(method_family = none)]
        unsafe fn pointOfInterestTemplate_didSelectPointOfInterest(
            &self,
            point_of_interest_template: &CPPointOfInterestTemplate,
            point_of_interest: &CPPointOfInterest,
        );
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/carplay/cppointofinteresttemplate?language=objc)
    #[unsafe(super(CPTemplate, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CPTemplate")]
    pub struct CPPointOfInterestTemplate;
);

#[cfg(all(feature = "CPBarButtonProviding", feature = "CPTemplate"))]
extern_conformance!(
    unsafe impl CPBarButtonProviding for CPPointOfInterestTemplate {}
);

#[cfg(feature = "CPTemplate")]
extern_conformance!(
    unsafe impl NSCoding for CPPointOfInterestTemplate {}
);

#[cfg(feature = "CPTemplate")]
extern_conformance!(
    unsafe impl NSObjectProtocol for CPPointOfInterestTemplate {}
);

#[cfg(feature = "CPTemplate")]
extern_conformance!(
    unsafe impl NSSecureCoding for CPPointOfInterestTemplate {}
);

#[cfg(feature = "CPTemplate")]
impl CPPointOfInterestTemplate {
    extern_methods!(
        #[cfg(feature = "CPPointOfInterest")]
        /// CPPointOfInterestTemplate displays a map view with selectable points of interest.
        ///
        ///
        /// Parameter `title`: Template title
        ///
        /// Parameter `pointsOfInterest`: Points of interest to be presented in the map.
        ///
        /// Parameter `selectedIndex`: Index of selected point of interest instance referenced in the pointsOfInterest array. Use NSNotFound to indicate no selection.
        ///
        ///
        /// Note: the maximum number of POIs is 12. If you provide more than 12, only the first 12 will be used.
        #[unsafe(method(initWithTitle:pointsOfInterest:selectedIndex:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithTitle_pointsOfInterest_selectedIndex(
            this: Allocated<Self>,
            title: &NSString,
            points_of_interest: &NSArray<CPPointOfInterest>,
            selected_index: NSUInteger,
        ) -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        /// Template title appears on the template point of interest picker
        #[unsafe(method(title))]
        #[unsafe(method_family = none)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        /// Setter for [`title`][Self::title].
        #[unsafe(method(setTitle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[cfg(feature = "CPPointOfInterest")]
        /// Update the template with a list of points of interests to dispay
        ///
        ///
        /// Parameter `pointsOfInterest`: Points of interest to be presented in the map.
        ///
        /// Parameter `selectedIndex`: Index of selected point of interest instance referenced in the pointsOfInterest array. Use NSNotFound to indicate no selection.
        ///
        ///
        /// Note: the maximum number of POIs is 12. If you provide more than 12, only the first 12 will be used.
        #[unsafe(method(setPointsOfInterest:selectedIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPointsOfInterest_selectedIndex(
            &self,
            points_of_interest: &NSArray<CPPointOfInterest>,
            selected_index: NSUInteger,
        );

        #[cfg(feature = "CPPointOfInterest")]
        #[unsafe(method(pointsOfInterest))]
        #[unsafe(method_family = none)]
        pub unsafe fn pointsOfInterest(&self) -> Retained<NSArray<CPPointOfInterest>>;

        /// Designate an instance in the pointsOfInterest array to highlight. Use NSNotFound to indicate no selection.
        #[unsafe(method(selectedIndex))]
        #[unsafe(method_family = none)]
        pub unsafe fn selectedIndex(&self) -> NSUInteger;

        /// Setter for [`selectedIndex`][Self::selectedIndex].
        #[unsafe(method(setSelectedIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSelectedIndex(&self, selected_index: NSUInteger);

        /// The Point of Interest template's delegate is informed of user events.
        #[unsafe(method(pointOfInterestDelegate))]
        #[unsafe(method_family = none)]
        pub unsafe fn pointOfInterestDelegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn CPPointOfInterestTemplateDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`pointOfInterestDelegate`][Self::pointOfInterestDelegate].
        #[unsafe(method(setPointOfInterestDelegate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPointOfInterestDelegate(
            &self,
            point_of_interest_delegate: Option<
                &ProtocolObject<dyn CPPointOfInterestTemplateDelegate>,
            >,
        );
    );
}
