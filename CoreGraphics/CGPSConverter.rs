//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpsconverterref?language=objc)
pub type CGPSConverterRef = *mut c_void;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpsconverterbegindocumentcallback?language=objc)
pub type CGPSConverterBeginDocumentCallback = Option<unsafe extern "C-unwind" fn(*mut c_void)>;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpsconverterenddocumentcallback?language=objc)
pub type CGPSConverterEndDocumentCallback = Option<unsafe extern "C-unwind" fn(*mut c_void, bool)>;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpsconverterbeginpagecallback?language=objc)
#[cfg(feature = "objc2-core-foundation")]
pub type CGPSConverterBeginPageCallback =
    Option<unsafe extern "C-unwind" fn(*mut c_void, usize, CFDictionaryRef)>;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpsconverterendpagecallback?language=objc)
#[cfg(feature = "objc2-core-foundation")]
pub type CGPSConverterEndPageCallback =
    Option<unsafe extern "C-unwind" fn(*mut c_void, usize, CFDictionaryRef)>;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpsconverterprogresscallback?language=objc)
pub type CGPSConverterProgressCallback = Option<unsafe extern "C-unwind" fn(*mut c_void)>;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpsconvertermessagecallback?language=objc)
#[cfg(feature = "objc2-core-foundation")]
pub type CGPSConverterMessageCallback =
    Option<unsafe extern "C-unwind" fn(*mut c_void, CFStringRef)>;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpsconverterreleaseinfocallback?language=objc)
pub type CGPSConverterReleaseInfoCallback = Option<unsafe extern "C-unwind" fn(*mut c_void)>;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpsconvertercallbacks?language=objc)
#[cfg(feature = "objc2-core-foundation")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CGPSConverterCallbacks {
    pub version: c_uint,
    pub beginDocument: CGPSConverterBeginDocumentCallback,
    pub endDocument: CGPSConverterEndDocumentCallback,
    pub beginPage: CGPSConverterBeginPageCallback,
    pub endPage: CGPSConverterEndPageCallback,
    pub noteProgress: CGPSConverterProgressCallback,
    pub noteMessage: CGPSConverterMessageCallback,
    pub releaseInfo: CGPSConverterReleaseInfoCallback,
}

#[cfg(all(feature = "objc2", feature = "objc2-core-foundation"))]
unsafe impl Encode for CGPSConverterCallbacks {
    const ENCODING: Encoding = Encoding::Struct(
        "CGPSConverterCallbacks",
        &[
            <c_uint>::ENCODING,
            <CGPSConverterBeginDocumentCallback>::ENCODING,
            Encoding::Pointer(&Encoding::Unknown),
            <CGPSConverterBeginPageCallback>::ENCODING,
            <CGPSConverterEndPageCallback>::ENCODING,
            <CGPSConverterProgressCallback>::ENCODING,
            <CGPSConverterMessageCallback>::ENCODING,
            <CGPSConverterReleaseInfoCallback>::ENCODING,
        ],
    );
}

#[cfg(all(feature = "objc2", feature = "objc2-core-foundation"))]
unsafe impl RefEncode for CGPSConverterCallbacks {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPSConverterCreate(
        info: *mut c_void,
        callbacks: NonNull<CGPSConverterCallbacks>,
        options: CFDictionaryRef,
    ) -> CGPSConverterRef;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CGDataConsumer",
        feature = "CGDataProvider",
        feature = "objc2-core-foundation"
    ))]
    pub fn CGPSConverterConvert(
        converter: CGPSConverterRef,
        provider: CGDataProviderRef,
        consumer: CGDataConsumerRef,
        options: CFDictionaryRef,
    ) -> bool;
}

extern "C-unwind" {
    pub fn CGPSConverterAbort(converter: CGPSConverterRef) -> bool;
}

extern "C-unwind" {
    pub fn CGPSConverterIsConverting(converter: CGPSConverterRef) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGPSConverterGetTypeID() -> CFTypeID;
}
