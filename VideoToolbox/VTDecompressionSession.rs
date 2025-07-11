//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::cell::UnsafeCell;
use core::ffi::*;
use core::marker::{PhantomData, PhantomPinned};
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
#[cfg(feature = "objc2-core-video")]
use objc2_core_video::*;

use crate::*;

/// A reference to a Video Toolbox Decompression Session.
///
/// A decompression session supports the decompression of a sequence of video frames.
/// The session reference is a reference-counted CF object.
/// To create a decompression session, call VTDecompressionSessionCreate;
/// then you can optionally configure the session using VTSessionSetProperty;
/// then to decode frames, call VTDecompressionSessionDecodeFrame.
/// When you are done with the session, you should call VTDecompressionSessionInvalidate
/// to tear it down and CFRelease to release your object reference.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/videotoolbox/vtdecompressionsession?language=objc)
#[repr(C)]
pub struct VTDecompressionSession {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

cf_type!(
    unsafe impl VTDecompressionSession {}
);
#[cfg(feature = "objc2")]
cf_objc2_type!(
    unsafe impl RefEncode<"OpaqueVTDecompressionSession"> for VTDecompressionSession {}
);

/// Prototype for callback invoked when frame decompression is complete.
///
/// When you create a decompression session, you pass in a callback function to be called
/// for decompressed frames.  This function will not necessarily be called in display order.
///
/// Parameter `decompressionOutputRefCon`: The callback's reference value, copied from the decompressionOutputRefCon field of the
/// VTDecompressionOutputCallbackRecord structure.
///
/// Parameter `sourceFrameRefCon`: The frame's reference value, copied from the sourceFrameRefCon argument to
/// VTDecompressionSessionDecodeFrame.
///
/// Parameter `status`: noErr if decompression was successful; an error code if decompression was not successful.
///
/// Parameter `infoFlags`: Contains information about the decode operation.
/// The kVTDecodeInfo_Asynchronous bit may be set if the decode ran asynchronously.
/// The kVTDecodeInfo_FrameDropped bit may be set if the frame was dropped.
/// If the kVTDecodeInfo_ImageBufferModifiable bit is set, it is safe for the client to modify the imageBuffer.
///
/// Parameter `imageBuffer`: Contains the decompressed frame, if decompression was successful; otherwise, NULL.
/// IMPORTANT: The video decompressor may still be referencing the imageBuffer returned in this
/// callback if the kVTDecodeInfo_ImageBufferModifiable flag is not set.  Unless this flag
/// is set, it is not safe to modify the returned imageBuffer.
///
/// Parameter `presentationTimeStamp`: The frame's presentation timestamp, which will be determined by calling
/// CMSampleBufferGetOutputPresentationTimeStamp; kCMTimeInvalid if not available.
///
/// Parameter `presentationDuration`: The frame's presentation duration, which will be determined by calling
/// CMSampleBufferGetOutputDuration; kCMTimeInvalid if not available.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/videotoolbox/vtdecompressionoutputcallback?language=objc)
#[cfg(all(
    feature = "VTErrors",
    feature = "objc2-core-media",
    feature = "objc2-core-video"
))]
pub type VTDecompressionOutputCallback = Option<
    unsafe extern "C-unwind" fn(
        *mut c_void,
        *mut c_void,
        OSStatus,
        VTDecodeInfoFlags,
        *mut CVImageBuffer,
        CMTime,
        CMTime,
    ),
>;

/// [Apple's documentation](https://developer.apple.com/documentation/videotoolbox/vtdecompressionoutputcallbackrecord?language=objc)
#[cfg(all(
    feature = "VTErrors",
    feature = "objc2-core-media",
    feature = "objc2-core-video"
))]
#[repr(C, packed(4))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VTDecompressionOutputCallbackRecord {
    pub decompressionOutputCallback: VTDecompressionOutputCallback,
    pub decompressionOutputRefCon: *mut c_void,
}

#[cfg(all(
    feature = "VTErrors",
    feature = "objc2",
    feature = "objc2-core-media",
    feature = "objc2-core-video"
))]
unsafe impl Encode for VTDecompressionOutputCallbackRecord {
    const ENCODING: Encoding = Encoding::Struct(
        "VTDecompressionOutputCallbackRecord",
        &[
            <VTDecompressionOutputCallback>::ENCODING,
            <*mut c_void>::ENCODING,
        ],
    );
}

