//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// Indicates that the camera can capture a picture while it is connected, if the client sends a 'requestTakePicture' message to it.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/imagecapturecore/iccameradevicecantakepicture?language=objc)
    #[cfg(feature = "ICDevice")]
    pub static ICCameraDeviceCanTakePicture: &'static ICDeviceCapability;
}

extern "C" {
    /// Indicates that the camera can capture a picture while it is connected, if the user presses the shutter release on the camera.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/imagecapturecore/iccameradevicecantakepictureusingshutterreleaseoncamera?language=objc)
    #[cfg(feature = "ICDevice")]
    pub static ICCameraDeviceCanTakePictureUsingShutterReleaseOnCamera: &'static ICDeviceCapability;
}

extern "C" {
    /// Indicates that the camera can delete a file at a time while it is connected.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/imagecapturecore/iccameradevicecandeleteonefile?language=objc)
    #[cfg(feature = "ICDevice")]
    pub static ICCameraDeviceCanDeleteOneFile: &'static ICDeviceCapability;
}

extern "C" {
    /// Indicates that the camera can delete all files in a single operation while it is connected.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/imagecapturecore/iccameradevicecandeleteallfiles?language=objc)
    #[cfg(feature = "ICDevice")]
    pub static ICCameraDeviceCanDeleteAllFiles: &'static ICDeviceCapability;
}

extern "C" {
    /// Indicates that the camera can synchronize its date and time with that of the host computer.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/imagecapturecore/iccameradevicecansyncclock?language=objc)
    #[cfg(feature = "ICDevice")]
    pub static ICCameraDeviceCanSyncClock: &'static ICDeviceCapability;
}

extern "C" {
    /// Indicates that the host can upload files to the camera.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/imagecapturecore/iccameradevicecanreceivefile?language=objc)
    #[cfg(feature = "ICDevice")]
    pub static ICCameraDeviceCanReceiveFile: &'static ICDeviceCapability;
}

extern "C" {
    /// Indicates that the camera can accept PTP commands.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/imagecapturecore/iccameradevicecanacceptptpcommands?language=objc)
    #[cfg(feature = "ICDevice")]
    pub static ICCameraDeviceCanAcceptPTPCommands: &'static ICDeviceCapability;
}

extern "C" {
    /// Indicates that the camera supports HEIF transcoding, and can change the presentation of converted assets and original assets on the fly.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/imagecapturecore/iccameradevicesupportsheif?language=objc)
    #[cfg(feature = "ICDevice")]
    pub static ICCameraDeviceSupportsHEIF: &'static ICDeviceCapability;
}

/// [Apple's documentation](https://developer.apple.com/documentation/imagecapturecore/icuploadoption?language=objc)
// NS_TYPED_ENUM
pub type ICUploadOption = NSString;

/// [Apple's documentation](https://developer.apple.com/documentation/imagecapturecore/icdeleteresult?language=objc)
// NS_TYPED_ENUM
pub type ICDeleteResult = NSString;

extern "C" {
    /// The value for this key should be an NSArray
    /// <ICCameraItem
    /// *>*
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/imagecapturecore/icdeletesuccessful?language=objc)
    pub static ICDeleteSuccessful: &'static ICDeleteResult;
}

extern "C" {
    /// The value for this key should be an NSArray
    /// <ICCameraItem
    /// *>*
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/imagecapturecore/icdeletecanceled?language=objc)
    pub static ICDeleteCanceled: &'static ICDeleteResult;
}

extern "C" {
    /// The value for this key should be an NSArray
    /// <ICCameraItem
    /// *>*
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/imagecapturecore/icdeletefailed?language=objc)
    pub static ICDeleteFailed: &'static ICDeleteResult;
}

/// [Apple's documentation](https://developer.apple.com/documentation/imagecapturecore/icdeleteerror?language=objc)
// NS_TYPED_ENUM
pub type ICDeleteError = NSString;

extern "C" {
    /// The value for this key should be an ICCameraItem*
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/imagecapturecore/icdeleteerrorreadonly?language=objc)
    pub static ICDeleteErrorReadOnly: &'static ICDeleteError;
}

