//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-ui-kit")]
use objc2_ui_kit::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/carplay/cpmaneuvertype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CPManeuverType(pub NSUInteger);
impl CPManeuverType {
    #[doc(alias = "CPManeuverTypeNoTurn")]
    pub const NoTurn: Self = Self(0);
    #[doc(alias = "CPManeuverTypeLeftTurn")]
    pub const LeftTurn: Self = Self(1);
    #[doc(alias = "CPManeuverTypeRightTurn")]
    pub const RightTurn: Self = Self(2);
    #[doc(alias = "CPManeuverTypeStraightAhead")]
    pub const StraightAhead: Self = Self(3);
    #[doc(alias = "CPManeuverTypeUTurn")]
    pub const UTurn: Self = Self(4);
    #[doc(alias = "CPManeuverTypeFollowRoad")]
    pub const FollowRoad: Self = Self(5);
    #[doc(alias = "CPManeuverTypeEnterRoundabout")]
    pub const EnterRoundabout: Self = Self(6);
    #[doc(alias = "CPManeuverTypeExitRoundabout")]
    pub const ExitRoundabout: Self = Self(7);
    #[doc(alias = "CPManeuverTypeOffRamp")]
    pub const OffRamp: Self = Self(8);
    #[doc(alias = "CPManeuverTypeOnRamp")]
    pub const OnRamp: Self = Self(9);
    #[doc(alias = "CPManeuverTypeArriveEndOfNavigation")]
    pub const ArriveEndOfNavigation: Self = Self(10);
    #[doc(alias = "CPManeuverTypeStartRoute")]
    pub const StartRoute: Self = Self(11);
    #[doc(alias = "CPManeuverTypeArriveAtDestination")]
    pub const ArriveAtDestination: Self = Self(12);
    #[doc(alias = "CPManeuverTypeKeepLeft")]
    pub const KeepLeft: Self = Self(13);
    #[doc(alias = "CPManeuverTypeKeepRight")]
    pub const KeepRight: Self = Self(14);
    #[doc(alias = "CPManeuverTypeEnter_Ferry")]
    pub const Enter_Ferry: Self = Self(15);
    #[doc(alias = "CPManeuverTypeExitFerry")]
    pub const ExitFerry: Self = Self(16);
    #[doc(alias = "CPManeuverTypeChangeFerry")]
    pub const ChangeFerry: Self = Self(17);
    #[doc(alias = "CPManeuverTypeStartRouteWithUTurn")]
    pub const StartRouteWithUTurn: Self = Self(18);
    #[doc(alias = "CPManeuverTypeUTurnAtRoundabout")]
    pub const UTurnAtRoundabout: Self = Self(19);
    #[doc(alias = "CPManeuverTypeLeftTurnAtEnd")]
    pub const LeftTurnAtEnd: Self = Self(20);
    #[doc(alias = "CPManeuverTypeRightTurnAtEnd")]
    pub const RightTurnAtEnd: Self = Self(21);
    #[doc(alias = "CPManeuverTypeHighwayOffRampLeft")]
    pub const HighwayOffRampLeft: Self = Self(22);
    #[doc(alias = "CPManeuverTypeHighwayOffRampRight")]
    pub const HighwayOffRampRight: Self = Self(23);
    #[doc(alias = "CPManeuverTypeArriveAtDestinationLeft")]
    pub const ArriveAtDestinationLeft: Self = Self(24);
    #[doc(alias = "CPManeuverTypeArriveAtDestinationRight")]
    pub const ArriveAtDestinationRight: Self = Self(25);
    #[doc(alias = "CPManeuverTypeUTurnWhenPossible")]
    pub const UTurnWhenPossible: Self = Self(26);
    #[doc(alias = "CPManeuverTypeArriveEndOfDirections")]
    pub const ArriveEndOfDirections: Self = Self(27);
    #[doc(alias = "CPManeuverTypeRoundaboutExit1")]
    pub const RoundaboutExit1: Self = Self(28);
    #[doc(alias = "CPManeuverTypeRoundaboutExit2")]
    pub const RoundaboutExit2: Self = Self(29);
    #[doc(alias = "CPManeuverTypeRoundaboutExit3")]
    pub const RoundaboutExit3: Self = Self(30);
    #[doc(alias = "CPManeuverTypeRoundaboutExit4")]
    pub const RoundaboutExit4: Self = Self(31);
    #[doc(alias = "CPManeuverTypeRoundaboutExit5")]
    pub const RoundaboutExit5: Self = Self(32);
    #[doc(alias = "CPManeuverTypeRoundaboutExit6")]
    pub const RoundaboutExit6: Self = Self(33);
    #[doc(alias = "CPManeuverTypeRoundaboutExit7")]
    pub const RoundaboutExit7: Self = Self(34);
    #[doc(alias = "CPManeuverTypeRoundaboutExit8")]
    pub const RoundaboutExit8: Self = Self(35);
    #[doc(alias = "CPManeuverTypeRoundaboutExit9")]
    pub const RoundaboutExit9: Self = Self(36);
    #[doc(alias = "CPManeuverTypeRoundaboutExit10")]
    pub const RoundaboutExit10: Self = Self(37);
    #[doc(alias = "CPManeuverTypeRoundaboutExit11")]
    pub const RoundaboutExit11: Self = Self(38);
    #[doc(alias = "CPManeuverTypeRoundaboutExit12")]
    pub const RoundaboutExit12: Self = Self(39);
    #[doc(alias = "CPManeuverTypeRoundaboutExit13")]
    pub const RoundaboutExit13: Self = Self(40);
    #[doc(alias = "CPManeuverTypeRoundaboutExit14")]
    pub const RoundaboutExit14: Self = Self(41);
    #[doc(alias = "CPManeuverTypeRoundaboutExit15")]
    pub const RoundaboutExit15: Self = Self(42);
    #[doc(alias = "CPManeuverTypeRoundaboutExit16")]
    pub const RoundaboutExit16: Self = Self(43);
    #[doc(alias = "CPManeuverTypeRoundaboutExit17")]
    pub const RoundaboutExit17: Self = Self(44);
    #[doc(alias = "CPManeuverTypeRoundaboutExit18")]
    pub const RoundaboutExit18: Self = Self(45);
    #[doc(alias = "CPManeuverTypeRoundaboutExit19")]
    pub const RoundaboutExit19: Self = Self(46);
    #[doc(alias = "CPManeuverTypeSharpLeftTurn")]
    pub const SharpLeftTurn: Self = Self(47);
    #[doc(alias = "CPManeuverTypeSharpRightTurn")]
    pub const SharpRightTurn: Self = Self(48);
    #[doc(alias = "CPManeuverTypeSlightLeftTurn")]
    pub const SlightLeftTurn: Self = Self(49);
    #[doc(alias = "CPManeuverTypeSlightRightTurn")]
    pub const SlightRightTurn: Self = Self(50);
    #[doc(alias = "CPManeuverTypeChangeHighway")]
    pub const ChangeHighway: Self = Self(51);
    #[doc(alias = "CPManeuverTypeChangeHighwayLeft")]
    pub const ChangeHighwayLeft: Self = Self(52);
    #[doc(alias = "CPManeuverTypeChangeHighwayRight")]
    pub const ChangeHighwayRight: Self = Self(53);
}

