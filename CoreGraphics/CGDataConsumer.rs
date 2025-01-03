//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgdataconsumerref?language=objc)
pub type CGDataConsumerRef = *mut c_void;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgdataconsumerputbytescallback?language=objc)
pub type CGDataConsumerPutBytesCallback =
    Option<unsafe extern "C-unwind" fn(*mut c_void, NonNull<c_void>, usize) -> usize>;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgdataconsumerreleaseinfocallback?language=objc)
pub type CGDataConsumerReleaseInfoCallback = Option<unsafe extern "C-unwind" fn(*mut c_void)>;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgdataconsumercallbacks?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CGDataConsumerCallbacks {
    pub putBytes: CGDataConsumerPutBytesCallback,
    pub releaseConsumer: CGDataConsumerReleaseInfoCallback,
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CGDataConsumerCallbacks {
    const ENCODING: Encoding = Encoding::Struct(
        "CGDataConsumerCallbacks",
        &[
            <CGDataConsumerPutBytesCallback>::ENCODING,
            <CGDataConsumerReleaseInfoCallback>::ENCODING,
        ],
    );
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CGDataConsumerCallbacks {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGDataConsumerGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    pub fn CGDataConsumerCreate(
        info: *mut c_void,
        cbks: *const CGDataConsumerCallbacks,
    ) -> CGDataConsumerRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGDataConsumerCreateWithURL(url: CFURLRef) -> CGDataConsumerRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGDataConsumerCreateWithCFData(data: CFMutableDataRef) -> CGDataConsumerRef;
}

extern "C-unwind" {
    pub fn CGDataConsumerRetain(consumer: CGDataConsumerRef) -> CGDataConsumerRef;
}

extern "C-unwind" {
    pub fn CGDataConsumerRelease(consumer: CGDataConsumerRef);
}
