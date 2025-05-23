//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;
use objc2_foundation::*;

use crate::*;

/// A relevant daily routine situation.
///
/// See also: INDailyRoutineRelevanceProvider
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/intents/indailyroutinesituation?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct INDailyRoutineSituation(pub NSInteger);
impl INDailyRoutineSituation {
    /// A situation that occurs in the morning, around the time the user wakes up.
    #[doc(alias = "INDailyRoutineSituationMorning")]
    pub const Morning: Self = Self(0);
    /// A situation that occurs in the evening, around the time the user goes to bed.
    #[doc(alias = "INDailyRoutineSituationEvening")]
    pub const Evening: Self = Self(1);
    /// A situation that occurs when the user is at home.
    ///
    /// Note: Your app needs Always location authorization to use this situation.
    ///
    /// See also: CLLocationManager
    #[doc(alias = "INDailyRoutineSituationHome")]
    pub const Home: Self = Self(2);
    /// A situation that occurs when the user is at work.
    ///
    /// Note: Your app needs Always location authorization to use this situation.
    ///
    /// See also: CLLocationManager
    #[doc(alias = "INDailyRoutineSituationWork")]
    pub const Work: Self = Self(3);
    /// A situation that occurs when the user is at school.
    ///
    /// Note: Your app needs Always location authorization to use this situation.
    ///
    /// See also: CLLocationManager
    #[doc(alias = "INDailyRoutineSituationSchool")]
    pub const School: Self = Self(4);
    /// A situation that occurs when the user is at the gym.
    ///
    /// Note: Your app needs Always location authorization to use this situation.
    ///
    /// See also: CLLocationManager
    #[doc(alias = "INDailyRoutineSituationGym")]
    pub const Gym: Self = Self(5);
    /// A situation that occurs when the user is commuting, for example driving into work.
    #[doc(alias = "INDailyRoutineSituationCommute")]
    pub const Commute: Self = Self(6);
    /// A situation that occurs when the user connects headphones.
    #[doc(alias = "INDailyRoutineSituationHeadphonesConnected")]
    pub const HeadphonesConnected: Self = Self(7);
    /// A situation that occurs when the user is currently in a workout.
    #[doc(alias = "INDailyRoutineSituationActiveWorkout")]
    pub const ActiveWorkout: Self = Self(8);
    /// A situation that occurs when the user is expected to perform more physical activity during the day.
    #[doc(alias = "INDailyRoutineSituationPhysicalActivityIncomplete")]
    pub const PhysicalActivityIncomplete: Self = Self(9);
}

unsafe impl Encode for INDailyRoutineSituation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for INDailyRoutineSituation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// A relevance provider represents a piece of relevance information that can be used by Siri when predicting relevant shortcuts.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/intents/inrelevanceprovider?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct INRelevanceProvider;
);

extern_conformance!(
    unsafe impl NSCoding for INRelevanceProvider {}
);

extern_conformance!(
    unsafe impl NSCopying for INRelevanceProvider {}
);

unsafe impl CopyingHelper for INRelevanceProvider {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for INRelevanceProvider {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for INRelevanceProvider {}
);

impl INRelevanceProvider {
    extern_methods!(
        /// Note: `INRelevanceProvider`should not be initilaized directly. Use one of the subclasses instead.
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl INRelevanceProvider {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// A relevance provider to indicate relevance at a date or date interval.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/intents/indaterelevanceprovider?language=objc)
    #[unsafe(super(INRelevanceProvider, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct INDateRelevanceProvider;
);

extern_conformance!(
    unsafe impl NSCoding for INDateRelevanceProvider {}
);

extern_conformance!(
    unsafe impl NSCopying for INDateRelevanceProvider {}
);

unsafe impl CopyingHelper for INDateRelevanceProvider {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for INDateRelevanceProvider {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for INDateRelevanceProvider {}
);

impl INDateRelevanceProvider {
    extern_methods!(
        /// The start date of the relevant time interval.
        #[unsafe(method(startDate))]
        #[unsafe(method_family = none)]
        pub unsafe fn startDate(&self) -> Retained<NSDate>;

        /// The end date of the relevant time interval.
        ///
        /// Note: If
        /// `endDate`is
        /// `nil,`the relevant time interval will be assumed to represent a single point in time instead of a time interval.
        #[unsafe(method(endDate))]
        #[unsafe(method_family = none)]
        pub unsafe fn endDate(&self) -> Option<Retained<NSDate>>;

        /// Initializes a date relevance provider with the specified relevant date interval.
        #[unsafe(method(initWithStartDate:endDate:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithStartDate_endDate(
            this: Allocated<Self>,
            start_date: &NSDate,
            end_date: Option<&NSDate>,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `INRelevanceProvider`.
impl INDateRelevanceProvider {
    extern_methods!(
        /// Note: `INRelevanceProvider`should not be initilaized directly. Use one of the subclasses instead.
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl INDateRelevanceProvider {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/intents/inlocationrelevanceprovider?language=objc)
    #[unsafe(super(INRelevanceProvider, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct INLocationRelevanceProvider;
);

extern_conformance!(
    unsafe impl NSCoding for INLocationRelevanceProvider {}
);

extern_conformance!(
    unsafe impl NSCopying for INLocationRelevanceProvider {}
);

unsafe impl CopyingHelper for INLocationRelevanceProvider {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for INLocationRelevanceProvider {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for INLocationRelevanceProvider {}
);

impl INLocationRelevanceProvider {
    extern_methods!(
        #[cfg(feature = "objc2-core-location")]
        /// The region representing the relevant location.
        ///
        /// See also: CLCircularRegion
        #[unsafe(method(region))]
        #[unsafe(method_family = none)]
        pub unsafe fn region(&self) -> Retained<CLRegion>;

        #[cfg(feature = "objc2-core-location")]
        /// Initializes a location relevance provider with the specified region.
        #[unsafe(method(initWithRegion:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithRegion(this: Allocated<Self>, region: &CLRegion) -> Retained<Self>;
    );
}

/// Methods declared on superclass `INRelevanceProvider`.
impl INLocationRelevanceProvider {
    extern_methods!(
        /// Note: `INRelevanceProvider`should not be initilaized directly. Use one of the subclasses instead.
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl INLocationRelevanceProvider {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// A relevance provider that specifies relevance during a specific situation.
    ///
    /// See also: INDailyRoutineSituation
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/intents/indailyroutinerelevanceprovider?language=objc)
    #[unsafe(super(INRelevanceProvider, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct INDailyRoutineRelevanceProvider;
);

extern_conformance!(
    unsafe impl NSCoding for INDailyRoutineRelevanceProvider {}
);

extern_conformance!(
    unsafe impl NSCopying for INDailyRoutineRelevanceProvider {}
);

unsafe impl CopyingHelper for INDailyRoutineRelevanceProvider {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for INDailyRoutineRelevanceProvider {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for INDailyRoutineRelevanceProvider {}
);

impl INDailyRoutineRelevanceProvider {
    extern_methods!(
        /// The relevant daily routine situation of the provider.
        #[unsafe(method(situation))]
        #[unsafe(method_family = none)]
        pub unsafe fn situation(&self) -> INDailyRoutineSituation;

        /// Initializes a daily routine relevance provider with the specified situation.
        #[unsafe(method(initWithSituation:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithSituation(
            this: Allocated<Self>,
            situation: INDailyRoutineSituation,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `INRelevanceProvider`.
impl INDailyRoutineRelevanceProvider {
    extern_methods!(
        /// Note: `INRelevanceProvider`should not be initilaized directly. Use one of the subclasses instead.
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl INDailyRoutineRelevanceProvider {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
