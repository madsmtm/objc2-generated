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
    /// AVCaptureStillImageOutput is a concrete subclass of AVCaptureOutput that can be used to capture high-quality still images with accompanying metadata.
    ///
    ///
    /// Instances of AVCaptureStillImageOutput can be used to capture, on demand, high quality snapshots from a realtime capture source. Clients can request a still image for the current time using the captureStillImageAsynchronouslyFromConnection:completionHandler: method. Clients can also configure still image outputs to produce still images in specific image formats.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturestillimageoutput?language=objc)
    #[unsafe(super(AVCaptureOutput, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AVCaptureOutputBase")]
    #[deprecated = "Use AVCapturePhotoOutput instead."]
    pub struct AVCaptureStillImageOutput;
);

#[cfg(feature = "AVCaptureOutputBase")]
extern_conformance!(
    unsafe impl NSObjectProtocol for AVCaptureStillImageOutput {}
);

#[cfg(feature = "AVCaptureOutputBase")]
impl AVCaptureStillImageOutput {
    extern_methods!(
        #[deprecated = "Use AVCapturePhotoOutput instead."]
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        /// Specifies the options the receiver uses to encode still images before they are delivered.
        ///
        ///
        /// See AVVideoSettings.h for more information on how to construct an output settings dictionary.
        ///
        /// On iOS, the only currently supported keys are AVVideoCodecKey and kCVPixelBufferPixelFormatTypeKey. Use -availableImageDataCVPixelFormatTypes and -availableImageDataCodecTypes to determine what codec keys and pixel formats are supported. AVVideoQualityKey is supported on iOS 6.0 and later and may only be used when AVVideoCodecKey is set to AVVideoCodecTypeJPEG.
        #[deprecated = "Use AVCapturePhotoOutput instead."]
        #[unsafe(method(outputSettings))]
        #[unsafe(method_family = none)]
        pub unsafe fn outputSettings(&self) -> Retained<NSDictionary<NSString, AnyObject>>;

        /// Setter for [`outputSettings`][Self::outputSettings].
        #[deprecated = "Use AVCapturePhotoOutput instead."]
        #[unsafe(method(setOutputSettings:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setOutputSettings(&self, output_settings: &NSDictionary<NSString, AnyObject>);

        /// Indicates the supported image pixel formats that can be specified in outputSettings.
        ///
        ///
        /// The value of this property is an NSArray of NSNumbers that can be used as values for the kCVPixelBufferPixelFormatTypeKey in the receiver's outputSettings property. The first format in the returned list is the most efficient output format.
        #[deprecated = "Use AVCapturePhotoOutput instead."]
        #[unsafe(method(availableImageDataCVPixelFormatTypes))]
        #[unsafe(method_family = none)]
        pub unsafe fn availableImageDataCVPixelFormatTypes(&self) -> Retained<NSArray<NSNumber>>;

        #[cfg(feature = "AVVideoSettings")]
        /// Indicates the supported image codec formats that can be specified in outputSettings.
        ///
        ///
        /// The value of this property is an NSArray of AVVideoCodecTypes that can be used as values for the AVVideoCodecKey in the receiver's outputSettings property.
        #[deprecated = "Use AVCapturePhotoOutput instead."]
        #[unsafe(method(availableImageDataCodecTypes))]
        #[unsafe(method_family = none)]
        pub unsafe fn availableImageDataCodecTypes(&self) -> Retained<NSArray<AVVideoCodecType>>;

        /// Indicates whether the receiver supports still image stabilization.
        ///
        ///
        /// The receiver's automaticallyEnablesStillImageStabilizationWhenAvailable property can only be set if this property returns YES. Its value may change as the session's -sessionPreset or input device's -activeFormat changes.
        #[unsafe(method(isStillImageStabilizationSupported))]
        #[unsafe(method_family = none)]
        pub unsafe fn isStillImageStabilizationSupported(&self) -> bool;

        /// Indicates whether the receiver should automatically use still image stabilization when necessary.
        ///
        ///
        /// On a receiver where -isStillImageStabilizationSupported returns YES, image stabilization may be applied to reduce blur commonly found in low light photos. When stabilization is enabled, still image captures incur additional latency. The default value is YES when supported, NO otherwise. Setting this property throws an NSInvalidArgumentException if -isStillImageStabilizationSupported returns NO.
        #[unsafe(method(automaticallyEnablesStillImageStabilizationWhenAvailable))]
        #[unsafe(method_family = none)]
        pub unsafe fn automaticallyEnablesStillImageStabilizationWhenAvailable(&self) -> bool;

        /// Setter for [`automaticallyEnablesStillImageStabilizationWhenAvailable`][Self::automaticallyEnablesStillImageStabilizationWhenAvailable].
        #[unsafe(method(setAutomaticallyEnablesStillImageStabilizationWhenAvailable:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAutomaticallyEnablesStillImageStabilizationWhenAvailable(
            &self,
            automatically_enables_still_image_stabilization_when_available: bool,
        );