extern "C" {
    /// The value for this key should be an ICCameraItem*
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/imagecapturecore/icdeleteerrorfilemissing?language=objc)
    pub static ICDeleteErrorFileMissing: &'static ICDeleteError;
}

extern "C" {
    /// The value for this key should be an ICCameraItem*
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/imagecapturecore/icdeleteerrordevicemissing?language=objc)
    pub static ICDeleteErrorDeviceMissing: &'static ICDeleteError;
}

extern "C" {
    /// The value for this key should be an ICCameraItem*
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/imagecapturecore/icdeleteerrorcanceled?language=objc)
    pub static ICDeleteErrorCanceled: &'static ICDeleteError;
}

/// [Apple's documentation](https://developer.apple.com/documentation/imagecapturecore/icmediapresentation?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ICMediaPresentation(pub NSUInteger);
impl ICMediaPresentation {
    #[doc(alias = "ICMediaPresentationConvertedAssets")]
    pub const ConvertedAssets: Self = Self(1);
    #[doc(alias = "ICMediaPresentationOriginalAssets")]
    pub const OriginalAssets: Self = Self(2);
}

unsafe impl Encode for ICMediaPresentation {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for ICMediaPresentation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// ICCameraDevice is a concrete subclass of ICDevice class. ICDeviceBrowser creates instances of this class.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/imagecapturecore/iccameradevice?language=objc)
    #[unsafe(super(ICDevice, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ICDevice")]
    pub struct ICCameraDevice;
);

#[cfg(feature = "ICDevice")]
extern_conformance!(
    unsafe impl NSObjectProtocol for ICCameraDevice {}
);

#[cfg(feature = "ICDevice")]
impl ICCameraDevice {
    extern_methods!(
        /// ￼Indicates the percentage of content cataloging completed on the device. Its value ranges from 0 to 100.
        #[unsafe(method(contentCatalogPercentCompleted))]
        #[unsafe(method_family = none)]
        pub unsafe fn contentCatalogPercentCompleted(&self) -> NSUInteger;

        #[cfg(feature = "ICCameraItem")]
        /// ￼Contents of the camera. The structure of the elements in this array will reflect the folder structure of the storage reported by the camera. Each item in this array will correspond to a storage on the camera.
        #[unsafe(method(contents))]
        #[unsafe(method_family = none)]
        pub unsafe fn contents(&self) -> Option<Retained<NSArray<ICCameraItem>>>;

        #[cfg(feature = "ICCameraItem")]
        /// ￼The property mediaFiles represents all image, movie and audio files on the camera. These files are returned as a single array without regard to the folder hierarchy used to store these files on the camera.
        #[unsafe(method(mediaFiles))]
        #[unsafe(method_family = none)]
        pub unsafe fn mediaFiles(&self) -> Option<Retained<NSArray<ICCameraItem>>>;

        /// ￼Indicates whether the device can be 'soft' removed or disconnected.
        #[unsafe(method(isEjectable))]
        #[unsafe(method_family = none)]
        pub unsafe fn isEjectable(&self) -> bool;

        /// ￼Indicates whether the device is locked.  A locked device does not allow for deletion of any asset.
        #[unsafe(method(isLocked))]
        #[unsafe(method_family = none)]
        pub unsafe fn isLocked(&self) -> bool;

        /// Set to YES if the device is made by Apple and is pass-coded locked and connected to an untrusted host.
        #[unsafe(method(isAccessRestrictedAppleDevice))]
        #[unsafe(method_family = none)]
        pub unsafe fn isAccessRestrictedAppleDevice(&self) -> bool;

        /// Set to YES if the device is made by Apple and is pass-coded locked and connected to an untrusted host.
        #[unsafe(method(iCloudPhotosEnabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn iCloudPhotosEnabled(&self) -> bool;

        /// Filesystem mount point for a device with transportType of ICTransportTypeMassStorage. This will be NULL for all other devices.
        #[unsafe(method(mountPoint))]
        #[unsafe(method_family = none)]
        pub unsafe fn mountPoint(&self) -> Option<Retained<NSString>>;

        /// The media presentation describes the visible assets from a device that may contain multiple formats of each media asset.  The asigngments are of the type ICMediaPresentation enumeration.  This property is available only if the capability ICCameraDeviceSupportsHEIF is  present.
        ///
        ///
        /// A device supporting this capability can specify the
        /// following presentations:
        ///
        /// ICMediaPresentationConverted - The default behavior for applications retrieving images from a device supporting HEIF is to show only converted JPG from HEIF originals, and only H264 encoded video assets from HEVC.
        /// ICMediaPresentationOriginal - This presentation will show only original images from a device supporting HEIF and HEVC.  Burned in renders are always exported in JPG, as are burned in effects for MOV clips.
        #[unsafe(method(mediaPresentation))]
        #[unsafe(method_family = none)]
        pub unsafe fn mediaPresentation(&self) -> ICMediaPresentation;

        /// Setter for [`mediaPresentation`][Self::mediaPresentation].
        #[unsafe(method(setMediaPresentation:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMediaPresentation(&self, media_presentation: ICMediaPresentation);

        /// This method returns an array of files on the camera of type fileType.
        ///
        /// The fileType string is one of the following Uniform Type Identifier strings: kUTTypeImage, kUTTypeMovie, kUTTypeAudio, or kUTTypeData.
        #[unsafe(method(filesOfType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn filesOfType(
            &self,
            file_ut_type: &NSString,
        ) -> Option<Retained<NSArray<NSString>>>;

