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
unsafe impl GKViewController for GKLeaderboardViewController {}

#[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSCoding for GKLeaderboardViewController {}

#[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSEditor for GKLeaderboardViewController {}

#[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSObjectProtocol for GKLeaderboardViewController {}

#[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSSeguePerforming for GKLeaderboardViewController {}

#[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSUserInterfaceItemIdentification for GKLeaderboardViewController {}

extern_methods!(
    #[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl GKLeaderboardViewController {}
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl GKLeaderboardViewController {
        #[unsafe(method_family(init))]
        #[method_id(initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl GKLeaderboardViewController {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl GKLeaderboardViewController {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    #[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl GKLeaderboardViewController {
        #[cfg(feature = "GKLeaderboard")]
        #[deprecated]
        #[method(timeScope)]
        pub unsafe fn timeScope(&self) -> GKLeaderboardTimeScope;

        #[cfg(feature = "GKLeaderboard")]
        /// Setter for [`timeScope`][Self::timeScope].
        #[deprecated]
        #[method(setTimeScope:)]
        pub unsafe fn setTimeScope(&self, time_scope: GKLeaderboardTimeScope);

        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(category)]
        pub unsafe fn category(&self) -> Retained<NSString>;

        /// Setter for [`category`][Self::category].
        #[deprecated]
        #[method(setCategory:)]
        pub unsafe fn setCategory(&self, category: Option<&NSString>);

        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(leaderboardDelegate)]
        pub unsafe fn leaderboardDelegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn GKLeaderboardViewControllerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`leaderboardDelegate`][Self::leaderboardDelegate].
        #[deprecated]
        #[method(setLeaderboardDelegate:)]
        pub unsafe fn setLeaderboardDelegate(
            &self,
            leaderboard_delegate: Option<&ProtocolObject<dyn GKLeaderboardViewControllerDelegate>>,
        );
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamekit/gkleaderboardviewcontrollerdelegate?language=objc)
    #[deprecated]
    pub unsafe trait GKLeaderboardViewControllerDelegate: NSObjectProtocol {
        #[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        /// The leaderboard view has finished
        #[deprecated]
        #[method(leaderboardViewControllerDidFinish:)]
        unsafe fn leaderboardViewControllerDidFinish(
            &self,
            view_controller: Option<&GKLeaderboardViewController>,
        );
    }
);
