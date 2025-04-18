//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Abstract class representing a series of detections of the same subject over time.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/cinematic/cndetectiontrack?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNDetectionTrack;
);

extern_conformance!(
    unsafe impl NSCopying for CNDetectionTrack {}
);

unsafe impl CopyingHelper for CNDetectionTrack {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for CNDetectionTrack {}
);

impl CNDetectionTrack {
    extern_methods!(
        #[cfg(feature = "CNDetection")]
        /// The type of subject detected by this detection track.
        #[unsafe(method(detectionType))]
        #[unsafe(method_family = none)]
        pub unsafe fn detectionType(&self) -> CNDetectionType;

        #[cfg(feature = "CNDetection")]
        /// The detectionID of the subject detected during this track; unique within a cinematic script.
        #[unsafe(method(detectionID))]
        #[unsafe(method_family = none)]
        pub unsafe fn detectionID(&self) -> CNDetectionID;

        #[cfg(feature = "CNDetection")]
        /// The detectionGroupID of the subject detected by the track.
        ///
        /// The detectionGroupID can be used to associate related detections such as the face and torso of the same person.
        #[unsafe(method(detectionGroupID))]
        #[unsafe(method_family = none)]
        pub unsafe fn detectionGroupID(&self) -> CNDetectionGroupID;

        /// Whether this detection track was created by the client.
        #[unsafe(method(isUserCreated))]
        #[unsafe(method_family = none)]
        pub unsafe fn isUserCreated(&self) -> bool;

        /// Whether this detection track has discrete detections (otherwise continuous).
        ///
        /// A discrete detection track will return detections only at the specific times a detection occurs.
        /// A continuous detection track will return a detection for any requested time and an empty array for time ranges.
        #[unsafe(method(isDiscrete))]
        #[unsafe(method_family = none)]
        pub unsafe fn isDiscrete(&self) -> bool;

        #[cfg(all(feature = "CNDetection", feature = "objc2-core-media"))]
        #[unsafe(method(detectionAtOrBeforeTime:))]
        #[unsafe(method_family = none)]
        pub unsafe fn detectionAtOrBeforeTime(&self, time: CMTime)
            -> Option<Retained<CNDetection>>;

        #[cfg(all(feature = "CNDetection", feature = "objc2-core-media"))]
        #[unsafe(method(detectionNearestTime:))]
        #[unsafe(method_family = none)]
        pub unsafe fn detectionNearestTime(&self, time: CMTime) -> Option<Retained<CNDetection>>;

        #[cfg(all(feature = "CNDetection", feature = "objc2-core-media"))]
        /// Gets the array of detections in the detection track within the given time range. Makes sense for discrete detection tracks only.
        #[unsafe(method(detectionsInTimeRange:))]
        #[unsafe(method_family = none)]
        pub unsafe fn detectionsInTimeRange(
            &self,
            time_range: CMTimeRange,
        ) -> Retained<NSArray<CNDetection>>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// A continuous detection track representing focus at a fixed disparity.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/cinematic/cnfixeddetectiontrack?language=objc)
    #[unsafe(super(CNDetectionTrack, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNFixedDetectionTrack;
);

extern_conformance!(
    unsafe impl NSCopying for CNFixedDetectionTrack {}
);

unsafe impl CopyingHelper for CNFixedDetectionTrack {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for CNFixedDetectionTrack {}
);

impl CNFixedDetectionTrack {
    extern_methods!(
        /// Create a detection track with fixed focus at the given disparity.
        #[unsafe(method(initWithFocusDisparity:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithFocusDisparity(
            this: Allocated<Self>,
            focus_disparity: c_float,
        ) -> Retained<Self>;

        #[cfg(feature = "CNDetection")]
        /// Create a detection track with fixed focus at the disparity of an existing detection.
        #[unsafe(method(initWithOriginalDetection:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithOriginalDetection(
            this: Allocated<Self>,
            original_detection: &CNDetection,
        ) -> Retained<Self>;

        #[unsafe(method(focusDisparity))]
        #[unsafe(method_family = none)]
        pub unsafe fn focusDisparity(&self) -> c_float;

        #[cfg(feature = "CNDetection")]
        /// The original detection upon which this fixed detection track was based, if any.
        ///
        /// This is the way to determine the time and rect from which fixed focus originated, if any.
        /// This detection is not part of the detection track and has a different detectionID or none.
        ///
        /// - Important: To get a detection from the fixed detection track, use detectionAtOrBeforeTime: instead, which will return a properly time-stamped detection.
        #[unsafe(method(originalDetection))]
        #[unsafe(method_family = none)]
        pub unsafe fn originalDetection(&self) -> Option<Retained<CNDetection>>;
    );
}

/// Methods declared on superclass `CNDetectionTrack`.
impl CNFixedDetectionTrack {
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
    /// A discrete detection track composed of individual detections.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/cinematic/cncustomdetectiontrack?language=objc)
    #[unsafe(super(CNDetectionTrack, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CNCustomDetectionTrack;
);

extern_conformance!(
    unsafe impl NSCopying for CNCustomDetectionTrack {}
);

unsafe impl CopyingHelper for CNCustomDetectionTrack {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for CNCustomDetectionTrack {}
);

impl CNCustomDetectionTrack {
    extern_methods!(
        #[cfg(feature = "CNDetection")]
        /// Initialize a custom detection track with an array of detections, optionally applying smoothing.
        ///
        /// The smoothing algorithm used is the same one that is used for built-in detections during recording.
        /// It compensates for some amount of jitter in the disparity measure by smoothing out variability.
        #[unsafe(method(initWithDetections:smooth:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithDetections_smooth(
            this: Allocated<Self>,
            detections: &NSArray<CNDetection>,
            apply_smoothing: bool,
        ) -> Retained<Self>;

        #[cfg(feature = "CNDetection")]
        #[unsafe(method(allDetections))]
        #[unsafe(method_family = none)]
        pub unsafe fn allDetections(&self) -> Retained<NSArray<CNDetection>>;
    );
}

/// Methods declared on superclass `CNDetectionTrack`.
impl CNCustomDetectionTrack {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
