//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-metal")]
#[cfg(not(target_os = "watchos"))]
use objc2_metal::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvmetalbuffercachemaximumbufferagekey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVMetalBufferCacheMaximumBufferAgeKey: CFStringRef;
}

/// [Apple's documentation](https://developer.apple.com/documentation/corevideo/cvmetalbuffercacheref?language=objc)
pub type CVMetalBufferCacheRef = *mut c_void;

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CVMetalBufferCacheGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CVReturn",
        feature = "objc2-core-foundation",
        feature = "objc2-metal"
    ))]
    #[cfg(not(target_os = "watchos"))]
    pub fn CVMetalBufferCacheCreate(
        allocator: CFAllocatorRef,
        cache_attributes: CFDictionaryRef,
        metal_device: &ProtocolObject<dyn MTLDevice>,
        cache_out: NonNull<CVMetalBufferCacheRef>,
    ) -> CVReturn;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CVBuffer",
        feature = "CVImageBuffer",
        feature = "CVMetalBuffer",
        feature = "CVReturn",
        feature = "objc2-core-foundation"
    ))]
    pub fn CVMetalBufferCacheCreateBufferFromImage(
        allocator: CFAllocatorRef,
        buffer_cache: CVMetalBufferCacheRef,
        image_buffer: CVImageBufferRef,
        buffer_out: NonNull<CVMetalBufferRef>,
    ) -> CVReturn;
}

extern "C-unwind" {
    #[cfg(feature = "CVBase")]
    pub fn CVMetalBufferCacheFlush(buffer_cache: CVMetalBufferCacheRef, options: CVOptionFlags);
}