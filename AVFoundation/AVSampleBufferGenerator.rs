//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avsamplebuffergenerator?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVSampleBufferGenerator;
);

unsafe impl Send for AVSampleBufferGenerator {}

unsafe impl Sync for AVSampleBufferGenerator {}

unsafe impl NSObjectProtocol for AVSampleBufferGenerator {}

extern_methods!(
    unsafe impl AVSampleBufferGenerator {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(all(feature = "AVAsset", feature = "objc2-core-media"))]
        /// Creates an instance of AVSampleBufferGenerator to generate sample buffers from the specified asset.
        ///
        /// Parameter `asset`: The asset from which sample buffers will be created.
        ///
        /// Parameter `timebase`: The generator timebase, which governs when sample data for sample buffers is loaded. If NULL, sample data is loaded synchronously.
        ///
        /// Returns: An instance of AVSampleBufferGenerator.
        ///
        /// If the specified asset is an HTTP Live Streaming asset, the generator cannot create sample buffers.
        #[method_id(@__retain_semantics Init initWithAsset:timebase:)]
        pub unsafe fn initWithAsset_timebase(
            this: Allocated<Self>,
            asset: &AVAsset,
            timebase: Option<&CMTimebase>,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-media")]
        #[deprecated = "Use -createSampleBufferForRequest: error:, passing NULL for the error if not required"]
        #[method_id(@__retain_semantics Other createSampleBufferForRequest:)]
        pub unsafe fn createSampleBufferForRequest(
            &self,
            request: &AVSampleBufferRequest,
        ) -> Option<Retained<CMSampleBuffer>>;

        /// Creates a batch to handle multiple sample buffers, allowing to asynchronously load sample data and optimize I/O when possible.
        ///
        /// Returns: An instance of an AVSampleBufferGeneratorBatch that can be used in calls to createSampleBufferForRequest:addingToBatch:error: of the same AVSampleBufferGenerator instance.
        #[method_id(@__retain_semantics Other makeBatch)]
        pub unsafe fn makeBatch(&self) -> Retained<AVSampleBufferGeneratorBatch>;

        #[cfg(all(feature = "block2", feature = "objc2-core-media"))]
        /// Allows the client to get notified when the sample buffer data is ready, or as soon as an error has occured.
        ///
        /// Parameter `completionHandler`: The completionHandler will be called, when the sample buffer data is ready, or as soon as an error has occurred.
        #[method(notifyOfDataReadyForSampleBuffer:completionHandler:)]
        pub unsafe fn notifyOfDataReadyForSampleBuffer_completionHandler(
            sbuf: &CMSampleBuffer,
            completion_handler: &block2::Block<dyn Fn(Bool, *mut NSError)>,
        );
    }
);

/// Indicates the direction in which the samples should be generated for the AVSampleBufferRequest.
///
///
/// Indicates only one sample will be loaded at [AVSampleBufferRequest startCursor], and the [AVSampleBufferRequest limitCursor], [AVSampleBufferRequest preferredMinSampleCount], and [AVSampleBufferRequest maxSampleCount] will be ignored.
///
/// Indicates zero or more following samples may be loaded from [AVSampleBufferRequest startCursor], subject to [AVSampleBufferRequest limitCursor], [AVSampleBufferRequest preferredMinSampleCount], and [AVSampleBufferRequest maxSampleCount]
///
/// Indicates zero or more preceeding samples may be loaded from [AVSampleBufferRequest startCursor], subject to [AVSampleBufferRequest limitCursor], [AVSampleBufferRequest preferredMinSampleCount], and [AVSampleBufferRequest maxSampleCount]
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avsamplebufferrequestdirection?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVSampleBufferRequestDirection(pub NSInteger);
impl AVSampleBufferRequestDirection {
    #[doc(alias = "AVSampleBufferRequestDirectionForward")]
    pub const Forward: Self = Self(1);
    #[doc(alias = "AVSampleBufferRequestDirectionNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "AVSampleBufferRequestDirectionReverse")]
    pub const Reverse: Self = Self(-1);
}

unsafe impl Encode for AVSampleBufferRequestDirection {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVSampleBufferRequestDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Defines the allowed values for AVSampleBufferRequest's mode property.
///
///
/// Sample data for requests with AVSampleBufferRequestModeImmediate will be loaded as soon as possible. The current time represented by AVSampleBufferGenerator's timebase has no influence on these requests.
///
/// AVSampleBufferRequestModeScheduled indicates that a request is needed by the time AVSampleBufferGenerator's timebase reaches the CMSampleBuffer's presentationTime or, if specificed, AVSampleBufferRequest's overrideTime. The AVSampleBufferGenerator will attempt to deliver sample data with sufficient leeway for downstream processing. It will also attempt to hold off on loading data until the CMSampleBuffer is needed. If AVSampleBufferGenerator's timebase has a rate of zero, this mode behaves like AVSampleBufferRequestModeImmediate.
///
/// The AVSampleBufferGenerator will attempt to read data for opportunistic requests as soon as possible. However, in situations with multiple competing requests, the AVSampleBufferGenerator may defer an opportunistic request in favor of another immediate request or a scheduled requests with a presentation time close to the timebase time. Because a request with AVSampleBufferRequestModeOpportunistic may be postponed indefinitely, this mode should not be used for time-sensitive processing.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avsamplebufferrequestmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVSampleBufferRequestMode(pub NSInteger);
impl AVSampleBufferRequestMode {
    #[doc(alias = "AVSampleBufferRequestModeImmediate")]
    pub const Immediate: Self = Self(0);
    #[doc(alias = "AVSampleBufferRequestModeScheduled")]
    pub const Scheduled: Self = Self(1);
    #[doc(alias = "AVSampleBufferRequestModeOpportunistic")]
    pub const Opportunistic: Self = Self(2);
}

unsafe impl Encode for AVSampleBufferRequestMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVSampleBufferRequestMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// An AVSampleBufferRequest describes a CMSampleBuffer creation request.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avsamplebufferrequest?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVSampleBufferRequest;
);