        /// Indicates whether still image stabilization is in use for the current capture.
        ///
        ///
        /// On a receiver where -isStillImageStabilizationSupported returns YES, and automaticallyEnablesStillImageStabilizationWhenAvailable is set to YES, this property may be key-value observed, or queried from inside your key-value observation callback for the
        /// "
        /// capturingStillImage" property, to find out if still image stabilization is being applied to the current capture.
        #[unsafe(method(isStillImageStabilizationActive))]
        #[unsafe(method_family = none)]
        pub unsafe fn isStillImageStabilizationActive(&self) -> bool;

        /// Indicates whether the receiver should emit still images at the highest resolution supported by its source AVCaptureDevice's activeFormat.
        ///
        ///
        /// By default, AVCaptureStillImageOutput emits images with the same dimensions as its source AVCaptureDevice's activeFormat.formatDescription. However, if you set this property to YES, the receiver emits still images at its source AVCaptureDevice's activeFormat.highResolutionStillImageDimensions. Note that if you enable video stabilization (see AVCaptureConnection's preferredVideoStabilizationMode) for any output, the high resolution still images emitted by AVCaptureStillImageOutput may be smaller by 10 or more percent.
        #[unsafe(method(isHighResolutionStillImageOutputEnabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn isHighResolutionStillImageOutputEnabled(&self) -> bool;

