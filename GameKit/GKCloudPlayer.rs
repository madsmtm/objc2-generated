//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamekit/gkcloudplayer?language=objc)
    #[unsafe(super(GKBasePlayer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GKBasePlayer")]
    #[deprecated]
    pub struct GKCloudPlayer;
);

#[cfg(feature = "GKBasePlayer")]
extern_conformance!(
    unsafe impl NSObjectProtocol for GKCloudPlayer {}
);

#[cfg(feature = "GKBasePlayer")]
impl GKCloudPlayer {
    extern_methods!(
        #[cfg(feature = "block2")]
        /// Retrieve a player instance representing the active iCloud account for a given iCloud container. Returns nil and an error if the user is not signed in to iCloud or the container is invalid.
        #[deprecated]
        #[unsafe(method(getCurrentSignedInPlayerForContainer:completionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn getCurrentSignedInPlayerForContainer_completionHandler(
            container_name: Option<&NSString>,
            handler: &block2::DynBlock<dyn Fn(*mut GKCloudPlayer, *mut NSError)>,
        );
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "GKBasePlayer")]
impl GKCloudPlayer {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