        #[cfg(all(feature = "ICCameraFile", feature = "ICCameraItem", feature = "libc"))]
        /// This method asynchronously reads data of a specified length from a specified offset.
        ///
        /// The readDelegate passed must not be nil. When this request is completed, the didReadDataSelector of the readDelegate object is called. The didReadDataSelector should have the same signature as: - (void)didReadData:(NSData*)data fromFile:(ICCameraFile*)file error:(NSError*)error contextInfo:(void*)contextInfo. The content of error returned should be examined to determine if the request completed successfully.
        #[unsafe(method(requestReadDataFromFile:atOffset:length:readDelegate:didReadDataSelector:contextInfo:))]
        #[unsafe(method_family = none)]
        pub unsafe fn requestReadDataFromFile_atOffset_length_readDelegate_didReadDataSelector_contextInfo(
            &self,
            file: &ICCameraFile,
            offset: libc::off_t,
            length: libc::off_t,
            read_delegate: &AnyObject,
            selector: Sel,
            context_info: *mut c_void,
        );

        #[cfg(all(feature = "ICCameraFile", feature = "ICCameraItem"))]
        /// Download a file from the camera. Please refer to the top of this header for information about the options.
        ///
        /// The downloadDelegate passed must not be nil. When this request is completed, the didDownloadSelector of the downloadDelegate object is called.The didDownloadSelector should have the same signature as: - (void)didDownloadFile:(ICCameraFile*)file error:(NSError*)error options:(NSDictionary*)options contextInfo:(void*)contextInfo. The content of error returned should be examined to determine if the request completed successfully.
        #[unsafe(method(requestDownloadFile:options:downloadDelegate:didDownloadSelector:contextInfo:))]
        #[unsafe(method_family = none)]
        pub unsafe fn requestDownloadFile_options_downloadDelegate_didDownloadSelector_contextInfo(
            &self,
            file: &ICCameraFile,
            options: &NSDictionary<ICDownloadOption, AnyObject>,
            download_delegate: &ProtocolObject<dyn ICCameraDeviceDownloadDelegate>,
            selector: Sel,
            context_info: *mut c_void,
        );

        /// Cancels the current download operation if supported
        #[unsafe(method(cancelDownload))]
        #[unsafe(method_family = none)]
        pub unsafe fn cancelDownload(&self);

        #[cfg(feature = "ICCameraItem")]
        /// Deletes files.
        #[unsafe(method(requestDeleteFiles:))]
        #[unsafe(method_family = none)]
        pub unsafe fn requestDeleteFiles(&self, files: &NSArray<ICCameraItem>);

