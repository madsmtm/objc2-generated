//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// Options for specifying which statistics to calculate
///
/// When querying for HKStatistics objects, an options bitmask will specify which statistics will be
/// calculated.
///
/// Statistics are classified as discrete or cumulative.  If a discrete statistics option is specified for a
/// cumulative HKQuantityType, an exception will be thrown.  If a cumulative statistics options is specified
/// for a discrete HKQuantityType, an exception will also be thrown.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkstatisticsoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKStatisticsOptions(pub NSUInteger);
bitflags::bitflags! {
    impl HKStatisticsOptions: NSUInteger {
        #[doc(alias = "HKStatisticsOptionNone")]
        const None = 0;
        #[doc(alias = "HKStatisticsOptionSeparateBySource")]
        const SeparateBySource = 1<<0;
        #[doc(alias = "HKStatisticsOptionDiscreteAverage")]
        const DiscreteAverage = 1<<1;
        #[doc(alias = "HKStatisticsOptionDiscreteMin")]
        const DiscreteMin = 1<<2;
        #[doc(alias = "HKStatisticsOptionDiscreteMax")]
        const DiscreteMax = 1<<3;
        #[doc(alias = "HKStatisticsOptionCumulativeSum")]
        const CumulativeSum = 1<<4;
        #[doc(alias = "HKStatisticsOptionMostRecent")]
        const MostRecent = 1<<5;
#[deprecated]
        #[doc(alias = "HKStatisticsOptionDiscreteMostRecent")]
        const DiscreteMostRecent = HKStatisticsOptions::MostRecent.0;
        #[doc(alias = "HKStatisticsOptionDuration")]
        const Duration = 1<<6;
    }
}

unsafe impl Encode for HKStatisticsOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for HKStatisticsOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// Represents statistics for quantity samples over a period of time.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkstatistics?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKStatistics;
);

unsafe impl Send for HKStatistics {}

unsafe impl Sync for HKStatistics {}

unsafe impl NSCoding for HKStatistics {}

unsafe impl NSCopying for HKStatistics {}

unsafe impl CopyingHelper for HKStatistics {
    type Result = Self;
}

unsafe impl NSObjectProtocol for HKStatistics {}

unsafe impl NSSecureCoding for HKStatistics {}