#[cfg(all(
    feature = "VTErrors",
    feature = "objc2",
    feature = "objc2-core-media",
    feature = "objc2-core-video"
))]
unsafe impl RefEncode for VTDecompressionOutputCallbackRecord {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

impl VTDecompressionSession {
    /// Creates a session for decompressing video frames.
    ///
    /// Decompressed frames will be emitted through calls to outputCallback.
    ///
    /// Parameter `allocator`: An allocator for the session.  Pass NULL to use the default allocator.
    ///
    /// Parameter `videoFormatDescription`: Describes the source video frames.
    ///
    /// Parameter `videoDecoderSpecification`: Specifies a particular video decoder that must be used.
    /// Pass NULL to let the video toolbox choose a decoder.
    ///
    /// Parameter `destinationImageBufferAttributes`: Describes requirements for emitted pixel buffers.
    /// Pass NULL to set no requirements.
    ///
    /// Parameter `outputCallback`: The callback to be called with decompressed frames.
    /// Pass NULL if and only if you will be calling VTDecompressionSessionDecodeFrameWithOutputHandler for decoding frames.
    ///
    /// Parameter `decompressionSessionOut`: Points to a variable to receive the new decompression session.
    #[doc(alias = "VTDecompressionSessionCreate")]
    #[cfg(all(
        feature = "VTErrors",
        feature = "objc2-core-media",
        feature = "objc2-core-video"
    ))]
    #[inline]
    pub unsafe fn create(
        allocator: Option<&CFAllocator>,
        video_format_description: &CMVideoFormatDescription,
        video_decoder_specification: Option<&CFDictionary>,
        destination_image_buffer_attributes: Option<&CFDictionary>,
        output_callback: *const VTDecompressionOutputCallbackRecord,
        decompression_session_out: NonNull<*mut VTDecompressionSession>,
    ) -> OSStatus {
        extern "C-unwind" {
            fn VTDecompressionSessionCreate(
                allocator: Option<&CFAllocator>,
                video_format_description: &CMVideoFormatDescription,
                video_decoder_specification: Option<&CFDictionary>,
                destination_image_buffer_attributes: Option<&CFDictionary>,
                output_callback: *const VTDecompressionOutputCallbackRecord,
                decompression_session_out: NonNull<*mut VTDecompressionSession>,
            ) -> OSStatus;
        }
        unsafe {
            VTDecompressionSessionCreate(
                allocator,
                video_format_description,
                video_decoder_specification,
                destination_image_buffer_attributes,
                output_callback,
                decompression_session_out,
            )
        }
    }

    /// Tears down a decompression session.
    ///
    /// When you are done with a decompression session you created, call VTDecompressionSessionInvalidate
    /// to tear it down and then CFRelease to release your object reference.
    /// When a decompression session's retain count reaches zero, it is automatically invalidated, but
    /// since sessions may be retained by multiple parties, it can be hard to predict when this will happen.
    /// Calling VTDecompressionSessionInvalidate ensures a deterministic, orderly teardown.
    #[doc(alias = "VTDecompressionSessionInvalidate")]
    #[inline]
    pub unsafe fn invalidate(self: &VTDecompressionSession) {
        extern "C-unwind" {
            fn VTDecompressionSessionInvalidate(session: &VTDecompressionSession);
        }
        unsafe { VTDecompressionSessionInvalidate(self) }
    }
}

unsafe impl ConcreteType for VTDecompressionSession {
    /// Returns the CFTypeID for decompression sessions.
    #[doc(alias = "VTDecompressionSessionGetTypeID")]
    #[inline]
    fn type_id() -> CFTypeID {
        extern "C-unwind" {
            fn VTDecompressionSessionGetTypeID() -> CFTypeID;
        }
        unsafe { VTDecompressionSessionGetTypeID() }
    }
}

