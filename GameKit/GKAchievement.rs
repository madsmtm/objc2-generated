//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamekit/gkachievement?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKAchievement;
);

unsafe impl NSCoding for GKAchievement {}

unsafe impl NSObjectProtocol for GKAchievement {}

unsafe impl NSSecureCoding for GKAchievement {}

extern_methods!(
    unsafe impl GKAchievement {
        #[cfg(feature = "block2")]
        /// Asynchronously load all achievements for the local player
        #[method(loadAchievementsWithCompletionHandler:)]
        pub unsafe fn loadAchievementsWithCompletionHandler(
            completion_handler: Option<
                &block2::Block<dyn Fn(*mut NSArray<GKAchievement>, *mut NSError)>,
            >,
        );

        #[cfg(feature = "block2")]
        /// Reset the achievements progress for the local player. All the entries for the local player are removed from the server. Error will be nil on success.
        /// Possible reasons for error:
        /// 1. Local player not authenticated
        /// 2. Communications failure
        #[method(resetAchievementsWithCompletionHandler:)]
        pub unsafe fn resetAchievementsWithCompletionHandler(
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        /// Designated initializer
        #[unsafe(method_family(init))]
        #[method_id(initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Allocated<Self>,
            identifier: &NSString,
        ) -> Retained<Self>;

        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer"))]
        /// Initialize the achievement for a specific player. Use to submit participant achievements when ending a turn-based match.
        #[unsafe(method_family(init))]
        #[method_id(initWithIdentifier:player:)]
        pub unsafe fn initWithIdentifier_player(
            this: Allocated<Self>,
            identifier: &NSString,
            player: &GKPlayer,
        ) -> Retained<Self>;

        #[cfg(feature = "block2")]
        /// Report an array of achievements to the server. Percent complete is required. Points, completed state are set based on percentComplete. isHidden is set to NO anytime this method is invoked. Date is optional. Error will be nil on success.
        /// Possible reasons for error:
        /// 1. Local player not authenticated
        /// 2. Communications failure
        /// 3. Reported Achievement does not exist
        #[method(reportAchievements:withCompletionHandler:)]
        pub unsafe fn reportAchievements_withCompletionHandler(
            achievements: &NSArray<GKAchievement>,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        /// Achievement identifier
        #[unsafe(method_family(none))]
        #[method_id(identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        /// Setter for [`identifier`][Self::identifier].
        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: &NSString);

        /// Required, Percentage of achievement complete.
        #[method(percentComplete)]
        pub unsafe fn percentComplete(&self) -> c_double;

        /// Setter for [`percentComplete`][Self::percentComplete].
        #[method(setPercentComplete:)]
        pub unsafe fn setPercentComplete(&self, percent_complete: c_double);

        /// Set to NO until percentComplete = 100.
        #[method(isCompleted)]
        pub unsafe fn isCompleted(&self) -> bool;

        /// Date the achievement was last reported. Read-only. Created at initialization
        #[unsafe(method_family(none))]
        #[method_id(lastReportedDate)]
        pub unsafe fn lastReportedDate(&self) -> Retained<NSDate>;

        /// A banner will be momentarily displayed after reporting a completed achievement
        #[method(showsCompletionBanner)]
        pub unsafe fn showsCompletionBanner(&self) -> bool;

        /// Setter for [`showsCompletionBanner`][Self::showsCompletionBanner].
        #[method(setShowsCompletionBanner:)]
        pub unsafe fn setShowsCompletionBanner(&self, shows_completion_banner: bool);

        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer"))]
        /// The identifier of the player that earned the achievement.
        #[unsafe(method_family(none))]
        #[method_id(player)]
        pub unsafe fn player(&self) -> Option<Retained<GKPlayer>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GKAchievement {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// Deprecated
    unsafe impl GKAchievement {
        #[cfg(feature = "block2")]
        #[deprecated]
        #[method(reportAchievementWithCompletionHandler:)]
        pub unsafe fn reportAchievementWithCompletionHandler(
            &self,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        #[deprecated]
        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;
    }
);

extern_methods!(
    /// Obsoleted
    unsafe impl GKAchievement {
        /// * This method is obsolete. Calling this initialiser does nothing and will return nil **
        #[deprecated]
        #[unsafe(method_family(init))]
        #[method_id(initWithIdentifier:forPlayer:)]
        pub unsafe fn initWithIdentifier_forPlayer(
            this: Allocated<Self>,
            identifier: Option<&NSString>,
            player_id: &NSString,
        ) -> Option<Retained<Self>>;

        /// * This property is obsolete. **
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(playerID)]
        pub unsafe fn playerID(&self) -> Option<Retained<NSString>>;
    }
);
