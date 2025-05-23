//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A wrapper class which contains a metric payload and associated properties of that payload.
    ///
    /// MXMetricPayload encapsulates currently supported metric types that can be vended by MetricKit. MXMetric subclasses on MXMetricPayload are nullable. If an MXMetric subclass is nil, it indicates that the data is not available for this payload.
    ///
    /// MXMetricPayload exposes a convenience function, JSONRepresentation, to convert the contents of the payload to a human readable JSON. This should be used in conjunction with other APIs that accept NSData.
    ///
    /// An MXMetricPayload contains data that covers a 24 hour period of application usage. The properties timeStampBegin and timeStampEnd should be used to determine which time range the payload covers.
    ///
    /// It is possible for an MXMetricPayload to cover regions of time where an application was updated, and thus had multiple different app version strings. The property latestApplicationVersion will always reflect the latest appVersion at the time the metric payload was created. Use includesMultipleApplicationVersions to determine if an application changed versions during the time range the payload covers.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metrickit/mxmetricpayload?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MXMetricPayload;
);

extern_conformance!(
    unsafe impl NSCoding for MXMetricPayload {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MXMetricPayload {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for MXMetricPayload {}
);

impl MXMetricPayload {
    extern_methods!(
        /// An NSString representation of the application version from which this payload was generated.
        ///
        /// If the application version was changed during the aggregation of this data, this value will reflect the latest application version at the time of retrieval.
        #[unsafe(method(latestApplicationVersion))]
        #[unsafe(method_family = none)]
        pub unsafe fn latestApplicationVersion(&self) -> Retained<NSString>;

        /// A bool which indicates whether or not this payload contains data from multiple application versions.
        ///
        /// A value of YES indicates that this payload's data reflects multiple application versions.
        ///
        /// A value of NO indicates that this payload only reflects data from the application version specified by latestApplicationVersion.
        #[unsafe(method(includesMultipleApplicationVersions))]
        #[unsafe(method_family = none)]
        pub unsafe fn includesMultipleApplicationVersions(&self) -> bool;

        /// An NSDate object that indicates the time which the payload was generated.
        #[unsafe(method(timeStampBegin))]
        #[unsafe(method_family = none)]
        pub unsafe fn timeStampBegin(&self) -> Retained<NSDate>;

        /// An NSDate object that indicates the time which the payload was generated.
        #[unsafe(method(timeStampEnd))]
        #[unsafe(method_family = none)]
        pub unsafe fn timeStampEnd(&self) -> Retained<NSDate>;

        #[cfg(all(feature = "MXCPUMetric", feature = "MXMetric"))]
        /// An object containing CPU metrics for this application.
        #[unsafe(method(cpuMetrics))]
        #[unsafe(method_family = none)]
        pub unsafe fn cpuMetrics(&self) -> Option<Retained<MXCPUMetric>>;

        #[cfg(all(feature = "MXGPUMetric", feature = "MXMetric"))]
        /// An object containing GPU metrics for this application.
        #[unsafe(method(gpuMetrics))]
        #[unsafe(method_family = none)]
        pub unsafe fn gpuMetrics(&self) -> Option<Retained<MXGPUMetric>>;

        #[cfg(all(feature = "MXCellularConditionMetric", feature = "MXMetric"))]
        /// An object containing a cellular condition metrics for this application.
        #[unsafe(method(cellularConditionMetrics))]
        #[unsafe(method_family = none)]
        pub unsafe fn cellularConditionMetrics(
            &self,
        ) -> Option<Retained<MXCellularConditionMetric>>;

        #[cfg(all(feature = "MXAppRunTimeMetric", feature = "MXMetric"))]
        /// An object containing running mode metrics for this application.
        #[unsafe(method(applicationTimeMetrics))]
        #[unsafe(method_family = none)]
        pub unsafe fn applicationTimeMetrics(&self) -> Option<Retained<MXAppRunTimeMetric>>;

        #[cfg(all(feature = "MXLocationActivityMetric", feature = "MXMetric"))]
        /// An object containing location activity metrics for this application.
        #[unsafe(method(locationActivityMetrics))]
        #[unsafe(method_family = none)]
        pub unsafe fn locationActivityMetrics(&self) -> Option<Retained<MXLocationActivityMetric>>;

        #[cfg(all(feature = "MXMetric", feature = "MXNetworkTransferMetric"))]
        /// An object containing network transfer metrics for this application.
        #[unsafe(method(networkTransferMetrics))]
        #[unsafe(method_family = none)]
        pub unsafe fn networkTransferMetrics(&self) -> Option<Retained<MXNetworkTransferMetric>>;

        #[cfg(all(feature = "MXAppLaunchMetric", feature = "MXMetric"))]
        /// An object containing launch metrics for this application.
        #[unsafe(method(applicationLaunchMetrics))]
        #[unsafe(method_family = none)]
        pub unsafe fn applicationLaunchMetrics(&self) -> Option<Retained<MXAppLaunchMetric>>;

        #[cfg(all(feature = "MXAppResponsivenessMetric", feature = "MXMetric"))]
        /// An object containing hang metrics for this application.
        #[unsafe(method(applicationResponsivenessMetrics))]
        #[unsafe(method_family = none)]
        pub unsafe fn applicationResponsivenessMetrics(
            &self,
        ) -> Option<Retained<MXAppResponsivenessMetric>>;

        #[cfg(all(feature = "MXDiskIOMetric", feature = "MXMetric"))]
        /// An object containing disk IO metrics for this application.
        #[unsafe(method(diskIOMetrics))]
        #[unsafe(method_family = none)]
        pub unsafe fn diskIOMetrics(&self) -> Option<Retained<MXDiskIOMetric>>;

        #[cfg(all(feature = "MXMemoryMetric", feature = "MXMetric"))]
        /// An object containing memory metrics for this application.
        #[unsafe(method(memoryMetrics))]
        #[unsafe(method_family = none)]
        pub unsafe fn memoryMetrics(&self) -> Option<Retained<MXMemoryMetric>>;

        #[cfg(all(feature = "MXDisplayMetric", feature = "MXMetric"))]
        /// An object containing display metrics for this application.
        #[unsafe(method(displayMetrics))]
        #[unsafe(method_family = none)]
        pub unsafe fn displayMetrics(&self) -> Option<Retained<MXDisplayMetric>>;

        #[cfg(all(feature = "MXAnimationMetric", feature = "MXMetric"))]
        /// An object containing animation metrics for this application.
        #[unsafe(method(animationMetrics))]
        #[unsafe(method_family = none)]
        pub unsafe fn animationMetrics(&self) -> Option<Retained<MXAnimationMetric>>;

        #[cfg(all(feature = "MXAppExitMetric", feature = "MXMetric"))]
        /// An object containing exit metrics for this application.
        #[unsafe(method(applicationExitMetrics))]
        #[unsafe(method_family = none)]
        pub unsafe fn applicationExitMetrics(&self) -> Option<Retained<MXAppExitMetric>>;

        #[cfg(all(feature = "MXMetric", feature = "MXSignpostMetric"))]
        /// An array containing signpost metrics for this application.
        #[unsafe(method(signpostMetrics))]
        #[unsafe(method_family = none)]
        pub unsafe fn signpostMetrics(&self) -> Option<Retained<NSArray<MXSignpostMetric>>>;

        #[cfg(feature = "MXMetaData")]
        /// An object containing extra metadata for this payload.
        #[unsafe(method(metaData))]
        #[unsafe(method_family = none)]
        pub unsafe fn metaData(&self) -> Option<Retained<MXMetaData>>;

        /// Convenience method to return a JSON representation of this payload.
        ///
        /// Returns: An NSData object containing the JSON representation
        #[unsafe(method(JSONRepresentation))]
        #[unsafe(method_family = none)]
        pub unsafe fn JSONRepresentation(&self) -> Retained<NSData>;

        /// Convenience method to return a NSDictionary representation of this payload.
        ///
        /// Returns: An NSDictionary object containing the dictionary representation
        #[deprecated]
        #[unsafe(method(DictionaryRepresentation))]
        #[unsafe(method_family = none)]
        pub unsafe fn DictionaryRepresentation(&self) -> Retained<NSDictionary>;

        /// Convenience method to return a NSDictionary representation of this payload.
        ///
        /// Returns: An NSDictionary object containing the dictionary representation
        #[unsafe(method(dictionaryRepresentation))]
        #[unsafe(method_family = none)]
        pub unsafe fn dictionaryRepresentation(&self) -> Retained<NSDictionary>;
    );
}

/// Methods declared on superclass `NSObject`.
impl MXMetricPayload {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
