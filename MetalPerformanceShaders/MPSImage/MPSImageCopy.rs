//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// The MPSImageCopyToMatrix copies image data to a MPSMatrix.
    /// The image data is stored in a row of a matrix.  The dataLayout
    /// specifies the order in which the feature channels in the MPSImage
    /// get stored in the matrix.  If MPSImage stores a batch of images,
    /// the images are copied into multiple rows, one row per image.
    ///
    /// The number of elements in a row in the matrix must be >= image width *
    /// image height * number of featureChannels in the image.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimagecopytomatrix?language=objc)
    #[unsafe(super(MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
    pub struct MPSImageCopyToMatrix;
);

#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
extern_conformance!(
    unsafe impl NSCoding for MPSImageCopyToMatrix {}
);

#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
extern_conformance!(
    unsafe impl NSCopying for MPSImageCopyToMatrix {}
);

#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSImageCopyToMatrix {
    type Result = Self;
}

#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
extern_conformance!(
    unsafe impl NSObjectProtocol for MPSImageCopyToMatrix {}
);

#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
extern_conformance!(
    unsafe impl NSSecureCoding for MPSImageCopyToMatrix {}
);

#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
impl MPSImageCopyToMatrix {
    extern_methods!(
        /// The origin, relative to [0, 0] in the destination matrix, at which to
        /// start writing results.  This property is modifiable and defaults
        /// to [0, 0] at initialization time.  If a different origin is desired
        /// then this should be modified prior to encoding the kernel.  The z
        /// value must be 0.
        #[unsafe(method(destinationMatrixOrigin))]
        #[unsafe(method_family = none)]
        pub unsafe fn destinationMatrixOrigin(&self) -> MTLOrigin;

        /// Setter for [`destinationMatrixOrigin`][Self::destinationMatrixOrigin].
        #[unsafe(method(setDestinationMatrixOrigin:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDestinationMatrixOrigin(&self, destination_matrix_origin: MTLOrigin);

        /// The index of the destination matrix in the batch.  This property is
        /// modifiable and defaults to 0 at initialization time.
        #[unsafe(method(destinationMatrixBatchIndex))]
        #[unsafe(method_family = none)]
        pub unsafe fn destinationMatrixBatchIndex(&self) -> NSUInteger;

        /// Setter for [`destinationMatrixBatchIndex`][Self::destinationMatrixBatchIndex].
        #[unsafe(method(setDestinationMatrixBatchIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDestinationMatrixBatchIndex(
            &self,
            destination_matrix_batch_index: NSUInteger,
        );

        /// The data layout to use
        ///
        /// Returns the data layout.  When copying from a MPSImage to a MPSMatrix, this
        /// describes the order in which the image values are stored in the buffer associated
        /// with the MPSMatrix.
        /// Default: MPSDataLayoutFeatureChannelsxHeightxWidth
        #[unsafe(method(dataLayout))]
        #[unsafe(method_family = none)]
        pub unsafe fn dataLayout(&self) -> MPSDataLayout;

        /// Initialize a MPSMatrixCopy object on a device
        ///
        /// Parameter `device`: The device the kernel will run on
        ///
        /// Parameter `dataLayout`: The data layout
        ///
        /// Returns: A valid MPSMatrixCopy object or nil, if failure.
        #[unsafe(method(initWithDevice:dataLayout:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithDevice_dataLayout(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            data_layout: MPSDataLayout,
        ) -> Retained<Self>;

        /// NSSecureCoding compatability
        ///
        /// While the standard NSSecureCoding/NSCoding method
        /// -initWithCoder: should work, since the file can't
        /// know which device your data is allocated on, we
        /// have to guess and may guess incorrectly.  To avoid
        /// that problem, use initWithCoder:device instead.
        ///
        /// Parameter `aDecoder`: The NSCoder subclass with your serialized MPSKernel
        ///
        /// Parameter `device`: The MTLDevice on which to make the MPSKernel
        ///
        /// Returns: A new MPSKernel object, or nil if failure.
        #[unsafe(method(initWithCoder:device:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "MPSMatrix")]
        /// Encode a kernel that copies a MPSImage to a MPSMatrix into a command buffer
        /// using a MTLComputeCommandEncoder.
        ///
        /// The kernel copies feature channels from sourceImage to the buffer
        /// associated with destinationMatrix.  The kernel will not begin to execute until
        /// after the command buffer has been enqueued and committed.
        ///
        /// NOTE: The destinationMatrix.dataType must match the feature channel data type in sourceImage.
        ///
        ///
        /// Parameter `commandBuffer`: A valid MTLCommandBuffer.
        ///
        /// Parameter `sourceImage`: A valid MPSImage describing the image to copy from.
        ///
        /// Parameter `destinationMatrix`: A valid MPSMatrix or MPSTemporaryMatrix object describing the matrix to copy to.
        #[unsafe(method(encodeToCommandBuffer:sourceImage:destinationMatrix:))]
        #[unsafe(method_family = none)]
        pub unsafe fn encodeToCommandBuffer_sourceImage_destinationMatrix(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_image: &MPSImage,
            destination_matrix: &MPSMatrix,
        );

        #[cfg(all(feature = "MPSMatrix", feature = "MPSNDArray"))]
        /// Encode a kernel that copies a MPSImageBatch to a MPSMatrix into a command buffer
        /// using a MTLComputeCommandEncoder.
        ///
        /// The kernel copies feature channels from sourceImage to the buffer
        /// associated with destinationMatrix.  The kernel will not begin to execute until
        /// after the command buffer has been enqueued and committed.
        /// Each image will be copied to its own row in the matrix, starting with row
        /// destinationMatrixOrigin.x.
        ///
        /// NOTE: The destinationMatrix.dataType must match the feature channel data type in sourceImage.
        /// NOTE: All the images in the source batch should be of the same size and have numberOfImages = 1.
        ///
        ///
        /// Parameter `commandBuffer`: A valid MTLCommandBuffer.
        ///
        /// Parameter `sourceImages`: A valid MPSImageBatch describing the images to copy from.
        ///
        /// Parameter `destinationMatrix`: A valid MPSMatrix or MPSTemporaryMatrix object describing the matrix to copy to.
        #[unsafe(method(encodeBatchToCommandBuffer:sourceImages:destinationMatrix:))]
        #[unsafe(method_family = none)]
        pub unsafe fn encodeBatchToCommandBuffer_sourceImages_destinationMatrix(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_images: &MPSImageBatch,
            destination_matrix: &MPSMatrix,
        );
    );
}

