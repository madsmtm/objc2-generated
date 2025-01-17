//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// Base class for an audio output stream sink.
    ///
    /// An audio output stream sink defines how audio data from a guest is consumed on the host system.
    ///
    /// VZAudioOutputStreamSink should not be instantiated directly. One of its subclasses should be used instead.
    ///
    ///
    /// See: VZHostAudioOutputStreamSink
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzaudiooutputstreamsink?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZAudioOutputStreamSink;
);

unsafe impl NSObjectProtocol for VZAudioOutputStreamSink {}

extern_methods!(
    unsafe impl VZAudioOutputStreamSink {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