        /// Setter for [`isHighResolutionStillImageOutputEnabled`][Self::isHighResolutionStillImageOutputEnabled].
        #[unsafe(method(setHighResolutionStillImageOutputEnabled:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setHighResolutionStillImageOutputEnabled(
            &self,
            high_resolution_still_image_output_enabled: bool,
        );

        /// A boolean value that becomes true when a still image is being captured.
        ///
        ///
        /// The value of this property is a BOOL that becomes true when a still image is being captured, and false when no still image capture is underway. This property is key-value observable.
        #[unsafe(method(isCapturingStillImage))]
        #[unsafe(method_family = none)]
        pub unsafe fn isCapturingStillImage(&self) -> bool;

        #[cfg(all(
            feature = "AVCaptureSession",
            feature = "block2",
            feature = "objc2-core-media"
        ))]
        /// Initiates an asynchronous still image capture, returning the result to a completion handler.
        ///
        ///
        /// Parameter `connection`: The AVCaptureConnection object from which to capture the still image.
        ///
        /// Parameter `handler`: A block that will be called when the still image capture is complete. The block will be passed a CMSampleBuffer object containing the image data or an NSError object if an image could not be captured.
        ///
        ///
        /// This method will return immediately after it is invoked, later calling the provided completion handler block when image data is ready. If the request could not be completed, the error parameter will contain an NSError object describing the failure.
        ///
        /// Attachments to the image data sample buffer may contain metadata appropriate to the image data format. For instance, a sample buffer containing JPEG data may carry a kCGImagePropertyExifDictionary as an attachment. See
        /// <ImageIO
        /// /CGImageProperties.h> for a list of keys and value types.
        ///
        /// Clients should not assume that the completion handler will be called on a specific thread.
        ///
        /// Calls to captureStillImageAsynchronouslyFromConnection:completionHandler: are not synchronized with AVCaptureDevice manual control completion handlers. Setting a device manual control, waiting for its completion, then calling captureStillImageAsynchronouslyFromConnection:completionHandler: DOES NOT ensure that the still image returned reflects your manual control change. It may be from an earlier time. You can compare your manual control completion handler sync time to the returned still image's presentation time. You can retrieve the sample buffer's pts using CMSampleBufferGetPresentationTimestamp(). If the still image has an earlier timestamp, your manual control command does not apply to it.
        #[deprecated = "Use AVCapturePhotoOutput instead."]
        #[unsafe(method(captureStillImageAsynchronouslyFromConnection:completionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn captureStillImageAsynchronouslyFromConnection_completionHandler(
            &self,
            connection: &AVCaptureConnection,
            handler: &block2::DynBlock<dyn Fn(*mut CMSampleBuffer, *mut NSError)>,
        );

        #[cfg(feature = "objc2-core-media")]
        /// Converts the still image data and metadata attachments in a JPEG sample buffer to an NSData representation.
        ///
        ///
        /// Parameter `jpegSampleBuffer`: The sample buffer carrying JPEG image data, optionally with Exif metadata sample buffer attachments. This method throws an NSInvalidArgumentException if jpegSampleBuffer is NULL or not in the JPEG format.
        ///
        ///
        /// This method returns an NSData representation of a JPEG still image sample buffer, merging the image data and Exif metadata sample buffer attachments without recompressing the image. The returned NSData is suitable for writing to disk.
        #[deprecated = "Use AVCapturePhotoOutput instead."]
        #[unsafe(method(jpegStillImageNSDataRepresentation:))]
        #[unsafe(method_family = none)]
        pub unsafe fn jpegStillImageNSDataRepresentation(
            jpeg_sample_buffer: &CMSampleBuffer,
        ) -> Option<Retained<NSData>>;
    );
}

extern_class!(
    /// AVCaptureBracketedStillImageSettings is an abstract base class that defines an interface for settings pertaining to a bracketed capture.
    ///
    ///
    /// AVCaptureBracketedStillImageSettings may not be instantiated directly.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturebracketedstillimagesettings?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureBracketedStillImageSettings;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for AVCaptureBracketedStillImageSettings {}
);

impl AVCaptureBracketedStillImageSettings {
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
    /// AVCaptureManualExposureBracketedStillImageSettings is a concrete subclass of AVCaptureBracketedStillImageSettings to be used when bracketing exposure duration and ISO.
    ///
    ///
    /// An AVCaptureManualExposureBracketedStillImageSettings instance defines the exposure duration and ISO settings that should be applied to one image in a bracket. An array of settings objects is passed to -[AVCaptureStillImageOutput captureStillImageBracketAsynchronouslyFromConnection:withSettingsArray:completionHandler:]. Min and max duration and ISO values are queryable properties of the AVCaptureDevice supplying data to an AVCaptureStillImageOutput instance. If you wish to leave exposureDuration unchanged for this bracketed still image, you may pass the special value AVCaptureExposureDurationCurrent. To keep ISO unchanged, you may pass AVCaptureISOCurrent (see AVCaptureDevice.h).
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturemanualexposurebracketedstillimagesettings?language=objc)
    #[unsafe(super(AVCaptureBracketedStillImageSettings, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureManualExposureBracketedStillImageSettings;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for AVCaptureManualExposureBracketedStillImageSettings {}
);

