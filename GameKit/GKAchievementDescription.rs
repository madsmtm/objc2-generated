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
    /// GKAchievementDescription is a full description of the achievement as defined before app submission in App Store Connect.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/gamekit/gkachievementdescription?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKAchievementDescription;
);

unsafe impl NSCoding for GKAchievementDescription {}

unsafe impl NSObjectProtocol for GKAchievementDescription {}

unsafe impl NSSecureCoding for GKAchievementDescription {}

extern_methods!(
    unsafe impl GKAchievementDescription {
        #[cfg(feature = "block2")]
        /// Asynchronously load all achievement descriptions
        #[method(loadAchievementDescriptionsWithCompletionHandler:)]
        pub unsafe fn loadAchievementDescriptionsWithCompletionHandler(
            completion_handler: Option<
                &block2::Block<dyn Fn(*mut NSArray<GKAchievementDescription>, *mut NSError)>,
            >,
        );

        #[unsafe(method_family(none))]
        #[method_id(identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        /// The group identifier for the achievement, if one exists.
        #[unsafe(method_family(none))]
        #[method_id(groupIdentifier)]
        pub unsafe fn groupIdentifier(&self) -> Option<Retained<NSString>>;

        /// The title of the achievement.
        #[unsafe(method_family(none))]
        #[method_id(title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        /// The description for an unachieved achievement.
        #[unsafe(method_family(none))]
        #[method_id(achievedDescription)]
        pub unsafe fn achievedDescription(&self) -> Retained<NSString>;

        /// The description for an achieved achievement.
        #[unsafe(method_family(none))]
        #[method_id(unachievedDescription)]
        pub unsafe fn unachievedDescription(&self) -> Retained<NSString>;

        /// Maximum points available for completing this achievement.
        #[method(maximumPoints)]
        pub unsafe fn maximumPoints(&self) -> NSInteger;

        /// Whether or not the achievement should be listed or displayed if not yet unhidden by the game.
        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;

        /// Whether or not the achievement will be reported by the game when the user earns it again. This allows the achievement to be used for challenges when the recipient has previously earned it.
        #[method(isReplayable)]
        pub unsafe fn isReplayable(&self) -> bool;

        /// If present, the rarity of the achievement expressed as a percentage of players that earned it. Null if not enough data is available to compute it.
        #[unsafe(method_family(none))]
        #[method_id(rarityPercent)]
        pub unsafe fn rarityPercent(&self) -> Option<Retained<NSNumber>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GKAchievementDescription {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// UI
    unsafe impl GKAchievementDescription {
        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(image)]
        pub unsafe fn image(&self) -> Option<Retained<NSImage>>;

        #[cfg(all(feature = "block2", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        #[method(loadImageWithCompletionHandler:)]
        pub unsafe fn loadImageWithCompletionHandler(
            &self,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSImage, *mut NSError)>>,
        );

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[unsafe(method_family(none))]
        #[method_id(incompleteAchievementImage)]
        pub unsafe fn incompleteAchievementImage() -> Retained<NSImage>;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[unsafe(method_family(none))]
        #[method_id(placeholderCompletedAchievementImage)]
        pub unsafe fn placeholderCompletedAchievementImage() -> Retained<NSImage>;
    }
);
