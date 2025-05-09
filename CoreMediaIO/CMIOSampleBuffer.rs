//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
#[cfg(feature = "objc2-core-video")]
use objc2_core_video::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmioinvalidsequencenumber?language=objc)
pub const kCMIOInvalidSequenceNumber: c_uint = !(0);
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebuffernodiscontinuities?language=objc)
pub const kCMIOSampleBufferNoDiscontinuities: c_uint = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferdiscontinuityflag_unknowndiscontinuity?language=objc)
pub const kCMIOSampleBufferDiscontinuityFlag_UnknownDiscontinuity: c_uint = 1 << 0;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferdiscontinuityflag_timecodediscontinuity?language=objc)
pub const kCMIOSampleBufferDiscontinuityFlag_TimecodeDiscontinuity: c_uint = 1 << 1;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferdiscontinuityflag_packeterror?language=objc)
pub const kCMIOSampleBufferDiscontinuityFlag_PacketError: c_uint = 1 << 2;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferdiscontinuityflag_streamdiscontinuity?language=objc)
pub const kCMIOSampleBufferDiscontinuityFlag_StreamDiscontinuity: c_uint = 1 << 3;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferdiscontinuityflag_malformeddata?language=objc)
pub const kCMIOSampleBufferDiscontinuityFlag_MalformedData: c_uint = 1 << 4;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferdiscontinuityflag_datawasflushed?language=objc)
pub const kCMIOSampleBufferDiscontinuityFlag_DataWasFlushed: c_uint = 1 << 5;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferdiscontinuityflag_datawasdropped?language=objc)
pub const kCMIOSampleBufferDiscontinuityFlag_DataWasDropped: c_uint = 1 << 6;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferdiscontinuityflag_bufferoverrun?language=objc)
pub const kCMIOSampleBufferDiscontinuityFlag_BufferOverrun: c_uint = 1 << 7;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferdiscontinuityflag_discontinuityindts?language=objc)
pub const kCMIOSampleBufferDiscontinuityFlag_DiscontinuityInDTS: c_uint = 1 << 8;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferdiscontinuityflag_relatedtodiscontinuity?language=objc)
pub const kCMIOSampleBufferDiscontinuityFlag_RelatedToDiscontinuity: c_uint = 1 << 9;
/// (as in another picture in the same MPEG-2
/// GOP) exhibits a discontinuity.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferdiscontinuityflag_clientsyncdiscontinuity?language=objc)
pub const kCMIOSampleBufferDiscontinuityFlag_ClientSyncDiscontinuity: c_uint = 1 << 10;
/// as a way of syncronizing the graph to
/// a known state.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferdiscontinuityflag_trickplay?language=objc)
pub const kCMIOSampleBufferDiscontinuityFlag_TrickPlay: c_uint = 1 << 11;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferdiscontinuityflag_nodatamarker?language=objc)
pub const kCMIOSampleBufferDiscontinuityFlag_NoDataMarker: c_uint = 1 << 12;
/// the graph if the source has no data (for example
/// an HDV camera running on empty tape).
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferdiscontinuityflag_dataformatchanged?language=objc)
pub const kCMIOSampleBufferDiscontinuityFlag_DataFormatChanged: c_uint = 1 << 13;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferdiscontinuityflag_timingreferencejumped?language=objc)
pub const kCMIOSampleBufferDiscontinuityFlag_TimingReferenceJumped: c_uint = 1 << 14;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferdiscontinuityflag_durationwasextended?language=objc)
pub const kCMIOSampleBufferDiscontinuityFlag_DurationWasExtended: c_uint = 1 << 15;
/// by increasing the duration of known good media;  this is a "soft"
/// discontinuity, much like kCMIOSampleBufferDiscontinuityFlag_TimecodeDiscontinuity,
/// meaning that the stream isn't necessarily broken, but clients might want to force
/// capture of all media.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferdiscontinuityflag_sleepwakecycle?language=objc)
pub const kCMIOSampleBufferDiscontinuityFlag_SleepWakeCycle: c_uint = 1 << 16;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferdiscontinuityflag_codecsettingschanged?language=objc)
pub const kCMIOSampleBufferDiscontinuityFlag_CodecSettingsChanged: c_uint = 1 << 17;

