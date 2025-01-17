//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkworkouteffortrelationship?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HKWorkoutEffortRelationship;
);

unsafe impl Send for HKWorkoutEffortRelationship {}

unsafe impl Sync for HKWorkoutEffortRelationship {}

unsafe impl NSCoding for HKWorkoutEffortRelationship {}

unsafe impl NSCopying for HKWorkoutEffortRelationship {}

unsafe impl CopyingHelper for HKWorkoutEffortRelationship {
    type Result = Self;
}

unsafe impl NSObjectProtocol for HKWorkoutEffortRelationship {}

unsafe impl NSSecureCoding for HKWorkoutEffortRelationship {}

extern_methods!(
    unsafe impl HKWorkoutEffortRelationship {
        #[cfg(all(feature = "HKObject", feature = "HKSample", feature = "HKWorkout"))]
        #[unsafe(method_family(none))]
        #[method_id(workout)]
        pub unsafe fn workout(&self) -> Retained<HKWorkout>;

        #[cfg(feature = "HKWorkoutActivity")]
        #[unsafe(method_family(none))]
        #[method_id(activity)]
        pub unsafe fn activity(&self) -> Option<Retained<HKWorkoutActivity>>;

        #[cfg(all(feature = "HKObject", feature = "HKSample"))]
        /// The samples related to the workout but not any sub-activities
        #[unsafe(method_family(none))]
        #[method_id(samples)]
        pub unsafe fn samples(&self) -> Option<Retained<NSArray<HKSample>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl HKWorkoutEffortRelationship {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// Option for specifying which workout effort relationship sample(s) to retrieve
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkworkouteffortrelationshipqueryoptions?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HKWorkoutEffortRelationshipQueryOptions(pub NSInteger);
impl HKWorkoutEffortRelationshipQueryOptions {
    #[doc(alias = "HKWorkoutEffortRelationshipQueryOptionsDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "HKWorkoutEffortRelationshipQueryOptionsMostRelevant")]
    pub const MostRelevant: Self = Self(1 << 0);
}

unsafe impl Encode for HKWorkoutEffortRelationshipQueryOptions {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for HKWorkoutEffortRelationshipQueryOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// A concrete subclass of HKQuery that provides an interface to observe associations with a workout sample.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkworkouteffortrelationshipquery?language=objc)
    #[unsafe(super(HKQuery, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HKQuery")]
    pub struct HKWorkoutEffortRelationshipQuery;
);

#[cfg(feature = "HKQuery")]
unsafe impl Send for HKWorkoutEffortRelationshipQuery {}

#[cfg(feature = "HKQuery")]
unsafe impl Sync for HKWorkoutEffortRelationshipQuery {}

#[cfg(feature = "HKQuery")]
unsafe impl NSObjectProtocol for HKWorkoutEffortRelationshipQuery {}

extern_methods!(
    #[cfg(feature = "HKQuery")]
    unsafe impl HKWorkoutEffortRelationshipQuery {
        #[cfg(all(feature = "HKQueryAnchor", feature = "block2"))]
        /// Returns a query that will retrieve HKWorkoutEffortRelationship matching the given predicate that are
        /// newer than the given anchor.
        ///
        /// This is a long running query and it is the responsibility of the caller to stop the query
        /// after they have received the results they desire.
        /// The first call to resultsHandler will contain the inital results which may be empty and future callbacks
        /// will contain new relationships as well as any changes to previous relationships along with a new anchor
        ///
        /// Parameter `predicate`: The predicate on the workout(s) which samples should match.
        ///
        /// Parameter `anchor`: The anchor which was returned by a previous HKWorkoutEffortRelationshipQuery result or update
        /// handler.  Pass nil when querying for the first time.
        ///
        /// Parameter `options`: The options for the query, one of types from `HKWorkoutEffortRelationshipQueryOptions`
        ///
        /// Parameter `resultsHandler`: The block to invoke with related sample results
        #[unsafe(method_family(init))]
        #[method_id(initWithPredicate:anchor:options:resultsHandler:)]
        pub unsafe fn initWithPredicate_anchor_options_resultsHandler(
            this: Allocated<Self>,
            predicate: Option<&NSPredicate>,
            anchor: Option<&HKQueryAnchor>,
            options: HKWorkoutEffortRelationshipQueryOptions,
            results_handler: &block2::Block<
                dyn Fn(
                    NonNull<HKWorkoutEffortRelationshipQuery>,
                    *mut NSArray<HKWorkoutEffortRelationship>,
                    *mut HKQueryAnchor,
                    *mut NSError,
                ),
            >,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKQuery`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKWorkoutEffortRelationshipQuery {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKWorkoutEffortRelationshipQuery {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
