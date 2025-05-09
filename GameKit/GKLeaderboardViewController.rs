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
    /// View controller that provides the standard user interface for leaderboards.  Present modally from the top view controller.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gamekit/gkleaderboardviewcontroller?language=objc)
    #[unsafe(super(GKGameCenterViewController, NSViewController, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    #[deprecated]
    pub struct GKLeaderboardViewController;
);

#[cfg(all(
    feature = "GKDialogController",
    feature = "GKGameCenterViewController",
    feature = "objc2-app-kit"
))]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl GKViewController for GKLeaderboardViewController {}
);

#[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSCoding for GKLeaderboardViewController {}
);

#[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSEditor for GKLeaderboardViewController {}
);

#[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSObjectProtocol for GKLeaderboardViewController {}
);

#[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSSeguePerforming for GKLeaderboardViewController {}
);

#[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
extern_conformance!(
    unsafe impl NSUserInterfaceItemIdentification for GKLeaderboardViewController {}
);

#[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
impl GKLeaderboardViewController {
    extern_methods!();
}

/// Methods declared on superclass `NSViewController`.
#[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
impl GKLeaderboardViewController {
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
#[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
impl GKLeaderboardViewController {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
impl GKLeaderboardViewController {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}

#[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
impl GKLeaderboardViewController {
    extern_methods!(
        #[cfg(feature = "GKLeaderboard")]
        #[deprecated]
        #[unsafe(method(timeScope))]
        #[unsafe(method_family = none)]
        pub unsafe fn timeScope(&self) -> GKLeaderboardTimeScope;

        #[cfg(feature = "GKLeaderboard")]
        /// Setter for [`timeScope`][Self::timeScope].
        #[deprecated]
        #[unsafe(method(setTimeScope:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTimeScope(&self, time_scope: GKLeaderboardTimeScope);

        #[deprecated]
        #[unsafe(method(category))]
        #[unsafe(method_family = none)]
        pub unsafe fn category(&self) -> Retained<NSString>;

        /// Setter for [`category`][Self::category].
        #[deprecated]
        #[unsafe(method(setCategory:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCategory(&self, category: Option<&NSString>);

        #[deprecated]
        #[unsafe(method(leaderboardDelegate))]
        #[unsafe(method_family = none)]
        pub unsafe fn leaderboardDelegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn GKLeaderboardViewControllerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`leaderboardDelegate`][Self::leaderboardDelegate].
        #[deprecated]
        #[unsafe(method(setLeaderboardDelegate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLeaderboardDelegate(
            &self,
            leaderboard_delegate: Option<&ProtocolObject<dyn GKLeaderboardViewControllerDelegate>>,
        );
    );
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamekit/gkleaderboardviewcontrollerdelegate?language=objc)
    #[deprecated]
    pub unsafe trait GKLeaderboardViewControllerDelegate: NSObjectProtocol {
        #[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        /// The leaderboard view has finished
        #[deprecated]
        #[unsafe(method(leaderboardViewControllerDidFinish:))]
        #[unsafe(method_family = none)]
        unsafe fn leaderboardViewControllerDidFinish(
            &self,
            view_controller: Option<&GKLeaderboardViewController>,
        );
    }
);
