//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsndarraydescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPSNDArrayDescriptor;
);

unsafe impl NSObjectProtocol for MPSNDArrayDescriptor {}

extern_methods!(
    unsafe impl MPSNDArrayDescriptor {
        #[cfg(feature = "MPSCoreTypes")]
        #[method(dataType)]
        pub unsafe fn dataType(&self) -> MPSDataType;

        #[cfg(feature = "MPSCoreTypes")]
        #[method(setDataType:)]
        pub unsafe fn setDataType(&self, data_type: MPSDataType);

        #[method(numberOfDimensions)]
        pub unsafe fn numberOfDimensions(&self) -> NSUInteger;

        #[method(setNumberOfDimensions:)]
        pub unsafe fn setNumberOfDimensions(&self, number_of_dimensions: NSUInteger);

        #[method(preferPackedRows)]
        pub unsafe fn preferPackedRows(&self) -> bool;

        #[method(setPreferPackedRows:)]
        pub unsafe fn setPreferPackedRows(&self, prefer_packed_rows: bool);

        #[method(lengthOfDimension:)]
        pub unsafe fn lengthOfDimension(&self, dimension_index: NSUInteger) -> NSUInteger;

        #[cfg(feature = "MPSCoreTypes")]
        #[method(sliceRangeForDimension:)]
        pub unsafe fn sliceRangeForDimension(
            &self,
            dimension_index: NSUInteger,
        ) -> MPSDimensionSlice;

        #[cfg(feature = "MPSCoreTypes")]
        #[method(sliceDimension:withSubrange:)]
        pub unsafe fn sliceDimension_withSubrange(
            &self,
            dimension_index: NSUInteger,
            sub_range: MPSDimensionSlice,
        );

        #[method(transposeDimension:withDimension:)]
        pub unsafe fn transposeDimension_withDimension(
            &self,
            dimension_index: NSUInteger,
            dimension_index2: NSUInteger,
        );

        #[method(permuteWithDimensionOrder:)]
        pub unsafe fn permuteWithDimensionOrder(&self, dimension_order: NonNull<NSUInteger>);

        #[method_id(@__retain_semantics Other getShape)]
        pub unsafe fn getShape(&self) -> Retained<NSArray<NSNumber>>;

        #[cfg(feature = "MPSCoreTypes")]
        #[method_id(@__retain_semantics Other descriptorWithDataType:dimensionCount:dimensionSizes:)]
        pub unsafe fn descriptorWithDataType_dimensionCount_dimensionSizes(
            data_type: MPSDataType,
            number_of_dimensions: NSUInteger,
            dimension_sizes: NonNull<NSUInteger>,
        ) -> Retained<Self>;

        #[cfg(feature = "MPSCoreTypes")]
        #[method_id(@__retain_semantics Other descriptorWithDataType:shape:)]
        pub unsafe fn descriptorWithDataType_shape(
            data_type: MPSDataType,
            shape: &NSArray<NSNumber>,
        ) -> Retained<Self>;

        #[method(reshapeWithDimensionCount:dimensionSizes:)]
        pub unsafe fn reshapeWithDimensionCount_dimensionSizes(
            &self,
            number_of_dimensions: NSUInteger,
            dimension_sizes: NonNull<NSUInteger>,
        );

        #[method(reshapeWithShape:)]
        pub unsafe fn reshapeWithShape(&self, shape: &NSArray<NSNumber>);

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPSNDArrayDescriptor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsndarrayallocator?language=objc)
    pub unsafe trait MPSNDArrayAllocator:
        NSCopying + NSObjectProtocol + NSSecureCoding
    {
        #[cfg(feature = "MPSKernel")]
        #[method_id(@__retain_semantics Other arrayForCommandBuffer:arrayDescriptor:kernel:)]
        unsafe fn arrayForCommandBuffer_arrayDescriptor_kernel(
            &self,
            cmd_buf: &ProtocolObject<dyn MTLCommandBuffer>,
            descriptor: &MPSNDArrayDescriptor,
            kernel: &MPSKernel,
        ) -> Retained<MPSNDArray>;
    }

    unsafe impl ProtocolType for dyn MPSNDArrayAllocator {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsndarray?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPSNDArray;
);

unsafe impl NSObjectProtocol for MPSNDArray {}

extern_methods!(
    unsafe impl MPSNDArray {
        #[method_id(@__retain_semantics Other defaultAllocator)]
        pub unsafe fn defaultAllocator() -> Retained<ProtocolObject<dyn MPSNDArrayAllocator>>;

        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Option<Retained<NSString>>;

        #[method(setLabel:)]
        pub unsafe fn setLabel(&self, label: Option<&NSString>);

        #[cfg(feature = "MPSCoreTypes")]
        #[method(dataType)]
        pub unsafe fn dataType(&self) -> MPSDataType;

        #[method(dataTypeSize)]
        pub unsafe fn dataTypeSize(&self) -> usize;

        #[method(numberOfDimensions)]
        pub unsafe fn numberOfDimensions(&self) -> NSUInteger;

        #[method(lengthOfDimension:)]
        pub unsafe fn lengthOfDimension(&self, dimension_index: NSUInteger) -> NSUInteger;

        #[method_id(@__retain_semantics Other device)]
        pub unsafe fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        #[method_id(@__retain_semantics Other descriptor)]
        pub unsafe fn descriptor(&self) -> Retained<MPSNDArrayDescriptor>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:descriptor:)]
        pub unsafe fn initWithDevice_descriptor(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            descriptor: &MPSNDArrayDescriptor,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:scalar:)]
        pub unsafe fn initWithDevice_scalar(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            value: c_double,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithBuffer:offset:descriptor:)]
        pub unsafe fn initWithBuffer_offset_descriptor(
            this: Allocated<Self>,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: NSUInteger,
            descriptor: &MPSNDArrayDescriptor,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other userBuffer)]
        pub unsafe fn userBuffer(&self) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        #[method(resourceSize)]
        pub unsafe fn resourceSize(&self) -> NSUInteger;

        #[cfg(feature = "MPSCoreTypes")]
        #[method_id(@__retain_semantics Other arrayViewWithCommandBuffer:descriptor:aliasing:)]
        pub unsafe fn arrayViewWithCommandBuffer_descriptor_aliasing(
            &self,
            cmd_buf: &ProtocolObject<dyn MTLCommandBuffer>,
            descriptor: &MPSNDArrayDescriptor,
            aliasing: MPSAliasingStrategy,
        ) -> Option<Retained<MPSNDArray>>;

        #[method_id(@__retain_semantics Other arrayViewWithDescriptor:)]
        pub unsafe fn arrayViewWithDescriptor(
            &self,
            descriptor: &MPSNDArrayDescriptor,
        ) -> Option<Retained<MPSNDArray>>;

        #[cfg(feature = "MPSCoreTypes")]
        #[method_id(@__retain_semantics Other arrayViewWithShape:strides:)]
        pub unsafe fn arrayViewWithShape_strides(
            &self,
            shape: Option<&MPSShape>,
            strides: &MPSShape,
        ) -> Option<Retained<MPSNDArray>>;

        #[method_id(@__retain_semantics Other arrayViewWithDimensionCount:dimensionSizes:strides:)]
        pub unsafe fn arrayViewWithDimensionCount_dimensionSizes_strides(
            &self,
            number_of_dimensions: NSUInteger,
            dimension_sizes: NonNull<NSUInteger>,
            dim_strides: NonNull<NSUInteger>,
        ) -> Option<Retained<MPSNDArray>>;

        #[method_id(@__retain_semantics Other parent)]
        pub unsafe fn parent(&self) -> Option<Retained<MPSNDArray>>;

        #[cfg(feature = "MPSCoreTypes")]
        #[method(exportDataWithCommandBuffer:toBuffer:destinationDataType:offset:rowStrides:)]
        pub unsafe fn exportDataWithCommandBuffer_toBuffer_destinationDataType_offset_rowStrides(
            &self,
            cmd_buf: &ProtocolObject<dyn MTLCommandBuffer>,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            destination_data_type: MPSDataType,
            offset: NSUInteger,
            row_strides: *mut NSInteger,
        );

        #[cfg(feature = "MPSCoreTypes")]
        #[method(importDataWithCommandBuffer:fromBuffer:sourceDataType:offset:rowStrides:)]
        pub unsafe fn importDataWithCommandBuffer_fromBuffer_sourceDataType_offset_rowStrides(
            &self,
            cmd_buf: &ProtocolObject<dyn MTLCommandBuffer>,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            source_data_type: MPSDataType,
            offset: NSUInteger,
            row_strides: *mut NSInteger,
        );

        #[cfg(all(feature = "MPSCoreTypes", feature = "MPSImage"))]
        #[method(exportDataWithCommandBuffer:toImages:offset:)]
        pub unsafe fn exportDataWithCommandBuffer_toImages_offset(
            &self,
            cmd_buf: &ProtocolObject<dyn MTLCommandBuffer>,
            images: &MPSImageBatch,
            offset: MPSImageCoordinate,
        );

        #[cfg(all(feature = "MPSCoreTypes", feature = "MPSImage"))]
        #[method(importDataWithCommandBuffer:fromImages:offset:)]
        pub unsafe fn importDataWithCommandBuffer_fromImages_offset(
            &self,
            cmd_buf: &ProtocolObject<dyn MTLCommandBuffer>,
            images: &MPSImageBatch,
            offset: MPSImageCoordinate,
        );

        #[method(readBytes:strideBytes:)]
        pub unsafe fn readBytes_strideBytes(
            &self,
            buffer: NonNull<c_void>,
            stride_bytes_per_dimension: *mut NSInteger,
        );

        #[method(writeBytes:strideBytes:)]
        pub unsafe fn writeBytes_strideBytes(
            &self,
            buffer: NonNull<c_void>,
            stride_bytes_per_dimension: *mut NSInteger,
        );

        #[method(synchronizeOnCommandBuffer:)]
        pub unsafe fn synchronizeOnCommandBuffer(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPSNDArray {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpstemporaryndarray?language=objc)
    #[unsafe(super(MPSNDArray, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPSTemporaryNDArray;
);

