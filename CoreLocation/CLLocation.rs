//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corelocation/cllocationdegrees?language=objc)
pub type CLLocationDegrees = c_double;

/// [Apple's documentation](https://developer.apple.com/documentation/corelocation/cllocationaccuracy?language=objc)
pub type CLLocationAccuracy = c_double;

/// [Apple's documentation](https://developer.apple.com/documentation/corelocation/cllocationspeed?language=objc)
pub type CLLocationSpeed = c_double;

/// [Apple's documentation](https://developer.apple.com/documentation/corelocation/cllocationspeedaccuracy?language=objc)
pub type CLLocationSpeedAccuracy = c_double;

/// [Apple's documentation](https://developer.apple.com/documentation/corelocation/cllocationdirection?language=objc)
pub type CLLocationDirection = c_double;

/// [Apple's documentation](https://developer.apple.com/documentation/corelocation/cllocationdirectionaccuracy?language=objc)
pub type CLLocationDirectionAccuracy = c_double;

/// [Apple's documentation](https://developer.apple.com/documentation/corelocation/cllocationcoordinate2d?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLLocationCoordinate2D {
    pub latitude: CLLocationDegrees,
    pub longitude: CLLocationDegrees,
}

unsafe impl Encode for CLLocationCoordinate2D {
    const ENCODING: Encoding = Encoding::Struct(
        "CLLocationCoordinate2D",
        &[<CLLocationDegrees>::ENCODING, <CLLocationDegrees>::ENCODING],
    );
}

unsafe impl RefEncode for CLLocationCoordinate2D {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/corelocation/cllocationdistance?language=objc)
pub type CLLocationDistance = c_double;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corelocation/kcldistancefilternone?language=objc)
    pub static kCLDistanceFilterNone: CLLocationDistance;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corelocation/kcllocationaccuracybestfornavigation?language=objc)
    pub static kCLLocationAccuracyBestForNavigation: CLLocationAccuracy;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corelocation/kcllocationaccuracybest?language=objc)
    pub static kCLLocationAccuracyBest: CLLocationAccuracy;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corelocation/kcllocationaccuracynearesttenmeters?language=objc)
    pub static kCLLocationAccuracyNearestTenMeters: CLLocationAccuracy;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corelocation/kcllocationaccuracyhundredmeters?language=objc)
    pub static kCLLocationAccuracyHundredMeters: CLLocationAccuracy;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corelocation/kcllocationaccuracykilometer?language=objc)
    pub static kCLLocationAccuracyKilometer: CLLocationAccuracy;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corelocation/kcllocationaccuracythreekilometers?language=objc)
    pub static kCLLocationAccuracyThreeKilometers: CLLocationAccuracy;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corelocation/kcllocationaccuracyreduced?language=objc)
    pub static kCLLocationAccuracyReduced: CLLocationAccuracy;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corelocation/cllocationdistancemax?language=objc)
    pub static CLLocationDistanceMax: CLLocationDistance;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corelocation/cltimeintervalmax?language=objc)
    pub static CLTimeIntervalMax: NSTimeInterval;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corelocation/kcllocationcoordinate2dinvalid?language=objc)
    pub static kCLLocationCoordinate2DInvalid: CLLocationCoordinate2D;
}

impl CLLocationCoordinate2D {
    #[doc(alias = "CLLocationCoordinate2DIsValid")]
    #[inline]
    pub unsafe fn is_valid(self: CLLocationCoordinate2D) -> bool {
        extern "C-unwind" {
            fn CLLocationCoordinate2DIsValid(coord: CLLocationCoordinate2D) -> Bool;
        }
        unsafe { CLLocationCoordinate2DIsValid(self) }.as_bool()
    }

    #[doc(alias = "CLLocationCoordinate2DMake")]
    #[inline]
    pub unsafe fn new(
        latitude: CLLocationDegrees,
        longitude: CLLocationDegrees,
    ) -> CLLocationCoordinate2D {
        extern "C-unwind" {
            fn CLLocationCoordinate2DMake(
                latitude: CLLocationDegrees,
                longitude: CLLocationDegrees,
            ) -> CLLocationCoordinate2D;
        }
        unsafe { CLLocationCoordinate2DMake(latitude, longitude) }
    }
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/corelocation/clfloor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CLFloor;
);

