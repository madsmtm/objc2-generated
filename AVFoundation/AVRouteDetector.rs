//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// Posted when the value of multipleRoutesDetected changes.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avroutedetectormultipleroutesdetecteddidchangenotification?language=objc)
    pub static AVRouteDetectorMultipleRoutesDetectedDidChangeNotification:
        &'static NSNotificationName;
}

extern_class!(
    /// AVRouteDetector detects the presence of media playback routes.
    ///
    /// If route detection is enabled (it is disabled by default), AVRouteDetector reports whether or not multiple playback routes have been detected. If this is the case, AVKit's AVRoutePickerView can be used to allow users to pick from the set of available routes.
    ///
    /// Subclasses of this type that are used from Swift must fulfill the requirements of a Sendable type.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avroutedetector?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVRouteDetector;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for AVRouteDetector {}
);

impl AVRouteDetector {
    extern_methods!(
        /// Whether or not route detection is enabled. The default value is NO.
        ///
        /// Route detection significantly increases power consumption and must be turned off when it's no longer needed.
        #[unsafe(method(isRouteDetectionEnabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn isRouteDetectionEnabled(&self) -> bool;

        /// Setter for [`isRouteDetectionEnabled`][Self::isRouteDetectionEnabled].
        #[unsafe(method(setRouteDetectionEnabled:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRouteDetectionEnabled(&self, route_detection_enabled: bool);

        /// This property is YES if, in addition to the local playback route, at least one more playback route has been detected.
        ///
        /// If multiple route have been detected, AVKit's AVRoutePickerView can be used to allow users to pick from the set of available routes. When the values of this property changes AVRouteDetectorMultipleRoutesDetectedDidChangeNotification is posted.
        #[unsafe(method(multipleRoutesDetected))]
        #[unsafe(method_family = none)]
        pub unsafe fn multipleRoutesDetected(&self) -> bool;

        /// Whether or not route detection will include custom routes. The default value is NO.
        ///
        /// Only set this to YES if also using AVCustomRoutingController.
        #[unsafe(method(detectsCustomRoutes))]
        #[unsafe(method_family = none)]
        pub unsafe fn detectsCustomRoutes(&self) -> bool;

        /// Setter for [`detectsCustomRoutes`][Self::detectsCustomRoutes].
        #[unsafe(method(setDetectsCustomRoutes:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDetectsCustomRoutes(&self, detects_custom_routes: bool);
    );
}

/// Methods declared on superclass `NSObject`.
impl AVRouteDetector {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