        #[cfg(all(feature = "ICCameraItem", feature = "block2"))]
        /// Allows for deletion of an array of ICCameraItem objects, with the added ability to catch delete failures using the
        /// 'deleteFailed' block, and a completion block that will return the overall state of the request.
        ///
        /// The deleteFailed block will return:
        /// - NSDictionary
        /// <ICDeleteError
        /// , ICCameraItem*>*
        ///
        /// The completion block will return:
        /// — error:
        /// - nil if successful
        /// - NSError* with an code set to ICReturnDeleteFilesFailed if any file failed.
        ///
        /// - result: NSDictionary
        /// <ICDeleteResult
        /// , NSArray
        /// <ICCameraItem
        /// *>*>* result
        /// - ICDeleteSuccessful: NSArray
        /// <ICCameraItem
        /// *>* success
        /// - ICDeleteFailed: NSArray
        /// <ICCameraItem
        /// *>* failed
        #[unsafe(method(requestDeleteFiles:deleteFailed:completion:))]
        #[unsafe(method_family = none)]
        pub unsafe fn requestDeleteFiles_deleteFailed_completion(
            &self,
            files: &NSArray<ICCameraItem>,
            delete_failed: &block2::DynBlock<
                dyn Fn(NonNull<NSDictionary<ICDeleteError, ICCameraItem>>),
            >,
            completion: &block2::DynBlock<
                dyn Fn(NonNull<NSDictionary<ICDeleteResult, NSArray<ICCameraItem>>>, *mut NSError),
            >,
        ) -> Option<Retained<NSProgress>>;

        /// Cancels the current delete operation started by sending a 'requestDeleteFiles:'. This will only cancel operations in flight when a batch of files have been requested for deletion.
        #[unsafe(method(cancelDelete))]
        #[unsafe(method_family = none)]
        pub unsafe fn cancelDelete(&self);

        /// Synchronize camera's clock with the computer's clock. You should send this request only if the camera has the 'ICCameraDeviceCanSyncClock' capability.
        #[unsafe(method(requestSyncClock))]
        #[unsafe(method_family = none)]
        pub unsafe fn requestSyncClock(&self);

        /// Indicates the time offset, in seconds, between the camera's clock and the computer's clock￼. This value is positive if the camera's clock is ahead of the computer's clock. This property should be ignored if the camera's capabilities property does not contain ICCameraDeviceCanSyncClock.
        #[unsafe(method(timeOffset))]
        #[unsafe(method_family = none)]
        pub unsafe fn timeOffset(&self) -> NSTimeInterval;

        /// Indicates if the device has reported battery charge level￼.
        #[unsafe(method(batteryLevelAvailable))]
        #[unsafe(method_family = none)]
        pub unsafe fn batteryLevelAvailable(&self) -> bool;

        /// ￼Indicates the battery charge level. Its value ranges from 0 to 100.
        #[unsafe(method(batteryLevel))]
        #[unsafe(method_family = none)]
        pub unsafe fn batteryLevel(&self) -> NSUInteger;

        /// Upload a file at fileURL to the camera. The options dictionary is not used in this version.
        ///
        /// The uploadDelegate passed must not be nil. When this request is completed, the didUploadSelector of the uploadDelegate object is called. The didUploadSelector should have the same signature as: - (void)didUploadFile:(NSURL*)fileURL error:(NSError*)error contextInfo:(void*)contextInfo. The content of error returned should be examined to determine if the request completed successfully.
        #[deprecated = "Sandbox restrictions prohibit writing directly to device hardware"]
        #[unsafe(method(requestUploadFile:options:uploadDelegate:didUploadSelector:contextInfo:))]
        #[unsafe(method_family = none)]
        pub unsafe fn requestUploadFile_options_uploadDelegate_didUploadSelector_contextInfo(
            &self,
            file_url: &NSURL,
            options: &NSDictionary<ICUploadOption, AnyObject>,
            upload_delegate: &AnyObject,
            selector: Sel,
            context_info: *mut c_void,
        );

        /// This property is always set to YES when the device has the capability 'ICCameraDeviceCanTakePicture'
        ///
        /// requestEnableTethering/requestDisableTethering is no longer required to setup and destroy the standard
        /// take picture functionality of supported cameras.
        #[unsafe(method(tetheredCaptureEnabled))]
        #[unsafe(method_family = none)]
        pub unsafe fn tetheredCaptureEnabled(&self) -> bool;

        /// Capture a new image using the camera, the camera capabilities include 'ICCameraDeviceCanTakePicture'.
        #[unsafe(method(requestTakePicture))]
        #[unsafe(method_family = none)]
        pub unsafe fn requestTakePicture(&self);