impl VTDecompressionSession {
    /// Decompresses a video frame.
    ///
    /// If an error is returned from this function, there will be no callback.  Otherwise
    /// the callback provided during VTDecompressionSessionCreate will be called.
    ///
    /// Parameter `session`: The decompression session.
    ///
    /// Parameter `sampleBuffer`: A CMSampleBuffer containing one or more video frames.
    ///
    /// Parameter `decodeFlags`: A bitfield of directives to the decompression session and decoder.
    /// The kVTDecodeFrame_EnableAsynchronousDecompression bit indicates whether the video decoder
    /// may decompress the frame asynchronously.
    /// The kVTDecodeFrame_EnableTemporalProcessing bit indicates whether the decoder may delay calls to the output callback
    /// so as to enable processing in temporal (display) order.
    /// If both flags are clear, the decompression shall complete and your output callback function will be called
    /// before VTDecompressionSessionDecodeFrame returns.
    /// If either flag is set, VTDecompressionSessionDecodeFrame may return before the output callback function is called.
    ///
    /// Parameter `sourceFrameRefCon`: Your reference value for the frame.
    /// Note that if sampleBuffer contains multiple frames, the output callback function will be called
    /// multiple times with this sourceFrameRefCon.
    ///
    /// Parameter `infoFlagsOut`: Points to a VTDecodeInfoFlags to receive information about the decode operation.
    /// The kVTDecodeInfo_Asynchronous bit may be set if the decode is (or was) running
    /// asynchronously.
    /// The kVTDecodeInfo_FrameDropped bit may be set if the frame was dropped (synchronously).
    /// Pass NULL if you do not want to receive this information.
    #[doc(alias = "VTDecompressionSessionDecodeFrame")]
    #[cfg(all(feature = "VTErrors", feature = "objc2-core-media"))]
    #[inline]
    pub unsafe fn decode_frame(
        self: &VTDecompressionSession,
        sample_buffer: &CMSampleBuffer,
        decode_flags: VTDecodeFrameFlags,
        source_frame_ref_con: *mut c_void,
        info_flags_out: *mut VTDecodeInfoFlags,
    ) -> OSStatus {
        extern "C-unwind" {
            fn VTDecompressionSessionDecodeFrame(
                session: &VTDecompressionSession,
                sample_buffer: &CMSampleBuffer,
                decode_flags: VTDecodeFrameFlags,
                source_frame_ref_con: *mut c_void,
                info_flags_out: *mut VTDecodeInfoFlags,
            ) -> OSStatus;
        }
        unsafe {
            VTDecompressionSessionDecodeFrame(
                self,
                sample_buffer,
                decode_flags,
                source_frame_ref_con,
                info_flags_out,
            )
        }
    }
}

/// Prototype for block invoked when frame decompression is complete.
///
/// When you decode a frame, you pass in a callback block to be called
/// for that decompressed frame.  This block will not necessarily be called in display order.
/// If the VTDecompressionSessionDecodeFrameWithOutputHandler call returns an error, the block
/// will not be called.
///
/// Parameter `status`: noErr if decompression was successful; an error code if decompression was not successful.
///
/// Parameter `infoFlags`: Contains information about the decode operation.
/// The kVTDecodeInfo_Asynchronous bit may be set if the decode ran asynchronously.
/// The kVTDecodeInfo_FrameDropped bit may be set if the frame was dropped.
/// If the kVTDecodeInfo_ImageBufferModifiable bit is set, it is safe for the client to modify the imageBuffer.
///
/// Parameter `imageBuffer`: Contains the decompressed frame, if decompression was successful; otherwise, NULL.
/// IMPORTANT: The video decompressor may still be referencing the imageBuffer returned in this
/// callback if the kVTDecodeInfo_ImageBufferModifiable flag is not set.  Unless this flag
/// is set, it is not safe to modify the returned imageBuffer.
///
/// Parameter `presentationTimeStamp`: The frame's presentation timestamp; kCMTimeInvalid if not available.
///
/// Parameter `presentationDuration`: The frame's presentation duration; kCMTimeInvalid if not available.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/videotoolbox/vtdecompressionoutputhandler?language=objc)
#[cfg(all(
    feature = "VTErrors",
    feature = "block2",
    feature = "objc2-core-media",
    feature = "objc2-core-video"
))]
pub type VTDecompressionOutputHandler =
    *mut block2::DynBlock<dyn Fn(OSStatus, VTDecodeInfoFlags, *mut CVImageBuffer, CMTime, CMTime)>;