unsafe impl Encode for CPManeuverType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for CPManeuverType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/carplay/cpjunctiontype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CPJunctionType(pub NSUInteger);
impl CPJunctionType {
    #[doc(alias = "CPJunctionTypeIntersection")]
    pub const Intersection: Self = Self(0);
    #[doc(alias = "CPJunctionTypeRoundabout")]
    pub const Roundabout: Self = Self(1);
}

unsafe impl Encode for CPJunctionType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for CPJunctionType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/carplay/cptrafficside?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CPTrafficSide(pub NSUInteger);
impl CPTrafficSide {
    #[doc(alias = "CPTrafficSideRight")]
    pub const Right: Self = Self(0);
    #[doc(alias = "CPTrafficSideLeft")]
    pub const Left: Self = Self(1);
}

unsafe impl Encode for CPTrafficSide {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for CPTrafficSide {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/carplay/cpmaneuverstate?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CPManeuverState(pub NSInteger);
impl CPManeuverState {
    #[doc(alias = "CPManeuverStateContinue")]
    pub const Continue: Self = Self(0);
    #[doc(alias = "CPManeuverStateInitial")]
    pub const Initial: Self = Self(1);
    #[doc(alias = "CPManeuverStatePrepare")]
    pub const Prepare: Self = Self(2);
    #[doc(alias = "CPManeuverStateExecute")]
    pub const Execute: Self = Self(3);
}

unsafe impl Encode for CPManeuverState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CPManeuverState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// `CPManeuver`describes a navigation instruction.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/carplay/cpmaneuver?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CPManeuver;
);

extern_conformance!(
    unsafe impl NSCoding for CPManeuver {}
);

extern_conformance!(
    unsafe impl NSCopying for CPManeuver {}
);

unsafe impl CopyingHelper for CPManeuver {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for CPManeuver {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for CPManeuver {}
);

impl CPManeuver {
    extern_methods!(
        #[cfg(feature = "CPImageSet")]
        /// symbolSet is a
        /// `CPImageSet`representing the maneuver.
        #[deprecated]
        #[unsafe(method(symbolSet))]
        #[unsafe(method_family = none)]
        pub unsafe fn symbolSet(&self) -> Option<Retained<CPImageSet>>;

        #[cfg(feature = "CPImageSet")]
        /// Setter for [`symbolSet`][Self::symbolSet].
        #[deprecated]
        #[unsafe(method(setSymbolSet:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSymbolSet(&self, symbol_set: Option<&CPImageSet>);

        #[cfg(feature = "objc2-ui-kit")]
        /// Takes precedence over
        /// `guidanceBackgroundColor`set in
        /// `CPMapTemplate.`
        #[unsafe(method(cardBackgroundColor))]
        #[unsafe(method_family = none)]
        pub unsafe fn cardBackgroundColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "objc2-ui-kit")]
        /// Setter for [`cardBackgroundColor`][Self::cardBackgroundColor].
        #[unsafe(method(setCardBackgroundColor:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCardBackgroundColor(&self, card_background_color: Option<&UIColor>);