/// Methods declared on superclass `MPSKernel`.
#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
impl MPSImageCopyToMatrix {
    extern_methods!(
        /// Standard init with default properties per filter type
        ///
        /// Parameter `device`: The device that the filter will be used on. May not be NULL.
        ///
        /// Returns: a pointer to the newly initialized object. This will fail, returning
        /// nil if the device is not supported. Devices must be
        /// MTLFeatureSet_iOS_GPUFamily2_v1 or later.
        #[unsafe(method(initWithDevice:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        /// Called by NSCoder to decode MPSKernels
        ///
        /// This isn't the right interface to decode a MPSKernel, but
        /// it is the one that NSCoder uses. To enable your NSCoder
        /// (e.g. NSKeyedUnarchiver) to set which device to use
        /// extend the object to adopt the MPSDeviceProvider
        /// protocol. Otherwise, the Metal system default device
        /// will be used.
        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
impl MPSImageCopyToMatrix {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// The MPSMatrixCopyToImage copies matrix data to a MPSImage.
    /// The operation is the reverse of MPSImageCopyToMatrix.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsmatrixcopytoimage?language=objc)
    #[unsafe(super(MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
    pub struct MPSMatrixCopyToImage;
);

#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
extern_conformance!(
    unsafe impl NSCoding for MPSMatrixCopyToImage {}
);

#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
extern_conformance!(
    unsafe impl NSCopying for MPSMatrixCopyToImage {}
);

#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSMatrixCopyToImage {
    type Result = Self;
}

#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
extern_conformance!(
    unsafe impl NSObjectProtocol for MPSMatrixCopyToImage {}
);

#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
extern_conformance!(
    unsafe impl NSSecureCoding for MPSMatrixCopyToImage {}
);

#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
impl MPSMatrixCopyToImage {
    extern_methods!(
        /// The origin, relative to [0, 0] in the source matrix.
        /// This property is modifiable and defaults
        /// to [0, 0] at initialization time.  If a different origin is desired
        /// then this should be modified prior to encoding the kernel.  The z
        /// value must be 0.
        #[unsafe(method(sourceMatrixOrigin))]
        #[unsafe(method_family = none)]
        pub unsafe fn sourceMatrixOrigin(&self) -> MTLOrigin;

        /// Setter for [`sourceMatrixOrigin`][Self::sourceMatrixOrigin].
        #[unsafe(method(setSourceMatrixOrigin:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSourceMatrixOrigin(&self, source_matrix_origin: MTLOrigin);

        /// The index of the source matrix in the batch.  This property is
        /// modifiable and defaults to 0 at initialization time.
        #[unsafe(method(sourceMatrixBatchIndex))]
        #[unsafe(method_family = none)]
        pub unsafe fn sourceMatrixBatchIndex(&self) -> NSUInteger;

        /// Setter for [`sourceMatrixBatchIndex`][Self::sourceMatrixBatchIndex].
        #[unsafe(method(setSourceMatrixBatchIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSourceMatrixBatchIndex(&self, source_matrix_batch_index: NSUInteger);

        /// The data layout to use
        ///
        /// Returns the data layout.  When copying from a MPSMatrix to a MPSImage, this
        /// describes the order in which the image values are to be stored in the buffer associated
        /// with the MPSMatrix.
        /// Default: MPSDataLayoutFeatureChannelsxHeightxWidth
        #[unsafe(method(dataLayout))]
        #[unsafe(method_family = none)]
        pub unsafe fn dataLayout(&self) -> MPSDataLayout;

        /// Initialize a MPSMatrixCopyToImage object on a device
        ///
        /// Parameter `device`: The device the kernel will run on
        ///
        /// Parameter `dataLayout`: The data layout
        ///
        /// Returns: A valid MPSMatrixCopyToImage object or nil, if failure.
        #[unsafe(method(initWithDevice:dataLayout:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithDevice_dataLayout(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            data_layout: MPSDataLayout,
        ) -> Retained<Self>;