        /// Send this message to enable tethered capture on the camera device if the camera has the 'ICCameraDeviceCanTakePicture' capability.
        #[deprecated = "Third party cameras that support the standard take picture command will have the capability enabled by default. This call will have no effect"]
        #[unsafe(method(requestEnableTethering))]
        #[unsafe(method_family = none)]
        pub unsafe fn requestEnableTethering(&self);

        /// Send this message to disable tethered capture on the camera device if the camera has the 'ICCameraDeviceCanTakePicture' capability and if your process has already sent a 'requestEnableTethering' to it.
        #[deprecated = "Third party cameras that support the standard take picture command will have the capability enabled by default. This call will have no effect"]
        #[unsafe(method(requestDisableTethering))]
        #[unsafe(method_family = none)]
        pub unsafe fn requestDisableTethering(&self);

        #[cfg(feature = "block2")]
        /// As an alternative to setting up an object to handle PTP event packets, a handler can be set.  The handler will always be called in place of the delegate if non-nil.  If the handler is not present, the delegate will be called if present. It is guaranteed only one of the methods will be called if both are implemented.
        #[unsafe(method(ptpEventHandler))]
        #[unsafe(method_family = none)]
        pub unsafe fn ptpEventHandler(&self) -> NonNull<block2::DynBlock<dyn Fn(NonNull<NSData>)>>;