/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebuffernodataevent_unknown?language=objc)
pub const kCMIOSampleBufferNoDataEvent_Unknown: c_uint = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebuffernodataevent_nomedia?language=objc)
pub const kCMIOSampleBufferNoDataEvent_NoMedia: c_uint = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebuffernodataevent_devicedidnotsync?language=objc)
pub const kCMIOSampleBufferNoDataEvent_DeviceDidNotSync: c_uint = 2;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebuffernodataevent_deviceinwrongmode?language=objc)
pub const kCMIOSampleBufferNoDataEvent_DeviceInWrongMode: c_uint = 3;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebuffernodataevent_processingerror?language=objc)
pub const kCMIOSampleBufferNoDataEvent_ProcessingError: c_uint = 4;
/// [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebuffernodataevent_sleepwakecycle?language=objc)
pub const kCMIOSampleBufferNoDataEvent_SleepWakeCycle: c_uint = 5;

extern "C" {
    /// A CFNumber of kCFNumberSInt32Type. The discontinuity flags
    /// are used to denote that the given buffer represents a
    /// discontinuity in a stream of buffers.  Its various values
    /// are defined in CMIOTypes.h.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachmentkey_discontinuityflags?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachmentKey_DiscontinuityFlags: Option<&'static CFString>;
}

extern "C" {
    /// A CFNumber of kCFNumberSInt64Type. Provides a number that
    /// increments monotonically for every buffer of a given stream;
    /// it can be inquired upon and used to detect gaps in the
    /// stream (for example, a buffer was dropped somewhere will
    /// reveil itself by a gap in the sequence of sequence numbers).
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachmentkey_sequencenumber?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachmentKey_SequenceNumber: Option<&'static CFString>;
}

extern "C" {
    /// An AVS::HDV1PackData structure, as defined by AVC Video Services.
    /// Attached to video MPEG-2 video buffers that came from a transmit
    /// stream that had HDV-1 Pack data.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachmentkey_hdv1_packdata?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachmentKey_HDV1_PackData: Option<&'static CFString>;
}

extern "C" {
    /// An AVS::HDV2VideoFramePack structure, as defined by AVC Video
    /// Services.  Attached to video MPEG-2 video buffers that came from
    /// a transmit stream that had HDV-2 VAUX data.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachmentkey_hdv2_vaux?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachmentKey_HDV2_VAUX: Option<&'static CFString>;
}

extern "C" {
    /// A CFData that contains a CoreAudio AudioTimeStamp structure, as
    /// defined CoreAudioTypes.h.  Attached to buffers provided by
    /// CoreAudio devices.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachmentkey_caaudiotimestamp?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachmentKey_CAAudioTimeStamp: Option<&'static CFString>;
}

extern "C" {
    /// A CFData that contains CoreAudio SMPTETime structure, as defined
    /// CoreAudioTypes.h.  Attached to buffers that have an associated
    /// SMPTE time.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachmentkey_smptetime?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachmentKey_SMPTETime: Option<&'static CFString>;
}

extern "C" {
    /// A CFNumber of kCFNumberSInt32Type.  Attached to buffers that are
    /// associated with a SMPTE timecode that increments at a rate that
    /// is not the rate represented by the buffer.  For example, a buffer
    /// might contain 24P video that is associated with a 30FPS SMPTE
    /// timecode;  as the SMPTE timecodes are viewed as a stream, there
    /// will be gaps.  This attachment allows for an annotation to the
    /// buffer that will not include gaps.  Buffer clients interested in
    /// looking for gaps in  the SMPTE timecode can inquire about this
    /// property, and if it exists, check to see that it increments
    /// monotonically.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachmentkey_nativesmpteframecount?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachmentKey_NativeSMPTEFrameCount: Option<&'static CFString>;
}

