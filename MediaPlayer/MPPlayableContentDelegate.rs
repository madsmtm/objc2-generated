//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

extern_protocol!(
    #[deprecated = "Use CarPlay framework"]
    #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
    pub unsafe trait MPPlayableContentDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSIndexPath",
            feature = "MediaPlayer_MPPlayableContentManager"
        ))]
        #[deprecated = "Use CarPlay framework"]
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[optional]
        #[method(playableContentManager:initiatePlaybackOfContentItemAtIndexPath:completionHandler:)]
        unsafe fn playableContentManager_initiatePlaybackOfContentItemAtIndexPath_completionHandler(
            &self,
            content_manager: &MPPlayableContentManager,
            index_path: &NSIndexPath,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "MediaPlayer_MPPlayableContentManager"
        ))]
        #[deprecated = "Use Intents framework for initiating playback queues."]
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[optional]
        #[method(playableContentManager:initializePlaybackQueueWithCompletionHandler:)]
        unsafe fn playableContentManager_initializePlaybackQueueWithCompletionHandler(
            &self,
            content_manager: &MPPlayableContentManager,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "MediaPlayer_MPPlayableContentManager"
        ))]
        #[deprecated = "Use Intents framework for initiating playback queues."]
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[optional]
        #[method(playableContentManager:initializePlaybackQueueWithContentItems:completionHandler:)]
        unsafe fn playableContentManager_initializePlaybackQueueWithContentItems_completionHandler(
            &self,
            content_manager: &MPPlayableContentManager,
            content_items: Option<&NSArray>,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(all(
            feature = "MediaPlayer_MPPlayableContentManager",
            feature = "MediaPlayer_MPPlayableContentManagerContext"
        ))]
        #[deprecated = "Use CarPlay framework"]
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[optional]
        #[method(playableContentManager:didUpdateContext:)]
        unsafe fn playableContentManager_didUpdateContext(
            &self,
            content_manager: &MPPlayableContentManager,
            context: &MPPlayableContentManagerContext,
        );
    }

    unsafe impl ProtocolType for dyn MPPlayableContentDelegate {}
);