        #[cfg(feature = "block2")]
        /// Setter for [`ptpEventHandler`][Self::ptpEventHandler].
        #[unsafe(method(setPtpEventHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPtpEventHandler(
            &self,
            ptp_event_handler: &block2::DynBlock<dyn Fn(NonNull<NSData>)>,
        );

        /// This method asynchronously sends a PTP command to a camera.
        ///
        /// This should be sent only if the 'capabilities' property contains 'ICCameraDeviceCanAcceptPTPCommands'. All PTP cameras have this capability. The response to this command will be delivered using didSendCommandSelector of sendCommandDelegate. The didSendCommandSelector should have the same signature as: - (void)didSendPTPCommand:(NSData*)command inData:(NSData*)data response:(NSData*)response error:(NSError*)error contextInfo:(void*)contextInfo. The content of error returned should be examined to determine if the request completed successfully.
        #[unsafe(method(requestSendPTPCommand:outData:sendCommandDelegate:didSendCommandSelector:contextInfo:))]
        #[unsafe(method_family = none)]
        pub unsafe fn requestSendPTPCommand_outData_sendCommandDelegate_didSendCommandSelector_contextInfo(
            &self,
            command: &NSData,
            data: Option<&NSData>,
            send_command_delegate: &AnyObject,
            selector: Sel,
            context_info: *mut c_void,
        );

        #[cfg(feature = "block2")]
        /// This method asynchronously sends a PTP command to a camera.
        ///
        /// The response, data, and any error message will be returned the block.
        #[unsafe(method(requestSendPTPCommand:outData:completion:))]
        #[unsafe(method_family = none)]
        pub unsafe fn requestSendPTPCommand_outData_completion(
            &self,
            ptp_command: &NSData,
            ptp_data: Option<&NSData>,
            completion: &block2::DynBlock<dyn Fn(NonNull<NSData>, NonNull<NSData>, *mut NSError)>,
        );
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "ICDevice")]
impl ICCameraDevice {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_protocol!(
    /// A delegate of ICCameraDevice must conform to ICCameraDeviceDelegate protocol.
    ///
    /// The ICCameraDeviceDelegate protocol inherits from the ICDeviceDelegate protocol.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/imagecapturecore/iccameradevicedelegate?language=objc)
    #[cfg(feature = "ICDevice")]
    pub unsafe trait ICCameraDeviceDelegate: ICDeviceDelegate {
        #[cfg(feature = "ICCameraItem")]
        /// This message is sent when objects are added to the device.
        ///
        /// The objects in items are instances ICCameraFile class.
        #[unsafe(method(cameraDevice:didAddItems:))]
        #[unsafe(method_family = none)]
        unsafe fn cameraDevice_didAddItems(
            &self,
            camera: &ICCameraDevice,
            items: &NSArray<ICCameraItem>,
        );

        #[cfg(feature = "ICCameraItem")]
        /// This message is sent when objects are removed from the device.
        ///
        /// The objects in items are instances ICCameraFile class.
        #[unsafe(method(cameraDevice:didRemoveItems:))]
        #[unsafe(method_family = none)]
        unsafe fn cameraDevice_didRemoveItems(
            &self,
            camera: &ICCameraDevice,
            items: &NSArray<ICCameraItem>,
        );

        #[cfg(all(feature = "ICCameraItem", feature = "objc2-core-graphics"))]
        /// This message is sent when the thumbnail requested for an item on a device is available.
        #[unsafe(method(cameraDevice:didReceiveThumbnail:forItem:error:))]
        #[unsafe(method_family = none)]
        unsafe fn cameraDevice_didReceiveThumbnail_forItem_error(
            &self,
            camera: &ICCameraDevice,
            thumbnail: Option<&CGImage>,
            item: &ICCameraItem,
            error: Option<&NSError>,
        );

        #[cfg(feature = "ICCameraItem")]
        /// This message is sent when the metadata requested for an item on a device is available.
        #[unsafe(method(cameraDevice:didReceiveMetadata:forItem:error:))]
        #[unsafe(method_family = none)]
        unsafe fn cameraDevice_didReceiveMetadata_forItem_error(
            &self,
            camera: &ICCameraDevice,
            metadata: Option<&NSDictionary>,
            item: &ICCameraItem,
            error: Option<&NSError>,
        );

        #[cfg(feature = "ICCameraItem")]
        /// This message is sent when an object or objects are renamed on the device.
        ///
        /// The objects may be instances of ICCameraFolder or ICCameraFile class.
        #[unsafe(method(cameraDevice:didRenameItems:))]
        #[unsafe(method_family = none)]
        unsafe fn cameraDevice_didRenameItems(
            &self,
            camera: &ICCameraDevice,
            items: &NSArray<ICCameraItem>,
        );

        /// This message is sent when a capability of a device changes.
        ///
        /// Detailed capabilitiy descriptions are provided at the top of this header file.
        #[unsafe(method(cameraDeviceDidChangeCapability:))]
        #[unsafe(method_family = none)]
        unsafe fn cameraDeviceDidChangeCapability(&self, camera: &ICCameraDevice);

        /// This message is sent to the delegate to convey a PTP event.
        #[unsafe(method(cameraDevice:didReceivePTPEvent:))]
        #[unsafe(method_family = none)]
        unsafe fn cameraDevice_didReceivePTPEvent(
            &self,
            camera: &ICCameraDevice,
            event_data: &NSData,
        );

        /// This message is sent when the camera device is done enumerating its content and is ready to receive requests.
        ///
        /// A session must be opened on the device in order to enumerate its content and make it ready to receive requests.
        #[unsafe(method(deviceDidBecomeReadyWithCompleteContentCatalog:))]
        #[unsafe(method_family = none)]
        unsafe fn deviceDidBecomeReadyWithCompleteContentCatalog(&self, device: &ICCameraDevice);

        /// This message is sent when an Apple device has been unlocked, paired to the host, and media is available.
        #[unsafe(method(cameraDeviceDidRemoveAccessRestriction:))]
        #[unsafe(method_family = none)]
        unsafe fn cameraDeviceDidRemoveAccessRestriction(&self, device: &ICDevice);

        /// This message is sent when an Apple device has been locked, and media is unavailable until the restriction
        /// has been removed.
        #[unsafe(method(cameraDeviceDidEnableAccessRestriction:))]
        #[unsafe(method_family = none)]
        unsafe fn cameraDeviceDidEnableAccessRestriction(&self, device: &ICDevice);

        #[cfg(feature = "ICCameraItem")]
        /// This message is sent when the camera device is about to execute queued requests for the thumbnail of a specific item.
        /// If the request is no longer wanted, eg: the item is no longer displayed on the screen, the client can return NO and abort sending
        /// a request down to the camera device, speeding up the exection queue.
        #[optional]
        #[unsafe(method(cameraDevice:shouldGetThumbnailOfItem:))]
        #[unsafe(method_family = none)]
        unsafe fn cameraDevice_shouldGetThumbnailOfItem(
            &self,
            camera_device: &ICCameraDevice,
            item: &ICCameraItem,
        ) -> bool;

        #[cfg(feature = "ICCameraItem")]
        /// This message is sent when the camera device is about to execute queued requests for the metadata of a specific item.
        /// If the request is no longer wanted, eg: the item is no longer displayed on the screen, the client can return NO and abort sending
        /// a request down to the camera device, speeding up the execution queue.
        #[optional]
        #[unsafe(method(cameraDevice:shouldGetMetadataOfItem:))]
        #[unsafe(method_family = none)]
        unsafe fn cameraDevice_shouldGetMetadataOfItem(
            &self,
            camera_device: &ICCameraDevice,
            item: &ICCameraItem,
        ) -> bool;

        /// This message is sent after the camera device completes a delete operation initiated by sending a 'requestDeleteFiles:' message to that device.
        ///
        /// This message is sent after the camera device completes a delete operation initiated by sending a 'requestDeleteFiles:' message to that device.
        #[optional]
        #[unsafe(method(cameraDevice:didCompleteDeleteFilesWithError:))]
        #[unsafe(method_family = none)]
        unsafe fn cameraDevice_didCompleteDeleteFilesWithError(
            &self,
            camera: &ICCameraDevice,
            error: Option<&NSError>,
        );

        #[cfg(feature = "ICCameraItem")]
        #[deprecated]
        #[optional]
        #[unsafe(method(cameraDevice:didAddItem:))]
        #[unsafe(method_family = none)]
        unsafe fn cameraDevice_didAddItem(&self, camera: &ICCameraDevice, item: &ICCameraItem);

        #[cfg(feature = "ICCameraItem")]
        #[deprecated]
        #[optional]
        #[unsafe(method(cameraDevice:didRemoveItem:))]
        #[unsafe(method_family = none)]
        unsafe fn cameraDevice_didRemoveItem(&self, camera: &ICCameraDevice, item: &ICCameraItem);

        #[cfg(feature = "ICCameraItem")]
        #[deprecated]
        #[optional]
        #[unsafe(method(cameraDevice:didReceiveThumbnailForItem:))]
        #[unsafe(method_family = none)]
        unsafe fn cameraDevice_didReceiveThumbnailForItem(
            &self,
            camera: &ICCameraDevice,
            item: &ICCameraItem,
        );

        #[cfg(feature = "ICCameraItem")]
        #[deprecated]
        #[optional]
        #[unsafe(method(cameraDevice:didReceiveMetadataForItem:))]
        #[unsafe(method_family = none)]
        unsafe fn cameraDevice_didReceiveMetadataForItem(
            &self,
            camera: &ICCameraDevice,
            item: &ICCameraItem,
        );
    }
);

extern_protocol!(
    /// The object passed in as 'downloadDelegate' in the 'requestDownloadFile:options:downloadDelegate:didDownloadSelector:contextInfo:' message must conform to ICCameraDeviceDownloadDelegate protocol.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/imagecapturecore/iccameradevicedownloaddelegate?language=objc)
    pub unsafe trait ICCameraDeviceDownloadDelegate: NSObjectProtocol {
        #[cfg(all(feature = "ICCameraFile", feature = "ICCameraItem"))]
        /// This message is sent to the delegate when the requested download operation is complete.
        #[optional]
        #[unsafe(method(didDownloadFile:error:options:contextInfo:))]
        #[unsafe(method_family = none)]
        unsafe fn didDownloadFile_error_options_contextInfo(
            &self,
            file: &ICCameraFile,
            error: Option<&NSError>,
            options: &NSDictionary<NSString, AnyObject>,
            context_info: *mut c_void,
        );

        #[cfg(all(feature = "ICCameraFile", feature = "ICCameraItem", feature = "libc"))]
        /// This message is sent to the delegate to provide status of the download operation.
        #[optional]
        #[unsafe(method(didReceiveDownloadProgressForFile:downloadedBytes:maxBytes:))]
        #[unsafe(method_family = none)]
        unsafe fn didReceiveDownloadProgressForFile_downloadedBytes_maxBytes(
            &self,
            file: &ICCameraFile,
            downloaded_bytes: libc::off_t,
            max_bytes: libc::off_t,
        );
    }
);
