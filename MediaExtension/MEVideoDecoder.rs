//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
#[cfg(feature = "objc2-core-video")]
use objc2_core_video::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// Provides a stateless factory interface for creating new MEVideoDecoder instances.
    ///
    /// The MEVideoDecoderExtension protocol provides a factory method to create a new MEVideoDecoder instance for a codecType implemented by the extension. A single MEVideoDecoderExtension is instantiated by the Video Toolbox, and will be called to create individual MEVideoDecoder instances as needed. If the codecType or FormatDescription passed to videoDecoderWithCodecType is not compatible with the MEVideoDecoder implementation, the factory call should fail and return MEErrorUnsupportedFeature.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/mediaextension/mevideodecoderextension?language=objc)
    pub unsafe trait MEVideoDecoderExtension: NSObjectProtocol {
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "objc2-core-media")]
        /// The factory method to create a new MEVideoDecoder.
        ///
        /// Creates a new MEVideoDecoder matching the given CMVideoCodecType and CMVideoFormatDescriptionRef.  If these parameters are not compatible with the MEVideoDecoder, the call should fail, returnig MEErrorUnsupportedFeature.
        ///
        /// Parameter `codecType`: The codec type for the requested decoder
        ///
        /// Parameter `videoFormatDescription`: A CMVideoFormatDescription describing the video data.
        ///
        /// Parameter `videoDecoderSpecifications`: A dictionary of videoDecoderSpecification values (See kVTVideoDecoderSpecification keys in VideoToolbox/VTDecompressionProperties.h).  This may be empty.
        ///
        /// Parameter `extensionDecoderPixelBufferManager`: An MEVideoDecoderPixelBufferManager instance that should be retained by the new MEVideoDecoder instance and used for output pixelBuffer configuration and allocation.
        ///
        /// Parameter `error`: On return, if initialization of the MEVideoDecoder fails, points to an NSError describing the nature of the failure.
        ///
        /// Returns: A newly created instance conforming to MEVideoDecoder.
        #[unsafe(method(videoDecoderWithCodecType:videoFormatDescription:videoDecoderSpecifications:extensionDecoderPixelBufferManager:error:_))]
        #[unsafe(method_family = none)]
        unsafe fn videoDecoderWithCodecType_videoFormatDescription_videoDecoderSpecifications_extensionDecoderPixelBufferManager_error(
            &self,
            codec_type: CMVideoCodecType,
            video_format_description: &CMVideoFormatDescription,
            video_decoder_specifications: &NSDictionary<NSString, AnyObject>,
            extension_decoder_pixel_buffer_manager: &MEVideoDecoderPixelBufferManager,
        ) -> Result<Retained<ProtocolObject<dyn MEVideoDecoder>>, Retained<NSError>>;
    }
);

extern_class!(
    /// Describes pixel buffer requirements and creates new pixel buffers.
    ///
    /// Contains the interfaces that the App Extension video decoder uses for two tasks. First, to declare its set of requirements for output CVPixelBuffers in the form of a pixelBufferAttributes dictionary. Second, to create pixelBuffers which match decoder output requirements but also satisfy VideoToolbox and client requirements.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/mediaextension/mevideodecoderpixelbuffermanager?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MEVideoDecoderPixelBufferManager;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MEVideoDecoderPixelBufferManager {}
);

impl MEVideoDecoderPixelBufferManager {
    extern_methods!(
        /// VideoToolbox will use these attributes when creating a PixelBuffer for the decoder.
        ///
        /// This can be updated by the decoder before requesting a new pixelBuffer.
        #[unsafe(method(pixelBufferAttributes))]
        #[unsafe(method_family = none)]
        pub unsafe fn pixelBufferAttributes(&self) -> Retained<NSDictionary<NSString, AnyObject>>;

        /// Setter for [`pixelBufferAttributes`][Self::pixelBufferAttributes].
        #[unsafe(method(setPixelBufferAttributes:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPixelBufferAttributes(
            &self,
            pixel_buffer_attributes: &NSDictionary<NSString, AnyObject>,
        );