extern "C" {
    /// A CFNumber of kCFNumberSInt32Type.  Attached to buffers containing
    /// an MPEG-2 transmit stream that was contains multiplexed video frames.
    /// It specifies how many video frames are represented by the buffer.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachmentkey_numberofvideoframesinbuffer?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachmentKey_NumberOfVideoFramesInBuffer:
        Option<&'static CFString>;
}

extern "C" {
    /// A CFNumber of kCFNumberSInt32Type. Attached to buffers containing
    /// an MPEG-2 video I-Frame that is being multiplexed with audio for
    /// output.  It specifies how many frames are contained in the GOP
    /// that is started by the I-Frame.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachmentkey_numberofvideoframesingop?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachmentKey_NumberOfVideoFramesInGOP: Option<&'static CFString>;
}

extern "C" {
    /// A CFDictionary that is a serialized CMTime. Attached to buffers
    /// of multiplexed data, and indicates a presentation timestamp for
    /// the muxed data that is based on the source data.  Typically used
    /// to drive the clock abstraction for an output device in order to
    /// provide a preview of the source data.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachmentkey_muxedsourcepresentationtimestamp?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachmentKey_MuxedSourcePresentationTimeStamp:
        Option<&'static CFString>;
}

extern "C" {
    /// A CFNumber of kCFNumberSInt64Type. Attached to buffers that are
    /// associated with a realtime source or destination that is related
    /// to the CPU's hosttime in nanoseconds.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachmentkey_hosttime?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachmentKey_HostTime: Option<&'static CFString>;
}

extern "C" {
    /// A CFBoolean. Attached to buffers (and having the value
    /// kCFBooleanTrue) if the contents of that buffer is identical
    /// to the previous buffer in its stream.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachmentkey_repeatedbuffercontents?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachmentKey_RepeatedBufferContents: Option<&'static CFString>;
}

extern "C" {
    /// A CMFormatDescription. Audio buffers traveling through a
    /// CMIO graph may be converted, mixed, or otherwise transformed.
    /// Downstream units may still desire to know the source format
    /// from which an audio buffer was derived.  Units dealing with
    /// audio data should propagate this attachment if it is present.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachmentkey_sourceaudioformatdescription?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachmentKey_SourceAudioFormatDescription:
        Option<&'static CFString>;
}

extern "C" {
    /// A CMIOPulldownCadenceInfo. Video buffers may come from a source
    /// that has been pulled down (for example, 24p buffers recorded at
    /// 30p).  These buffers can use this attachment to specify where
    /// this buffer falls in the cadence.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachmentkey_pulldowncadenceinfo?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachmentKey_PulldownCadenceInfo: Option<&'static CFString>;
}

extern "C" {
    /// A CMSampleBuffer. Video buffers with associated closed caption
    /// data may attach the data as a CMSampleBuffer.  This technique
    /// is used by the CMIO VDIG input unit if closed caption data
    /// is available for the current video frame.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachmentkey_closedcaptionsamplebuffer?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachmentKey_ClosedCaptionSampleBuffer: Option<&'static CFString>;
}

extern "C" {
    /// A CF obect.  Attached to buffers output from units that
    /// support kCMIOUnitProperty_ClientSequenceID.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachmentkey_clientsequenceid?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachmentKey_ClientSequenceID: Option<&'static CFString>;
}

extern "C" {
    /// A CFDictionary. Screen capture buffers will have this
    /// attachment so that clients can have some information
    /// as to the approximate state of the mouse and keyboard
    /// modifiers when the screen was captured.  The following
    /// keys will be present in the buffer:
    ///
    /// kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_CursorPositionX
    /// kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_CursorPositionY
    /// kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_MouseButtonState
    /// kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_CursorIsVisible
    /// kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_CursorFrameRect
    /// kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_CursorReference
    /// kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_CursorScale
    /// kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_KeyboardModifiers
    /// kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_KeyboardModifiersEvent
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachmentkey_mouseandkeyboardmodifiers?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachmentKey_MouseAndKeyboardModifiers: Option<&'static CFString>;
}