extern_conformance!(
    unsafe impl NSCoding for CLFloor {}
);

extern_conformance!(
    unsafe impl NSCopying for CLFloor {}
);

unsafe impl CopyingHelper for CLFloor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for CLFloor {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for CLFloor {}
);

impl CLFloor {
    extern_methods!(
        #[unsafe(method(level))]
        #[unsafe(method_family = none)]
        pub unsafe fn level(&self) -> NSInteger;
    );
}

/// Methods declared on superclass `NSObject`.
impl CLFloor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/corelocation/cllocationsourceinformation?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CLLocationSourceInformation;
);

extern_conformance!(
    unsafe impl NSCoding for CLLocationSourceInformation {}
);

extern_conformance!(
    unsafe impl NSCopying for CLLocationSourceInformation {}
);

unsafe impl CopyingHelper for CLLocationSourceInformation {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for CLLocationSourceInformation {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for CLLocationSourceInformation {}
);

impl CLLocationSourceInformation {
    extern_methods!(
        #[unsafe(method(initWithSoftwareSimulationState:andExternalAccessoryState:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithSoftwareSimulationState_andExternalAccessoryState(
            this: Allocated<Self>,
            is_software: bool,
            is_accessory: bool,
        ) -> Retained<Self>;

        #[unsafe(method(isSimulatedBySoftware))]
        #[unsafe(method_family = none)]
        pub unsafe fn isSimulatedBySoftware(&self) -> bool;

        #[unsafe(method(isProducedByAccessory))]
        #[unsafe(method_family = none)]
        pub unsafe fn isProducedByAccessory(&self) -> bool;
    );
}

/// Methods declared on superclass `NSObject`.
impl CLLocationSourceInformation {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/corelocation/cllocation?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CLLocation;
);

unsafe impl Send for CLLocation {}

unsafe impl Sync for CLLocation {}

extern_conformance!(
    unsafe impl NSCoding for CLLocation {}
);

extern_conformance!(
    unsafe impl NSCopying for CLLocation {}
);

unsafe impl CopyingHelper for CLLocation {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for CLLocation {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for CLLocation {}
);

impl CLLocation {
    extern_methods!(
        #[unsafe(method(initWithLatitude:longitude:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithLatitude_longitude(
            this: Allocated<Self>,
            latitude: CLLocationDegrees,
            longitude: CLLocationDegrees,
        ) -> Retained<Self>;

        #[unsafe(method(initWithCoordinate:altitude:horizontalAccuracy:verticalAccuracy:timestamp:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoordinate_altitude_horizontalAccuracy_verticalAccuracy_timestamp(
            this: Allocated<Self>,
            coordinate: CLLocationCoordinate2D,
            altitude: CLLocationDistance,
            h_accuracy: CLLocationAccuracy,
            v_accuracy: CLLocationAccuracy,
            timestamp: &NSDate,
        ) -> Retained<Self>;