impl VTDecompressionSession {
    /// Decompresses a video frame.
    ///
    /// Cannot be called with a session created with a VTDecompressionOutputCallbackRecord.
    /// If the VTDecompressionSessionDecodeFrameWithOutputHandler call returns an error, the block
    /// will not be called.
    ///
    /// Parameter `session`: The decompression session.
    ///
    /// Parameter `sampleBuffer`: A CMSampleBuffer containing one or more video frames.
    ///
    /// Parameter `decodeFlags`: A bitfield of directives to the decompression session and decoder.
    /// The kVTDecodeFrame_EnableAsynchronousDecompression bit indicates whether the video decoder
    /// may decompress the frame asynchronously.
    /// The kVTDecodeFrame_EnableTemporalProcessing bit indicates whether the decoder may delay calls to the output callback
    /// so as to enable processing in temporal (display) order.
    /// If both flags are clear, the decompression shall complete and your output callback function will be called
    /// before VTDecompressionSessionDecodeFrame returns.
    /// If either flag is set, VTDecompressionSessionDecodeFrame may return before the output callback function is called.
    ///
    /// Parameter `infoFlagsOut`: Points to a VTDecodeInfoFlags to receive information about the decode operation.
    /// The kVTDecodeInfo_Asynchronous bit may be set if the decode is (or was) running
    /// asynchronously.
    /// The kVTDecodeInfo_FrameDropped bit may be set if the frame was dropped (synchronously).
    /// Pass NULL if you do not want to receive this information.
    ///
    /// Parameter `outputHandler`: The block to be called when decoding the frame is completed.  If the VTDecompressionSessionDecodeFrameWithOutputHandler
    /// call returns an error, the block will not be called.
    #[doc(alias = "VTDecompressionSessionDecodeFrameWithOutputHandler")]
    #[cfg(all(
        feature = "VTErrors",
        feature = "block2",
        feature = "objc2-core-media",
        feature = "objc2-core-video"
    ))]
    #[inline]
    pub unsafe fn decode_frame_with_output_handler(
        self: &VTDecompressionSession,
        sample_buffer: &CMSampleBuffer,
        decode_flags: VTDecodeFrameFlags,
        info_flags_out: *mut VTDecodeInfoFlags,
        output_handler: VTDecompressionOutputHandler,
    ) -> OSStatus {
        extern "C-unwind" {
            fn VTDecompressionSessionDecodeFrameWithOutputHandler(
                session: &VTDecompressionSession,
                sample_buffer: &CMSampleBuffer,
                decode_flags: VTDecodeFrameFlags,
                info_flags_out: *mut VTDecodeInfoFlags,
                output_handler: VTDecompressionOutputHandler,
            ) -> OSStatus;
        }
        unsafe {
            VTDecompressionSessionDecodeFrameWithOutputHandler(
                self,
                sample_buffer,
                decode_flags,
                info_flags_out,
                output_handler,
            )
        }
    }

    /// Directs the decompression session to emit all delayed frames.
    ///
    /// By default, the decompression session may not delay frames indefinitely;
    /// frames may only be indefinitely delayed if the client opts in via
    /// kVTDecodeFrame_EnableTemporalProcessing.
    /// IMPORTANT NOTE: This function may return before all delayed frames are emitted.
    /// To wait for them, call VTDecompressionSessionWaitForAsynchronousFrames instead.
    #[doc(alias = "VTDecompressionSessionFinishDelayedFrames")]
    #[inline]
    pub unsafe fn finish_delayed_frames(self: &VTDecompressionSession) -> OSStatus {
        extern "C-unwind" {
            fn VTDecompressionSessionFinishDelayedFrames(
                session: &VTDecompressionSession,
            ) -> OSStatus;
        }
        unsafe { VTDecompressionSessionFinishDelayedFrames(self) }
    }

    /// Indicates whether the session can decode frames with the given format description.
    ///
    /// Some video decoders are able to accommodate minor changes in format without needing to be
    /// completely reset in a new session.  This function can be used to test whether a format change
    /// is sufficiently minor.
    #[doc(alias = "VTDecompressionSessionCanAcceptFormatDescription")]
    #[cfg(feature = "objc2-core-media")]
    #[inline]
    pub unsafe fn can_accept_format_description(
        self: &VTDecompressionSession,
        new_format_desc: &CMFormatDescription,
    ) -> bool {
        extern "C-unwind" {
            fn VTDecompressionSessionCanAcceptFormatDescription(
                session: &VTDecompressionSession,
                new_format_desc: &CMFormatDescription,
            ) -> Boolean;
        }
        let ret =
            unsafe { VTDecompressionSessionCanAcceptFormatDescription(self, new_format_desc) };
        ret != 0
    }

    /// Waits for any and all outstanding asynchronous and delayed frames to complete, then returns.
    ///
    /// This function automatically calls VTDecompressionSessionFinishDelayedFrames,
    /// so clients don't have to call both.
    #[doc(alias = "VTDecompressionSessionWaitForAsynchronousFrames")]
    #[inline]
    pub unsafe fn wait_for_asynchronous_frames(self: &VTDecompressionSession) -> OSStatus {
        extern "C-unwind" {
            fn VTDecompressionSessionWaitForAsynchronousFrames(
                session: &VTDecompressionSession,
            ) -> OSStatus;
        }
        unsafe { VTDecompressionSessionWaitForAsynchronousFrames(self) }
    }

    /// Copies a black pixel buffer from the decompression session.
    ///
    /// The pixel buffer is in the same format that the session is decompressing to.
    ///
    /// Parameter `session`: The decompression session.
    ///
    /// Parameter `pixelBufferOut`: Points to a variable to receive the copied pixel buffer.
    #[doc(alias = "VTDecompressionSessionCopyBlackPixelBuffer")]
    #[cfg(feature = "objc2-core-video")]
    #[inline]
    pub unsafe fn copy_black_pixel_buffer(
        self: &VTDecompressionSession,
        pixel_buffer_out: NonNull<*mut CVPixelBuffer>,
    ) -> OSStatus {
        extern "C-unwind" {
            fn VTDecompressionSessionCopyBlackPixelBuffer(
                session: &VTDecompressionSession,
                pixel_buffer_out: NonNull<*mut CVPixelBuffer>,
            ) -> OSStatus;
        }
        unsafe { VTDecompressionSessionCopyBlackPixelBuffer(self, pixel_buffer_out) }
    }
}

