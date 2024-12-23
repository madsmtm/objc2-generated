//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A class that represents a record of signpost instance.
    ///
    /// Signpost instances are either Signpost intervals or events and MXSignpostRecord captures information reagarding such signpost instances
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metrickit/mxsignpostrecord?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MXSignpostRecord;
);

unsafe impl NSCoding for MXSignpostRecord {}

unsafe impl NSObjectProtocol for MXSignpostRecord {}

unsafe impl NSSecureCoding for MXSignpostRecord {}

extern_methods!(
    unsafe impl MXSignpostRecord {
        /// An NSString representation of the subsystem of the signpost instance.
        #[method_id(@__retain_semantics Other subsystem)]
        pub unsafe fn subsystem(&self) -> Retained<NSString>;

        /// An NSString representation of the category of the signpost instance.
        #[method_id(@__retain_semantics Other category)]
        pub unsafe fn category(&self) -> Retained<NSString>;

        /// An NSString representation of the name of the signpost instance.
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        /// An NSDate representation of the begin time stamp of the signpost instance.
        #[method_id(@__retain_semantics Other beginTimeStamp)]
        pub unsafe fn beginTimeStamp(&self) -> Retained<NSDate>;

        /// An NSDate representation of the end time stamp of the signpost instances which are intervals and will be nil for signpost events.
        #[method_id(@__retain_semantics Other endTimeStamp)]
        pub unsafe fn endTimeStamp(&self) -> Option<Retained<NSDate>>;

        /// An NSMeasurement representing the duration in milliseconds of signpost instances which are intervals and will be nil for signpost events.
        #[method_id(@__retain_semantics Other duration)]
        pub unsafe fn duration(&self) -> Option<Retained<NSMeasurement<NSUnitDuration>>>;

        /// A BOOL denoting whether the signpost instance is an interval or not..
        #[method(isInterval)]
        pub unsafe fn isInterval(&self) -> bool;

        /// Convenience method to return a JSON representation of this SignpostRecord.
        ///
        /// Returns: An NSData object containing the JSON representation
        #[method_id(@__retain_semantics Other JSONRepresentation)]
        pub unsafe fn JSONRepresentation(&self) -> Retained<NSData>;

        /// Convenience method to return a NSDictionary representation of this SignpostRecord.
        ///
        /// Returns: An NSDictionary object containing the dictionary representation
        #[method_id(@__retain_semantics Other dictionaryRepresentation)]
        pub unsafe fn dictionaryRepresentation(&self) -> Retained<NSDictionary>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MXSignpostRecord {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