        #[cfg(feature = "objc2-core-video")]
        /// Generates a pixel buffer using the session's pixel buffer pool.
        ///
        /// If implemented in Objective-C, the caller is responsible for releasing the returned CVPixelBuffer.
        ///
        /// Parameter `error`: If provided, returns error information in the event that the method fails.
        ///
        /// Returns: A pixel buffer compatible with the extension's most recently set pixelBufferAttributes
        #[unsafe(method(createPixelBufferAndReturnError:_))]
        // required for soundness, method has `returns_retained` attribute.
        #[unsafe(method_family = copy)]
        pub unsafe fn createPixelBufferAndReturnError(
            &self,
        ) -> Result<Retained<CVPixelBuffer>, Retained<NSError>>;

        /// VideoToolbox will register the described pixelFormat in both the Extension process and the client process.
        ///
        /// This property is appropriate for decoders which produce output in a custom pixel format.  This will generally only be used by decoders which produce RAW output, where the decoder's output buffers will only be consumed by an MERAWProcessor extension which registers the same pixel format. MERAWProcessor needs to manually register the custom pixel format using CVPixelFormatDescriptionRegisterDescriptionWithPixelFormatType().
        ///
        /// Parameter `customPixelFormat`: This dictionary contains a set of keys and values as described in CoreVideo/CVPixelFormatDescription.h suitable for providing
        /// as the 'description' parameter to CVPixelFormatDescriptionRegisterDescriptionWithPixelFormatType.  This must contain the
        /// custom pixel format fourCC as the value for the kCVPixelFormatCodecType key.
        #[unsafe(method(registerCustomPixelFormat:))]
        #[unsafe(method_family = none)]
        pub unsafe fn registerCustomPixelFormat(
            &self,
            custom_pixel_format: &NSDictionary<NSString, AnyObject>,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl MEVideoDecoderPixelBufferManager {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// Conveys directives or options from the VideoToolbox to guide decoder operation on a per-frame basis.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/mediaextension/medecodeframeoptions?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MEDecodeFrameOptions;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MEDecodeFrameOptions {}
);

impl MEDecodeFrameOptions {
    extern_methods!(
        /// A hint to the video decoder that a CVImageBuffer should not be emitted for this frame.  NULL will be returned instead.
        #[unsafe(method(doNotOutputFrame))]
        #[unsafe(method_family = none)]
        pub unsafe fn doNotOutputFrame(&self) -> bool;

        /// Setter for [`doNotOutputFrame`][Self::doNotOutputFrame].
        #[unsafe(method(setDoNotOutputFrame:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDoNotOutputFrame(&self, do_not_output_frame: bool);

        /// A hint to the video decoder that it would be OK to use a low-power mode that can not decode faster than 1x realtime.
        ///
        /// Note that this hint only takes the current decode session into account.  For example, if multiple instances of a decoder are operating at once, it may not actually be OK to use such a low-power mode if real-time playback might not be sustained across all the streams. This hint will be set to false during all uses other than 1x forward real-time playback, including seeking, playback at other rates, and export.
        #[unsafe(method(realTimePlayback))]
        #[unsafe(method_family = none)]
        pub unsafe fn realTimePlayback(&self) -> bool;

        /// Setter for [`realTimePlayback`][Self::realTimePlayback].
        #[unsafe(method(setRealTimePlayback:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setRealTimePlayback(&self, real_time_playback: bool);
    );
}

/// Methods declared on superclass `NSObject`.
impl MEDecodeFrameOptions {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaextension/mevideodecoderreadyformoremediadatadidchangenotification?language=objc)
    pub static MEVideoDecoderReadyForMoreMediaDataDidChangeNotification:
        &'static NSNotificationName;
}

/// These values are used to convey non-error status related to a frame decode operation.
///
/// Set by the decoder to indicate that no non-error status information is available.
///
/// Set by the decoder to report that output of this frame was dropped for non-error reasons, for example, if doNotOutputFrame was specified.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/mediaextension/medecodeframestatus?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MEDecodeFrameStatus(pub NSUInteger);
bitflags::bitflags! {
    impl MEDecodeFrameStatus: NSUInteger {
        #[doc(alias = "MEDecodeFrameNoStatus")]
        const NoStatus = 0;
        #[doc(alias = "MEDecodeFrameFrameDropped")]
        const FrameDropped = 1<<0;
    }
}

unsafe impl Encode for MEDecodeFrameStatus {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MEDecodeFrameStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// The primary object for a MediaExtension video decoder, providing an interface for VideoToolbox to talk to the decoder.
    ///
    /// The MEVideoDecoder protocol provides an interface for the VideoToolbox to interact with MediaExtension video decoders. MEVideoDecoder objects are always instantiated by the VideoToolbox. To create an MEVideoDecoder, the VideoToolbox first creates an MEVideoDecoderExtension object and calls its videoDecoderWithCodecType: method. MEVideoDecoders should expect to run in a sandboxed process without access to the file system, network, or other kernel resources.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/mediaextension/mevideodecoder?language=objc)
    pub unsafe trait MEVideoDecoder: NSObjectProtocol {
        /// The extension should implement this property returning YES if the decoder produces RAW ouput which requires the use of an MERAWProcessor for post-decode processing to produce renderable output.
        ///
        /// This optional property is queried on the extension when a VideoToolbox client queries the kVTDecompressionPropertyKey_DecoderProducesRAWOutput property on the hosting VTDecompressionSession.
        #[optional]
        #[unsafe(method(producesRAWOutput))]
        #[unsafe(method_family = none)]
        unsafe fn producesRAWOutput(&self) -> bool;

