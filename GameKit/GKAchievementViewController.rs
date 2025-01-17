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
    /// View controller that provides the standard user interface for achievements. Present modally from the top view controller.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gamekit/gkachievementviewcontroller?language=objc)
    #[unsafe(super(GKGameCenterViewController, NSViewController, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    #[deprecated]
    pub struct GKAchievementViewController;
);

#[cfg(all(
    feature = "GKDialogController",
    feature = "GKGameCenterViewController",
    feature = "objc2-app-kit"
))]
#[cfg(target_os = "macos")]
unsafe impl GKViewController for GKAchievementViewController {}

#[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSCoding for GKAchievementViewController {}

#[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSEditor for GKAchievementViewController {}

#[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSObjectProtocol for GKAchievementViewController {}

#[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSSeguePerforming for GKAchievementViewController {}

#[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSUserInterfaceItemIdentification for GKAchievementViewController {}

extern_methods!(
    #[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl GKAchievementViewController {
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(achievementDelegate)]
        pub unsafe fn achievementDelegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn GKAchievementViewControllerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`achievementDelegate`][Self::achievementDelegate].
        #[deprecated]
        #[method(setAchievementDelegate:)]
        pub unsafe fn setAchievementDelegate(
            &self,
            achievement_delegate: Option<&ProtocolObject<dyn GKAchievementViewControllerDelegate>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl GKAchievementViewController {
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
    unsafe impl GKAchievementViewController {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl GKAchievementViewController {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// Optional delegate
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gamekit/gkachievementviewcontrollerdelegate?language=objc)
    #[deprecated]
    pub unsafe trait GKAchievementViewControllerDelegate: NSObjectProtocol {
        #[cfg(all(feature = "GKGameCenterViewController", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        /// The achievement view has finished
        #[deprecated]
        #[method(achievementViewControllerDidFinish:)]
        unsafe fn achievementViewControllerDidFinish(
            &self,
            view_controller: Option<&GKAchievementViewController>,
        );
    }
);
