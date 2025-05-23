//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckmodifysubscriptionsoperation?language=objc)
    #[unsafe(super(CKDatabaseOperation, CKOperation, NSOperation, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    pub struct CKModifySubscriptionsOperation;
);

#[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
extern_conformance!(
    unsafe impl NSObjectProtocol for CKModifySubscriptionsOperation {}
);

#[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
impl CKModifySubscriptionsOperation {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "CKSubscription")]
        #[unsafe(method(initWithSubscriptionsToSave:subscriptionIDsToDelete:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithSubscriptionsToSave_subscriptionIDsToDelete(
            this: Allocated<Self>,
            subscriptions_to_save: Option<&NSArray<CKSubscription>>,
            subscription_i_ds_to_delete: Option<&NSArray<CKSubscriptionID>>,
        ) -> Retained<Self>;

        #[cfg(feature = "CKSubscription")]
        #[unsafe(method(subscriptionsToSave))]
        #[unsafe(method_family = none)]
        pub unsafe fn subscriptionsToSave(&self) -> Option<Retained<NSArray<CKSubscription>>>;

        #[cfg(feature = "CKSubscription")]
        /// Setter for [`subscriptionsToSave`][Self::subscriptionsToSave].
        #[unsafe(method(setSubscriptionsToSave:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSubscriptionsToSave(
            &self,
            subscriptions_to_save: Option<&NSArray<CKSubscription>>,
        );

        #[cfg(feature = "CKSubscription")]
        #[unsafe(method(subscriptionIDsToDelete))]
        #[unsafe(method_family = none)]
        pub unsafe fn subscriptionIDsToDelete(&self)
            -> Option<Retained<NSArray<CKSubscriptionID>>>;

        #[cfg(feature = "CKSubscription")]
        /// Setter for [`subscriptionIDsToDelete`][Self::subscriptionIDsToDelete].
        #[unsafe(method(setSubscriptionIDsToDelete:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSubscriptionIDsToDelete(
            &self,
            subscription_i_ds_to_delete: Option<&NSArray<CKSubscriptionID>>,
        );

        #[cfg(all(feature = "CKSubscription", feature = "block2"))]
        /// Called on success or failure of a subscription save
        ///
        ///
        /// Each
        /// `CKOperation`instance has a private serial queue. This queue is used for all callback block invocations.
        /// This block may share mutable state with other blocks assigned to this operation, but any such mutable state
        /// should not be concurrently used outside of blocks assigned to this operation.
        #[unsafe(method(perSubscriptionSaveBlock))]
        #[unsafe(method_family = none)]
        pub unsafe fn perSubscriptionSaveBlock(
            &self,
        ) -> *mut block2::DynBlock<
            dyn Fn(NonNull<CKSubscriptionID>, *mut CKSubscription, *mut NSError),
        >;

        #[cfg(all(feature = "CKSubscription", feature = "block2"))]
        /// Setter for [`perSubscriptionSaveBlock`][Self::perSubscriptionSaveBlock].
        #[unsafe(method(setPerSubscriptionSaveBlock:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPerSubscriptionSaveBlock(
            &self,
            per_subscription_save_block: Option<
                &block2::DynBlock<
                    dyn Fn(NonNull<CKSubscriptionID>, *mut CKSubscription, *mut NSError),
                >,
            >,
        );

        #[cfg(all(feature = "CKSubscription", feature = "block2"))]
        /// Called on success or failure of a subscription deletion
        ///
        ///
        /// Each
        /// `CKOperation`instance has a private serial queue. This queue is used for all callback block invocations.
        /// This block may share mutable state with other blocks assigned to this operation, but any such mutable state
        /// should not be concurrently used outside of blocks assigned to this operation.
        #[unsafe(method(perSubscriptionDeleteBlock))]
        #[unsafe(method_family = none)]
        pub unsafe fn perSubscriptionDeleteBlock(
            &self,
        ) -> *mut block2::DynBlock<dyn Fn(NonNull<CKSubscriptionID>, *mut NSError)>;

        #[cfg(all(feature = "CKSubscription", feature = "block2"))]
        /// Setter for [`perSubscriptionDeleteBlock`][Self::perSubscriptionDeleteBlock].
        #[unsafe(method(setPerSubscriptionDeleteBlock:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPerSubscriptionDeleteBlock(
            &self,
            per_subscription_delete_block: Option<
                &block2::DynBlock<dyn Fn(NonNull<CKSubscriptionID>, *mut NSError)>,
            >,
        );

        #[cfg(all(feature = "CKSubscription", feature = "block2"))]
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
        /// `CKErrorPartialFailure,`the error's userInfo dictionary contains a dictionary of subscriptionIDs to errors keyed off of
        /// `CKPartialErrorsByItemIDKey.``savedSubscriptions,``deletedSubscriptionIDs`and any
        /// `CKPartialErrorsByItemIDKey`errors are repeats of the data sent back in previous
        /// `perSubscriptionSaveBlock`and
        /// `perSubscriptionDeleteBlock`invocations
        /// Each
        /// `CKOperation`instance has a private serial queue. This queue is used for all callback block invocations.
        /// This block may share mutable state with other blocks assigned to this operation, but any such mutable state
        /// should not be concurrently used outside of blocks assigned to this operation.
        #[unsafe(method(modifySubscriptionsCompletionBlock))]
        #[unsafe(method_family = none)]
        pub unsafe fn modifySubscriptionsCompletionBlock(
            &self,
        ) -> *mut block2::DynBlock<
            dyn Fn(*mut NSArray<CKSubscription>, *mut NSArray<CKSubscriptionID>, *mut NSError),
        >;

        #[cfg(all(feature = "CKSubscription", feature = "block2"))]
        /// Setter for [`modifySubscriptionsCompletionBlock`][Self::modifySubscriptionsCompletionBlock].
        #[unsafe(method(setModifySubscriptionsCompletionBlock:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setModifySubscriptionsCompletionBlock(
            &self,
            modify_subscriptions_completion_block: Option<
                &block2::DynBlock<
                    dyn Fn(
                        *mut NSArray<CKSubscription>,
                        *mut NSArray<CKSubscriptionID>,
                        *mut NSError,
                    ),
                >,
            >,
        );
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
impl CKModifySubscriptionsOperation {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