unsafe impl NSObjectProtocol for MPSTemporaryNDArray {}

extern_methods!(
    unsafe impl MPSTemporaryNDArray {
        #[method_id(@__retain_semantics Other defaultAllocator)]
        pub unsafe fn defaultAllocator() -> Retained<ProtocolObject<dyn MPSNDArrayAllocator>>;

        #[method_id(@__retain_semantics Other temporaryNDArrayWithCommandBuffer:descriptor:)]
        pub unsafe fn temporaryNDArrayWithCommandBuffer_descriptor(
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            descriptor: &MPSNDArrayDescriptor,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:descriptor:)]
        pub unsafe fn initWithDevice_descriptor(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            descriptor: &MPSNDArrayDescriptor,
        ) -> Retained<Self>;

        #[method(readCount)]
        pub unsafe fn readCount(&self) -> NSUInteger;

        #[method(setReadCount:)]
        pub unsafe fn setReadCount(&self, read_count: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSNDArray`
    unsafe impl MPSTemporaryNDArray {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:scalar:)]
        pub unsafe fn initWithDevice_scalar(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            value: c_double,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithBuffer:offset:descriptor:)]
        pub unsafe fn initWithBuffer_offset_descriptor(
            this: Allocated<Self>,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: NSUInteger,
            descriptor: &MPSNDArrayDescriptor,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPSTemporaryNDArray {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
