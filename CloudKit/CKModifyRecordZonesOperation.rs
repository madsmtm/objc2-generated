//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckmodifyrecordzonesoperation?language=objc)
    #[unsafe(super(CKDatabaseOperation, CKOperation, NSOperation, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    pub struct CKModifyRecordZonesOperation;
);

#[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
extern_conformance!(
    unsafe impl NSObjectProtocol for CKModifyRecordZonesOperation {}
);

#[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
impl CKModifyRecordZonesOperation {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(feature = "CKRecordZone", feature = "CKRecordZoneID"))]
        #[unsafe(method(initWithRecordZonesToSave:recordZoneIDsToDelete:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithRecordZonesToSave_recordZoneIDsToDelete(
            this: Allocated<Self>,
            record_zones_to_save: Option<&NSArray<CKRecordZone>>,
            record_zone_i_ds_to_delete: Option<&NSArray<CKRecordZoneID>>,
        ) -> Retained<Self>;

        #[cfg(feature = "CKRecordZone")]
        #[unsafe(method(recordZonesToSave))]
        #[unsafe(method_family = none)]
        pub unsafe fn recordZonesToSave(&self) -> Option<Retained<NSArray<CKRecordZone>>>;

        #[cfg(feature = "CKRecordZone")]
        /// Setter for [`recordZonesToSave`][Self::recordZonesToSave].
        #[unsafe(method(setRecordZonesToSave:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRecordZonesToSave(
            &self,
            record_zones_to_save: Option<&NSArray<CKRecordZone>>,
        );

        #[cfg(feature = "CKRecordZoneID")]
        #[unsafe(method(recordZoneIDsToDelete))]
        #[unsafe(method_family = none)]
        pub unsafe fn recordZoneIDsToDelete(&self) -> Option<Retained<NSArray<CKRecordZoneID>>>;

        #[cfg(feature = "CKRecordZoneID")]
        /// Setter for [`recordZoneIDsToDelete`][Self::recordZoneIDsToDelete].
        #[unsafe(method(setRecordZoneIDsToDelete:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRecordZoneIDsToDelete(
            &self,
            record_zone_i_ds_to_delete: Option<&NSArray<CKRecordZoneID>>,
        );

        #[cfg(all(
            feature = "CKRecordZone",
            feature = "CKRecordZoneID",
            feature = "block2"
        ))]
        /// Called on success or failure of a record zone save
        ///
        ///
        /// Each
        /// `CKOperation`instance has a private serial queue. This queue is used for all callback block invocations.
        /// This block may share mutable state with other blocks assigned to this operation, but any such mutable state
        /// should not be concurrently used outside of blocks assigned to this operation.
        #[unsafe(method(perRecordZoneSaveBlock))]
        #[unsafe(method_family = none)]
        pub unsafe fn perRecordZoneSaveBlock(
            &self,
        ) -> *mut block2::DynBlock<dyn Fn(NonNull<CKRecordZoneID>, *mut CKRecordZone, *mut NSError)>;

        #[cfg(all(
            feature = "CKRecordZone",
            feature = "CKRecordZoneID",
            feature = "block2"
        ))]
        /// Setter for [`perRecordZoneSaveBlock`][Self::perRecordZoneSaveBlock].
        #[unsafe(method(setPerRecordZoneSaveBlock:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPerRecordZoneSaveBlock(
            &self,
            per_record_zone_save_block: Option<
                &block2::DynBlock<dyn Fn(NonNull<CKRecordZoneID>, *mut CKRecordZone, *mut NSError)>,
            >,
        );

        #[cfg(all(feature = "CKRecordZoneID", feature = "block2"))]
        /// Called on success or failure of a record zone deletion
        ///
        ///
        /// Each
        /// `CKOperation`instance has a private serial queue. This queue is used for all callback block invocations.
        /// This block may share mutable state with other blocks assigned to this operation, but any such mutable state
        /// should not be concurrently used outside of blocks assigned to this operation.
        #[unsafe(method(perRecordZoneDeleteBlock))]
        #[unsafe(method_family = none)]
        pub unsafe fn perRecordZoneDeleteBlock(
            &self,
        ) -> *mut block2::DynBlock<dyn Fn(NonNull<CKRecordZoneID>, *mut NSError)>;

        #[cfg(all(feature = "CKRecordZoneID", feature = "block2"))]
        /// Setter for [`perRecordZoneDeleteBlock`][Self::perRecordZoneDeleteBlock].
        #[unsafe(method(setPerRecordZoneDeleteBlock:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPerRecordZoneDeleteBlock(
            &self,
            per_record_zone_delete_block: Option<
                &block2::DynBlock<dyn Fn(NonNull<CKRecordZoneID>, *mut NSError)>,
            >,
        );

        #[cfg(all(
            feature = "CKRecordZone",
            feature = "CKRecordZoneID",
            feature = "block2"
        ))]
        /// This block is called when the operation completes.
        ///
        ///
        /// The
        ///
        /// ```text
        ///  -[NSOperation completionBlock]
        /// ```
        ///
        /// will also be called if both are set.
        /// If the error is
        /// `CKErrorPartialFailure,`the error's userInfo dictionary contains a dictionary of recordZoneIDs to errors keyed off of
        /// `CKPartialErrorsByItemIDKey.``savedRecordZones,``deletedRecordZoneIDs`and any
        /// `CKPartialErrorsByItemIDKey`errors are repeats of the data sent back in previous
        /// `perRecordZoneSaveBlock`and
        /// `perRecordZoneDeleteBlock`invocations
        /// Each
        /// `CKOperation`instance has a private serial queue. This queue is used for all callback block invocations.
        /// This block may share mutable state with other blocks assigned to this operation, but any such mutable state
        /// should not be concurrently used outside of blocks assigned to this operation.
        #[unsafe(method(modifyRecordZonesCompletionBlock))]
        #[unsafe(method_family = none)]
        pub unsafe fn modifyRecordZonesCompletionBlock(
            &self,
        ) -> *mut block2::DynBlock<
            dyn Fn(*mut NSArray<CKRecordZone>, *mut NSArray<CKRecordZoneID>, *mut NSError),
        >;

        #[cfg(all(
            feature = "CKRecordZone",
            feature = "CKRecordZoneID",
            feature = "block2"
        ))]
        /// Setter for [`modifyRecordZonesCompletionBlock`][Self::modifyRecordZonesCompletionBlock].
        #[unsafe(method(setModifyRecordZonesCompletionBlock:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setModifyRecordZonesCompletionBlock(
            &self,
            modify_record_zones_completion_block: Option<
                &block2::DynBlock<
                    dyn Fn(*mut NSArray<CKRecordZone>, *mut NSArray<CKRecordZoneID>, *mut NSError),
                >,
            >,
        );
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
impl CKModifyRecordZonesOperation {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
