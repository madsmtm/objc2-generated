//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

extern_class!(
    /// A CoreAnimation layer subclass for previewing the visual output of an AVCaptureSession.
    ///
    ///
    /// An AVCaptureVideoPreviewLayer instance is a subclass of CALayer and is therefore suitable for insertion in a layer hierarchy as part of a graphical interface. One creates an AVCaptureVideoPreviewLayer instance with the capture session to be previewed, using +layerWithSession: or -initWithSession:. Using the
    /// "
    /// videoGravity" property, one can influence how content is viewed relative to the layer bounds. On some hardware configurations, the orientation of the layer can be manipulated using @"orientation" and @"mirrored".
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturevideopreviewlayer?language=objc)
    #[unsafe(super(CALayer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-quartz-core")]
    #[cfg(not(target_os = "watchos"))]
    pub struct AVCaptureVideoPreviewLayer;
);

#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
unsafe impl CAMediaTiming for AVCaptureVideoPreviewLayer {}

#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
unsafe impl NSCoding for AVCaptureVideoPreviewLayer {}

#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
unsafe impl NSObjectProtocol for AVCaptureVideoPreviewLayer {}

#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
unsafe impl NSSecureCoding for AVCaptureVideoPreviewLayer {}

extern_methods!(
    #[cfg(feature = "objc2-quartz-core")]
    #[cfg(not(target_os = "watchos"))]
    unsafe impl AVCaptureVideoPreviewLayer {
        #[cfg(feature = "AVCaptureSession")]
        /// Creates an AVCaptureVideoPreviewLayer for previewing the visual output of the specified AVCaptureSession.
        ///
        ///
        /// Parameter `session`: The AVCaptureSession instance to be previewed.
        ///
        /// Returns: A newly initialized AVCaptureVideoPreviewLayer instance.
        #[method_id(@__retain_semantics Other layerWithSession:)]
        pub unsafe fn layerWithSession(session: &AVCaptureSession) -> Retained<Self>;

        #[cfg(feature = "AVCaptureSession")]
        /// Creates an AVCaptureVideoPreviewLayer for previewing the visual output of the specified AVCaptureSession.
        ///
        ///
        /// Parameter `session`: The AVCaptureSession instance to be previewed.
        ///
        /// Returns: A newly initialized AVCaptureVideoPreviewLayer instance.
        #[method_id(@__retain_semantics Init initWithSession:)]
        pub unsafe fn initWithSession(
            this: Allocated<Self>,
            session: &AVCaptureSession,
        ) -> Retained<Self>;

        #[cfg(feature = "AVCaptureSession")]
        /// Creates an AVCaptureVideoPreviewLayer for previewing the visual output of the specified AVCaptureSession, but creates no connections to any of the session's eligible video inputs. Only use this initializer if you intend to manually form a connection between a desired AVCaptureInputPort and the receiver using AVCaptureSession's -addConnection: method.
        ///
        ///
        /// Parameter `session`: The AVCaptureSession instance to be previewed.
        ///
        /// Returns: A newly initialized AVCaptureVideoPreviewLayer instance.
        #[method_id(@__retain_semantics Other layerWithSessionWithNoConnection:)]
        pub unsafe fn layerWithSessionWithNoConnection(
            session: &AVCaptureSession,
        ) -> Retained<Self>;

        #[cfg(feature = "AVCaptureSession")]
        /// Creates an AVCaptureVideoPreviewLayer for previewing the visual output of the specified AVCaptureSession, but creates no connections to any of the session's eligible video inputs. Only use this initializer if you intend to manually form a connection between a desired AVCaptureInputPort and the receiver using AVCaptureSession's -addConnection: method.
        ///
        ///
        /// Parameter `session`: The AVCaptureSession instance to be previewed.
        ///
        /// Returns: A newly initialized AVCaptureVideoPreviewLayer instance.
        #[method_id(@__retain_semantics Init initWithSessionWithNoConnection:)]
        pub unsafe fn initWithSessionWithNoConnection(
            this: Allocated<Self>,
            session: &AVCaptureSession,
        ) -> Retained<Self>;

        #[cfg(feature = "AVCaptureSession")]
        /// The AVCaptureSession instance being previewed by the receiver.
        ///
        ///
        /// The session is retained by the preview layer.
        #[method_id(@__retain_semantics Other session)]
        pub unsafe fn session(&self) -> Option<Retained<AVCaptureSession>>;

        #[cfg(feature = "AVCaptureSession")]
        /// Setter for [`session`][Self::session].
        #[method(setSession:)]
        pub unsafe fn setSession(&self, session: Option<&AVCaptureSession>);

        #[cfg(feature = "AVCaptureSession")]
        /// method setSessionWithNoConnection:
        ///
        /// Attaches the receiver to a given session without implicitly forming a connection to the first eligible video AVCaptureInputPort. Only use this setter if you intend to manually form a connection between a desired AVCaptureInputPort and the receiver using AVCaptureSession's -addConnection: method.
        ///
        ///
        /// The session is retained by the preview layer.
        #[method(setSessionWithNoConnection:)]
        pub unsafe fn setSessionWithNoConnection(&self, session: &AVCaptureSession);

        #[cfg(feature = "AVCaptureSession")]
        /// The AVCaptureConnection instance describing the AVCaptureInputPort to which the receiver is connected.
        ///
        ///
        /// When calling initWithSession: or setSession: with a valid AVCaptureSession instance, a connection is formed to the first eligible video AVCaptureInput. If the receiver is detached from a session, the connection property becomes nil.
        #[method_id(@__retain_semantics Other connection)]
        pub unsafe fn connection(&self) -> Option<Retained<AVCaptureConnection>>;

        #[cfg(feature = "AVAnimation")]
        /// A string defining how the video is displayed within an AVCaptureVideoPreviewLayer bounds rect.
        ///
        ///
        /// Options are AVLayerVideoGravityResize, AVLayerVideoGravityResizeAspect and AVLayerVideoGravityResizeAspectFill. AVLayerVideoGravityResizeAspect is default. See
        /// <AVFoundation
        /// /AVAnimation.h> for a description of these options.
        #[method_id(@__retain_semantics Other videoGravity)]
        pub unsafe fn videoGravity(&self) -> Retained<AVLayerVideoGravity>;

        #[cfg(feature = "AVAnimation")]
        /// Setter for [`videoGravity`][Self::videoGravity].
        #[method(setVideoGravity:)]
        pub unsafe fn setVideoGravity(&self, video_gravity: &AVLayerVideoGravity);

        /// A BOOL value indicating whether the receiver is currently rendering video frames from its source.
        ///
        ///
        /// An AVCaptureVideoPreviewLayer begins previewing when -[AVCaptureSession startRunning] is called. When associated with an AVCaptureMultiCamSession, all video preview layers are guaranteed to be previewing by the time the blocking call to -startRunning or -commitConfiguration returns. While a session is running, you may enable or disable a video preview layer's connection to re-start or stop the flow of video to the layer. Once you've set enabled to YES, you can observe this property changing from NO to YES and synchronize any UI to take place precisely when the video resumes rendering to the video preview layer.
        #[method(isPreviewing)]
        pub unsafe fn isPreviewing(&self) -> bool;

        #[cfg(feature = "objc2-core-foundation")]
        /// Converts a point in layer coordinates to a point of interest in the coordinate space of the capture device providing input to the layer.
        ///
        ///
        /// Parameter `pointInLayer`: A CGPoint in layer coordinates.
        ///
        /// Returns: A CGPoint in the coordinate space of the capture device providing input to the layer.
        ///
        ///
        /// AVCaptureDevice pointOfInterest is expressed as a CGPoint where {0,0} represents the top left of the picture area, and {1,1} represents the bottom right on an unrotated picture. This convenience method converts a point in the coordinate space of the receiver to a point of interest in the coordinate space of the AVCaptureDevice providing input to the receiver. The conversion takes frameSize and videoGravity into consideration.
        #[method(captureDevicePointOfInterestForPoint:)]
        pub unsafe fn captureDevicePointOfInterestForPoint(
            &self,
            point_in_layer: CGPoint,
        ) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        /// Converts a point of interest in the coordinate space of the capture device providing input to the layer to a point in layer coordinates.
        ///
        ///
        /// Parameter `captureDevicePointOfInterest`: A CGPoint in the coordinate space of the capture device providing input to the layer.
        ///
        /// Returns: A CGPoint in layer coordinates.
        ///
        ///
        /// AVCaptureDevice pointOfInterest is expressed as a CGPoint where {0,0} represents the top left of the picture area, and {1,1} represents the bottom right on an unrotated picture. This convenience method converts a point in the coordinate space of the AVCaptureDevice providing input to the coordinate space of the receiver. The conversion takes frame size and videoGravity into consideration.
        #[method(pointForCaptureDevicePointOfInterest:)]
        pub unsafe fn pointForCaptureDevicePointOfInterest(
            &self,
            capture_device_point_of_interest: CGPoint,
        ) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        /// Converts a rectangle in layer coordinates to a rectangle of interest in the coordinate space of an AVCaptureMetadataOutput whose capture device is providing input to the layer.
        ///
        ///
        /// Parameter `rectInLayerCoordinates`: A CGRect in layer coordinates.
        ///
        /// Returns: A CGRect in the coordinate space of the metadata output whose capture device is providing input to the layer.
        ///
        ///
        /// AVCaptureMetadataOutput rectOfInterest is expressed as a CGRect where {0,0} represents the top left of the picture area, and {1,1} represents the bottom right on an unrotated picture. This convenience method converts a rectangle in the coordinate space of the receiver to a rectangle of interest in the coordinate space of an AVCaptureMetadataOutput whose AVCaptureDevice is providing input to the receiver. The conversion takes frame size and videoGravity into consideration.
        #[method(metadataOutputRectOfInterestForRect:)]
        pub unsafe fn metadataOutputRectOfInterestForRect(
            &self,
            rect_in_layer_coordinates: CGRect,
        ) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        /// Converts a rectangle of interest in the coordinate space of an AVCaptureMetadataOutput whose capture device is providing input to the layer to a rectangle in layer coordinates.
        ///
        ///
        /// Parameter `rectInMetadataOutputCoordinates`: A CGRect in the coordinate space of the metadata output whose capture device is providing input to the layer.
        ///
        /// Returns: A CGRect in layer coordinates.
        ///
        ///
        /// AVCaptureMetadataOutput rectOfInterest is expressed as a CGRect where {0,0} represents the top left of the picture area, and {1,1} represents the bottom right on an unrotated picture. This convenience method converts a rectangle in the coordinate space of an AVCaptureMetadataOutput whose AVCaptureDevice is providing input to the coordinate space of the receiver. The conversion takes frame size and videoGravity into consideration.
        #[method(rectForMetadataOutputRectOfInterest:)]
        pub unsafe fn rectForMetadataOutputRectOfInterest(
            &self,
            rect_in_metadata_output_coordinates: CGRect,
        ) -> CGRect;

        #[cfg(feature = "AVMetadataObject")]
        /// Converts an AVMetadataObject's visual properties to layer coordinates.
        ///
        ///
        /// Parameter `metadataObject`: An AVMetadataObject originating from the same AVCaptureInput as the preview layer.
        ///
        /// Returns: An AVMetadataObject whose properties are in layer coordinates.
        ///
        ///
        /// AVMetadataObject bounds may be expressed as a rect where {0,0} represents the top left of the picture area, and {1,1} represents the bottom right on an unrotated picture. Face metadata objects likewise express yaw and roll angles with respect to an unrotated picture. -transformedMetadataObjectForMetadataObject: converts the visual properties in the coordinate space of the supplied AVMetadataObject to the coordinate space of the receiver. The conversion takes orientation, mirroring, layer bounds and videoGravity into consideration. If the provided metadata object originates from an input source other than the preview layer's, nil will be returned.
        #[method_id(@__retain_semantics Other transformedMetadataObjectForMetadataObject:)]
        pub unsafe fn transformedMetadataObjectForMetadataObject(
            &self,
            metadata_object: &AVMetadataObject,
        ) -> Option<Retained<AVMetadataObject>>;

        /// Specifies whether or not the preview layer supports orientation.
        ///
        ///
        /// Changes in orientation are not supported on all hardware configurations. An application should check the value of
        /// "
        /// orientationSupported" before attempting to manipulate the orientation of the receiver. This property is deprecated. Use AVCaptureConnection's -isVideoOrientationSupported instead.
        #[deprecated = "Use AVCaptureConnection's isVideoOrientationSupported instead."]
        #[method(isOrientationSupported)]
        pub unsafe fn isOrientationSupported(&self) -> bool;

        #[cfg(feature = "AVCaptureSession")]
        /// Specifies the orientation of the preview layer.
        ///
        ///
        /// AVCaptureVideoOrientation and its constants are defined in AVCaptureSession.h. The value of
        /// "
        /// orientationSupported" must be YES in order to set @"orientation". An exception will be raised if this requirement is ignored. This property is deprecated. Use AVCaptureConnection's -videoOrientation instead.
        #[deprecated = "Use AVCaptureConnection's videoOrientation instead."]
        #[method(orientation)]
        pub unsafe fn orientation(&self) -> AVCaptureVideoOrientation;

        #[cfg(feature = "AVCaptureSession")]
        /// Setter for [`orientation`][Self::orientation].
        #[deprecated = "Use AVCaptureConnection's videoOrientation instead."]
        #[method(setOrientation:)]
        pub unsafe fn setOrientation(&self, orientation: AVCaptureVideoOrientation);

        /// Specifies whether or not the preview layer supports mirroring.
        ///
        ///
        /// Mirroring is not supported on all hardware configurations. An application should check the value of
        /// "
        /// mirroringSupported" before attempting to manipulate mirroring on the receiver. This property is deprecated. Use AVCaptureConnection's -isVideoMirroringSupported instead.
        #[deprecated = "Use AVCaptureConnection's isVideoMirroringSupported instead."]
        #[method(isMirroringSupported)]
        pub unsafe fn isMirroringSupported(&self) -> bool;

        /// Specifies whether or not the value of
        /// "
        /// mirrored" can change based on configuration of the session.
        ///
        ///
        /// For some session configurations, preview will be mirrored by default. When the value of this property is YES, the value of
        /// "
        /// mirrored" may change depending on the configuration of the session, for example after switching to a different AVCaptureDeviceInput. The default value is YES. This property is deprecated. Use AVCaptureConnection's -automaticallyAdjustsVideoMirroring instead.
        #[deprecated = "Use AVCaptureConnection's automaticallyAdjustsVideoMirroring instead."]
        #[method(automaticallyAdjustsMirroring)]
        pub unsafe fn automaticallyAdjustsMirroring(&self) -> bool;

        /// Setter for [`automaticallyAdjustsMirroring`][Self::automaticallyAdjustsMirroring].
        #[deprecated = "Use AVCaptureConnection's automaticallyAdjustsVideoMirroring instead."]
        #[method(setAutomaticallyAdjustsMirroring:)]
        pub unsafe fn setAutomaticallyAdjustsMirroring(
            &self,
            automatically_adjusts_mirroring: bool,
        );

        /// Specifies whether or not the preview is flipped over a vertical axis.
        ///
        ///
        /// For most applications, it is unnecessary to manipulate preview mirroring manually if
        /// "
        /// automaticallyAdjustsMirroring" is set to YES. The value of @"automaticallyAdjustsMirroring" must be NO in order to set @"mirrored". The value of @"mirroringSupported" must be YES in order to set @"mirrored". An exception will be raised if the value of @"mirrored" is mutated without respecting these requirements. This property is deprecated. Use AVCaptureConnection's -videoMirrored instead.
        #[deprecated = "Use AVCaptureConnection's videoMirrored instead."]
        #[method(isMirrored)]
        pub unsafe fn isMirrored(&self) -> bool;

        /// Setter for [`isMirrored`][Self::isMirrored].
        #[deprecated = "Use AVCaptureConnection's videoMirrored instead."]
        #[method(setMirrored:)]
        pub unsafe fn setMirrored(&self, mirrored: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `CALayer`
    #[cfg(feature = "objc2-quartz-core")]
    #[cfg(not(target_os = "watchos"))]
    unsafe impl AVCaptureVideoPreviewLayer {
        /// Layer creation and initialization. *
        #[method_id(@__retain_semantics Other layer)]
        pub unsafe fn layer() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn initWithLayer(this: Allocated<Self>, layer: &AnyObject) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "objc2-quartz-core")]
    #[cfg(not(target_os = "watchos"))]
    unsafe impl AVCaptureVideoPreviewLayer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);