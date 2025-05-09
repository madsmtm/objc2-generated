//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckfetchshareparticipantsoperation?language=objc)
    #[unsafe(super(CKOperation, NSOperation, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CKOperation")]
    pub struct CKFetchShareParticipantsOperation;
);

#[cfg(feature = "CKOperation")]
extern_conformance!(
    unsafe impl NSObjectProtocol for CKFetchShareParticipantsOperation {}
);

#[cfg(feature = "CKOperation")]
impl CKFetchShareParticipantsOperation {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "CKUserIdentityLookupInfo")]
        #[unsafe(method(initWithUserIdentityLookupInfos:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithUserIdentityLookupInfos(
            this: Allocated<Self>,
            user_identity_lookup_infos: &NSArray<CKUserIdentityLookupInfo>,
        ) -> Retained<Self>;

        #[cfg(feature = "CKUserIdentityLookupInfo")]
        #[unsafe(method(userIdentityLookupInfos))]
        #[unsafe(method_family = none)]
        pub unsafe fn userIdentityLookupInfos(
            &self,
        ) -> Option<Retained<NSArray<CKUserIdentityLookupInfo>>>;

        #[cfg(feature = "CKUserIdentityLookupInfo")]
        /// Setter for [`userIdentityLookupInfos`][Self::userIdentityLookupInfos].
        #[unsafe(method(setUserIdentityLookupInfos:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setUserIdentityLookupInfos(
            &self,
            user_identity_lookup_infos: Option<&NSArray<CKUserIdentityLookupInfo>>,
        );

        #[cfg(all(feature = "CKShareParticipant", feature = "block2"))]
        /// Called once for each share participant created from a submitted user identity lookup info.
        ///
        ///
        /// If the replacement callback
        /// `perShareParticipantCompletionBlock`is set, this callback block is ignored.
        /// Each
        /// `CKOperation`instance has a private serial queue. This queue is used for all callback block invocations.
        /// This block may share mutable state with other blocks assigned to this operation, but any such mutable state
        /// should not be concurrently used outside of blocks assigned to this operation.
        #[deprecated = "Use perShareParticipantCompletionBlock instead, which surfaces per-share-participant errors"]
        #[unsafe(method(shareParticipantFetchedBlock))]
        #[unsafe(method_family = none)]
        pub unsafe fn shareParticipantFetchedBlock(
            &self,
        ) -> *mut block2::DynBlock<dyn Fn(NonNull<CKShareParticipant>)>;

        #[cfg(all(feature = "CKShareParticipant", feature = "block2"))]
        /// Setter for [`shareParticipantFetchedBlock`][Self::shareParticipantFetchedBlock].
        #[deprecated = "Use perShareParticipantCompletionBlock instead, which surfaces per-share-participant errors"]
        #[unsafe(method(setShareParticipantFetchedBlock:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setShareParticipantFetchedBlock(
            &self,
            share_participant_fetched_block: Option<
                &block2::DynBlock<dyn Fn(NonNull<CKShareParticipant>)>,
            >,
        );

        #[cfg(all(
            feature = "CKShareParticipant",
            feature = "CKUserIdentityLookupInfo",
            feature = "block2"
        ))]
        /// Called once for each lookup info.
        ///
        ///
        /// Each
        /// `CKOperation`instance has a private serial queue. This queue is used for all callback block invocations.
        /// This block may share mutable state with other blocks assigned to this operation, but any such mutable state
        /// should not be concurrently used outside of blocks assigned to this operation.
        #[unsafe(method(perShareParticipantCompletionBlock))]
        #[unsafe(method_family = none)]
        pub unsafe fn perShareParticipantCompletionBlock(
            &self,
        ) -> *mut block2::DynBlock<
            dyn Fn(NonNull<CKUserIdentityLookupInfo>, *mut CKShareParticipant, *mut NSError),
        >;

        #[cfg(all(
            feature = "CKShareParticipant",
            feature = "CKUserIdentityLookupInfo",
            feature = "block2"
        ))]
        /// Setter for [`perShareParticipantCompletionBlock`][Self::perShareParticipantCompletionBlock].
        #[unsafe(method(setPerShareParticipantCompletionBlock:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPerShareParticipantCompletionBlock(
            &self,
            per_share_participant_completion_block: Option<
                &block2::DynBlock<
                    dyn Fn(
                        NonNull<CKUserIdentityLookupInfo>,
                        *mut CKShareParticipant,
                        *mut NSError,
                    ),
                >,
            >,
        );

        #[cfg(feature = "block2")]
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
        /// `CKErrorPartialFailure,`the error's userInfo dictionary contains a dictionary of lookup infos to errors keyed off of
        /// `CKPartialErrorsByItemIDKey.`These errors are repeats of those sent back in previous
        /// `perShareParticipantCompletionBlock`invocations
        /// Each
        /// `CKOperation`instance has a private serial queue. This queue is used for all callback block invocations.
        /// This block may share mutable state with other blocks assigned to this operation, but any such mutable state
        /// should not be concurrently used outside of blocks assigned to this operation.
        #[unsafe(method(fetchShareParticipantsCompletionBlock))]
        #[unsafe(method_family = none)]
        pub unsafe fn fetchShareParticipantsCompletionBlock(
            &self,
        ) -> *mut block2::DynBlock<dyn Fn(*mut NSError)>;

        #[cfg(feature = "block2")]
        /// Setter for [`fetchShareParticipantsCompletionBlock`][Self::fetchShareParticipantsCompletionBlock].
        #[unsafe(method(setFetchShareParticipantsCompletionBlock:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFetchShareParticipantsCompletionBlock(
            &self,
            fetch_share_participants_completion_block: Option<
                &block2::DynBlock<dyn Fn(*mut NSError)>,
            >,
        );
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "CKOperation")]
impl CKFetchShareParticipantsOperation {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
