//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;

use crate::*;

/// These constants are returned by the AVPlayerLooper status property to indicate whether it can successfully accomplish looping playback.
///
/// Indicates that the status of the looper is not yet known.
///
/// Indicates that the looper is ready for looping playback.
///
/// Indicates that the looper is not able to perform looping playback because of an error. The error is described by the value of the error property.
///
/// Indicates that the looper is no longer looping because -disableLooping was invoked.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avplayerlooperstatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVPlayerLooperStatus(pub NSInteger);
impl AVPlayerLooperStatus {
    #[doc(alias = "AVPlayerLooperStatusUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "AVPlayerLooperStatusReady")]
    pub const Ready: Self = Self(1);
    #[doc(alias = "AVPlayerLooperStatusFailed")]
    pub const Failed: Self = Self(2);
    #[doc(alias = "AVPlayerLooperStatusCancelled")]
    pub const Cancelled: Self = Self(3);
}

unsafe impl Encode for AVPlayerLooperStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVPlayerLooperStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// These constants are the allowable values of Looper's existingItemsOrder initization parameter.
///
/// Indicates that the looper will insert the replica items before any existing items in the specified AVQueuePlayer's play queue. This is default behavior.
///
/// Indicates that the looper will insert the replica items after any existing items in the specified AVQueuePlayer's play queue.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avplayerlooperitemordering?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVPlayerLooperItemOrdering(pub NSInteger);
impl AVPlayerLooperItemOrdering {
    #[doc(alias = "AVPlayerLooperItemOrderingLoopingItemsPrecedeExistingItems")]
    pub const LoopingItemsPrecedeExistingItems: Self = Self(0);
    #[doc(alias = "AVPlayerLooperItemOrderingLoopingItemsFollowExistingItems")]
    pub const LoopingItemsFollowExistingItems: Self = Self(1);
}

