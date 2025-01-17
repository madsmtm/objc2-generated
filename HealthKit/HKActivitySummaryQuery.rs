//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkactivitysummaryquery?language=objc)
    #[unsafe(super(HKQuery, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HKQuery")]
    pub struct HKActivitySummaryQuery;
);

#[cfg(feature = "HKQuery")]
unsafe impl Send for HKActivitySummaryQuery {}

#[cfg(feature = "HKQuery")]
unsafe impl Sync for HKActivitySummaryQuery {}

#[cfg(feature = "HKQuery")]
unsafe impl NSObjectProtocol for HKActivitySummaryQuery {}

extern_methods!(
    #[cfg(feature = "HKQuery")]
    unsafe impl HKActivitySummaryQuery {
        #[cfg(all(feature = "HKActivitySummary", feature = "block2"))]
        /// An optional handler to be called when activity summaries matching the given predicate are updated.
        ///
        /// This property may not be modified once the query has been executed. If this property is nonnull, then
        /// the query must be manually stopped.
        #[method(updateHandler)]
        pub unsafe fn updateHandler(
            &self,
        ) -> *mut block2::Block<
            dyn Fn(NonNull<HKActivitySummaryQuery>, *mut NSArray<HKActivitySummary>, *mut NSError),
        >;

        #[cfg(all(feature = "HKActivitySummary", feature = "block2"))]
        /// Setter for [`updateHandler`][Self::updateHandler].
        #[method(setUpdateHandler:)]
        pub unsafe fn setUpdateHandler(
            &self,
            update_handler: Option<
                &block2::Block<
                    dyn Fn(
                        NonNull<HKActivitySummaryQuery>,
                        *mut NSArray<HKActivitySummary>,
                        *mut NSError,
                    ),
                >,
            >,
        );

        #[cfg(all(feature = "HKActivitySummary", feature = "block2"))]
        /// Returns a query that will retrieve HKActivitySummaries matching the given predicate.
        ///
        /// If no updateHandler is set on the query, the query will automatically stop after calling resultsHandler.
        /// Otherwise, the query continues to run and calls the updateHandler as HKActivitySummaries matching the
        /// predicate are updated.
        ///
        /// Parameter `predicate`: The predicate which HKActivitySummaries should match.
        ///
        /// Parameter `handler`: The block to invoke with results when the query has finished.
        #[unsafe(method_family(init))]
        #[method_id(initWithPredicate:resultsHandler:)]
        pub unsafe fn initWithPredicate_resultsHandler(
            this: Allocated<Self>,
            predicate: Option<&NSPredicate>,
            handler: &block2::Block<
                dyn Fn(
                    NonNull<HKActivitySummaryQuery>,
                    *mut NSArray<HKActivitySummary>,
                    *mut NSError,
                ),
            >,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKQuery`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKActivitySummaryQuery {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKActivitySummaryQuery {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