impl AVCaptureManualExposureBracketedStillImageSettings {
    extern_methods!(
        #[cfg(feature = "objc2-core-media")]
        /// Creates an AVCaptureManualExposureBracketedStillImageSettings using the specified exposure duration and ISO.
        ///
        ///
        /// Parameter `duration`: The exposure duration in seconds. Pass AVCaptureExposureDurationCurrent to leave the duration unchanged for this bracketed image.
        ///
        /// Parameter `ISO`: The ISO. Pass AVCaptureISOCurrent to leave the ISO unchanged for this bracketed image.
        ///
        /// Returns: An initialized AVCaptureManualExposureBracketedStillImageSettings instance.
        #[unsafe(method(manualExposureSettingsWithExposureDuration:ISO:))]
        #[unsafe(method_family = none)]
        pub unsafe fn manualExposureSettingsWithExposureDuration_ISO(
            duration: CMTime,
            iso: c_float,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-media")]
        /// The exposure duration for the still image.
        #[unsafe(method(exposureDuration))]
        #[unsafe(method_family = none)]
        pub unsafe fn exposureDuration(&self) -> CMTime;

        /// The ISO for the still image.
        #[unsafe(method(ISO))]
        #[unsafe(method_family = none)]
        pub unsafe fn ISO(&self) -> c_float;
    );
}

/// Methods declared on superclass `AVCaptureBracketedStillImageSettings`.
impl AVCaptureManualExposureBracketedStillImageSettings {
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
    /// AVCaptureAutoExposureBracketedStillImageSettings is a concrete subclass of AVCaptureBracketedStillImageSettings to be used when bracketing exposure target bias.
    ///
    ///
    /// An AVCaptureAutoExposureBracketedStillImageSettings instance defines the exposure target bias setting that should be applied to one image in a bracket. An array of settings objects is passed to -[AVCaptureStillImageOutput captureStillImageBracketAsynchronouslyFromConnection:withSettingsArray:completionHandler:]. Min and max exposure target bias are queryable properties of the AVCaptureDevice supplying data to an AVCaptureStillImageOutput instance. If you wish to leave exposureTargetBias unchanged for this bracketed still image, you may pass the special value AVCaptureExposureTargetBiasCurrent (see AVCaptureDevice.h).
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptureautoexposurebracketedstillimagesettings?language=objc)
    #[unsafe(super(AVCaptureBracketedStillImageSettings, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureAutoExposureBracketedStillImageSettings;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for AVCaptureAutoExposureBracketedStillImageSettings {}
);

impl AVCaptureAutoExposureBracketedStillImageSettings {
    extern_methods!(
        /// Creates an AVCaptureAutoExposureBracketedStillImageSettings using the specified exposure target bias.
        ///
        ///
        /// Parameter `exposureTargetBias`: The exposure target bias. Pass AVCaptureExposureTargetBiasCurrent to leave the exposureTargetBias unchanged for this image.
        ///
        /// Returns: An initialized AVCaptureAutoExposureBracketedStillImageSettings instance.
        #[unsafe(method(autoExposureSettingsWithExposureTargetBias:))]
        #[unsafe(method_family = none)]
        pub unsafe fn autoExposureSettingsWithExposureTargetBias(
            exposure_target_bias: c_float,
        ) -> Retained<Self>;

        /// The exposure bias for the auto exposure bracketed settings
        #[unsafe(method(exposureTargetBias))]
        #[unsafe(method_family = none)]
        pub unsafe fn exposureTargetBias(&self) -> c_float;
    );
}

