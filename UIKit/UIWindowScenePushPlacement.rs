//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Background the scene of the provided scene session and present the
    /// activated scene in its place.
    ///
    /// The provided scene session will be backgrounded. The activated scene will
    /// be center aligned with the backgrounded scene. Closing the activated window
    /// will result in the backgrounded scene reappearing.
    ///
    /// Targeting a scene session that is currently pushed will result in an error
    /// being delivered to the `errorHandler` of
    /// ``-[UIApplication activateSceneSessionForRequest: errorHandler:]``.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/uikit/uiwindowscenepushplacement?language=objc)
    #[unsafe(super(UIWindowScenePlacement, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIWindowScenePlacement")]
    pub struct UIWindowScenePushPlacement;
);

#[cfg(feature = "UIWindowScenePlacement")]
unsafe impl NSCopying for UIWindowScenePushPlacement {}

#[cfg(feature = "UIWindowScenePlacement")]
unsafe impl CopyingHelper for UIWindowScenePushPlacement {
    type Result = Self;
}

#[cfg(feature = "UIWindowScenePlacement")]
unsafe impl NSObjectProtocol for UIWindowScenePushPlacement {}

extern_methods!(
    #[cfg(feature = "UIWindowScenePlacement")]
    unsafe impl UIWindowScenePushPlacement {
        #[cfg(feature = "UISceneSession")]
        /// Creates the placement that will target the given `sceneSession`.
        /// - Parameter targetSceneSession: The scene session of the window scene that will be backgrounded.
        #[method_id(@__retain_semantics Other placementTargetingSceneSession:)]
        pub unsafe fn placementTargetingSceneSession(
            target_scene_session: &UISceneSession,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIWindowScenePlacement`
    #[cfg(feature = "UIWindowScenePlacement")]
    unsafe impl UIWindowScenePushPlacement {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