extern "C" {
    /// Used to look up a CFNumber from the CFDictionary specified by
    /// kCMIOSampleBufferAttachmentKey_MouseAndKeyboardModifiers.  It specifies
    /// the approximate X coordinate of the cursor hotspot when the screen was
    /// captured (in the cursor’s flipped coordinate system, origin is the top left of the display and not relative to the capture area).
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachment_mouseandkeyboardmodifierskey_cursorpositionx?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_CursorPositionX:
        Option<&'static CFString>;
}

extern "C" {
    /// Used to look up a CFNumber from the CFDictionary specified by
    /// kCMIOSampleBufferAttachmentKey_MouseAndKeyboardModifiers.  It specifies
    /// the approximate Y coordinate of the cursor hotspot when the screen was
    /// captured (in the cursor’s flipped coordinate system, origin is the top left of the display and not relative to the capture area).
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachment_mouseandkeyboardmodifierskey_cursorpositiony?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_CursorPositionY:
        Option<&'static CFString>;
}

extern "C" {
    /// Used to look up a CFNumber from the CFDictionary specified by
    /// kCMIOSampleBufferAttachmentKey_MouseAndKeyboardModifiers.  It specifies
    /// the approximate state of the mouse buttons when the screen was
    /// captured.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachment_mouseandkeyboardmodifierskey_mousebuttonstate?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_MouseButtonState:
        Option<&'static CFString>;
}

extern "C" {
    /// Used to look up a CFBoolean from the CFDictionary specified by
    /// kCMIOSampleBufferAttachmentKey_MouseAndKeyboardModifiers.  It specifies
    /// the visibility of the mouse cursor when the screen was
    /// captured.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachment_mouseandkeyboardmodifierskey_cursorisvisible?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_CursorIsVisible:
        Option<&'static CFString>;
}

extern "C" {
    /// Used to look up a CFDictionary representation of a CGRect from the CFDictionary specified by
    /// kCMIOSampleBufferAttachmentKey_MouseAndKeyboardModifiers.  It specifies
    /// the frame CGRect as dictionary of the cursor when the screen was captured relative to the capture area.
    /// The origin is at the lower left of the capture area.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachment_mouseandkeyboardmodifierskey_cursorframerect?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_CursorFrameRect:
        Option<&'static CFString>;
}

extern "C" {
    /// Used to look up a NSCursor reference from the CFDictionary specified by
    /// kCMIOSampleBufferAttachmentKey_MouseAndKeyboardModifiers.  It specifies
    /// the NSCursor when the screen was captured.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachment_mouseandkeyboardmodifierskey_cursorreference?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_CursorReference:
        Option<&'static CFString>;
}

extern "C" {
    /// Used to look up the cursor seed value from the CFDictionary specified by
    /// kCMIOSampleBufferAttachmentKey_MouseAndKeyboardModifiers.  It specifies
    /// the CFNumber of the seed value for the cursor referenced by
    /// kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_CursorReference;
    /// changes in this number reflect that the cursor has changed.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachment_mouseandkeyboardmodifierskey_cursorseed?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_CursorSeed:
        Option<&'static CFString>;
}

extern "C" {
    /// Used to look up a CFNumber from the CFDictionary specified by
    /// kCMIOSampleBufferAttachmentKey_MouseAndKeyboardModifiers.  It specifies
    /// the cursor scaling when the screen was captured.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachment_mouseandkeyboardmodifierskey_cursorscale?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_CursorScale:
        Option<&'static CFString>;
}

