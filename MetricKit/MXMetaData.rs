//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A class that contains miscellaneous metadata about an associated payload.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metrickit/mxmetadata?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MXMetaData;
);

unsafe impl NSCoding for MXMetaData {}

unsafe impl NSObjectProtocol for MXMetaData {}

unsafe impl NSSecureCoding for MXMetaData {}

extern_methods!(
    unsafe impl MXMetaData {
        /// An NSString designating the region format associated with the application.
        #[unsafe(method_family(none))]
        #[method_id(regionFormat)]
        pub unsafe fn regionFormat(&self) -> Retained<NSString>;

        /// An NSString designating the OS version associated with the device.
        #[unsafe(method_family(none))]
        #[method_id(osVersion)]
        pub unsafe fn osVersion(&self) -> Retained<NSString>;

        /// An NSString designating the device type associated with this device.
        #[unsafe(method_family(none))]
        #[method_id(deviceType)]
        pub unsafe fn deviceType(&self) -> Retained<NSString>;

        /// An NSString designating the app build version.
        #[unsafe(method_family(none))]
        #[method_id(applicationBuildVersion)]
        pub unsafe fn applicationBuildVersion(&self) -> Retained<NSString>;

        /// An NSString designating the current architecture.
        #[unsafe(method_family(none))]
        #[method_id(platformArchitecture)]
        pub unsafe fn platformArchitecture(&self) -> Retained<NSString>;

        /// A boolean representing low power mode enablement on device
        #[method(lowPowerModeEnabled)]
        pub unsafe fn lowPowerModeEnabled(&self) -> bool;

        /// A boolean representing if the app is registered as a testFlightApp
        #[method(isTestFlightApp)]
        pub unsafe fn isTestFlightApp(&self) -> bool;

        #[cfg(feature = "libc")]
        /// pid of the process
        ///
        /// Note: A value of -1 indicates that the PID was unavailable for the containing payload.
        #[method(pid)]
        pub unsafe fn pid(&self) -> libc::pid_t;

        /// Convenience method to return a JSON representation of this metadata.
        ///
        /// Returns: An NSData object containing the JSON representation
        #[unsafe(method_family(none))]
        #[method_id(JSONRepresentation)]
        pub unsafe fn JSONRepresentation(&self) -> Retained<NSData>;

        /// Convenience method to return a NSDictionary representation of this metadata.
        ///
        /// Returns: An NSDictionary object containing the dictionary representation
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(DictionaryRepresentation)]
        pub unsafe fn DictionaryRepresentation(&self) -> Retained<NSDictionary>;

        /// Convenience method to return a NSDictionary representation of this metadata.
        ///
        /// Returns: An NSDictionary object containing the dictionary representation
        #[unsafe(method_family(none))]
        #[method_id(dictionaryRepresentation)]
        pub unsafe fn dictionaryRepresentation(&self) -> Retained<NSDictionary>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MXMetaData {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