unsafe impl Encode for AVPlayerLooperItemOrdering {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVPlayerLooperItemOrdering {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avplayerlooper?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVPlayerLooper;
);

unsafe impl Send for AVPlayerLooper {}

unsafe impl Sync for AVPlayerLooper {}

unsafe impl NSObjectProtocol for AVPlayerLooper {}

extern_methods!(
    unsafe impl AVPlayerLooper {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(all(
            feature = "AVPlayer",
            feature = "AVPlayerItem",
            feature = "objc2-core-media"
        ))]
        /// Returns an instance of AVPlayerLooper to loop specified AVPlayerItem within the specified time range with specified AVQueuePlayer.
        ///
        /// Parameter `player`: Must not be nil
        ///
        /// Parameter `itemToLoop`: Must not be nil
        ///
        /// Parameter `loopRange`: Playback time range in [0, itemToLoop's duration]. kCMTimeRangeInvalid means [0, itemToLoop's duration].
        ///
        /// Returns: An instance of AVPlayerLooper.
        ///
        /// The specified AVPlayerItem will be used as a template to generate at least 3 AVPlayerItem replicas and the replicas will be inserted into specified AVQueuePlayer's play queue to accomplish the looping playback. The specified AVPlayerItem should have its asset's duration property loaded beforehand so looping setup work would not be blocked until the duration value is known. Otherwise, AVPlayerLooper's status property is  AVPlayerLooperStatusUnknown until the duration property is loaded. The specified AVPlayerItem will not be used in the actual looping playback. Furthermore, AVPlayerItem replicas will be generated at initialization time so any changes made to the specified AVPlayerItem's property afterwards will not be reflected in the replicas used for looping playback. Specified CMTimeRange will limit each item loop iteration to playing within the specified time range. To play from beginning and the whole duration of the item, specify kCMTimeRangeInvalid for the range parameter. Time range will be accomplished by seeking to range start time and setting AVPlayerItem's forwardPlaybackEndTime property on the looping item replicas. Client should not modify AVQueuePlayer's play queue while AVPlayerLooper is performing the looping. AVPlayerLooper will insert the replica items before any existing items in the specified AVQueuePlayer's play queue and change the actionAtItemEnd to AVPlayerActionAtItemEndAdvance if required. AVQueuePlayer's play queue and actionAtItemEnd will be restored when -disableLooping method is called and then current looping item replicas completes playback or when AVPlayerLooper is destroyed. While AVPlayerLooper is being initialized, the specified AVQueuePlayer will be paused (rate of 0.0) if necessary and the original player rate will be restored after initialization completes. The client shall set the specified AVQueuePlayer's rate to 0 beforehand if additional set-up work needs to be performed after AVPlayerLooper initialization and before starting looping playback. An NSInvalidArgumentException will be raised if the player and template item are not specified or the template item has a 0 duration. An NSInvalidArgumentException will be raised if a valid time range has a duration of 0 or is not contained within time 0 and duration of the templateItem.
        #[method_id(@__retain_semantics Other playerLooperWithPlayer:templateItem:timeRange:)]
        pub unsafe fn playerLooperWithPlayer_templateItem_timeRange(
            player: &AVQueuePlayer,
            item_to_loop: &AVPlayerItem,
            loop_range: CMTimeRange,
        ) -> Retained<Self>;

        #[cfg(all(feature = "AVPlayer", feature = "AVPlayerItem"))]
        /// Returns an instance of AVPlayerLooper to loop specified AVPlayerItem with specified AVQueuePlayer.
        ///
        /// Parameter `player`: Must not be nil
        ///
        /// Parameter `itemToLoop`: Must not be nil
        ///
        /// Returns: An instance of AVPlayerLooper.
        ///
        /// Equivalent to +playerLooperWithPlayer:templateItem:timeRange: and passing in kCMTimeRangeInvalid for timeRange parameter.
        #[method_id(@__retain_semantics Other playerLooperWithPlayer:templateItem:)]
        pub unsafe fn playerLooperWithPlayer_templateItem(
            player: &AVQueuePlayer,
            item_to_loop: &AVPlayerItem,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "AVPlayer",
            feature = "AVPlayerItem",
            feature = "objc2-core-media"
        ))]
        /// Initializes an instance of AVPlayerLooper to loop specified AVPlayerItem within specified time range with specified AVQueuePlayer.
        ///
        /// Parameter `player`: Must not be nil
        ///
        /// Parameter `itemToLoop`: Must not be nil
        ///
        /// Parameter `loopRange`: Playback time range in [0, itemToLoop's duration]. kCMTimeRangeInvalid means [0, itemToLoop's duration].
        ///
        /// Returns: An initialized AVPlayerLooper.
        ///
        /// Equivalent to -initWithPlayer:templateItem:timeRange:existingItemsOrdering: and passing AVPlayerLooperItemOrderingLoopingItemsPrecedeExistingItems as the beforeOrAfter parameter.
        #[method_id(@__retain_semantics Init initWithPlayer:templateItem:timeRange:)]
        pub unsafe fn initWithPlayer_templateItem_timeRange(
            this: Allocated<Self>,
            player: &AVQueuePlayer,
            item_to_loop: &AVPlayerItem,
            loop_range: CMTimeRange,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "AVPlayer",
            feature = "AVPlayerItem",
            feature = "objc2-core-media"
        ))]
        /// Initializes an instance of AVPlayerLooper to loop specified AVPlayerItem within specified time range with specified AVQueuePlayer and adhering to specified ordering of existing items in the play queue.
        ///
        /// Parameter `player`: Must not be nil
        ///
        /// Parameter `itemToLoop`: Must not be nil
        ///
        /// Parameter `loopRange`: Playback time range in [0, itemToLoop's duration]. kCMTimeRangeInvalid means [0, itemToLoop's duration].
        ///
        /// Parameter `itemOrdering`: Specifes if replica items are to be inserted before or after existing items in the specified AVQueuePlayer.
        ///
        /// Returns: An initialized AVPlayerLooper.
        ///
        /// The specified AVPlayerItem will be used as a template to generate at least 3 AVPlayerItem replicas and the replicas will be inserted into specified AVQueuePlayer's play queue to accomplish the looping playback. The specified AVPlayerItem should have its asset's duration property loaded beforehand so looping setup work would not be blocked until the duration value is known. Otherwise, AVPlayerLooper's status property is  AVPlayerLooperStatusUnknown until the duration property is loaded. The specified AVPlayerItem will not be used in the actual looping playback. Furthermore, AVPlayerItem replicas will be generated at initialization time so any changes made to the specified AVPlayerItem's property afterwards will not be reflected in the replicas used for looping playback. Specified CMTimeRange will limit each item loop iteration to playing within the specified time range. To play from beginning and the whole duration of the item, specify kCMTimeRangeInvalid for the range parameter. Time range will be accomplished by seeking to range start time and setting AVPlayerItem's forwardPlaybackEndTime property on the looping item replicas. Client should not modify AVQueuePlayer's play queue while AVPlayerLooper is performing the looping. AVPlayerLooper will insert the replica items in the specified AVQueuePlayer's play queue before or after existing equeued items according to the specified AVPlayerLooperItemOrdering. The looper will change the actionAtItemEnd to AVPlayerActionAtItemEndAdvance if required. AVQueuePlayer's play queue and actionAtItemEnd will be restored when -disableLooping method is called and then current looping item replicas completes playback or when AVPlayerLooper is destroyed. While AVPlayerLooper is being initialized, the specified AVQueuePlayer will be paused (rate of 0.0) if necessary and the original player rate will be restored after initialization completes. The client shall set the specified AVQueuePlayer's rate to 0 beforehand if additional set-up work needs to be performed after AVPlayerLooper initialization and before starting looping playback. An NSInvalidArgumentException will be raised if the player and template item are not specified or the template item has a 0 duration. An NSInvalidArgumentException will be raised if a valid time range has a duration of 0 or is not contained within time 0 and duration of the templateItem.
        #[method_id(@__retain_semantics Init initWithPlayer:templateItem:timeRange:existingItemsOrdering:)]
        pub unsafe fn initWithPlayer_templateItem_timeRange_existingItemsOrdering(
            this: Allocated<Self>,
            player: &AVQueuePlayer,
            item_to_loop: &AVPlayerItem,
            loop_range: CMTimeRange,
            item_ordering: AVPlayerLooperItemOrdering,
        ) -> Retained<Self>;

        /// The ability of the receiver to be used for looping playback.
        ///
        /// The value of this property is an AVPlayerLooperStatus that indicates whether the receiver is ready for looping playback. When the value of this property is AVPlayerStatusFailed, the receiver can no longer be used for playback and a new instance needs to be created in its place. When this happens, clients can check the value of the error property to determine the nature of the failure. This property is key value observable.
        #[method(status)]
        pub unsafe fn status(&self) -> AVPlayerLooperStatus;

        /// If the receiver's status is AVPlayerLooperStatusFailed, this describes the error that caused the failure.
        ///
        /// The value of this property is a NSError that describes what caused the receiver to not be able to perform looping playback. If the receiver's status is not AVPlayerLooperStatusFailed, the value of this property is nil.
        #[method_id(@__retain_semantics Other error)]
        pub unsafe fn error(&self) -> Option<Retained<NSError>>;

        /// Disables the item looping
        ///
        /// AVPlayerLooper will stop performing player queue operations for looping and let the current looping item replica play to the end. The player's original actionAtItemEnd property will be restored afterwards. After this method is called, the value of the receiver's status property will be AVPlayerLooperStatusCancelled.
        #[method(disableLooping)]
        pub unsafe fn disableLooping(&self);

        /// Number of times the specified AVPlayerItem has been played
        ///
        /// Starts at 0 and increments when the player starts playback of the AVPlayerItem again. This property is key value observable.
        #[method(loopCount)]
        pub unsafe fn loopCount(&self) -> NSInteger;

        #[cfg(feature = "AVPlayerItem")]
        /// Returns an array containing replicas of specified AVPlayerItem used to accomplish the looping
        ///
        /// AVPlayerLooper creates replicas of the template AVPlayerItem using -copyWithZone: and inserts the replicas in the specified AVQueuePlayer to accomplish the looping. The AVPlayerItem replicas are for informational purposes and to allow the client to apply properties that are not transferred from the template AVPlayerItem to the replicas. The client can determine the number of replicas created and can listen for notifications and property changes from the replicas if desired. AVPlayerItemOutputs and AVPlayerItemMediaDataCollectors are not transferred to the replicas so the client should add them to each replica if desired. The client shall not modify the properties on the replicas that would disrupt looping playback. Examples of such properties are playhead time/date, selected media option, and forward playback end time. This property is key value observable.
        ///
        /// Returns: Array containing replicas of specified AVPlayerItem
        #[method_id(@__retain_semantics Other loopingPlayerItems)]
        pub unsafe fn loopingPlayerItems(
            &self,
            mtm: MainThreadMarker,
        ) -> Retained<NSArray<AVPlayerItem>>;
    }
);