extern "C" {
    /// Used to look up a CFBoolean from the CFDictionary specified by
    /// kCMIOSampleBufferAttachmentKey_MouseAndKeyboardModifiers.  It specifies
    /// if the cursor is being drawn at the display level when the screen was
    /// captured (if TRUE, the CursorFrameRect may not represent accurately the frame CGRect of the cursor image being drawn).
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachment_mouseandkeyboardmodifierskey_cursorisdrawninframebuffer?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_CursorIsDrawnInFramebuffer:
        Option<&'static CFString>;
}

extern "C" {
    /// Used to look up a CFNumber from the CFDictionary specified by
    /// kCMIOSampleBufferAttachmentKey_MouseAndKeyboardModifiers.  It specifies
    /// the approximate state of the keyboard modifiers when the screen was
    /// captured.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachment_mouseandkeyboardmodifierskey_keyboardmodifiers?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_KeyboardModifiers:
        Option<&'static CFString>;
}

extern "C" {
    /// Used to look up a CFNumber from the CFDictionary specified by
    /// kCMIOSampleBufferAttachmentKey_MouseAndKeyboardModifiers.  It specifies
    /// the approximate state of the keyboard modifiers when the screen was
    /// captured as define in NSEvent.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachment_mouseandkeyboardmodifierskey_keyboardmodifiersevent?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_KeyboardModifiersEvent:
        Option<&'static CFString>;
}

extern "C" {
    /// A CFBoolean indicating that the underlying pixel buffer has been overlaid by a static image.
    /// If this attachement exists it will contain the value kCFBooleanTrue indicating a static image overlay. Otherwise the pixel buffer has not been overlaid.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachmentkey_pixelbufferoverlaidbystaticimage?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachmentKey_PixelBufferOverlaidByStaticImage:
        Option<&'static CFString>;
}

extern "C" {
    /// A CFNumber of kCFNumberSInt32Type. Attached to buffers representing
    /// that a device has stopped returning data.  The value of this attachment
    /// is the same as for the discontinuity flags.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmiosamplebufferattachmentkey_nodatamarker?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOSampleBufferAttachmentKey_NoDataMarker: Option<&'static CFString>;
}

extern "C" {
    /// A CVPixelBufferRef, as defined by CoreVideo.
    /// Attached to block buffers that wrap a Core Video Pixel Buffer.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coremediaio/kcmioblockbufferattachmentkey_cvpixelbufferreference?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCMIOBlockBufferAttachmentKey_CVPixelBufferReference: Option<&'static CFString>;
}