        /// NSSecureCoding compatability
        ///
        /// While the standard NSSecureCoding/NSCoding method
        /// -initWithCoder: should work, since the file can't
        /// know which device your data is allocated on, we
        /// have to guess and may guess incorrectly.  To avoid
        /// that problem, use initWithCoder:device instead.
        ///
        /// Parameter `aDecoder`: The NSCoder subclass with your serialized MPSKernel
        ///
        /// Parameter `device`: The MTLDevice on which to make the MPSKernel
        ///
        /// Returns: A new MPSKernel object, or nil if failure.
        #[unsafe(method(initWithCoder:device:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "MPSMatrix")]
        /// Encode a kernel that copies a MPSMatrix to a MPSImage into a command buffer
        /// using a MTLComputeCommandEncoder.
        ///
        /// The kernel copies feature channels from sourceMatrix to the destinationImage.
        /// The kernel will not begin to execute until
        /// after the command buffer has been enqueued and committed.
        ///
        /// NOTE: The sourceMatrix.dataType must match the feature channel data type in destinationImage.
        ///
        ///
        /// Parameter `commandBuffer`: A valid MTLCommandBuffer.
        ///
        /// Parameter `sourceMatrix`: A valid MPSMatrix or MPSTemporaryMatrix object describing the source matrix.
        ///
        /// Parameter `destinationImage`: A valid MPSImage describing the image to copy to.
        #[unsafe(method(encodeToCommandBuffer:sourceMatrix:destinationImage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn encodeToCommandBuffer_sourceMatrix_destinationImage(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_matrix: &MPSMatrix,
            destination_image: &MPSImage,
        );

        #[cfg(all(feature = "MPSMatrix", feature = "MPSNDArray"))]
        /// Encode a kernel that copies a MPSMatrix to a MPSImageBatch into a command buffer
        /// using a MTLComputeCommandEncoder.
        ///
        /// The kernel copies feature channels from sourceImage to the buffer
        /// associated with destinationMatrix.  The kernel will not begin to execute until
        /// after the command buffer has been enqueued and committed.
        /// Each image will be copied to its own row in the matrix, starting with row
        /// destinationMatrixOrigin.x.
        ///
        /// NOTE: The destinationMatrix.dataType must match the feature channel data type in sourceImage.
        /// NOTE: All the images in the source batch should be of the same size and have numberOfImages = 1.
        ///
        ///
        /// Parameter `commandBuffer`: A valid MTLCommandBuffer.
        ///
        /// Parameter `sourceMatrix`: A valid MPSMatrix or MPSTemporaryMatrix object describing the source matrix.
        ///
        /// Parameter `destinationImages`: A valid MPSImageBatch describing the images to copy to.
        #[unsafe(method(encodeBatchToCommandBuffer:sourceMatrix:destinationImages:))]
        #[unsafe(method_family = none)]
        pub unsafe fn encodeBatchToCommandBuffer_sourceMatrix_destinationImages(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_matrix: &MPSMatrix,
            destination_images: &MPSImageBatch,
        );
    );
}

/// Methods declared on superclass `MPSKernel`.
#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
impl MPSMatrixCopyToImage {
    extern_methods!(
        /// Standard init with default properties per filter type
        ///
        /// Parameter `device`: The device that the filter will be used on. May not be NULL.
        ///
        /// Returns: a pointer to the newly initialized object. This will fail, returning
        /// nil if the device is not supported. Devices must be
        /// MTLFeatureSet_iOS_GPUFamily2_v1 or later.
        #[unsafe(method(initWithDevice:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        /// Called by NSCoder to decode MPSKernels
        ///
        /// This isn't the right interface to decode a MPSKernel, but
        /// it is the one that NSCoder uses. To enable your NSCoder
        /// (e.g. NSKeyedUnarchiver) to set which device to use
        /// extend the object to adopt the MPSDeviceProvider
        /// protocol. Otherwise, the Metal system default device
        /// will be used.
        #[unsafe(method(initWithCoder:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(feature = "MPSCore", feature = "MPSKernel"))]
impl MPSMatrixCopyToImage {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