extern_methods!(
    unsafe impl HKStatistics {
        #[cfg(feature = "HKObjectType")]
        #[unsafe(method_family(none))]
        #[method_id(quantityType)]
        pub unsafe fn quantityType(&self) -> Retained<HKQuantityType>;

        #[unsafe(method_family(none))]
        #[method_id(startDate)]
        pub unsafe fn startDate(&self) -> Retained<NSDate>;

        #[unsafe(method_family(none))]
        #[method_id(endDate)]
        pub unsafe fn endDate(&self) -> Retained<NSDate>;

        #[cfg(feature = "HKSource")]
        #[unsafe(method_family(none))]
        #[method_id(sources)]
        pub unsafe fn sources(&self) -> Option<Retained<NSArray<HKSource>>>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(feature = "HKQuantity", feature = "HKSource"))]
        /// Returns the average quantity for the given source in the time period represented by the receiver.
        ///
        /// If HKStatisticsOptionSeparateBySource is not specified, then this will always be nil.
        #[unsafe(method_family(none))]
        #[method_id(averageQuantityForSource:)]
        pub unsafe fn averageQuantityForSource(
            &self,
            source: &HKSource,
        ) -> Option<Retained<HKQuantity>>;

        #[cfg(feature = "HKQuantity")]
        /// Returns the average quantity in the time period represented by the receiver.
        #[unsafe(method_family(none))]
        #[method_id(averageQuantity)]
        pub unsafe fn averageQuantity(&self) -> Option<Retained<HKQuantity>>;

        #[cfg(all(feature = "HKQuantity", feature = "HKSource"))]
        /// Returns the minimum quantity for the given source in the time period represented by the receiver.
        ///
        /// If HKStatisticsOptionSeparateBySource is not specified, then this will always be nil.
        #[unsafe(method_family(none))]
        #[method_id(minimumQuantityForSource:)]
        pub unsafe fn minimumQuantityForSource(
            &self,
            source: &HKSource,
        ) -> Option<Retained<HKQuantity>>;

        #[cfg(feature = "HKQuantity")]
        /// Returns the minimum quantity in the time period represented by the receiver.
        #[unsafe(method_family(none))]
        #[method_id(minimumQuantity)]
        pub unsafe fn minimumQuantity(&self) -> Option<Retained<HKQuantity>>;

        #[cfg(all(feature = "HKQuantity", feature = "HKSource"))]
        /// Returns the maximum quantity for the given source in the time period represented by the receiver.
        ///
        /// If HKStatisticsOptionSeparateBySource is not specified, then this will always be nil.
        #[unsafe(method_family(none))]
        #[method_id(maximumQuantityForSource:)]
        pub unsafe fn maximumQuantityForSource(
            &self,
            source: &HKSource,
        ) -> Option<Retained<HKQuantity>>;

        #[cfg(feature = "HKQuantity")]
        /// Returns the maximum quantity in the time period represented by the receiver.
        #[unsafe(method_family(none))]
        #[method_id(maximumQuantity)]
        pub unsafe fn maximumQuantity(&self) -> Option<Retained<HKQuantity>>;

        #[cfg(all(feature = "HKQuantity", feature = "HKSource"))]
        /// Returns the most recent quantity for the given source in the time period represented by the receiver.
        ///
        /// If HKStatisticsOptionSeparateBySource is not specified, then this will always be nil.
        #[unsafe(method_family(none))]
        #[method_id(mostRecentQuantityForSource:)]
        pub unsafe fn mostRecentQuantityForSource(
            &self,
            source: &HKSource,
        ) -> Option<Retained<HKQuantity>>;

        #[cfg(feature = "HKQuantity")]
        /// Returns the most recent quantity in the time period represented by the receiver.
        #[unsafe(method_family(none))]
        #[method_id(mostRecentQuantity)]
        pub unsafe fn mostRecentQuantity(&self) -> Option<Retained<HKQuantity>>;

        #[cfg(feature = "HKSource")]
        /// Returns the date interval of the most recent quantity for the given source in the time period
        /// represented by the receiver.
        ///
        /// If HKStatisticsOptionSeparateBySource is not specified, then this will always be nil.
        #[unsafe(method_family(none))]
        #[method_id(mostRecentQuantityDateIntervalForSource:)]
        pub unsafe fn mostRecentQuantityDateIntervalForSource(
            &self,
            source: &HKSource,
        ) -> Option<Retained<NSDateInterval>>;

        /// Returns the date interval of the most recent quantity in the time period represented by the receiver.
        #[unsafe(method_family(none))]
        #[method_id(mostRecentQuantityDateInterval)]
        pub unsafe fn mostRecentQuantityDateInterval(&self) -> Option<Retained<NSDateInterval>>;

        #[cfg(all(feature = "HKQuantity", feature = "HKSource"))]
        /// Returns the sum quantity for the given source in the time period represented by the receiver.
        ///
        /// If HKStatisticsOptionSeparateBySource is not specified, then this will always be nil.
        #[unsafe(method_family(none))]
        #[method_id(sumQuantityForSource:)]
        pub unsafe fn sumQuantityForSource(
            &self,
            source: &HKSource,
        ) -> Option<Retained<HKQuantity>>;

        #[cfg(feature = "HKQuantity")]
        /// Returns the sum of quantities in the time period represented by the receiver.
        #[unsafe(method_family(none))]
        #[method_id(sumQuantity)]
        pub unsafe fn sumQuantity(&self) -> Option<Retained<HKQuantity>>;

        #[cfg(feature = "HKQuantity")]
        /// Total duration (in seconds) covered by the samples represented by these statistics.
        /// Only present if HKStatisticsOptionDuration is is specified.
        ///
        ///
        ///
        /// Total duration, as a time-unit compatible quantity, covered by the samples represented by these statistics.
        ///
        /// Only present if HKStatisticsOptionDuration is is specified.
        #[unsafe(method_family(none))]
        #[method_id(duration)]
        pub unsafe fn duration(&self) -> Option<Retained<HKQuantity>>;

        #[cfg(all(feature = "HKQuantity", feature = "HKSource"))]
        /// Returns the duration, as a time-unit compatible quantity, for the given source in the time period represented by the receiver.
        ///
        /// If HKStatisticsOptionSeparateBySource is not specified, then this will always be nil.
        #[unsafe(method_family(none))]
        #[method_id(durationForSource:)]
        pub unsafe fn durationForSource(&self, source: &HKSource) -> Option<Retained<HKQuantity>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKStatistics {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
