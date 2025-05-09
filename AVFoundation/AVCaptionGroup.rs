//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// An instance of AVCaptionGroup represents zero or more captions that intersect in time.
    ///
    /// The time range of each caption may overlap as there can be more than one active caption at a time. A sequence of AVCaptionGroup objects represents such overlapping caption timeline.
    ///
    /// An instance of AVCaptionGroup has a time range and a list of active captions for the time range. Two successive AVCaptionGroup objects have contiguous and non-overlapping time ranges. A new AVCaptionGroup time range commences whenever any of caption becomes active or inactive. When a caption spans over multiple AVCaptionGroup time ranges, these  AVCaptionGroup objects refer to an equal AVCaption object.
    ///
    /// An empty AVCaptionGroup represents the time range without any active captions.
    ///
    /// The list of captions in the group is ordered according to the document order. For example, suppose a TTML document has two temporally overhapping captions:
    ///
    /// <div>
    /// <p begin="1s" end="3s">
    /// Hello
    /// <p>
    /// <p begin="0s" end="2s">
    /// World
    /// <p>
    /// </div>
    ///
    /// AVCaptionGroup for time range 1s to 2s has the list of captions: Hello and World in this order despite the fact that "World" is shown earlier than "Hello".
    ///
    /// A client may use AVCaptionGroup to get the list of active captions for the time range. For example, presentation processing may find the AVCaptionGroup object for the current time, get the list of captions, and place them into the destination display region.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptiongroup?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptionGroup;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for AVCaptionGroup {}
);

impl AVCaptionGroup {
    extern_methods!(
        #[cfg(all(feature = "AVCaption", feature = "objc2-core-media"))]
        /// Initializes a caption group with the given set of captions and the time range.
        ///
        /// Every caption in the array must be equal or sub range of the time range, otherwise an exception is raised.
        ///
        /// Parameter `captions`: The captions that will be included in the group. The array is coped.
        ///
        /// Returns: A newly-initialized caption group.
        #[unsafe(method(initWithCaptions:timeRange:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCaptions_timeRange(
            this: Allocated<Self>,
            captions: &NSArray<AVCaption>,
            time_range: CMTimeRange,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-media")]
        /// Initializes an empty caption group with the given time range.
        ///
        /// This is a convenient initializer to create an empty caption group time range.
        ///
        /// Parameter `timeRange`: The time range for which there are no captions.
        ///
        /// Returns: A newly-initialized empty caption group.
        #[unsafe(method(initWithTimeRange:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithTimeRange(
            this: Allocated<Self>,
            time_range: CMTimeRange,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-media")]
        /// The time range represented by the caption group.
        ///
        /// If there are no captions in the group (i.e. the value of the captions property is an empty array), then the value of this property represents the time range of a sequence where no captions are present.
        #[unsafe(method(timeRange))]
        #[unsafe(method_family = none)]
        pub unsafe fn timeRange(&self) -> CMTimeRange;

        #[cfg(feature = "AVCaption")]
        /// An array of AVCaption objects.
        ///
        /// If the value is an empty array, the caption group represents a region of the timeline in which there are no captions.
        #[unsafe(method(captions))]
        #[unsafe(method_family = none)]
        pub unsafe fn captions(&self) -> Retained<NSArray<AVCaption>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl AVCaptionGroup {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