/// Indicates whether the current system supports hardware decode for a given codec
///
/// This routine reports whether the current system supports hardware decode.  Using
/// this information, clients can make informed decisions regarding remote assets to load,
/// favoring alternate encodings when hardware decode is not supported.
/// This call returning true does not guarantee that hardware decode resources will be
/// available at all times.
#[cfg(feature = "objc2-core-media")]
#[inline]
pub unsafe extern "C-unwind" fn VTIsHardwareDecodeSupported(codec_type: CMVideoCodecType) -> bool {
    extern "C-unwind" {
        fn VTIsHardwareDecodeSupported(codec_type: CMVideoCodecType) -> Boolean;
    }
    let ret = unsafe { VTIsHardwareDecodeSupported(codec_type) };
    ret != 0
}

/// Indicates whether the current system supports stereo MV-HEVC decode.
///
/// This call returning true does not guarantee that decode resources will be available at all times.
#[inline]
pub unsafe extern "C-unwind" fn VTIsStereoMVHEVCDecodeSupported() -> bool {
    extern "C-unwind" {
        fn VTIsStereoMVHEVCDecodeSupported() -> Boolean;
    }
    let ret = unsafe { VTIsStereoMVHEVCDecodeSupported() };
    ret != 0
}

/// Prototype for callback invoked when multi-image frame decompression is complete.
///
/// When you create a decompression session, you pass in a callback function to be called
/// for decompressed frames.  This function will not necessarily be called in display order.
///
/// Parameter `decompressionOutputMultiImageRefCon`: The callback's reference value, copied from the outputMultiImageRefcon passed to
/// VTDecompressionSessionSetMultiImageCallback.
///
/// Parameter `sourceFrameRefCon`: The frame's reference value, copied from the sourceFrameRefCon argument to
/// VTDecompressionSessionDecodeFrame.
///
/// Parameter `status`: noErr if decompression was successful; an error code if decompression was not successful.
///
/// Parameter `infoFlags`: Contains information about the decode operation.
/// The kVTDecodeInfo_Asynchronous bit may be set if the decode ran asynchronously.
/// The kVTDecodeInfo_FrameDropped bit may be set if the frame was dropped.
/// If the kVTDecodeInfo_ImageBufferModifiable bit is set, it is safe for the client to modify the imageBuffer.
///
/// Parameter `taggedBufferGroup`: Contains the decompressed frame's multiple images, if decompression was successful; otherwise, NULL.
/// IMPORTANT: The video decompressor may still be referencing the pixelBuffers returned in this
/// callback if the kVTDecodeInfo_ImageBufferModifiable flag is not set.  Unless this flag
/// is set, it is not safe to modify the returned pixelBuffers.
///
/// Parameter `presentationTimeStamp`: The frame's presentation timestamp, which will be determined by calling
/// CMSampleBufferGetOutputPresentationTimeStamp; kCMTimeInvalid if not available.
///
/// Parameter `presentationDuration`: The frame's presentation duration, which will be determined by calling
/// CMSampleBufferGetOutputDuration; kCMTimeInvalid if not available.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/videotoolbox/vtdecompressionoutputmultiimagecallback?language=objc)
#[cfg(all(feature = "VTErrors", feature = "objc2-core-media"))]
pub type VTDecompressionOutputMultiImageCallback = Option<
    unsafe extern "C-unwind" fn(
        *mut c_void,
        *mut c_void,
        OSStatus,
        VTDecodeInfoFlags,
        *mut CMTaggedBufferGroup,
        CMTime,
        CMTime,
    ),