unsafe impl NSObjectProtocol for AVSampleBufferRequest {}

extern_methods!(
    unsafe impl AVSampleBufferRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "AVSampleCursor")]
        #[method_id(@__retain_semantics Init initWithStartCursor:)]
        pub unsafe fn initWithStartCursor(
            this: Allocated<Self>,
            start_cursor: &AVSampleCursor,
        ) -> Retained<Self>;

        #[cfg(feature = "AVSampleCursor")]
        #[method_id(@__retain_semantics Other startCursor)]
        pub unsafe fn startCursor(&self) -> Retained<AVSampleCursor>;

        #[method(direction)]
        pub unsafe fn direction(&self) -> AVSampleBufferRequestDirection;

        /// Setter for [`direction`][Self::direction].
        #[method(setDirection:)]
        pub unsafe fn setDirection(&self, direction: AVSampleBufferRequestDirection);

        #[cfg(feature = "AVSampleCursor")]
        #[method_id(@__retain_semantics Other limitCursor)]
        pub unsafe fn limitCursor(&self) -> Option<Retained<AVSampleCursor>>;

        #[cfg(feature = "AVSampleCursor")]
        /// Setter for [`limitCursor`][Self::limitCursor].
        #[method(setLimitCursor:)]
        pub unsafe fn setLimitCursor(&self, limit_cursor: Option<&AVSampleCursor>);

        #[method(preferredMinSampleCount)]
        pub unsafe fn preferredMinSampleCount(&self) -> NSInteger;

        /// Setter for [`preferredMinSampleCount`][Self::preferredMinSampleCount].
        #[method(setPreferredMinSampleCount:)]
        pub unsafe fn setPreferredMinSampleCount(&self, preferred_min_sample_count: NSInteger);

        #[method(maxSampleCount)]
        pub unsafe fn maxSampleCount(&self) -> NSInteger;

        /// Setter for [`maxSampleCount`][Self::maxSampleCount].
        #[method(setMaxSampleCount:)]
        pub unsafe fn setMaxSampleCount(&self, max_sample_count: NSInteger);

        #[method(mode)]
        pub unsafe fn mode(&self) -> AVSampleBufferRequestMode;

        /// Setter for [`mode`][Self::mode].
        #[method(setMode:)]
        pub unsafe fn setMode(&self, mode: AVSampleBufferRequestMode);

        #[cfg(feature = "objc2-core-media")]
        #[method(overrideTime)]
        pub unsafe fn overrideTime(&self) -> CMTime;

        #[cfg(feature = "objc2-core-media")]
        /// Setter for [`overrideTime`][Self::overrideTime].
        #[method(setOverrideTime:)]
        pub unsafe fn setOverrideTime(&self, override_time: CMTime);
    }
);

extern_class!(
    /// An AVSampleBufferGeneratorBatch provides an optimized way to load sample data asynchronously for multiple CMSampleBuffers in an asset.
    ///
    /// The AVSampleBufferGeneratorBatch loads sample data asynchronously, by aggregating adjacent I/O requests and overlapping them when possible for all CMSampleBuffers within a batch.
    /// An AVSampleBufferGeneratorBatch is associated with an AVSampleBufferGenerator. See -[AVSampleBufferGenerator makeBatch] to create an AVSampleBufferGeneratorBatch.
    /// See -[AVSampleBufferGeneratorBatch createSampleBufferForRequest: addingToBatch: error:] to create a CMSampleBuffer, defer I/O for its data, and build up a batch.
    /// Subclasses of this type that are used from Swift must fulfill the requirements of a Sendable type.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avsamplebuffergeneratorbatch?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVSampleBufferGeneratorBatch;
);

unsafe impl Send for AVSampleBufferGeneratorBatch {}

unsafe impl Sync for AVSampleBufferGeneratorBatch {}

unsafe impl NSObjectProtocol for AVSampleBufferGeneratorBatch {}

extern_methods!(
    unsafe impl AVSampleBufferGeneratorBatch {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "block2")]
        /// Loads sample data asynchronously for all CMSampleBuffers within a batch.
        /// This can only be called once on a batch, an exception will be thrown otherwise.
        ///
        /// Parameter `completionHandler`: The completionHandler is called once, when all CMSampleBuffers in the batch are data-ready, or as soon as an error has occurred.
        #[method(makeDataReadyWithCompletionHandler:)]
        pub unsafe fn makeDataReadyWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        /// Attempt to cancel any I/O for this batch. The associated sample buffers will have their data ready handler invoked with an error.
        #[method(cancel)]
        pub unsafe fn cancel(&self);
    }
);