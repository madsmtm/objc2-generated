//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// This operation will fetch a web auth token given an API token obtained from the CloudKit Dashboard for your container
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckfetchwebauthtokenoperation?language=objc)
    #[unsafe(super(CKDatabaseOperation, CKOperation, NSOperation, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    pub struct CKFetchWebAuthTokenOperation;
);

#[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
extern_conformance!(
    unsafe impl NSObjectProtocol for CKFetchWebAuthTokenOperation {}
);

#[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
impl CKFetchWebAuthTokenOperation {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(initWithAPIToken:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithAPIToken(
            this: Allocated<Self>,
            api_token: &NSString,
        ) -> Retained<Self>;

        /// APIToken is expected to be set before you begin this operation.
        #[unsafe(method(APIToken))]
        #[unsafe(method_family = none)]
        pub unsafe fn APIToken(&self) -> Option<Retained<NSString>>;

        /// Setter for [`APIToken`][Self::APIToken].
        #[unsafe(method(setAPIToken:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAPIToken(&self, api_token: Option<&NSString>);

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
        #[unsafe(method(fetchWebAuthTokenCompletionBlock))]
        #[unsafe(method_family = none)]
        pub unsafe fn fetchWebAuthTokenCompletionBlock(
            &self,
        ) -> *mut block2::DynBlock<dyn Fn(*mut NSString, *mut NSError)>;

        #[cfg(feature = "block2")]
        /// Setter for [`fetchWebAuthTokenCompletionBlock`][Self::fetchWebAuthTokenCompletionBlock].
        #[unsafe(method(setFetchWebAuthTokenCompletionBlock:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFetchWebAuthTokenCompletionBlock(
            &self,
            fetch_web_auth_token_completion_block: Option<
                &block2::DynBlock<dyn Fn(*mut NSString, *mut NSError)>,
            >,
        );
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
impl CKFetchWebAuthTokenOperation {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
