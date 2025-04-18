//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Use CKFetchRecordZoneChangesOperation instead of this class.
    ///
    /// Any serverChangeTokens saved from a CKFetchRecordChangesOperation are usable as a serverRecordZoneChangeToken in CKFetchRecordZoneChangesOperation
    ///
    /// This operation will fetch records changes in the given record zone.
    ///
    /// If a change token from a previous
    /// `CKFetchRecordChangesOperation`is passed in, only the records that have changed since that token will be fetched.
    /// If this is your first fetch or if you wish to re-fetch all records, pass nil for the change token.
    /// Change tokens are opaque tokens and clients should not infer any behavior based on their content
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckfetchrecordchangesoperation?language=objc)
    #[unsafe(super(CKDatabaseOperation, CKOperation, NSOperation, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    #[deprecated]
    pub struct CKFetchRecordChangesOperation;
);

#[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
extern_conformance!(
    unsafe impl NSObjectProtocol for CKFetchRecordChangesOperation {}
);

#[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
impl CKFetchRecordChangesOperation {
    extern_methods!(
        #[deprecated]
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(feature = "CKRecordZoneID", feature = "CKServerChangeToken"))]
        #[deprecated]
        #[unsafe(method(initWithRecordZoneID:previousServerChangeToken:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithRecordZoneID_previousServerChangeToken(
            this: Allocated<Self>,
            record_zone_id: &CKRecordZoneID,
            previous_server_change_token: Option<&CKServerChangeToken>,
        ) -> Retained<Self>;

        #[cfg(feature = "CKRecordZoneID")]
        #[deprecated]
        #[unsafe(method(recordZoneID))]
        #[unsafe(method_family = none)]
        pub unsafe fn recordZoneID(&self) -> Option<Retained<CKRecordZoneID>>;

        #[cfg(feature = "CKRecordZoneID")]
        /// Setter for [`recordZoneID`][Self::recordZoneID].
        #[deprecated]
        #[unsafe(method(setRecordZoneID:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRecordZoneID(&self, record_zone_id: Option<&CKRecordZoneID>);

        #[cfg(feature = "CKServerChangeToken")]
        #[deprecated]
        #[unsafe(method(previousServerChangeToken))]
        #[unsafe(method_family = none)]
        pub unsafe fn previousServerChangeToken(&self) -> Option<Retained<CKServerChangeToken>>;

        #[cfg(feature = "CKServerChangeToken")]
        /// Setter for [`previousServerChangeToken`][Self::previousServerChangeToken].
        #[deprecated]
        #[unsafe(method(setPreviousServerChangeToken:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPreviousServerChangeToken(
            &self,
            previous_server_change_token: Option<&CKServerChangeToken>,
        );

        #[deprecated]
        #[unsafe(method(resultsLimit))]
        #[unsafe(method_family = none)]
        pub unsafe fn resultsLimit(&self) -> NSUInteger;

        /// Setter for [`resultsLimit`][Self::resultsLimit].
        #[deprecated]
        #[unsafe(method(setResultsLimit:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setResultsLimit(&self, results_limit: NSUInteger);

        #[cfg(feature = "CKRecord")]
        /// Declares which user-defined keys should be fetched and added to the resulting CKRecords.
        ///
        ///
        /// If nil, declares the entire record should be downloaded. If set to an empty array, declares that no user fields should be downloaded.
        /// Defaults to
        /// `nil.`
        #[deprecated]
        #[unsafe(method(desiredKeys))]
        #[unsafe(method_family = none)]
        pub unsafe fn desiredKeys(&self) -> Option<Retained<NSArray<CKRecordFieldKey>>>;

        #[cfg(feature = "CKRecord")]
        /// Setter for [`desiredKeys`][Self::desiredKeys].
        #[deprecated]
        #[unsafe(method(setDesiredKeys:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDesiredKeys(&self, desired_keys: Option<&NSArray<CKRecordFieldKey>>);

        #[cfg(all(feature = "CKRecord", feature = "block2"))]
        /// Each
        /// `CKOperation`instance has a private serial queue. This queue is used for all callback block invocations.
        /// This block may share mutable state with other blocks assigned to this operation, but any such mutable state
        /// should not be concurrently used outside of blocks assigned to this operation.
        #[deprecated]
        #[unsafe(method(recordChangedBlock))]
        #[unsafe(method_family = none)]
        pub unsafe fn recordChangedBlock(&self)
            -> *mut block2::DynBlock<dyn Fn(NonNull<CKRecord>)>;

        #[cfg(all(feature = "CKRecord", feature = "block2"))]
        /// Setter for [`recordChangedBlock`][Self::recordChangedBlock].
        #[deprecated]
        #[unsafe(method(setRecordChangedBlock:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRecordChangedBlock(
            &self,
            record_changed_block: Option<&block2::DynBlock<dyn Fn(NonNull<CKRecord>)>>,
        );

        #[cfg(all(feature = "CKRecordID", feature = "block2"))]
        /// Each
        /// `CKOperation`instance has a private serial queue. This queue is used for all callback block invocations.
        /// This block may share mutable state with other blocks assigned to this operation, but any such mutable state
        /// should not be concurrently used outside of blocks assigned to this operation.
        #[deprecated]
        #[unsafe(method(recordWithIDWasDeletedBlock))]
        #[unsafe(method_family = none)]
        pub unsafe fn recordWithIDWasDeletedBlock(
            &self,
        ) -> *mut block2::DynBlock<dyn Fn(NonNull<CKRecordID>)>;

        #[cfg(all(feature = "CKRecordID", feature = "block2"))]
        /// Setter for [`recordWithIDWasDeletedBlock`][Self::recordWithIDWasDeletedBlock].
        #[deprecated]
        #[unsafe(method(setRecordWithIDWasDeletedBlock:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRecordWithIDWasDeletedBlock(
            &self,
            record_with_id_was_deleted_block: Option<
                &block2::DynBlock<dyn Fn(NonNull<CKRecordID>)>,
            >,
        );

        /// If true, then the server wasn't able to return all the changes in this response.
        ///
        ///
        /// Will be set before fetchRecordChangesCompletionBlock is called.
        /// Another CKFetchRecordChangesOperation operation should be run with the updated serverChangeToken token from this operation.
        #[deprecated]
        #[unsafe(method(moreComing))]
        #[unsafe(method_family = none)]
        pub unsafe fn moreComing(&self) -> bool;

        #[cfg(all(feature = "CKServerChangeToken", feature = "block2"))]
        /// This block is called when the operation completes.
        ///
        ///
        /// Clients are responsible for saving the change token at the end of the operation and passing it in to the next call to
        /// `CKFetchRecordChangesOperation.`Note that a fetch can fail partway. If that happens, an updated change token may be returned in the completion block so that already fetched records don't need to be re-downloaded on a subsequent operation.
        /// The
        /// `clientChangeTokenData`from the most recent
        /// `CKModifyRecordsOperation`is also returned, or nil if none was provided.
        /// If the server returns a
        /// `CKErrorChangeTokenExpired`error, the
        /// `previousServerChangeToken`value was too old and the client should toss its local cache and re-fetch the changes in this record zone starting with a nil
        /// `previousServerChangeToken.`Each
        /// `CKOperation`instance has a private serial queue. This queue is used for all callback block invocations.
        /// This block may share mutable state with other blocks assigned to this operation, but any such mutable state
        /// should not be concurrently used outside of blocks assigned to this operation.
        #[deprecated]
        #[unsafe(method(fetchRecordChangesCompletionBlock))]
        #[unsafe(method_family = none)]
        pub unsafe fn fetchRecordChangesCompletionBlock(
            &self,
        ) -> *mut block2::DynBlock<dyn Fn(*mut CKServerChangeToken, *mut NSData, *mut NSError)>;

        #[cfg(all(feature = "CKServerChangeToken", feature = "block2"))]
        /// Setter for [`fetchRecordChangesCompletionBlock`][Self::fetchRecordChangesCompletionBlock].
        #[deprecated]
        #[unsafe(method(setFetchRecordChangesCompletionBlock:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFetchRecordChangesCompletionBlock(
            &self,
            fetch_record_changes_completion_block: Option<
                &block2::DynBlock<dyn Fn(*mut CKServerChangeToken, *mut NSData, *mut NSError)>,
            >,
        );
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
impl CKFetchRecordChangesOperation {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
