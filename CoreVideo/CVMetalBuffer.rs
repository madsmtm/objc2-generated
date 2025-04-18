//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
use objc2_core_foundation::*;
#[cfg(feature = "objc2-metal")]
#[cfg(not(target_os = "watchos"))]
use objc2_metal::*;

use crate::*;

/// Metal buffer based CVPixelBuffer wrapped buffer
///
/// IMPORTANT NOTE: Clients should retain CVMetalBuffer objects until they are done using the contents in them.
/// Retaining a CVMetalBuffer is your way to indicate that you're still using the image in the buffer, and that it should not be recycled yet.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/corevideo/cvmetalbuffer?language=objc)
#[cfg(feature = "CVBuffer")]
pub type CVMetalBuffer = CVBuffer;

#[inline]
pub extern "C-unwind" fn CVMetalBufferGetTypeID() -> CFTypeID {
    extern "C-unwind" {
        fn CVMetalBufferGetTypeID() -> CFTypeID;
    }
    unsafe { CVMetalBufferGetTypeID() }
}

/// Returns the Metal MTLBuffer object of the CVMetalBufferRef
///
/// Parameter `buffer`: Target CVMetalBuffer
///
/// Returns: Metal buffer
#[cfg(all(feature = "CVBuffer", feature = "objc2", feature = "objc2-metal"))]
#[cfg(not(target_os = "watchos"))]
#[inline]
pub unsafe extern "C-unwind" fn CVMetalBufferGetBuffer(
    buffer: &CVMetalBuffer,
) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>> {
    extern "C-unwind" {
        fn CVMetalBufferGetBuffer(buffer: &CVMetalBuffer) -> *mut ProtocolObject<dyn MTLBuffer>;
    }
    let ret = unsafe { CVMetalBufferGetBuffer(buffer) };
    unsafe { Retained::retain_autoreleased(ret) }
}