        /// The extension should return whether the content being decoded has interframe dependencies, if the decoder knows.
        ///
        /// This optional property is queried on the extension when a VideoToolbox client queries the kVTDecompressionPropertyKey_ContentHasInterframeDependencies property on the hosting VTDecompressionSession.
        #[optional]
        #[unsafe(method(contentHasInterframeDependencies))]
        #[unsafe(method_family = none)]
        unsafe fn contentHasInterframeDependencies(&self) -> bool;

        /// Sets the recommended number of threads to be used by the decoder.
        ///
        /// This optional property is set when a VideoToolbox client sets the kVTDecompressionPropertyKey_ThreadCount property on the hosting VTDecompressionSession.
        #[optional]
        #[unsafe(method(recommendedThreadCount))]
        #[unsafe(method_family = none)]
        unsafe fn recommendedThreadCount(&self) -> NSInteger;

        /// Setter for [`recommendedThreadCount`][Self::recommendedThreadCount].
        #[optional]
        #[unsafe(method(setRecommendedThreadCount:))]
        #[unsafe(method_family = none)]
        unsafe fn setRecommendedThreadCount(&self, recommended_thread_count: NSInteger);

        /// Returns the actual number of threads used by decoder.
        ///
        /// This optional property is queried when a VideoToolbox client queries the kVTDecompressionPropertyKey_ThreadCount property on the hosting VTDecompressionSession.
        #[optional]
        #[unsafe(method(actualThreadCount))]
        #[unsafe(method_family = none)]
        unsafe fn actualThreadCount(&self) -> NSInteger;

        /// Provides hints about quality tradeoffs between pixel formats.
        ///
        /// This optional property value is an array of NSNumbers with CMPixelFormatTypes values, ordered by quality from best to worse. This property is queried when a VideoToolbox client queries the kVTDecompressionPropertyKey_SupportedPixelFormatsOrderedByQuality property on the hosting VTDecompressionSession.
        #[optional]
        #[unsafe(method(supportedPixelFormatsOrderedByQuality))]
        #[unsafe(method_family = none)]
        unsafe fn supportedPixelFormatsOrderedByQuality(&self) -> Retained<NSArray<NSNumber>>;

        #[cfg(feature = "objc2-core-foundation")]
        /// Requests decoding at a smaller resolution than full-size.
        ///
        /// This optional property conveys a request for reduced resolution for decoding. Decoders that only support a fixed set of resolutions should pick the smallest resolution greater than or equal to the requested { width, height }. If the output CVPixelBuffer is not in a format where reduced resolution decoding is supported, this setting should be disregarded. This property is set on the extension when a VideoToolbox client sets the kVTDecompressionPropertyKey_ReducedResolutionDecode property on the hosting VTDecompressionSession.
        #[optional]
        #[unsafe(method(reducedResolution))]
        #[unsafe(method_family = none)]
        unsafe fn reducedResolution(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`reducedResolution`][Self::reducedResolution].
        #[optional]
        #[unsafe(method(setReducedResolution:))]
        #[unsafe(method_family = none)]
        unsafe fn setReducedResolution(&self, reduced_resolution: CGSize);

