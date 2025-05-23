//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkcumulativequantityseriessample?language=objc)
    #[unsafe(super(
        HKCumulativeQuantitySample,
        HKQuantitySample,
        HKSample,
        HKObject,
        NSObject
    ))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "HKCumulativeQuantitySample",
        feature = "HKObject",
        feature = "HKQuantitySample",
        feature = "HKSample"
    ))]
    #[deprecated]
    pub struct HKCumulativeQuantitySeriesSample;
);

#[cfg(all(
    feature = "HKCumulativeQuantitySample",
    feature = "HKObject",
    feature = "HKQuantitySample",
    feature = "HKSample"
))]
unsafe impl Send for HKCumulativeQuantitySeriesSample {}

#[cfg(all(
    feature = "HKCumulativeQuantitySample",
    feature = "HKObject",
    feature = "HKQuantitySample",
    feature = "HKSample"
))]
unsafe impl Sync for HKCumulativeQuantitySeriesSample {}

#[cfg(all(
    feature = "HKCumulativeQuantitySample",
    feature = "HKObject",
    feature = "HKQuantitySample",
    feature = "HKSample"
))]
extern_conformance!(
    unsafe impl NSCoding for HKCumulativeQuantitySeriesSample {}
);

#[cfg(all(
    feature = "HKCumulativeQuantitySample",
    feature = "HKObject",
    feature = "HKQuantitySample",
    feature = "HKSample"
))]
extern_conformance!(
    unsafe impl NSObjectProtocol for HKCumulativeQuantitySeriesSample {}
);

#[cfg(all(
    feature = "HKCumulativeQuantitySample",
    feature = "HKObject",
    feature = "HKQuantitySample",
    feature = "HKSample"
))]
extern_conformance!(
    unsafe impl NSSecureCoding for HKCumulativeQuantitySeriesSample {}
);

#[cfg(all(
    feature = "HKCumulativeQuantitySample",
    feature = "HKObject",
    feature = "HKQuantitySample",
    feature = "HKSample"
))]
impl HKCumulativeQuantitySeriesSample {
    extern_methods!(
        #[cfg(feature = "HKQuantity")]
        #[deprecated]
        #[unsafe(method(sum))]
        #[unsafe(method_family = none)]
        pub unsafe fn sum(&self) -> Retained<HKQuantity>;
    );
}

/// Methods declared on superclass `HKQuantitySample`.
#[cfg(all(
    feature = "HKCumulativeQuantitySample",
    feature = "HKObject",
    feature = "HKQuantitySample",
    feature = "HKSample"
))]
impl HKCumulativeQuantitySeriesSample {
    extern_methods!(
        #[cfg(all(feature = "HKObjectType", feature = "HKQuantity"))]
        /// Creates a new HKQuantitySample with the given type, quantity, start date, and end date.
        ///
        /// The quantity must have a unit that is compatible with the given quantity type.
        /// See -[HKQuantityType isCompatibleWithUnit:].
        #[unsafe(method(quantitySampleWithType:quantity:startDate:endDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn quantitySampleWithType_quantity_startDate_endDate(
            quantity_type: &HKQuantityType,
            quantity: &HKQuantity,
            start_date: &NSDate,
            end_date: &NSDate,
        ) -> Retained<Self>;

        #[cfg(all(feature = "HKObjectType", feature = "HKQuantity"))]
        /// Creates a new HKQuantitySample with the given type, quantity, start date, end date, and metadata.
        ///
        /// The quantity must have a unit that is compatible with the given quantity type.
        /// See -[HKQuantityType isCompatibleWithUnit:].
        #[unsafe(method(quantitySampleWithType:quantity:startDate:endDate:metadata:))]
        #[unsafe(method_family = none)]
        pub unsafe fn quantitySampleWithType_quantity_startDate_endDate_metadata(
            quantity_type: &HKQuantityType,
            quantity: &HKQuantity,
            start_date: &NSDate,
            end_date: &NSDate,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "HKDevice", feature = "HKObjectType", feature = "HKQuantity"))]
        /// Creates a new HKQuantitySample with the given type, quantity, start date, end date, and metadata.
        ///
        /// Parameter `quantityType`: The type of the sample.
        ///
        /// Parameter `startDate`: The start date of the sample.
        ///
        /// Parameter `endDate`: The end date of the sample.
        ///
        /// Parameter `device`: The HKDevice that generated the sample (optional).
        ///
        /// Parameter `metadata`: Metadata for the sample (optional).
        ///
        /// The quantity must have a unit that is compatible with the given quantity type.
        /// See -[HKQuantityType isCompatibleWithUnit:].
        #[unsafe(method(quantitySampleWithType:quantity:startDate:endDate:device:metadata:))]
        #[unsafe(method_family = none)]
        pub unsafe fn quantitySampleWithType_quantity_startDate_endDate_device_metadata(
            quantity_type: &HKQuantityType,
            quantity: &HKQuantity,
            start_date: &NSDate,
            end_date: &NSDate,
            device: Option<&HKDevice>,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `HKObject`.
#[cfg(all(
    feature = "HKCumulativeQuantitySample",
    feature = "HKObject",
    feature = "HKQuantitySample",
    feature = "HKSample"
))]
impl HKCumulativeQuantitySeriesSample {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(
    feature = "HKCumulativeQuantitySample",
    feature = "HKObject",
    feature = "HKQuantitySample",
    feature = "HKSample"
))]
impl HKCumulativeQuantitySeriesSample {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