>;

impl VTDecompressionSession {
    /// Provides a callback capable of receiving multiple images for individual DecodeFrame requests.
    ///
    /// The outputMultiImageCallback will be used when the video decoder outputs CMTaggedBufferGroups.
    /// When installed, outputMultiImageCallback will also be used when DecodeFrame operations fail and return a nonzero status.
    /// The original single-image callback will only be used in the case where the video decoder outputs a CVImageBuffer instead of a CMTaggedBufferGroup.
    /// Terminology note: in multi-image decompression, a single video sample (from one CMSampleBuffer) contains a single frame (with one PTS) that is decoded to produce multiple images.
    #[doc(alias = "VTDecompressionSessionSetMultiImageCallback")]
    #[cfg(all(feature = "VTErrors", feature = "objc2-core-media"))]
    #[inline]
    pub unsafe fn set_multi_image_callback(
        self: &VTDecompressionSession,
        output_multi_image_callback: VTDecompressionOutputMultiImageCallback,
        output_multi_image_refcon: *mut c_void,
    ) -> OSStatus {
        extern "C-unwind" {
            fn VTDecompressionSessionSetMultiImageCallback(
                decompression_session: &VTDecompressionSession,
                output_multi_image_callback: VTDecompressionOutputMultiImageCallback,
                output_multi_image_refcon: *mut c_void,
            ) -> OSStatus;
        }
        unsafe {
            VTDecompressionSessionSetMultiImageCallback(
                self,
                output_multi_image_callback,
                output_multi_image_refcon,
            )
        }
    }
}

/// Prototype for block invoked when frame decompression is complete.
///
/// When you decode a frame, you pass in a callback block to be called
/// for that decompressed frame.  This block will not necessarily be called in display order.
/// If the VTDecompressionSessionDecodeFrameWithOutputHandler call returns an error, the block
/// will not be called.
///
/// Parameter `status`: noErr if decompression was successful; an error code if decompression was not successful.
///
/// Parameter `infoFlags`: Contains information about the decode operation.
/// The kVTDecodeInfo_Asynchronous bit may be set if the decode ran asynchronously.
/// The kVTDecodeInfo_FrameDropped bit may be set if the frame was dropped.
/// If the kVTDecodeInfo_ImageBufferModifiable bit is set, it is safe for the client to modify the imageBuffer.
///
/// Parameter `imageBuffer`: Contains the decompressed frame, if decompression was successful and the CMSampleBuffer contained
/// a single image frame; otherwise, NULL.
/// IMPORTANT: The video decompressor may still be referencing the imageBuffer returned in this
/// callback if the kVTDecodeInfo_ImageBufferModifiable flag is not set.  Unless this flag
/// is set, it is not safe to modify the returned imageBuffer.
///
/// Parameter `taggedBufferGroup`: Contains the decompressed frame's multiple images, if decompression was successful and the CMSampleBuffer
/// contained a multi-image frame; otherwise, NULL.
/// IMPORTANT: The video decompressor may still be referencing the pixelBuffers returned in this
/// callback if the kVTDecodeInfo_ImageBufferModifiable flag is not set.  Unless this flag
/// is set, it is not safe to modify the returned pixelBuffers.
///
/// Parameter `presentationTimeStamp`: The frame's presentation timestamp; kCMTimeInvalid if not available.
///
/// Parameter `presentationDuration`: The frame's presentation duration; kCMTimeInvalid if not available.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/videotoolbox/vtdecompressionmultiimagecapableoutputhandler?language=objc)
#[cfg(all(
    feature = "VTErrors",
    feature = "block2",
    feature = "objc2-core-media",
    feature = "objc2-core-video"
))]
pub type VTDecompressionMultiImageCapableOutputHandler = *mut block2::DynBlock<
    dyn Fn(
        OSStatus,
        VTDecodeInfoFlags,
        *mut CVImageBuffer,
        *mut CMTaggedBufferGroup,
        CMTime,
        CMTime,
    ),