/// Methods declared on superclass `AVCaptureBracketedStillImageSettings`.
impl AVCaptureAutoExposureBracketedStillImageSettings {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

/// AVCaptureStillImageOutputBracketedCapture.
/// A category of methods for bracketed still image capture.
///
///
/// A "still image bracket" is a batch of images taken as quickly as possible in succession, optionally with different settings from picture to picture.
///
/// In a bracketed capture, AVCaptureDevice flashMode property is ignored (flash is forced off), as is AVCaptureStillImageOutput's automaticallyEnablesStillImageStabilizationWhenAvailable property (stabilization is forced off).
#[cfg(feature = "AVCaptureOutputBase")]
impl AVCaptureStillImageOutput {
    extern_methods!(
        /// Specifies the maximum number of still images that may be taken in a single bracket.
        ///
        ///
        /// AVCaptureStillImageOutput can only satisfy a limited number of image requests in a single bracket without exhausting system resources. The maximum number of still images that may be taken in a single bracket depends on the size of the images being captured, and consequently may vary with AVCaptureSession -sessionPreset and AVCaptureDevice -activeFormat. Some formats do not support bracketed capture and return a maxBracketedCaptureStillImageCount of 0. This read-only property is key-value observable. If you exceed -maxBracketedCaptureStillImageCount, then -captureStillImageBracketAsynchronouslyFromConnection:withSettingsArray:completionHandler: fails and the completionHandler is called [settings count] times with a NULL sample buffer and AVErrorMaximumStillImageCaptureRequestsExceeded.
        #[deprecated = "Use AVCapturePhotoOutput maxBracketedCapturePhotoCount instead."]
        #[unsafe(method(maxBracketedCaptureStillImageCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn maxBracketedCaptureStillImageCount(&self) -> NSUInteger;

        /// Indicates whether the receiver supports lens stabilization during bracketed captures.
        ///
        ///
        /// The receiver's lensStabilizationDuringBracketedCaptureEnabled property can only be set if this property returns YES. Its value may change as the session's -sessionPreset or input device's -activeFormat changes. This read-only property is key-value observable.
        #[deprecated = "Use AVCapturePhotoOutput lensStabilizationDuringBracketedCaptureSupported instead."]
        #[unsafe(method(isLensStabilizationDuringBracketedCaptureSupported))]
        #[unsafe(method_family = none)]
        pub unsafe fn isLensStabilizationDuringBracketedCaptureSupported(&self) -> bool;

        /// Indicates whether the receiver should use lens stabilization during bracketed captures.
        ///
        ///
        /// On a receiver where -isLensStabilizationDuringBracketedCaptureSupported returns YES, lens stabilization may be applied to the bracket to reduce blur commonly found in low light photos. When lens stabilization is enabled, bracketed still image captures incur additional latency. Lens stabilization is more effective with longer-exposure captures, and offers limited or no benefit for exposure durations shorter than 1/30 of a second. It is possible that during the bracket, the lens stabilization module may run out of correction range and therefore will not be active for every frame in the bracket. Each emitted CMSampleBuffer from the bracket will have an attachment of kCMSampleBufferAttachmentKey_StillImageLensStabilizationInfo indicating additional information about stabilization was applied to the buffer, if any. The default value of -isLensStabilizationDuringBracketedCaptureEnabled is NO. This value will be set to NO when -isLensStabilizationDuringBracketedCaptureSupported changes to NO. Setting this property throws an NSInvalidArgumentException if -isLensStabilizationDuringBracketedCaptureSupported returns NO. This property is key-value observable.
        #[deprecated = "Use AVCapturePhotoOutput with AVCapturePhotoBracketSettings instead."]
        #[unsafe(method(isLensStabilizationDuringBracketedCaptureEnabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn isLensStabilizationDuringBracketedCaptureEnabled(&self) -> bool;

        /// Setter for [`isLensStabilizationDuringBracketedCaptureEnabled`][Self::isLensStabilizationDuringBracketedCaptureEnabled].
        #[deprecated = "Use AVCapturePhotoOutput with AVCapturePhotoBracketSettings instead."]
        #[unsafe(method(setLensStabilizationDuringBracketedCaptureEnabled:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLensStabilizationDuringBracketedCaptureEnabled(
            &self,
            lens_stabilization_during_bracketed_capture_enabled: bool,
        );