        #[unsafe(method(initWithCoordinate:altitude:horizontalAccuracy:verticalAccuracy:course:speed:timestamp:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoordinate_altitude_horizontalAccuracy_verticalAccuracy_course_speed_timestamp(
            this: Allocated<Self>,
            coordinate: CLLocationCoordinate2D,
            altitude: CLLocationDistance,
            h_accuracy: CLLocationAccuracy,
            v_accuracy: CLLocationAccuracy,
            course: CLLocationDirection,
            speed: CLLocationSpeed,
            timestamp: &NSDate,
        ) -> Retained<Self>;

        #[unsafe(method(initWithCoordinate:altitude:horizontalAccuracy:verticalAccuracy:course:courseAccuracy:speed:speedAccuracy:timestamp:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoordinate_altitude_horizontalAccuracy_verticalAccuracy_course_courseAccuracy_speed_speedAccuracy_timestamp(
            this: Allocated<Self>,
            coordinate: CLLocationCoordinate2D,
            altitude: CLLocationDistance,
            h_accuracy: CLLocationAccuracy,
            v_accuracy: CLLocationAccuracy,
            course: CLLocationDirection,
            course_accuracy: CLLocationDirectionAccuracy,
            speed: CLLocationSpeed,
            speed_accuracy: CLLocationSpeedAccuracy,
            timestamp: &NSDate,
        ) -> Retained<Self>;

        #[unsafe(method(initWithCoordinate:altitude:horizontalAccuracy:verticalAccuracy:course:courseAccuracy:speed:speedAccuracy:timestamp:sourceInfo:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoordinate_altitude_horizontalAccuracy_verticalAccuracy_course_courseAccuracy_speed_speedAccuracy_timestamp_sourceInfo(
            this: Allocated<Self>,
            coordinate: CLLocationCoordinate2D,
            altitude: CLLocationDistance,
            h_accuracy: CLLocationAccuracy,
            v_accuracy: CLLocationAccuracy,
            course: CLLocationDirection,
            course_accuracy: CLLocationDirectionAccuracy,
            speed: CLLocationSpeed,
            speed_accuracy: CLLocationSpeedAccuracy,
            timestamp: &NSDate,
            source_info: &CLLocationSourceInformation,
        ) -> Retained<Self>;

        #[unsafe(method(coordinate))]
        #[unsafe(method_family = none)]
        pub unsafe fn coordinate(&self) -> CLLocationCoordinate2D;

        #[unsafe(method(altitude))]
        #[unsafe(method_family = none)]
        pub unsafe fn altitude(&self) -> CLLocationDistance;

        #[unsafe(method(ellipsoidalAltitude))]
        #[unsafe(method_family = none)]
        pub unsafe fn ellipsoidalAltitude(&self) -> CLLocationDistance;

        #[unsafe(method(horizontalAccuracy))]
        #[unsafe(method_family = none)]
        pub unsafe fn horizontalAccuracy(&self) -> CLLocationAccuracy;

        #[unsafe(method(verticalAccuracy))]
        #[unsafe(method_family = none)]
        pub unsafe fn verticalAccuracy(&self) -> CLLocationAccuracy;

        #[unsafe(method(course))]
        #[unsafe(method_family = none)]
        pub unsafe fn course(&self) -> CLLocationDirection;

        #[unsafe(method(courseAccuracy))]
        #[unsafe(method_family = none)]
        pub unsafe fn courseAccuracy(&self) -> CLLocationDirectionAccuracy;

        #[unsafe(method(speed))]
        #[unsafe(method_family = none)]
        pub unsafe fn speed(&self) -> CLLocationSpeed;

        #[unsafe(method(speedAccuracy))]
        #[unsafe(method_family = none)]
        pub unsafe fn speedAccuracy(&self) -> CLLocationSpeedAccuracy;

        #[unsafe(method(timestamp))]
        #[unsafe(method_family = none)]
        pub unsafe fn timestamp(&self) -> Retained<NSDate>;

        #[unsafe(method(floor))]
        #[unsafe(method_family = none)]
        pub unsafe fn floor(&self) -> Option<Retained<CLFloor>>;

        #[unsafe(method(sourceInformation))]
        #[unsafe(method_family = none)]
        pub unsafe fn sourceInformation(&self) -> Option<Retained<CLLocationSourceInformation>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl CLLocation {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

#[deprecated = "renamed to `CLLocationCoordinate2D::is_valid`"]
#[inline]
pub unsafe extern "C-unwind" fn CLLocationCoordinate2DIsValid(
    coord: CLLocationCoordinate2D,
) -> bool {
    extern "C-unwind" {
        fn CLLocationCoordinate2DIsValid(coord: CLLocationCoordinate2D) -> Bool;
    }
    unsafe { CLLocationCoordinate2DIsValid(coord) }.as_bool()
}

extern "C-unwind" {
    #[deprecated = "renamed to `CLLocationCoordinate2D::new`"]
    pub fn CLLocationCoordinate2DMake(
        latitude: CLLocationDegrees,
        longitude: CLLocationDegrees,
    ) -> CLLocationCoordinate2D;
}