        #[cfg(feature = "objc2-ui-kit")]
        /// symbolImage is a
        /// `UIImage`representing the maneuver. Provide variants for UIUserInterfaceStyleLight and UIUserInterfaceStyleDark that will be used against light backgrounds and dark backgrounds.
        #[unsafe(method(symbolImage))]
        #[unsafe(method_family = none)]
        pub unsafe fn symbolImage(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "objc2-ui-kit")]
        /// Setter for [`symbolImage`][Self::symbolImage].
        #[unsafe(method(setSymbolImage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSymbolImage(&self, symbol_image: Option<&UIImage>);

        #[cfg(feature = "objc2-ui-kit")]
        /// junctionImage is a
        /// `UIImage`used to display a junction for the maneuver.
        ///
        ///
        /// Note: The maximum image size is 140 points by 100 points. If necessary, images will be scaled down to fit while maintaining the aspect ratio.
        #[unsafe(method(junctionImage))]
        #[unsafe(method_family = none)]
        pub unsafe fn junctionImage(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "objc2-ui-kit")]
        /// Setter for [`junctionImage`][Self::junctionImage].
        #[unsafe(method(setJunctionImage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setJunctionImage(&self, junction_image: Option<&UIImage>);

        /// instructionVariants is an array of
        /// `NSString`representing the instruction for this maneuver, arranged from most to least preferred. You must provide at least one variant.
        /// The variant strings should be provided as localized, displayable content.
        #[unsafe(method(instructionVariants))]
        #[unsafe(method_family = none)]
        pub unsafe fn instructionVariants(&self) -> Retained<NSArray<NSString>>;

        /// Setter for [`instructionVariants`][Self::instructionVariants].
        #[unsafe(method(setInstructionVariants:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setInstructionVariants(&self, instruction_variants: &NSArray<NSString>);

        #[cfg(feature = "CPTravelEstimates")]
        /// initialTravelEstimates represents the estimates beginning from the end of the preceding maneuver.
        #[unsafe(method(initialTravelEstimates))]
        #[unsafe(method_family = none)]
        pub unsafe fn initialTravelEstimates(&self) -> Option<Retained<CPTravelEstimates>>;

        #[cfg(feature = "CPTravelEstimates")]
        /// Setter for [`initialTravelEstimates`][Self::initialTravelEstimates].
        #[unsafe(method(setInitialTravelEstimates:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setInitialTravelEstimates(
            &self,
            initial_travel_estimates: Option<&CPTravelEstimates>,
        );

        /// attributedInstructionVariants is an array of
        /// `NSAttributedString`representing the instruction for this maneuver,
        /// arranged from most to least preferred.
        ///
        /// `attributedInstructionVariants`will be preferred over instructionVariants. You must provide at least one variant for each maneuver.
        ///
        /// Only one type of attribute is presently supported: text attachments. You may annotate a maneuver instruction with images
        /// by including one or more text attachments. The maximum text attachment image size is 64x25 points.
        ///
        ///
        /// Warning: All attributes other than text attachment attributes will be removed from your attributed string.
        ///
        ///
        /// See: +[NSAttributedString attributedStringWithAttachment:], -[NSTextAttachment image]
        #[unsafe(method(attributedInstructionVariants))]
        #[unsafe(method_family = none)]
        pub unsafe fn attributedInstructionVariants(&self)
            -> Retained<NSArray<NSAttributedString>>;

        /// Setter for [`attributedInstructionVariants`][Self::attributedInstructionVariants].
        #[unsafe(method(setAttributedInstructionVariants:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAttributedInstructionVariants(
            &self,
            attributed_instruction_variants: &NSArray<NSAttributedString>,
        );

        #[cfg(feature = "objc2-ui-kit")]
        /// dashboardSymbolImage is a
        /// `UIImage`representing the maneuver that will be used on the dashboard. Provide variants for UIUserInterfaceStyleLight and UIUserInterfaceStyleDark that will be used against light backgrounds and dark backgrounds. If no dashboard image
        /// is provided, symbolImage will be used.
        #[unsafe(method(dashboardSymbolImage))]
        #[unsafe(method_family = none)]
        pub unsafe fn dashboardSymbolImage(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "objc2-ui-kit")]
        /// Setter for [`dashboardSymbolImage`][Self::dashboardSymbolImage].
        #[unsafe(method(setDashboardSymbolImage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDashboardSymbolImage(&self, dashboard_symbol_image: Option<&UIImage>);

        #[cfg(feature = "objc2-ui-kit")]
        /// dashboardJunctionImage is a
        /// `UIImage`used to display a junction for the maneuver on the dashboard. If no dashboard junction image is provided, junctionImage will be used.
        ///
        ///
        /// Note: The maximum image size is 140 points by 100 points. If necessary, images will be scaled down to fit while maintaining the aspect ratio.
        #[unsafe(method(dashboardJunctionImage))]
        #[unsafe(method_family = none)]
        pub unsafe fn dashboardJunctionImage(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "objc2-ui-kit")]
        /// Setter for [`dashboardJunctionImage`][Self::dashboardJunctionImage].
        #[unsafe(method(setDashboardJunctionImage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDashboardJunctionImage(&self, dashboard_junction_image: Option<&UIImage>);

        /// dashboardInstructionVariants is an array of
        /// `NSString`representing the instruction for this maneuver on the dashboard, arranged from most to least preferred. If no dashboard variants are provided, the system will check for attributedInstructionVariants, then instructionVariants.
        /// The variant strings should be provided as localized, displayable content.
        #[unsafe(method(dashboardInstructionVariants))]
        #[unsafe(method_family = none)]
        pub unsafe fn dashboardInstructionVariants(&self) -> Retained<NSArray<NSString>>;

        /// Setter for [`dashboardInstructionVariants`][Self::dashboardInstructionVariants].
        #[unsafe(method(setDashboardInstructionVariants:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDashboardInstructionVariants(
            &self,
            dashboard_instruction_variants: &NSArray<NSString>,
        );

        /// dashboardAttributedInstructionVariants is an array of
        /// `NSAttributedString`representing the instruction for this maneuver on the dashboard,
        /// arranged from most to least preferred.
        ///
        /// `dashboardAttributedInstructionVariants`will be preferred over dashboardInstructionVariants. If no dashboard attributed variants are provided, dashboardInstructionVariants will be used.
        ///
        /// Only one type of attribute is presently supported: text attachments. You may annotate a maneuver instruction with images
        /// by including one or more text attachments. The maximum text attachment image size is 64x25 points.
        ///
        ///
        /// Warning: All attributes other than text attachment attributes will be removed from your attributed string.
        ///
        ///
        /// See: +[NSAttributedString attributedStringWithAttachment:], -[NSTextAttachment image]
        #[unsafe(method(dashboardAttributedInstructionVariants))]
        #[unsafe(method_family = none)]
        pub unsafe fn dashboardAttributedInstructionVariants(
            &self,
        ) -> Retained<NSArray<NSAttributedString>>;

        /// Setter for [`dashboardAttributedInstructionVariants`][Self::dashboardAttributedInstructionVariants].
        #[unsafe(method(setDashboardAttributedInstructionVariants:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDashboardAttributedInstructionVariants(
            &self,
            dashboard_attributed_instruction_variants: &NSArray<NSAttributedString>,
        );

        #[cfg(feature = "objc2-ui-kit")]
        /// notificationSymbolImage is a
        /// `UIImage`representing the maneuver that will be used in a notification banner. Provide variants for UIUserInterfaceStyleLight and UIUserInterfaceStyleDark that will be used against light backgrounds and dark backgrounds. If no
        /// notification symbol image is provided, symbolImage will be used.
        #[unsafe(method(notificationSymbolImage))]
        #[unsafe(method_family = none)]
        pub unsafe fn notificationSymbolImage(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "objc2-ui-kit")]
        /// Setter for [`notificationSymbolImage`][Self::notificationSymbolImage].
        #[unsafe(method(setNotificationSymbolImage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setNotificationSymbolImage(
            &self,
            notification_symbol_image: Option<&UIImage>,
        );

        /// notificationInstructionVariants is an array of
        /// `NSString`representing the instruction for this maneuver in a notification banner, arranged from most to least preferred. If no notification instruction variants are provided, the system will check for attributedInstructionVariants, then instructionVariants.
        /// The variant strings should be provided as localized, displayable content.
        #[unsafe(method(notificationInstructionVariants))]
        #[unsafe(method_family = none)]
        pub unsafe fn notificationInstructionVariants(&self) -> Retained<NSArray<NSString>>;

        /// Setter for [`notificationInstructionVariants`][Self::notificationInstructionVariants].
        #[unsafe(method(setNotificationInstructionVariants:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setNotificationInstructionVariants(
            &self,
            notification_instruction_variants: &NSArray<NSString>,
        );

        /// notificationAttributedInstructionVariants is an array of
        /// `NSAttributedString`representing the instruction for this maneuver in a notification banner,
        /// arranged from most to least preferred.
        ///
        /// `notificationAttributedInstructionVariants`will be preferred over instructionVariants. If no notification attributed variants are provided, notificationInstructionVariants will be used.
        ///
        /// Only one type of attribute is presently supported: text attachments. You may annotate a maneuver instruction with images
        /// by including one or more text attachments. The maximum text attachment image size is 64x25 points.
        ///
        ///
        /// Warning: All attributes other than text attachment attributes will be removed from your attributed string.
        ///
        ///
        /// See: +[NSAttributedString attributedStringWithAttachment:], -[NSTextAttachment image]
        #[unsafe(method(notificationAttributedInstructionVariants))]
        #[unsafe(method_family = none)]
        pub unsafe fn notificationAttributedInstructionVariants(
            &self,
        ) -> Retained<NSArray<NSAttributedString>>;

        /// Setter for [`notificationAttributedInstructionVariants`][Self::notificationAttributedInstructionVariants].
        #[unsafe(method(setNotificationAttributedInstructionVariants:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setNotificationAttributedInstructionVariants(
            &self,
            notification_attributed_instruction_variants: &NSArray<NSAttributedString>,
        );

        /// maneuverType is a
        /// `CPManeuverType`representing the type of maneuver.
        #[unsafe(method(maneuverType))]
        #[unsafe(method_family = none)]
        pub unsafe fn maneuverType(&self) -> CPManeuverType;

        /// Setter for [`maneuverType`][Self::maneuverType].
        #[unsafe(method(setManeuverType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setManeuverType(&self, maneuver_type: CPManeuverType);

        /// roadFollowingManeuverVariants is an array of
        /// `NSString`representing the name of the road following this maneuver,
        /// arranged from most to least preferred. (arranged by space)
        #[unsafe(method(roadFollowingManeuverVariants))]
        #[unsafe(method_family = none)]
        pub unsafe fn roadFollowingManeuverVariants(&self) -> Option<Retained<NSArray<NSString>>>;

        /// Setter for [`roadFollowingManeuverVariants`][Self::roadFollowingManeuverVariants].
        #[unsafe(method(setRoadFollowingManeuverVariants:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRoadFollowingManeuverVariants(
            &self,
            road_following_maneuver_variants: Option<&NSArray<NSString>>,
        );

        /// trafficSide is a
        /// `CPTrafficSide`representing which side of the road the traffic drives on.
        #[unsafe(method(trafficSide))]
        #[unsafe(method_family = none)]
        pub unsafe fn trafficSide(&self) -> CPTrafficSide;

        /// Setter for [`trafficSide`][Self::trafficSide].
        #[unsafe(method(setTrafficSide:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTrafficSide(&self, traffic_side: CPTrafficSide);

        /// junctionType is a
        /// `CPJunctionType`representing the type of the junction associated with this maneuver
        #[unsafe(method(junctionType))]
        #[unsafe(method_family = none)]
        pub unsafe fn junctionType(&self) -> CPJunctionType;

        /// Setter for [`junctionType`][Self::junctionType].
        #[unsafe(method(setJunctionType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setJunctionType(&self, junction_type: CPJunctionType);

        /// junctionExitAngle is the angle of the exit road of this junction.
        #[unsafe(method(junctionExitAngle))]
        #[unsafe(method_family = none)]
        pub unsafe fn junctionExitAngle(&self) -> Option<Retained<NSMeasurement<NSUnitAngle>>>;

        /// Setter for [`junctionExitAngle`][Self::junctionExitAngle].
        #[unsafe(method(setJunctionExitAngle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setJunctionExitAngle(
            &self,
            junction_exit_angle: Option<&NSMeasurement<NSUnitAngle>>,
        );

        /// junctionElementAngles is a set of angles of the rest of the roads of this junction. This must not include
        /// `junctionExitAngle`.
        #[unsafe(method(junctionElementAngles))]
        #[unsafe(method_family = none)]
        pub unsafe fn junctionElementAngles(
            &self,
        ) -> Option<Retained<NSSet<NSMeasurement<NSUnitAngle>>>>;

        /// Setter for [`junctionElementAngles`][Self::junctionElementAngles].
        #[unsafe(method(setJunctionElementAngles:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setJunctionElementAngles(
            &self,
            junction_element_angles: Option<&NSSet<NSMeasurement<NSUnitAngle>>>,
        );

        #[cfg(feature = "CPLaneGuidance")]
        /// linkedLaneGuidance is the optional
        /// `CPLaneGuidance`associated with this maneuver // conditional - must be there if there is a corresponding lane guidance
        #[unsafe(method(linkedLaneGuidance))]
        #[unsafe(method_family = none)]
        pub unsafe fn linkedLaneGuidance(&self) -> Retained<CPLaneGuidance>;

        #[cfg(feature = "CPLaneGuidance")]
        /// Setter for [`linkedLaneGuidance`][Self::linkedLaneGuidance].
        #[unsafe(method(setLinkedLaneGuidance:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLinkedLaneGuidance(&self, linked_lane_guidance: &CPLaneGuidance);

        /// highwayExitLabel is a
        /// `NSString`describing a highway exit. Exit 123 for example.
        #[unsafe(method(highwayExitLabel))]
        #[unsafe(method_family = none)]
        pub unsafe fn highwayExitLabel(&self) -> Retained<NSString>;

        /// Setter for [`highwayExitLabel`][Self::highwayExitLabel].
        #[unsafe(method(setHighwayExitLabel:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setHighwayExitLabel(&self, highway_exit_label: &NSString);

        /// Any custom user info related to this maneuver.
        #[unsafe(method(userInfo))]
        #[unsafe(method_family = none)]
        pub unsafe fn userInfo(&self) -> Option<Retained<AnyObject>>;

        /// Setter for [`userInfo`][Self::userInfo].
        #[unsafe(method(setUserInfo:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setUserInfo(&self, user_info: Option<&AnyObject>);
    );
}

/// Methods declared on superclass `NSObject`.
impl CPManeuver {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
