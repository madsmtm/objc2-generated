//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// Dependencies: This depends on Metal.framework.
    ///
    /// An efficient kernel to handle copies, transposed-copies and reshapes.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsndarrayidentity?language=objc)
    #[unsafe(super(
        MPSNDArrayUnaryKernel,
        MPSNDArrayMultiaryKernel,
        MPSNDArrayMultiaryBase,
        MPSKernel,
        NSObject
    ))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "MPSCore",
        feature = "MPSKernel",
        feature = "MPSNDArrayKernel"
    ))]
    pub struct MPSNDArrayIdentity;
);

#[cfg(all(
    feature = "MPSCore",
    feature = "MPSKernel",
    feature = "MPSNDArrayKernel"
))]
unsafe impl NSCoding for MPSNDArrayIdentity {}

#[cfg(all(
    feature = "MPSCore",
    feature = "MPSKernel",
    feature = "MPSNDArrayKernel"
))]
unsafe impl NSCopying for MPSNDArrayIdentity {}

#[cfg(all(
    feature = "MPSCore",
    feature = "MPSKernel",
    feature = "MPSNDArrayKernel"
))]
unsafe impl CopyingHelper for MPSNDArrayIdentity {
    type Result = Self;
}

#[cfg(all(
    feature = "MPSCore",
    feature = "MPSKernel",
    feature = "MPSNDArrayKernel"
))]
unsafe impl NSObjectProtocol for MPSNDArrayIdentity {}

#[cfg(all(
    feature = "MPSCore",
    feature = "MPSKernel",
    feature = "MPSNDArrayKernel"
))]
unsafe impl NSSecureCoding for MPSNDArrayIdentity {}

extern_methods!(
    #[cfg(all(
        feature = "MPSCore",
        feature = "MPSKernel",
        feature = "MPSNDArrayKernel"
    ))]
    unsafe impl MPSNDArrayIdentity {
        #[unsafe(method_family(init))]
        #[method_id(initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[cfg(feature = "MPSCoreTypes")]
        /// Do a reshape operation, either by trying to alias the array, returning an arrayview, or by copying.
        ///
        /// Parameter `cmdBuf`: The command buffer into which to encode the kernel, or to create a temporary array alias.
        ///
        /// Parameter `sourceArray`: Source array. If this function returns a non-nil result, then the readCount of `sourceArray` is decremented.
        ///
        /// Parameter `shape`: The new shape, given in TF dimension ordering (as always with MPSShape).
        ///
        /// Parameter `destinationArray`: If not nil, then the result of reshape will be copied to this. Shape of `destinationArray` must match `shape`.
        ///
        /// Returns: If `destinationArray` is not nil, then `destinationArray`. Otherwise aliasing is tried, and if aliasing is not possible
        /// due to existing slices or transposes nil is returned. If aliasing is successful, then a new arrayview of `sourceArray`
        /// is returned; If `sourceArray` is a `MPSTemporaryArray` then a `MPSTemporaryArray` is returned referencing the same data,
        /// otherwise a `MPSNDArray` type result is returned.
        #[unsafe(method_family(none))]
        #[method_id(reshapeWithCommandBuffer:sourceArray:shape:destinationArray:)]
        pub unsafe fn reshapeWithCommandBuffer_sourceArray_shape_destinationArray(
            &self,
            cmd_buf: Option<&ProtocolObject<dyn MTLCommandBuffer>>,
            source_array: &MPSNDArray,
            shape: &MPSShape,
            destination_array: Option<&MPSNDArray>,
        ) -> Option<Retained<MPSNDArray>>;

        #[unsafe(method_family(none))]
        #[method_id(reshapeWithCommandBuffer:sourceArray:dimensionCount:dimensionSizes:destinationArray:)]
        pub unsafe fn reshapeWithCommandBuffer_sourceArray_dimensionCount_dimensionSizes_destinationArray(
            &self,
            cmd_buf: Option<&ProtocolObject<dyn MTLCommandBuffer>>,
            source_array: &MPSNDArray,
            number_of_dimensions: NSUInteger,
            dimension_sizes: NonNull<NSUInteger>,
            destination_array: Option<&MPSNDArray>,
        ) -> Option<Retained<MPSNDArray>>;

        #[cfg(feature = "MPSCoreTypes")]
        #[unsafe(method_family(none))]
        #[method_id(reshapeWithCommandEncoder:commandBuffer:sourceArray:shape:destinationArray:)]
        pub unsafe fn reshapeWithCommandEncoder_commandBuffer_sourceArray_shape_destinationArray(
            &self,
            encoder: Option<&ProtocolObject<dyn MTLComputeCommandEncoder>>,
            cmd_buf: Option<&ProtocolObject<dyn MTLCommandBuffer>>,
            source_array: &MPSNDArray,
            shape: &MPSShape,
            destination_array: Option<&MPSNDArray>,
        ) -> Option<Retained<MPSNDArray>>;

        #[unsafe(method_family(none))]
        #[method_id(reshapeWithCommandEncoder:commandBuffer:sourceArray:dimensionCount:dimensionSizes:destinationArray:)]
        pub unsafe fn reshapeWithCommandEncoder_commandBuffer_sourceArray_dimensionCount_dimensionSizes_destinationArray(
            &self,
            encoder: Option<&ProtocolObject<dyn MTLComputeCommandEncoder>>,
            cmd_buf: Option<&ProtocolObject<dyn MTLCommandBuffer>>,
            source_array: &MPSNDArray,
            number_of_dimensions: NSUInteger,
            dimension_sizes: NonNull<NSUInteger>,
            destination_array: Option<&MPSNDArray>,
        ) -> Option<Retained<MPSNDArray>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSNDArrayUnaryKernel`
    #[cfg(all(
        feature = "MPSCore",
        feature = "MPSKernel",
        feature = "MPSNDArrayKernel"
    ))]
    unsafe impl MPSNDArrayIdentity {
        #[unsafe(method_family(init))]
        #[method_id(initWithDevice:sourceCount:)]
        pub unsafe fn initWithDevice_sourceCount(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            count: NSUInteger,
        ) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            coder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(
        feature = "MPSCore",
        feature = "MPSKernel",
        feature = "MPSNDArrayKernel"
    ))]
    unsafe impl MPSNDArrayIdentity {
        /// Called by NSCoder to decode MPSKernels
        ///
        /// This isn't the right interface to decode a MPSKernel, but
        /// it is the one that NSCoder uses. To enable your NSCoder
        /// (e.g. NSKeyedUnarchiver) to set which device to use
        /// extend the object to adopt the MPSDeviceProvider
        /// protocol. Otherwise, the Metal system default device
        /// will be used.
        #[unsafe(method_family(init))]
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "MPSCore",
        feature = "MPSKernel",
        feature = "MPSNDArrayKernel"
    ))]
    unsafe impl MPSNDArrayIdentity {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