extern "C-unwind" {
    /// Creates a CoreMedia Sample Buffer that can be used in a CMIO Graph.
    ///
    /// While CoreMedia Sample Buffers are used in CMIO Graphs, the breadth
    /// of their flexibility is not supported.  For example, CoreMedia Sample
    /// Buffers may represent more than one frame of video data;  CMIO
    /// Graphs support only one frame of video per buffer.
    ///
    /// All parameters are copied; on return, the caller can release them,
    /// free them, reuse them or whatever.  On return, the caller owns the returned CMSampleBuffer, and
    /// must release it when done with it.
    ///
    /// Example of usage for in-display-order video frames:
    /// <ul>
    /// dataBuffer: contains 1 Motion JPEG frame
    /// <br>
    /// dataFormatDescription: describes Motion JPEG video
    /// <br>
    /// numSamples: 1
    /// <br>
    /// numSampleTimingEntries: 1
    /// <br>
    /// sampleTimingArray: {duration = 3003/90000, presentationTimeStamp = 0/90000, decodeTimeStamp = invalid }
    /// <br>
    /// numSampleSizeEntries = 1
    /// <br>
    /// sampleSizeArray: {size of the video frame}
    /// </ul>
    /// Example of usage for out-of-display-order video frames:
    /// <ul>
    /// dataBuffer: contains 1 HDV frame
    /// <br>
    /// dataFormatDescription: describes H.264 video
    /// <br>
    /// numSamples: 1
    /// <br>
    /// numSampleTimingEntries: 1
    /// <br>
    /// sampleTimingArray: {duration = 3003/90000, presentationTimeStamp = 9009/90000, decodeTimeStamp = 3003/90000}
    /// <br>
    /// numSampleSizeEntries = 1
    /// <br>
    /// sampleSizeArray: {size of the video frame}
    /// </ul>
    /// Example of usage for compressed audio:
    /// <ul>
    /// dataBuffer: contains 24 compressed AAC packets
    /// <br>
    /// dataFormatDescription: describes 44.1kHz AAC audio
    /// <br>
    /// numSamples: 24
    /// <br>
    /// numSampleTimingEntries: 1
    /// <br>
    /// sampleTimingArray: {duration = 1024/44100, presentationTimeStamp = 0/44100, decodeTimeStamp = invalid }
    /// <br>
    /// numSampleSizeEntries: 24
    /// <br>
    /// sampleSizeArray:
    /// <ul>
    /// {191, 183, 208, 213, 202, 206, 209, 206, 204, 192, 202, 277,
    /// <br>
    /// 282, 240, 209, 194, 193, 197, 196, 198, 168, 199, 171, 194}
    /// </ul>
    /// </ul>
    /// Example of usage for compressed audio:
    /// <ul>
    /// dataBuffer: contains 1 HDV audio packet
    /// <br>
    /// dataFormatDescription: describes 48kHz MPEG-1 Layer II audio
    /// <br>
    /// numSamples: 1
    /// <br>
    /// numSampleTimingEntries: 1
    /// <br>
    /// sampleTimingArray: {duration = 2160/90000, presentationTimeStamp = 0/90000, decodeTimeStamp = invalid }
    /// <br>
    /// numSampleSizeEntries = 1
    /// <br>
    /// sampleSizeArray: {1152}
    /// </ul>
    /// Example of usage for uncompressed interleaved audio:
    /// <ul>
    /// dataBuffer: contains 24000 uncompressed interleaved stereo frames, each containing 2 Float32s =
    /// <ul>
    /// {{L,R},
    /// <br>
    /// {L,R},
    /// <br>
    /// {L,R}, ...}
    /// </ul>
    /// <br>
    /// dataFormatDescription: describes two-channel 48kHz Float32 interleaved audio
    /// <br>
    /// numSamples: 24000
    /// <br>
    /// numSampleTimingEntries: 1
    /// <br>
    /// sampleTimingArray: {duration = 1/48000, presentationTimeStamp = 0/90000, decodeTimeStamp = invalid }
    /// <br>
    /// numSampleSizeEntries: 1
    /// <br>
    /// sampleSizeArray: {8}
    /// </ul>
    /// Example of usage for uncompressed non-interleaved audio:
    /// <ul>
    /// dataBuffer: contains 24000 uncompressed interleaved stereo frames, each containing 2 (non-contiguous) Float32s =
    /// <ul>
    /// {{L,L,L,L,L,...},
    /// <br>
    /// {R,R,R,R,R,...}}
    /// </ul>
    /// <br>
    /// dataFormatDescription: describes two-channel 48kHz Float32 non-interleaved audio
    /// <br>
    /// numSamples: 24000
    /// <br>
    /// numSampleTimingEntries: 1
    /// <br>
    /// sampleTimingArray: {duration = 1/48000, presentationTimeStamp = 0/90000, decodeTimeStamp = invalid }
    /// <br>
    /// numSampleSizeEntries: 0
    /// <br>
    /// sampleSizeArray: NULL (because the samples are not contiguous)
    /// </ul>
    ///
    ///
    /// Returns: Returns paramErr if there is an error in parameters, memFullErr if memory
    /// could not be allocated, and noErr for success.  In addition, errors
    /// returned by CMSampleBufferCreate() will be passed back.
    #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-core-media"))]
    pub fn CMIOSampleBufferCreate(
        allocator: Option<&CFAllocator>,
        data_buffer: Option<&CMBlockBuffer>,
        format_description: Option<&CMFormatDescription>,
        num_samples: u32,
        num_sample_timing_entries: u32,
        sample_timing_array: *const CMSampleTimingInfo,
        num_sample_size_entries: u32,
        sample_size_array: *const usize,
        sequence_number: u64,
        discontinuity_flags: u32,
        s_buf_out: *mut *mut CMSampleBuffer,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Creates a CMSampleBuffer that can be used in a CMIO Graph, that contains a CVImageBuffer
    /// instead of a CMBlockBuffer.
    ///
    /// This routine is a specialized version of CMIOSampleBufferCreate.
    /// See the description of that routine for background information.
    ///
    /// Unlike a CMBlockBuffer which can reference many samples, a CVImageBuffer is defined to
    /// reference only one sample;  therefore this routine has fewer parameters then
    /// CMIOSampleBufferCreate.
    ///
    /// Sample timing information, which is a vector for CMIOSampleBufferCreate,
    /// consists of only one value for this routine.
    ///
    /// The concept of sample size does not apply to CVImageBuffers.  As such, CMSampleBufferGetSampleSizeArray
    /// will return kCMSampleBufferError_BufferHasNoSampleSizes, and CMSampleBufferGetSampleSize
    /// will return 0.
    ///
    /// Because CVImageBuffers hold visual data, the format description provided is a
    /// CMVideoFormatDescription.  The format description must be consistent with formatting
    /// information attached to the CVImageBuffer. The width, height, and codecType must match
    /// (for CVPixelBuffers the codec type is given by CVPixelBufferGetPixelFormatType(pixelBuffer);
    /// for other CVImageBuffers, the codecType must be 0). The format description extensions must
    /// match the image buffer attachments for all the keys in the list returned by
    /// CMVideoFormatDescriptionGetExtensionKeysCommonWithImageBuffers (if absent in either they
    /// must be absent in both).
    #[cfg(all(
        feature = "objc2-core-foundation",
        feature = "objc2-core-media",
        feature = "objc2-core-video"
    ))]
    pub fn CMIOSampleBufferCreateForImageBuffer(
        allocator: Option<&CFAllocator>,
        image_buffer: Option<&CVImageBuffer>,
        format_description: Option<&CMVideoFormatDescription>,
        sample_timing: *const CMSampleTimingInfo,
        sequence_number: u64,
        discontinuity_flags: u32,
        s_buf_out: *mut *mut CMSampleBuffer,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Creates a CMSampleBuffer with no data and one buffer-level special marker attachment that
    /// denotes that no data is available from the device.
    ///
    /// It is often important for elements of a CMIO graph to know that a device has stopped
    /// sending data.  A special buffer can be placed in the stream of buffers processed by the
    /// Graph so that Units will synchronously get notified that the device has stopped sending
    /// data.
    ///
    ///
    /// Returns: Returns paramErr if there is an error in parameters, memFullErr if memory
    /// could not be allocated, and noErr for success.
    #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-core-media"))]
    pub fn CMIOSampleBufferCreateNoDataMarker(
        allocator: Option<&CFAllocator>,
        no_data_event: u32,
        format_description: Option<&CMFormatDescription>,
        sequence_number: u64,
        discontinuity_flags: u32,
        s_buf_out: *mut *mut CMSampleBuffer,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Sets the sequence number for a CoreMedia Sample Buffer that is being used in a CMIO Graph.
    ///
    /// Sequence numbers are a required attachment to CoreMedia Sample Buffers being used in a
    /// CMIO Graph.  They allow a unit to know the position of a buffer in a sequence of
    /// buffers, and to look for gaps where data may have been lost.  Normally, the sequence
    /// number starts at 0 and increases by one for each subsequent buffer.  As the buffer
    /// flows through a CMIO Graph, a Unit may have reason to reassign the sequence number
    /// to a new value.  NOTE:  in order to prevent memory corruption and other errors,
    /// this function should only be called if the caller is sure that it has the sole
    /// reference to the buffer;  if this cannot be guarenteed, then the caller should
    /// first create a copy of the buffer using CMSampleBufferCreateCopy().
    #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-core-media"))]
    pub fn CMIOSampleBufferSetSequenceNumber(
        allocator: Option<&CFAllocator>,
        sbuf: Option<&CMSampleBuffer>,
        sequence_number: u64,
    );
}

extern "C-unwind" {
    /// Returns the sequence number for a CoreMedia Sample Buffer that is being used in a CMIO Graph.
    ///
    /// Sequence numbers are a required attachment to CoreMedia Sample Buffers being used in a CMIO Graph.
    ///
    ///
    /// Returns: Returns kCMIOInvalidSequenceNumber if there is an error in parameters, or if no sequence number
    /// was attached to the buffer.
    #[cfg(feature = "objc2-core-media")]
    pub fn CMIOSampleBufferGetSequenceNumber(sbuf: Option<&CMSampleBuffer>) -> u64;
}

extern "C-unwind" {
    /// Sets the discontinuity flags for a CoreMedia Sample Buffer that is being used in a CMIO Graph.
    ///
    /// Discontinuity flags are a required attachment to CoreMedia Sample Buffers being used in a
    /// CMIO Graph.  As the buffer flows through a CMIO Graph, a Unit may detect a discontinuity
    /// and flag the buffer as having such.  NOTE:  in order to prevent memory corruption and other
    /// errors, this function should only be called if the caller is sure that it has the sole
    /// reference to the buffer;  if this cannot be guarenteed, then the caller should first create
    /// a copy of the buffer using CMSampleBufferCreateCopy().
    #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-core-media"))]
    pub fn CMIOSampleBufferSetDiscontinuityFlags(
        allocator: Option<&CFAllocator>,
        sbuf: Option<&CMSampleBuffer>,
        discontinuity_flags: u32,
    );
}

extern "C-unwind" {
    /// Returns the discontinuity flags for a CoreMedia Sample Buffer that is being used in a CMIO Graph.
    ///
    /// Discontinuity flags are a required attachment to CoreMedia Sample Buffers being used in a
    /// CMIO Graph.
    ///
    ///
    /// Returns: Returns kCMIOSampleBufferDiscontinuityFlag_UnknownDiscontinuity if an error occurs.
    #[cfg(feature = "objc2-core-media")]
    pub fn CMIOSampleBufferGetDiscontinuityFlags(sbuf: Option<&CMSampleBuffer>) -> u32;
}

extern "C-unwind" {
    /// Copies all optional attachments.
    ///
    /// This convenience function copies all non-required attachment
    /// values (i.e., the sequence number and discontinuity flags will not be
    /// copied).
    ///
    ///
    /// Returns: Returns paramErr if there is an error in parameters, and noErr for success.
    #[cfg(feature = "objc2-core-media")]
    pub fn CMIOSampleBufferCopyNonRequiredAttachments(
        source_s_buf: Option<&CMSampleBuffer>,
        dest_s_buf: Option<&CMSampleBuffer>,
        attachment_mode: CMAttachmentMode,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Copies sample attachments from the CMSampleBufferGetSampleAttachmentsArray()
    /// CFArrayRef from the source CMSampleBuffer to the destination
    /// CMSampleBuffer.
    ///
    /// This convenience function copies all sample attachments from the
    /// source buffer's sample attachments array to the destination buffer's.
    /// If the source CMSampleBuffer has no sample attachments, nothing
    /// happens.
    ///
    ///
    /// Returns: Returns paramErr if there is an error in parameters, and noErr for success.
    #[cfg(feature = "objc2-core-media")]
    pub fn CMIOSampleBufferCopySampleAttachments(
        source_s_buf: Option<&CMSampleBuffer>,
        dest_s_buf: Option<&CMSampleBuffer>,
    ) -> OSStatus;
}
