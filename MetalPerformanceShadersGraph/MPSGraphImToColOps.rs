//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-metal-performance-shaders")]
use objc2_metal_performance_shaders::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshadersgraph/mpsgraphimtocolopdescriptor?language=objc)
    #[unsafe(super(MPSGraphObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSGraphCore")]
    pub struct MPSGraphImToColOpDescriptor;
);

#[cfg(feature = "MPSGraphCore")]
unsafe impl NSCopying for MPSGraphImToColOpDescriptor {}

#[cfg(feature = "MPSGraphCore")]
unsafe impl CopyingHelper for MPSGraphImToColOpDescriptor {
    type Result = Self;
}

#[cfg(feature = "MPSGraphCore")]
unsafe impl NSObjectProtocol for MPSGraphImToColOpDescriptor {}

extern_methods!(
    #[cfg(feature = "MPSGraphCore")]
    unsafe impl MPSGraphImToColOpDescriptor {
        #[method(kernelWidth)]
        pub unsafe fn kernelWidth(&self) -> NSUInteger;

        #[method(setKernelWidth:)]
        pub unsafe fn setKernelWidth(&self, kernel_width: NSUInteger);

        #[method(kernelHeight)]
        pub unsafe fn kernelHeight(&self) -> NSUInteger;

        #[method(setKernelHeight:)]
        pub unsafe fn setKernelHeight(&self, kernel_height: NSUInteger);

        #[method(strideInX)]
        pub unsafe fn strideInX(&self) -> NSUInteger;

        #[method(setStrideInX:)]
        pub unsafe fn setStrideInX(&self, stride_in_x: NSUInteger);

        #[method(strideInY)]
        pub unsafe fn strideInY(&self) -> NSUInteger;

        #[method(setStrideInY:)]
        pub unsafe fn setStrideInY(&self, stride_in_y: NSUInteger);

        #[method(dilationRateInX)]
        pub unsafe fn dilationRateInX(&self) -> NSUInteger;

        #[method(setDilationRateInX:)]
        pub unsafe fn setDilationRateInX(&self, dilation_rate_in_x: NSUInteger);

        #[method(dilationRateInY)]
        pub unsafe fn dilationRateInY(&self) -> NSUInteger;

        #[method(setDilationRateInY:)]
        pub unsafe fn setDilationRateInY(&self, dilation_rate_in_y: NSUInteger);

        #[method(paddingLeft)]
        pub unsafe fn paddingLeft(&self) -> NSUInteger;

        #[method(setPaddingLeft:)]
        pub unsafe fn setPaddingLeft(&self, padding_left: NSUInteger);

        #[method(paddingRight)]
        pub unsafe fn paddingRight(&self) -> NSUInteger;

        #[method(setPaddingRight:)]
        pub unsafe fn setPaddingRight(&self, padding_right: NSUInteger);

        #[method(paddingTop)]
        pub unsafe fn paddingTop(&self) -> NSUInteger;

        #[method(setPaddingTop:)]
        pub unsafe fn setPaddingTop(&self, padding_top: NSUInteger);

        #[method(paddingBottom)]
        pub unsafe fn paddingBottom(&self) -> NSUInteger;

        #[method(setPaddingBottom:)]
        pub unsafe fn setPaddingBottom(&self, padding_bottom: NSUInteger);

        #[method(dataLayout)]
        pub unsafe fn dataLayout(&self) -> MPSGraphTensorNamedDataLayout;

        #[method(setDataLayout:)]
        pub unsafe fn setDataLayout(&self, data_layout: MPSGraphTensorNamedDataLayout);

        #[method_id(@__retain_semantics Other descriptorWithKernelWidth:kernelHeight:strideInX:strideInY:dilationRateInX:dilationRateInY:paddingLeft:paddingRight:paddingTop:paddingBottom:dataLayout:)]
        pub unsafe fn descriptorWithKernelWidth_kernelHeight_strideInX_strideInY_dilationRateInX_dilationRateInY_paddingLeft_paddingRight_paddingTop_paddingBottom_dataLayout(
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
            stride_in_x: NSUInteger,
            stride_in_y: NSUInteger,
            dilation_rate_in_x: NSUInteger,
            dilation_rate_in_y: NSUInteger,
            padding_left: NSUInteger,
            padding_right: NSUInteger,
            padding_top: NSUInteger,
            padding_bottom: NSUInteger,
            data_layout: MPSGraphTensorNamedDataLayout,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other descriptorWithKernelWidth:kernelHeight:strideInX:strideInY:dilationRateInX:dilationRateInY:dataLayout:)]
        pub unsafe fn descriptorWithKernelWidth_kernelHeight_strideInX_strideInY_dilationRateInX_dilationRateInY_dataLayout(
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
            stride_in_x: NSUInteger,
            stride_in_y: NSUInteger,
            dilation_rate_in_x: NSUInteger,
            dilation_rate_in_y: NSUInteger,
            data_layout: MPSGraphTensorNamedDataLayout,
        ) -> Option<Retained<Self>>;

        #[method(setExplicitPaddingWithPaddingLeft:paddingRight:paddingTop:paddingBottom:)]
        pub unsafe fn setExplicitPaddingWithPaddingLeft_paddingRight_paddingTop_paddingBottom(
            &self,
            padding_left: NSUInteger,
            padding_right: NSUInteger,
            padding_top: NSUInteger,
            padding_bottom: NSUInteger,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MPSGraphCore")]
    unsafe impl MPSGraphImToColOpDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// MPSGraphImToColOps
    #[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
    unsafe impl MPSGraph {
        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other imToColWithSourceTensor:descriptor:name:)]
        pub unsafe fn imToColWithSourceTensor_descriptor_name(
            &self,
            source: &MPSGraphTensor,
            descriptor: &MPSGraphImToColOpDescriptor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        #[method_id(@__retain_semantics Other colToImWithSourceTensor:outputShape:descriptor:name:)]
        pub unsafe fn colToImWithSourceTensor_outputShape_descriptor_name(
            &self,
            source: &MPSGraphTensor,
            output_shape: &MPSShape,
            descriptor: &MPSGraphImToColOpDescriptor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;
    }
);
