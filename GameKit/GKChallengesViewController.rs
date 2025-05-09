//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamekit/gkchallengesviewcontroller?language=objc)
    #[unsafe(super(NSViewController, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    #[deprecated = "No longer supported"]
    pub struct GKChallengesViewController;
);

#[cfg(all(feature = "GKDialogController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl GKViewController for GKChallengesViewController {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSCoding for GKChallengesViewController {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSEditor for GKChallengesViewController {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSObjectProtocol for GKChallengesViewController {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSSeguePerforming for GKChallengesViewController {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSUserInterfaceItemIdentification for GKChallengesViewController {}
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
impl GKChallengesViewController {
    extern_methods!(
        #[deprecated = "No longer supported"]
        #[unsafe(method(challengeDelegate))]
        #[unsafe(method_family = none)]
        pub unsafe fn challengeDelegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn GKChallengesViewControllerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`challengeDelegate`][Self::challengeDelegate].
        #[deprecated = "No longer supported"]
        #[unsafe(method(setChallengeDelegate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setChallengeDelegate(
            &self,
            challenge_delegate: Option<&ProtocolObject<dyn GKChallengesViewControllerDelegate>>,
        );
    );
}

/// Methods declared on superclass `NSViewController`.
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
impl GKChallengesViewController {
    extern_methods!(
        #[unsafe(method(initWithNibName:bundle:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Retained<Self>;

        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    );
}

/// Methods declared on superclass `NSResponder`.
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
impl GKChallengesViewController {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
impl GKChallengesViewController {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamekit/gkchallengesviewcontrollerdelegate?language=objc)
    #[deprecated = "No longer supported"]
    pub unsafe trait GKChallengesViewControllerDelegate {
        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[deprecated = "No longer supported"]
        #[unsafe(method(challengesViewControllerDidFinish:))]
        #[unsafe(method_family = none)]
        unsafe fn challengesViewControllerDidFinish(
            &self,
            view_controller: Option<&GKChallengesViewController>,
        );
    }
);
