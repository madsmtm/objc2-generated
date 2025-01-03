//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corevideo/cvopenglbufferpoolref?language=objc)
pub type CVOpenGLBufferPoolRef = *mut c_void;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvopenglbufferpoolminimumbuffercountkey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVOpenGLBufferPoolMinimumBufferCountKey: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvopenglbufferpoolmaximumbufferagekey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCVOpenGLBufferPoolMaximumBufferAgeKey: CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated = "OpenGL/OpenGLES is no longer supported. Use Metal APIs instead. (Define COREVIDEO_SILENCE_GL_DEPRECATION to silence these warnings)"]
    pub fn CVOpenGLBufferPoolGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    /// Retains a CVOpenGLBufferPoolRef object
    ///
    /// Equivalent to CFRetain, but NULL safe
    ///
    /// Parameter `buffer`: A CVOpenGLBufferPoolRef object that you want to retain.
    ///
    /// Returns: A CVOpenGLBufferPoolRef object that is the same as the passed in buffer.
    #[deprecated = "OpenGL/OpenGLES is no longer supported. Use Metal APIs instead. (Define COREVIDEO_SILENCE_GL_DEPRECATION to silence these warnings)"]
    pub fn CVOpenGLBufferPoolRetain(
        open_gl_buffer_pool: CVOpenGLBufferPoolRef,
    ) -> CVOpenGLBufferPoolRef;
}

extern "C-unwind" {
    /// Creates a new OpenGL Buffer pool.
    ///
    /// Equivalent to CFRelease, but NULL safe
    ///
    /// Parameter `allocator`: The CFAllocatorRef to use for allocating this buffer pool.  May be NULL.
    ///
    /// Parameter `poolAttributes`: A CFDictionaryRef containing the attributes to be used for the pool itself.
    ///
    /// Parameter `openGLBufferAttributes`: A CFDictionaryRef containing the attributes to be used for creating new OpenGLBuffers within the pool.
    ///
    /// Parameter `poolOut`: The newly created pool will be placed here
    ///
    /// Returns: Returns kCVReturnSuccess on success
    #[cfg(all(feature = "CVReturn", feature = "objc2-core-foundation"))]
    #[deprecated = "OpenGL/OpenGLES is no longer supported. Use Metal APIs instead. (Define COREVIDEO_SILENCE_GL_DEPRECATION to silence these warnings)"]
    pub fn CVOpenGLBufferPoolCreate(
        allocator: CFAllocatorRef,
        pool_attributes: CFDictionaryRef,
        open_gl_buffer_attributes: CFDictionaryRef,
        pool_out: NonNull<CVOpenGLBufferPoolRef>,
    ) -> CVReturn;
}

extern "C-unwind" {
    /// Returns the pool attributes dictionary for a CVOpenGLBufferPool
    ///
    /// Parameter `pool`: The CVOpenGLBufferPoolRef to retrieve the attributes from
    ///
    /// Returns: Returns the pool attributes dictionary, or NULL on failure.
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated = "OpenGL/OpenGLES is no longer supported. Use Metal APIs instead. (Define COREVIDEO_SILENCE_GL_DEPRECATION to silence these warnings)"]
    pub fn CVOpenGLBufferPoolGetAttributes(pool: CVOpenGLBufferPoolRef) -> CFDictionaryRef;
}

extern "C-unwind" {
    /// Returns the attributes of OpenGL buffers that will be created from this pool.
    ///
    /// This function is provided for those cases where you may need to know some information about the buffers that
    /// will be created up front.
    ///
    /// Parameter `pool`: The CVOpenGLBufferPoolRef to retrieve the attributes from
    ///
    /// Returns: Returns the OpenGL buffer attributes dictionary, or NULL on failure.
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated = "OpenGL/OpenGLES is no longer supported. Use Metal APIs instead. (Define COREVIDEO_SILENCE_GL_DEPRECATION to silence these warnings)"]
    pub fn CVOpenGLBufferPoolGetOpenGLBufferAttributes(
        pool: CVOpenGLBufferPoolRef,
    ) -> CFDictionaryRef;
}

extern "C-unwind" {
    /// Creates a new OpenGLBuffer object from the pool.
    ///
    /// The function creates a new CVOpenGLBuffer with the default attachments using the OpenGL buffer attributes specifed during pool creation.
    ///
    /// Parameter `allocator`: The CFAllocatorRef to use for creating the OpenGL buffer.  May be NULL.
    ///
    /// Parameter `openGLBufferPool`: The CVOpenGLBufferPool that should create the new CVOpenGLBuffer.
    ///
    /// Parameter `openGLBufferOut`: The newly created OpenGL buffer will be placed here
    ///
    /// Returns: Returns kCVReturnSuccess on success
    #[cfg(all(
        feature = "CVBuffer",
        feature = "CVImageBuffer",
        feature = "CVOpenGLBuffer",
        feature = "CVReturn",
        feature = "objc2-core-foundation"
    ))]
    #[deprecated = "OpenGL/OpenGLES is no longer supported. Use Metal APIs instead. (Define COREVIDEO_SILENCE_GL_DEPRECATION to silence these warnings)"]
    pub fn CVOpenGLBufferPoolCreateOpenGLBuffer(
        allocator: CFAllocatorRef,
        open_gl_buffer_pool: CVOpenGLBufferPoolRef,
        open_gl_buffer_out: NonNull<CVOpenGLBufferRef>,
    ) -> CVReturn;
}
