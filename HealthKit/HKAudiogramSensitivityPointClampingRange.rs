//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Defines the range within which an ear's sensitivity point may have been clamped, if any.
    ///
    /// At times, it may be required to indicate that a sensitivity point has been clamped to a range. These reasons include but
    /// are not limited to user safety, hardware limitations, or algorithm features.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkaudiogramsensitivitypointclampingrange?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKAudiogramSensitivityPointClampingRange;
);

unsafe impl Send for HKAudiogramSensitivityPointClampingRange {}

unsafe impl Sync for HKAudiogramSensitivityPointClampingRange {}

extern_conformance!(
    unsafe impl NSCoding for HKAudiogramSensitivityPointClampingRange {}
);

extern_conformance!(
    unsafe impl NSCopying for HKAudiogramSensitivityPointClampingRange {}
);

unsafe impl CopyingHelper for HKAudiogramSensitivityPointClampingRange {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for HKAudiogramSensitivityPointClampingRange {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for HKAudiogramSensitivityPointClampingRange {}
);

impl HKAudiogramSensitivityPointClampingRange {
    extern_methods!(
        #[cfg(feature = "HKQuantity")]
        /// The lower bound of the clamping range, if any, in dBHL.
        #[unsafe(method(lowerBound))]
        #[unsafe(method_family = none)]
        pub unsafe fn lowerBound(&self) -> Option<Retained<HKQuantity>>;

        #[cfg(feature = "HKQuantity")]
        /// The upper bound of the clamping range, if any, in dBHL.
        #[unsafe(method(upperBound))]
        #[unsafe(method_family = none)]
        pub unsafe fn upperBound(&self) -> Option<Retained<HKQuantity>>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Creates a clamping range from a given lower and upper bound. At least one bound must be specified. If both bounds are provided, the lower bound must be less than the upper bound.
        ///
        /// Parameter `lowerBound`: The lower bound of the clamping range (if any)
        ///
        /// Parameter `upperBound`: The upper bound of the clamping range (if any)
        ///
        /// Parameter `errorOut`: If there was a problem creating this instance this will contain the error.
        ///
        /// Returns: New instance of a clamping range or nil if there were problems
        /// creating the instance.  Errors may include not having any bound or lower bound is greater than the upper bound
        #[unsafe(method(clampingRangeWithLowerBound:upperBound:error:_))]
        #[unsafe(method_family = none)]
        pub unsafe fn clampingRangeWithLowerBound_upperBound_error(
            lower_bound: Option<&NSNumber>,
            upper_bound: Option<&NSNumber>,
        ) -> Result<Retained<Self>, Retained<NSError>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl HKAudiogramSensitivityPointClampingRange {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
