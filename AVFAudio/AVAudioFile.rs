//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// An audio file opened for reading or writing.
    ///
    /// Regardless of the file's actual format, reading and writing the file is done via
    /// `AVAudioPCMBuffer` objects, containing samples in an `AVAudioCommonFormat`,
    /// referred to as the file's "processing format." Conversions are performed to and from
    /// the file's actual format.
    ///
    /// Reads and writes are always sequential, but random access is possible by setting the
    /// framePosition property.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiofile?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAudioFile;
);

unsafe impl NSObjectProtocol for AVAudioFile {}

extern_methods!(
    unsafe impl AVAudioFile {
        /// Open a file for reading.
        ///
        /// Parameter `fileURL`: the file to open
        ///
        /// Parameter `outError`: on exit, if an error occurs, a description of the error
        ///
        /// This opens the file for reading using the standard format (deinterleaved floating point).
        #[method_id(@__retain_semantics Init initForReading:error:_)]
        pub unsafe fn initForReading_error(
            this: Allocated<Self>,
            file_url: &NSURL,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "AVAudioFormat")]
        /// Open a file for reading, using a specified processing format.
        ///
        /// Parameter `fileURL`: the file to open
        ///
        /// Parameter `format`: the processing format to use when reading from the file
        ///
        /// Parameter `interleaved`: whether to use an interleaved processing format
        ///
        /// Parameter `outError`: on exit, if an error occurs, a description of the error
        #[method_id(@__retain_semantics Init initForReading:commonFormat:interleaved:error:_)]
        pub unsafe fn initForReading_commonFormat_interleaved_error(
            this: Allocated<Self>,
            file_url: &NSURL,
            format: AVAudioCommonFormat,
            interleaved: bool,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        /// Open a file for writing.
        ///
        /// Parameter `fileURL`: the path at which to create the file
        ///
        /// Parameter `settings`: the format of the file to create (See `AVAudioRecorder`.)  For linear PCM,
        /// only interleaved formats are supported for the saved file, non interleaved setting will be
        /// ignored and a warning is shown.
        ///
        /// Parameter `outError`: on exit, if an error occurs, a description of the error
        ///
        /// The file type to create can be set through the corresponding settings key. If not set, it will be
        /// inferred from the file extension. Will overwrite a file at the specified URL if a file exists.
        ///
        /// This opens the file for writing using the standard format (deinterleaved floating point).
        #[method_id(@__retain_semantics Init initForWriting:settings:error:_)]
        pub unsafe fn initForWriting_settings_error(
            this: Allocated<Self>,
            file_url: &NSURL,
            settings: &NSDictionary<NSString, AnyObject>,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[cfg(feature = "AVAudioFormat")]
        /// Open a file for writing.
        ///
        /// Parameter `fileURL`: the path at which to create the file
        ///
        /// Parameter `settings`: the format of the file to create (See `AVAudioRecorder`.) For linear PCM,
        /// only interleaved formats are supported for the saved file, non interleaved setting will be
        /// ignored and a warning is shown.
        ///
        /// Parameter `format`: the processing format to use when writing to the file.
        ///
        /// Parameter `interleaved`: whether to use an interleaved processing format
        ///
        /// Parameter `outError`: on exit, if an error occurs, a description of the error
        ///
        /// The file type to create can be set through the corresponding settings key. If not set, it will be
        /// inferred from the file extension. Will overwrite a file at the specified URL if a file exists.
        #[method_id(@__retain_semantics Init initForWriting:settings:commonFormat:interleaved:error:_)]
        pub unsafe fn initForWriting_settings_commonFormat_interleaved_error(
            this: Allocated<Self>,
            file_url: &NSURL,
            settings: &NSDictionary<NSString, AnyObject>,
            format: AVAudioCommonFormat,
            interleaved: bool,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        /// Close the audio file.
        ///
        /// The underlying file will be closed if open.
        ///
        /// - It is normally unnecessary to close a file opened for reading (it will be automatically closed
        /// when the object is released)
        /// - It is only necessary to close a file opened for writing in order to achieve specific control over
        /// when the file's header is updated.
        ///
        /// Note: Once closed, further file read or write operations will fail with kAudio_FileNotFoundError.
        #[method(close)]
        pub unsafe fn close(&self);

        #[cfg(feature = "AVAudioBuffer")]
        /// Read an entire buffer.
        ///
        /// Parameter `buffer`: The buffer into which to read from the file. Its format must match the file's
        /// processing format.
        ///
        /// Parameter `outError`: on exit, if an error occurs, a description of the error
        ///
        /// Returns: YES for success.
        ///
        /// Reading sequentially from framePosition, attempts to fill the buffer to its capacity. On
        /// return, the buffer's length indicates the number of sample frames successfully read.
        #[method(readIntoBuffer:error:_)]
        pub unsafe fn readIntoBuffer_error(
            &self,
            buffer: &AVAudioPCMBuffer,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(all(feature = "AVAudioBuffer", feature = "AVAudioTypes"))]
        /// Read a portion of a buffer.
        ///
        /// Parameter `frames`: The number of frames to read.
        ///
        /// Parameter `buffer`: The buffer into which to read from the file. Its format must match the file's
        /// processing format.
        ///
        /// Parameter `outError`: on exit, if an error occurs, a description of the error
        ///
        /// Returns: YES for success.
        ///
        /// Like `readIntoBuffer:error:`, but can be used to read fewer frames than buffer.frameCapacity.
        #[method(readIntoBuffer:frameCount:error:_)]
        pub unsafe fn readIntoBuffer_frameCount_error(
            &self,
            buffer: &AVAudioPCMBuffer,
            frames: AVAudioFrameCount,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "AVAudioBuffer")]
        /// Write a buffer.
        ///
        /// Parameter `buffer`: The buffer from which to write to the file. Its format must match the file's
        /// processing format.
        ///
        /// Parameter `outError`: on exit, if an error occurs, a description of the error
        ///
        /// Returns: YES for success.
        ///
        /// Writes sequentially. The buffer's frameLength signifies how much of the buffer is to be written.
        #[method(writeFromBuffer:error:_)]
        pub unsafe fn writeFromBuffer_error(
            &self,
            buffer: &AVAudioPCMBuffer,
        ) -> Result<(), Retained<NSError>>;

        /// Whether the file is open or not.
        #[method(isOpen)]
        pub unsafe fn isOpen(&self) -> bool;

        /// The URL the file is reading or writing.
        #[method_id(@__retain_semantics Other url)]
        pub unsafe fn url(&self) -> Retained<NSURL>;

        #[cfg(feature = "AVAudioFormat")]
        /// The on-disk format of the file.
        #[method_id(@__retain_semantics Other fileFormat)]
        pub unsafe fn fileFormat(&self) -> Retained<AVAudioFormat>;

        #[cfg(feature = "AVAudioFormat")]
        /// The processing format of the file.
        #[method_id(@__retain_semantics Other processingFormat)]
        pub unsafe fn processingFormat(&self) -> Retained<AVAudioFormat>;

        #[cfg(feature = "AVAudioTypes")]
        /// The number of sample frames in the file.
        ///
        /// Note: this can be expensive to compute for the first time.
        #[method(length)]
        pub unsafe fn length(&self) -> AVAudioFramePosition;

        #[cfg(feature = "AVAudioTypes")]
        /// The position in the file at which the next read or write will occur.
        ///
        /// Set framePosition to perform a seek before a read or write. A read or write operation advances the frame position by the number of frames read or written.
        #[method(framePosition)]
        pub unsafe fn framePosition(&self) -> AVAudioFramePosition;

        #[cfg(feature = "AVAudioTypes")]
        /// Setter for [`framePosition`][Self::framePosition].
        #[method(setFramePosition:)]
        pub unsafe fn setFramePosition(&self, frame_position: AVAudioFramePosition);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVAudioFile {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);