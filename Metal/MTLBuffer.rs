//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// A typeless allocation accessible by both the CPU and the GPU (MTLDevice) or by only the GPU when the storage mode is
    /// MTLResourceStorageModePrivate.
    ///
    ///
    /// Unlike in OpenGL and OpenCL, access to buffers is not synchronized.  The caller may use the CPU to modify the data at any time
    /// but is also responsible for ensuring synchronization and coherency.
    ///
    /// The contents become undefined if both the CPU and GPU write to the same buffer without a synchronizing action between those writes.
    /// This is true even when the regions written do not overlap.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlbuffer?language=objc)
    #[cfg(all(feature = "MTLAllocation", feature = "MTLResource"))]
    pub unsafe trait MTLBuffer: MTLResource {
        /// The length of the buffer in bytes.
        #[unsafe(method(length))]
        #[unsafe(method_family = none)]
        fn length(&self) -> NSUInteger;

        /// Returns the data pointer of this buffer's shared copy.
        #[unsafe(method(contents))]
        #[unsafe(method_family = none)]
        fn contents(&self) -> NonNull<c_void>;

        /// Inform the device of the range of a buffer that the CPU has modified, allowing the implementation to invalidate
        /// its caches of the buffer's content.
        ///
        /// When the application writes to a buffer's sysmem copy via
        /// _contents,_that range of the buffer immediately
        /// becomes undefined for any accesses by the GPU (MTLDevice).  To restore coherency, the buffer modification must be followed
        /// by -didModifyRange:, and then followed by a commit of the MTLCommandBuffer that will access the buffer.
        /// -didModifyRange does not make the contents coherent for any previously committed command buffers.
        /// Note: This method is only required if buffer is created with a storage mode of MTLResourceStorageModeManaged.
        /// It is not valid to invoke this method on buffers of other storage modes.
        ///
        /// Parameter `range`: The range of bytes that have been modified.
        #[unsafe(method(didModifyRange:))]
        #[unsafe(method_family = none)]
        fn didModifyRange(&self, range: NSRange);

        #[cfg(feature = "MTLTexture")]
        /// Create a 2D texture or texture buffer that shares storage with this buffer.
        #[unsafe(method(newTextureWithDescriptor:offset:bytesPerRow:))]
        #[unsafe(method_family = new)]
        fn newTextureWithDescriptor_offset_bytesPerRow(
            &self,
            descriptor: &MTLTextureDescriptor,
            offset: NSUInteger,
            bytes_per_row: NSUInteger,
        ) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;

        /// Adds a marker to a specific range in the buffer.
        /// When inspecting a buffer in the GPU debugging tools the marker will be shown.
        ///
        /// Parameter `marker`: A label used for the marker.
        ///
        /// Parameter `range`: The range of bytes the marker is using.
        #[unsafe(method(addDebugMarker:range:))]
        #[unsafe(method_family = none)]
        fn addDebugMarker_range(&self, marker: &NSString, range: NSRange);

        /// Removes all debug markers from a buffer.
        #[unsafe(method(removeAllDebugMarkers))]
        #[unsafe(method_family = none)]
        fn removeAllDebugMarkers(&self);

        /// For Metal buffer objects that are remote views, this returns the buffer associated with the storage on the originating device.
        #[unsafe(method(remoteStorageBuffer))]
        #[unsafe(method_family = none)]
        fn remoteStorageBuffer(&self) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        #[cfg(feature = "MTLDevice")]
        /// On Metal devices that support peer to peer transfers, this method is used to create a remote buffer view on another device
        /// within the peer group.  The receiver must use MTLStorageModePrivate or be backed by an IOSurface.
        #[unsafe(method(newRemoteBufferViewForDevice:))]
        #[unsafe(method_family = new)]
        fn newRemoteBufferViewForDevice(
            &self,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// Represents the GPU virtual address of a buffer resource
        #[unsafe(method(gpuAddress))]
        #[unsafe(method_family = none)]
        fn gpuAddress(&self) -> u64;
    }
);
