//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckdiscoveruseridentitiesoperation?language=objc)
    #[unsafe(super(CKOperation, NSOperation, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CKOperation")]
    #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
    pub struct CKDiscoverUserIdentitiesOperation;
);

#[cfg(feature = "CKOperation")]
unsafe impl NSObjectProtocol for CKDiscoverUserIdentitiesOperation {}

extern_methods!(
    #[cfg(feature = "CKOperation")]
    unsafe impl CKDiscoverUserIdentitiesOperation {
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "CKUserIdentityLookupInfo")]
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[unsafe(method_family(init))]
        #[method_id(initWithUserIdentityLookupInfos:)]
        pub unsafe fn initWithUserIdentityLookupInfos(
            this: Allocated<Self>,
            user_identity_lookup_infos: &NSArray<CKUserIdentityLookupInfo>,
        ) -> Retained<Self>;

        #[cfg(feature = "CKUserIdentityLookupInfo")]
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[unsafe(method_family(none))]
        #[method_id(userIdentityLookupInfos)]
        pub unsafe fn userIdentityLookupInfos(&self)
            -> Retained<NSArray<CKUserIdentityLookupInfo>>;

        #[cfg(feature = "CKUserIdentityLookupInfo")]
        /// Setter for [`userIdentityLookupInfos`][Self::userIdentityLookupInfos].
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(setUserIdentityLookupInfos:)]
        pub unsafe fn setUserIdentityLookupInfos(
            &self,
            user_identity_lookup_infos: &NSArray<CKUserIdentityLookupInfo>,
        );

        #[cfg(all(
            feature = "CKUserIdentity",
            feature = "CKUserIdentityLookupInfo",
            feature = "block2"
        ))]
        /// Called once for each user identity lookup info that was successfully discovered on the server
        ///
        ///
        /// Each
        /// `CKOperation`instance has a private serial queue. This queue is used for all callback block invocations.
        /// This block may share mutable state with other blocks assigned to this operation, but any such mutable state
        /// should not be concurrently used outside of blocks assigned to this operation.
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(userIdentityDiscoveredBlock)]
        pub unsafe fn userIdentityDiscoveredBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<CKUserIdentity>, NonNull<CKUserIdentityLookupInfo>)>;

        #[cfg(all(
            feature = "CKUserIdentity",
            feature = "CKUserIdentityLookupInfo",
            feature = "block2"
        ))]
        /// Setter for [`userIdentityDiscoveredBlock`][Self::userIdentityDiscoveredBlock].
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(setUserIdentityDiscoveredBlock:)]
        pub unsafe fn setUserIdentityDiscoveredBlock(
            &self,
            user_identity_discovered_block: Option<
                &block2::Block<dyn Fn(NonNull<CKUserIdentity>, NonNull<CKUserIdentityLookupInfo>)>,
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
        /// Each
        /// `CKOperation`instance has a private serial queue. This queue is used for all callback block invocations.
        /// This block may share mutable state with other blocks assigned to this operation, but any such mutable state
        /// should not be concurrently used outside of blocks assigned to this operation.
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(discoverUserIdentitiesCompletionBlock)]
        pub unsafe fn discoverUserIdentitiesCompletionBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(*mut NSError)>;

        #[cfg(feature = "block2")]
        /// Setter for [`discoverUserIdentitiesCompletionBlock`][Self::discoverUserIdentitiesCompletionBlock].
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(setDiscoverUserIdentitiesCompletionBlock:)]
        pub unsafe fn setDiscoverUserIdentitiesCompletionBlock(
            &self,
            discover_user_identities_completion_block: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CKOperation")]
    unsafe impl CKDiscoverUserIdentitiesOperation {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
