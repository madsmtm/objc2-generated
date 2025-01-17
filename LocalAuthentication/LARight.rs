//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// Each of the different states of a right
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/localauthentication/larightstate?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LARightState(pub NSInteger);
impl LARightState {
    /// Right has not been evaluated yet.
    ///
    /// This is the initial state of
    /// `LARight`and changes when
    /// `authorize`method is called.
    #[doc(alias = "LARightStateUnknown")]
    pub const Unknown: Self = Self(0);
    /// Requirements are currently being evaluated.
    ///
    /// This happens after calling
    /// `authorize`method but before the user has granted the right.
    #[doc(alias = "LARightStateAuthorizing")]
    pub const Authorizing: Self = Self(1);
    /// Authorization was granted
    ///
    /// This can be achieved by succesful authorization.
    #[doc(alias = "LARightStateAuthorized")]
    pub const Authorized: Self = Self(2);
    /// Authorization was rejected.
    ///
    /// This can be caused by several reasons. For example requirements were not satisified or user rejects to authorize.
    #[doc(alias = "LARightStateNotAuthorized")]
    pub const NotAuthorized: Self = Self(3);
}

unsafe impl Encode for LARightState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for LARightState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// Groups a set of requirements that need to be satisfied in order to grant access to certain resource or operation
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/localauthentication/laright?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct LARight;
);

unsafe impl NSObjectProtocol for LARight {}

extern_methods!(
    unsafe impl LARight {
        /// Provides the current authorization state of the
        /// `LARight`instance
        #[method(state)]
        pub unsafe fn state(&self) -> LARightState;

        /// An application-supplied integer that can be used to identify right intances. The default value is
        /// `0.`
        #[method(tag)]
        pub unsafe fn tag(&self) -> NSInteger;

        /// Setter for [`tag`][Self::tag].
        #[method(setTag:)]
        pub unsafe fn setTag(&self, tag: NSInteger);

        /// Constructs a right using default authorization requirements
        ///
        /// For authorizing a right with default requirements a user will be asked to authenticate using biometry or the device passcode.
        ///
        /// Returns: `LARight`instance
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "LARequirement")]
        /// Constructs a right that will be granted only when the given
        /// `LAAuthenticationRequirement`is statisfied.
        ///
        /// Parameter `requirement`: Requirement that needs to be satisfied to authorize the right
        ///
        /// Returns: `LARight`instance
        #[unsafe(method_family(init))]
        #[method_id(initWithRequirement:)]
        pub unsafe fn initWithRequirement(
            this: Allocated<Self>,
            requirement: &LAAuthenticationRequirement,
        ) -> Retained<Self>;

        #[cfg(feature = "block2")]
        /// Tries to authorize the right.
        ///
        /// Parameter `localizedReason`: Localized explanation for the authorization. Appears in the UI presented to the user.
        ///
        /// Parameter `handler`: Completion handler called after the authorization finishes. Returns an error when the authorization fails.
        #[method(authorizeWithLocalizedReason:completion:)]
        pub unsafe fn authorizeWithLocalizedReason_completion(
            &self,
            localized_reason: &NSString,
            handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "block2")]
        /// Checks whether the client can eventually be granted the right.
        ///
        /// Parameter `handler`: Completion handler. Returns
        /// `nil`if the right can be authorized or an error otherwise.
        #[method(checkCanAuthorizeWithCompletion:)]
        pub unsafe fn checkCanAuthorizeWithCompletion(
            &self,
            handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "block2")]
        /// Invalidates a previously authorized right.
        ///
        /// Parameter `handler`: Completion handler called after the right is deauthorized.
        #[method(deauthorizeWithCompletion:)]
        pub unsafe fn deauthorizeWithCompletion(&self, handler: &block2::Block<dyn Fn()>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl LARight {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
