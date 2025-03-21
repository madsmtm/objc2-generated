//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2_core_foundation::*;
#[cfg(feature = "objc2-io-surface")]
#[cfg(not(target_os = "watchos"))]
use objc2_io_surface::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvpixelbufferiosurfaceopengltexturecompatibilitykey?language=objc)
    pub static kCVPixelBufferIOSurfaceOpenGLTextureCompatibilityKey: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvpixelbufferiosurfaceopenglfbocompatibilitykey?language=objc)
    pub static kCVPixelBufferIOSurfaceOpenGLFBOCompatibilityKey: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvpixelbufferiosurfacecoreanimationcompatibilitykey?language=objc)
    pub static kCVPixelBufferIOSurfaceCoreAnimationCompatibilityKey: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvpixelbufferiosurfaceopenglestexturecompatibilitykey?language=objc)
    pub static kCVPixelBufferIOSurfaceOpenGLESTextureCompatibilityKey: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvpixelbufferiosurfaceopenglesfbocompatibilitykey?language=objc)
    pub static kCVPixelBufferIOSurfaceOpenGLESFBOCompatibilityKey: &'static CFString;
}

/// Returns the IOSurface backing the pixel buffer, or NULL if it is not backed by an IOSurface.
///
/// Parameter `pixelBuffer`: Target PixelBuffer.
#[cfg(all(
    feature = "CVBuffer",
    feature = "CVImageBuffer",
    feature = "CVPixelBuffer",
    feature = "objc2-io-surface"
))]
#[cfg(not(target_os = "watchos"))]
#[inline]
pub unsafe extern "C-unwind" fn CVPixelBufferGetIOSurface(
    pixel_buffer: Option<&CVPixelBuffer>,
) -> Option<CFRetained<IOSurfaceRef>> {
    extern "C-unwind" {
        fn CVPixelBufferGetIOSurface(
            pixel_buffer: Option<&CVPixelBuffer>,
        ) -> Option<NonNull<IOSurfaceRef>>;
    }
    let ret = unsafe { CVPixelBufferGetIOSurface(pixel_buffer) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

extern "C-unwind" {
    /// Call to create a single CVPixelBuffer for a passed-in IOSurface.
    ///
    /// The CVPixelBuffer will retain the IOSurface.
    /// IMPORTANT NOTE: If you are using IOSurface to share CVPixelBuffers between processes
    /// and those CVPixelBuffers are allocated via a CVPixelBufferPool, it is important
    /// that the CVPixelBufferPool does not reuse CVPixelBuffers whose IOSurfaces are still
    /// in use in other processes.
    /// CoreVideo and IOSurface will take care of this for if you use IOSurfaceCreateMachPort
    /// and IOSurfaceLookupFromMachPort, but NOT if you pass IOSurfaceIDs.
    ///
    /// Parameter `surface`: The IOSurface to wrap.
    ///
    /// Parameter `pixelBufferAttributes`: A dictionary with additional attributes for a a pixel buffer. This parameter is optional. See PixelBufferAttributes for more details.
    ///
    /// Parameter `pixelBufferOut`: The new pixel buffer will be returned here
    ///
    /// Returns: returns kCVReturnSuccess on success.
    #[cfg(all(
        feature = "CVBuffer",
        feature = "CVImageBuffer",
        feature = "CVPixelBuffer",
        feature = "CVReturn",
        feature = "objc2-io-surface"
    ))]
    #[cfg(not(target_os = "watchos"))]
    pub fn CVPixelBufferCreateWithIOSurface(
        allocator: Option<&CFAllocator>,
        surface: &IOSurfaceRef,
        pixel_buffer_attributes: Option<&CFDictionary>,
        pixel_buffer_out: NonNull<*mut CVPixelBuffer>,
    ) -> CVReturn;
}