>;

impl VTDecompressionSession {
    /// Decompresses a video frame.
    ///
    /// Cannot be called with a session created with a VTDecompressionOutputCallbackRecord.
    /// If the VTDecompressionSessionDecodeFrameWithOutputHandler call returns an error, the block
    /// will not be called.
    ///
    /// Parameter `session`: The decompression session.
    ///
    /// Parameter `sampleBuffer`: A CMSampleBuffer containing one or more video frames.
    ///
    /// Parameter `decodeFlags`: A bitfield of directives to the decompression session and decoder.
    /// The kVTDecodeFrame_EnableAsynchronousDecompression bit indicates whether the video decoder
    /// may decompress the frame asynchronously.
    /// The kVTDecodeFrame_EnableTemporalProcessing bit indicates whether the decoder may delay calls to the output callback
    /// so as to enable processing in temporal (display) order.
    /// If both flags are clear, the decompression shall complete and your output callback function will be called
    /// before VTDecompressionSessionDecodeFrame returns.
    /// If either flag is set, VTDecompressionSessionDecodeFrame may return before the output callback function is called.
    ///
    /// Parameter `infoFlagsOut`: Points to a VTDecodeInfoFlags to receive information about the decode operation.
    /// The kVTDecodeInfo_Asynchronous bit may be set if the decode is (or was) running
    /// asynchronously.
    /// The kVTDecodeInfo_FrameDropped bit may be set if the frame was dropped (synchronously).
    /// Pass NULL if you do not want to receive this information.
    ///
    /// Parameter `multiImageCapableHandler`: The block to be called when decoding the frame is completed.  If the
    /// VTDecompressionSessionDecodeFrameWithMultiImageCapableOutputHandler call returns an error,
    /// the block will not be called.
    #[doc(alias = "VTDecompressionSessionDecodeFrameWithMultiImageCapableOutputHandler")]
    #[cfg(all(
        feature = "VTErrors",
        feature = "block2",
        feature = "objc2-core-media",
        feature = "objc2-core-video"
    ))]
    #[inline]
    pub unsafe fn decode_frame_with_multi_image_capable_output_handler(
        self: &VTDecompressionSession,
        sample_buffer: &CMSampleBuffer,
        decode_flags: VTDecodeFrameFlags,
        info_flags_out: *mut VTDecodeInfoFlags,
        multi_image_capable_output_handler: VTDecompressionMultiImageCapableOutputHandler,
    ) -> OSStatus {
        extern "C-unwind" {
            fn VTDecompressionSessionDecodeFrameWithMultiImageCapableOutputHandler(
                session: &VTDecompressionSession,
                sample_buffer: &CMSampleBuffer,
                decode_flags: VTDecodeFrameFlags,
                info_flags_out: *mut VTDecodeInfoFlags,
                multi_image_capable_output_handler: VTDecompressionMultiImageCapableOutputHandler,
            ) -> OSStatus;
        }
        unsafe {
            VTDecompressionSessionDecodeFrameWithMultiImageCapableOutputHandler(
                self,
                sample_buffer,
                decode_flags,
                info_flags_out,
                multi_image_capable_output_handler,
            )
        }
    }
}