        /// Provides a list of output pixel formats where the decoder supports reduced resolution decoding
        ///
        /// This optional property should return an array of NSNumbers holding CMPixelFormatType values. This property is queried when a VideoToolbox client queries the kVTDecompressionPropertyKey_PixelFormatsWithReducedResolutionSupport property on the hosting VTDecompressionSession.
        #[optional]
        #[unsafe(method(pixelFormatsWithReducedResolutionDecodeSupport))]
        #[unsafe(method_family = none)]
        unsafe fn pixelFormatsWithReducedResolutionDecodeSupport(
            &self,
        ) -> Retained<NSArray<NSNumber>>;

        /// Indicates the readiness of the decoder to accept more sample buffers.
        ///
        /// Video decoders which operate asynchronously often have a fixed capacity for buffers in flight in the decoder. This property allows the decoder to signal to the VideoToolbox that its internal buffers are full and it cannot accept more samples at this time. The decoder must use MEVideoDecoderReadyForMoreMediaDataDidChangeNotification to notify the VideoToolbox when this property changes.
        #[unsafe(method(isReadyForMoreMediaData))]
        #[unsafe(method_family = none)]
        unsafe fn isReadyForMoreMediaData(&self) -> bool;

        #[cfg(all(
            feature = "block2",
            feature = "objc2-core-media",
            feature = "objc2-core-video"
        ))]
        /// Requests that the extension decode a video frame.
        ///
        /// The completionHandler block must be called for every decodeFrameFromSampleBuffer call when decode is complete. The completion handler block should return either a decoded pixelBuffer, an error, or a decodeStatus indicating the frame was dropped.
        ///
        /// Parameter `sampleBuffer`: A CMSampleBuffer containing one video frame.
        ///
        /// Parameter `options`: A MEDecodeFrameOptions containing specific requests for this frame.
        ///
        /// Parameter `completionHandler`: The handler that will be invoked when a frame is decoded and ready to be sent back to
        /// caller. This block will not necessarily be called in display order.
        /// 'imageBuffer'
        /// Contains the decompressed frame if decompression was successful, otherwise NULL. The imageBuffer must be allocated using MEVideoDecoderPixelBufferManager.
        /// 'decodeStatus'
        /// Reports the status of decoding the sampleBuffer.
        /// 'error'
        /// An NSError object that will contain error information if the method fails, otherwise nil. Errors outside of MediaExtensionErrorDomain will be reported as kVTVideoDecoderUnknownErr to the VTDecompressionSession client.
        #[unsafe(method(decodeFrameFromSampleBuffer:options:completionHandler:))]
        #[unsafe(method_family = none)]
        unsafe fn decodeFrameFromSampleBuffer_options_completionHandler(
            &self,
            sample_buffer: &CMSampleBuffer,
            options: &MEDecodeFrameOptions,
            completion_handler: &block2::DynBlock<
                dyn Fn(*mut CVImageBuffer, MEDecodeFrameStatus, *mut NSError),
            >,
        );

        #[cfg(feature = "objc2-core-media")]
        /// Asks the extension whether the decoder can decode frames with the given format description.
        ///
        /// Some video decoders are able to accommodate minor changes in format without needing to be completely reset in a new session. This function can be used to test whether a format change is allowed.
        ///
        /// Parameter `formatDescription`: The new format description that need to be evaluated.
        ///
        /// Returns: YES if the decoder can continue decoding samples without being reset, NO if this is not possible. If this method returns NO, typically the VideoToolbox will release the current decoder instance and instantiate a new one using the new format description.
        #[optional]
        #[unsafe(method(canAcceptFormatDescription:))]
        #[unsafe(method_family = none)]
        unsafe fn canAcceptFormatDescription(
            &self,
            format_description: &CMFormatDescription,
        ) -> bool;
    }
);
