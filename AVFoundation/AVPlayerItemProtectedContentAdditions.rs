//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// Possible status values resulting from a call to requestContentAuthorizationAsynchronouslyWithTimeoutInterval:CompletionHandler:.
///
///
///
///
///
///
///
///
/// Even if authorization is completed by the user, there is no guarantee that the content will then be authorized.  The caller should re-check
/// whether the content is authorized before proceeding.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcontentauthorizationstatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVContentAuthorizationStatus(pub NSInteger);
impl AVContentAuthorizationStatus {
    #[doc(alias = "AVContentAuthorizationUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "AVContentAuthorizationCompleted")]
    pub const Completed: Self = Self(1);
    #[doc(alias = "AVContentAuthorizationCancelled")]
    pub const Cancelled: Self = Self(2);
    #[doc(alias = "AVContentAuthorizationTimedOut")]
    pub const TimedOut: Self = Self(3);
    #[doc(alias = "AVContentAuthorizationBusy")]
    pub const Busy: Self = Self(4);
    #[doc(alias = "AVContentAuthorizationNotAvailable")]
    pub const NotAvailable: Self = Self(5);
    #[doc(alias = "AVContentAuthorizationNotPossible")]
    pub const NotPossible: Self = Self(6);
}

unsafe impl Encode for AVContentAuthorizationStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVContentAuthorizationStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// AVPlayerItemProtectedContent
    /// Methods supporting protected content.
    #[cfg(feature = "AVPlayerItem")]
    unsafe impl AVPlayerItem {
        /// Indicates whether or not authorization is required to play the content.
        ///
        /// This property reports whether or not authorization is required for the receiver's content to be played.  If it does not require authorization, then none of the other
        /// methods or properties in the AVPlayerItemProtectedContent category apply (though they will return sensible values where possible). This value is NOT key-value observable.
        #[method(isAuthorizationRequiredForPlayback)]
        pub unsafe fn isAuthorizationRequiredForPlayback(&self) -> bool;

        /// Indicates whether the calling application can be used to play the content.
        ///
        /// This property reports whether or not the calling application is authorized to play the content associated with the receiver.  Note that application authorization
        /// is independent of content authorization (see contentAuthorizedForPlayback) and that both must be granted in order for an application to be allowed to play protected content.
        /// Also, unlike content authorization, application authorization is not dependent on user credentials (i.e. if applicationAuthorizedForPlayback is NO, there are no means to obtain authorization).
        /// This value is NOT key-value observable.
        #[method(isApplicationAuthorizedForPlayback)]
        pub unsafe fn isApplicationAuthorizedForPlayback(&self) -> bool;

        /// Indicates whether the content has been authorized by the user (e.g. by authorizing the content's associated account in iTunes).
        ///
        /// This property reports whether or not the user has provided the necessary credentials to the system in order for the content to be decrypted for playback.
        /// Note that content authorization is independent of application authorization (see applicationAuthorizedForPlayback) and that both must be
        /// granted in order for an application to be allowed to play protected content. This value is NOT key-value observable.
        #[method(isContentAuthorizedForPlayback)]
        pub unsafe fn isContentAuthorizedForPlayback(&self) -> bool;

        #[cfg(feature = "block2")]
        /// Causes appropriate action to be taken to allow the user to authorize the content for playback.
        ///
        /// Calling this method will present the user with the opportunity to authorize the content (e.g. by launching iTunes and prompting the user to enter their Apple ID and password).
        /// When the user has taken action (or the timeout has elapsted), the completion handler will be invoked.  The status of the authorization attempt can be determined by checking
        /// the value of the contentAuthorizationRequestStatus property.  Note that even if the status indicates a completed authorization, the content may still not be authorized
        /// (e.g. if the user authorizes an Apple ID other than that associated with the content).  The contentAuthorizedForPlayback property should be re-checked to verify whether
        /// the content has actually been authorized before continuing.  It is not necessary to call this method if the value of contentAuthorizedForPlayback is already true.
        ///
        ///
        /// Parameter `timoutInterval`: The maximum amount of time to wait for the user to authorize the content in seconds before calling the handler block with a timeout result.
        ///
        /// Parameter `handler`: Block to be called upon completion.
        #[method(requestContentAuthorizationAsynchronouslyWithTimeoutInterval:completionHandler:)]
        pub unsafe fn requestContentAuthorizationAsynchronouslyWithTimeoutInterval_completionHandler(
            &self,
            timeout_interval: NSTimeInterval,
            handler: &block2::Block<dyn Fn()>,
        );

        /// Causes the currently outstanding content authorization request to be cancelled.
        ///
        /// Calling this method while a content authorization request is pending will cause that request to be cancelled and its completion handler to be invoked
        /// with a status of AVContentAuthorizationCancelled.  This call does not block.
        #[method(cancelContentAuthorizationRequest)]
        pub unsafe fn cancelContentAuthorizationRequest(&self);

        /// Indicates the status of the most recent call to requestContentAuthorizationAsynchronouslyWithTimeoutInterval:CompletionHandler:
        ///
        /// This property reports the authorization status as determined by the most recent call to requestContentAuthorizationAsynchronouslyWithTimeoutInterval:CompletionHandler:.
        /// The value will be AVContentAuthorizationUnknown before the first call and between the time a request call is made and just prior to the completion handler being executed
        /// (i.e. it is safe to query this property from the completion handler). This value is NOT key-value observable.
        #[method(contentAuthorizationRequestStatus)]
        pub unsafe fn contentAuthorizationRequestStatus(&self) -> AVContentAuthorizationStatus;
    }
);