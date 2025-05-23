//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// A type of right that, when authorized, grants access to a key and secret
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/localauthentication/lapersistedright?language=objc)
    #[unsafe(super(LARight, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "LARight")]
    pub struct LAPersistedRight;
);

#[cfg(feature = "LARight")]
extern_conformance!(
    unsafe impl NSObjectProtocol for LAPersistedRight {}
);

#[cfg(feature = "LARight")]
impl LAPersistedRight {
    extern_methods!(
        #[cfg(feature = "LAPrivateKey")]
        /// Managed private key
        #[unsafe(method(key))]
        #[unsafe(method_family = none)]
        pub unsafe fn key(&self) -> Retained<LAPrivateKey>;

        #[cfg(feature = "LASecret")]
        /// Generic secret
        ///
        /// This is the generic secret that would have been stored along with the right
        #[unsafe(method(secret))]
        #[unsafe(method_family = none)]
        pub unsafe fn secret(&self) -> Retained<LASecret>;

        /// Clients cannot create
        /// `LAPersistedRight`instances directly. They can only obtain them from the
        /// `LARightStore`.
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        /// Clients cannot create
        /// `LAPersistedRight`instances directly. They can only obtain them from the
        /// `LARightStore`.
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `LARight`.
#[cfg(feature = "LARight")]
impl LAPersistedRight {
    extern_methods!(
        #[cfg(feature = "LARequirement")]
        /// Constructs a right that will be granted only when the given
        /// `LAAuthenticationRequirement`is statisfied.
        ///
        /// Parameter `requirement`: Requirement that needs to be satisfied to authorize the right
        ///
        /// Returns: `LARight`instance
        #[unsafe(method(initWithRequirement:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithRequirement(
            this: Allocated<Self>,
            requirement: &LAAuthenticationRequirement,
        ) -> Retained<Self>;
    );
}