        #[cfg(all(feature = "AVCaptureSession", feature = "block2"))]
        /// Allows the receiver to prepare resources in advance of capturing a still image bracket.
        ///
        ///
        /// Parameter `connection`: The connection through which the still image bracket should be captured.
        ///
        /// Parameter `settings`: An array of AVCaptureBracketedStillImageSettings objects. All must be of the same kind of AVCaptureBracketedStillImageSettings subclass, or an NSInvalidArgumentException is thrown.
        ///
        /// Parameter `handler`: A user provided block that will be called asynchronously once resources have successfully been allocated for the specified bracketed capture operation. If sufficient resources could not be allocated, the "prepared" parameter contains NO, and "error" parameter contains a non-nil error value. If [settings count] exceeds -maxBracketedCaptureStillImageCount, then AVErrorMaximumStillImageCaptureRequestsExceeded is returned. You should not assume that the completion handler will be called on a specific thread.
        ///
        ///
        /// -maxBracketedCaptureStillImageCount tells you the maximum number of images that may be taken in a single bracket given the current AVCaptureDevice/AVCaptureSession/AVCaptureStillImageOutput configuration. But before taking a still image bracket, additional resources may need to be allocated. By calling -prepareToCaptureStillImageBracketFromConnection:withSettingsArray:completionHandler: first, you are able to deterministically know when the receiver is ready to capture the bracket with the specified settings array.
        #[deprecated = "Use AVCapturePhotoOutput setPreparedPhotoSettingsArray:completionHandler: instead."]
        #[unsafe(method(prepareToCaptureStillImageBracketFromConnection:withSettingsArray:completionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn prepareToCaptureStillImageBracketFromConnection_withSettingsArray_completionHandler(
            &self,
            connection: &AVCaptureConnection,
            settings: &NSArray<AVCaptureBracketedStillImageSettings>,
            handler: &block2::DynBlock<dyn Fn(Bool, *mut NSError)>,
        );

        #[cfg(all(
            feature = "AVCaptureSession",
            feature = "block2",
            feature = "objc2-core-media"
        ))]
        /// Captures a still image bracket.
        ///
        ///
        /// Parameter `connection`: The connection through which the still image bracket should be captured.
        ///
        /// Parameter `settings`: An array of AVCaptureBracketedStillImageSettings objects. All must be of the same kind of AVCaptureBracketedStillImageSettings subclass, or an NSInvalidArgumentException is thrown.
        ///
        /// Parameter `handler`: A user provided block that will be called asynchronously as each still image in the bracket is captured. If the capture request is successful, the "sampleBuffer" parameter contains a valid CMSampleBuffer, the "stillImageSettings" parameter contains the settings object corresponding to this still image, and a nil "error" parameter. If the bracketed capture fails, sample buffer is NULL and error is non-nil. If [settings count] exceeds -maxBracketedCaptureStillImageCount, then AVErrorMaximumStillImageCaptureRequestsExceeded is returned. You should not assume that the completion handler will be called on a specific thread.
        ///
        ///
        /// If you have not called -prepareToCaptureStillImageBracketFromConnection:withSettingsArray:completionHandler: for this still image bracket request, the bracket may not be taken immediately, as the receiver may internally need to prepare resources.
        #[deprecated = "Use AVCapturePhotoOutput capturePhotoWithSettings:delegate: instead."]
        #[unsafe(method(captureStillImageBracketAsynchronouslyFromConnection:withSettingsArray:completionHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn captureStillImageBracketAsynchronouslyFromConnection_withSettingsArray_completionHandler(
            &self,
            connection: &AVCaptureConnection,
            settings: &NSArray<AVCaptureBracketedStillImageSettings>,
            handler: &block2::DynBlock<
                dyn Fn(
                    *mut CMSampleBuffer,
                    *mut AVCaptureBracketedStillImageSettings,
                    *mut NSError,
                ),
            >,
        );
    );
}