extern "C-unwind" {
    #[cfg(all(
        feature = "VTErrors",
        feature = "objc2-core-media",
        feature = "objc2-core-video"
    ))]
    #[deprecated = "renamed to `VTDecompressionSession::create`"]
    pub fn VTDecompressionSessionCreate(
        allocator: Option<&CFAllocator>,
        video_format_description: &CMVideoFormatDescription,
        video_decoder_specification: Option<&CFDictionary>,
        destination_image_buffer_attributes: Option<&CFDictionary>,
        output_callback: *const VTDecompressionOutputCallbackRecord,
        decompression_session_out: NonNull<*mut VTDecompressionSession>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[deprecated = "renamed to `VTDecompressionSession::invalidate`"]
    pub fn VTDecompressionSessionInvalidate(session: &VTDecompressionSession);
}

extern "C-unwind" {
    #[cfg(all(feature = "VTErrors", feature = "objc2-core-media"))]
    #[deprecated = "renamed to `VTDecompressionSession::decode_frame`"]
    pub fn VTDecompressionSessionDecodeFrame(
        session: &VTDecompressionSession,
        sample_buffer: &CMSampleBuffer,
        decode_flags: VTDecodeFrameFlags,
        source_frame_ref_con: *mut c_void,
        info_flags_out: *mut VTDecodeInfoFlags,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "VTErrors",
        feature = "block2",
        feature = "objc2-core-media",
        feature = "objc2-core-video"
    ))]
    #[deprecated = "renamed to `VTDecompressionSession::decode_frame_with_output_handler`"]
    pub fn VTDecompressionSessionDecodeFrameWithOutputHandler(
        session: &VTDecompressionSession,
        sample_buffer: &CMSampleBuffer,
        decode_flags: VTDecodeFrameFlags,
        info_flags_out: *mut VTDecodeInfoFlags,
        output_handler: VTDecompressionOutputHandler,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[deprecated = "renamed to `VTDecompressionSession::finish_delayed_frames`"]
    pub fn VTDecompressionSessionFinishDelayedFrames(session: &VTDecompressionSession) -> OSStatus;
}

#[cfg(feature = "objc2-core-media")]
#[deprecated = "renamed to `VTDecompressionSession::can_accept_format_description`"]
#[inline]
pub unsafe extern "C-unwind" fn VTDecompressionSessionCanAcceptFormatDescription(
    session: &VTDecompressionSession,
    new_format_desc: &CMFormatDescription,
) -> bool {
    extern "C-unwind" {
        fn VTDecompressionSessionCanAcceptFormatDescription(
            session: &VTDecompressionSession,
            new_format_desc: &CMFormatDescription,
        ) -> Boolean;
    }
    let ret = unsafe { VTDecompressionSessionCanAcceptFormatDescription(session, new_format_desc) };
    ret != 0
}

extern "C-unwind" {
    #[deprecated = "renamed to `VTDecompressionSession::wait_for_asynchronous_frames`"]
    pub fn VTDecompressionSessionWaitForAsynchronousFrames(
        session: &VTDecompressionSession,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-video")]
    #[deprecated = "renamed to `VTDecompressionSession::copy_black_pixel_buffer`"]
    pub fn VTDecompressionSessionCopyBlackPixelBuffer(
        session: &VTDecompressionSession,
        pixel_buffer_out: NonNull<*mut CVPixelBuffer>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(all(feature = "VTErrors", feature = "objc2-core-media"))]
    #[deprecated = "renamed to `VTDecompressionSession::set_multi_image_callback`"]
    pub fn VTDecompressionSessionSetMultiImageCallback(
        decompression_session: &VTDecompressionSession,
        output_multi_image_callback: VTDecompressionOutputMultiImageCallback,
        output_multi_image_refcon: *mut c_void,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "VTErrors",
        feature = "block2",
        feature = "objc2-core-media",
        feature = "objc2-core-video"
    ))]
    #[deprecated = "renamed to `VTDecompressionSession::decode_frame_with_multi_image_capable_output_handler`"]
    pub fn VTDecompressionSessionDecodeFrameWithMultiImageCapableOutputHandler(
        session: &VTDecompressionSession,
        sample_buffer: &CMSampleBuffer,
        decode_flags: VTDecodeFrameFlags,
        info_flags_out: *mut VTDecodeInfoFlags,
        multi_image_capable_output_handler: VTDecompressionMultiImageCapableOutputHandler,
    ) -> OSStatus;
}
