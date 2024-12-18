//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-metal-performance-shaders")]
use objc2_metal_performance_shaders::*;

use crate::*;

extern_methods!(
    /// MPSGraphQuantizationOps
    #[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
    unsafe impl MPSGraph {
        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        #[method_id(@__retain_semantics Other quantizeTensor:scale:zeroPoint:dataType:name:)]
        pub unsafe fn quantizeTensor_scale_zeroPoint_dataType_name(
            &self,
            tensor: &MPSGraphTensor,
            scale: c_double,
            zero_point: c_double,
            data_type: MPSDataType,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        #[method_id(@__retain_semantics Other dequantizeTensor:scale:zeroPoint:dataType:name:)]
        pub unsafe fn dequantizeTensor_scale_zeroPoint_dataType_name(
            &self,
            tensor: &MPSGraphTensor,
            scale: c_double,
            zero_point: c_double,
            data_type: MPSDataType,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        #[method_id(@__retain_semantics Other quantizeTensor:scaleTensor:zeroPoint:dataType:axis:name:)]
        pub unsafe fn quantizeTensor_scaleTensor_zeroPoint_dataType_axis_name(
            &self,
            tensor: &MPSGraphTensor,
            scale_tensor: &MPSGraphTensor,
            zero_point: c_double,
            data_type: MPSDataType,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        #[method_id(@__retain_semantics Other dequantizeTensor:scaleTensor:zeroPoint:dataType:axis:name:)]
        pub unsafe fn dequantizeTensor_scaleTensor_zeroPoint_dataType_axis_name(
            &self,
            tensor: &MPSGraphTensor,
            scale_tensor: &MPSGraphTensor,
            zero_point: c_double,
            data_type: MPSDataType,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        #[method_id(@__retain_semantics Other quantizeTensor:scaleTensor:zeroPointTensor:dataType:axis:name:)]
        pub unsafe fn quantizeTensor_scaleTensor_zeroPointTensor_dataType_axis_name(
            &self,
            tensor: &MPSGraphTensor,
            scale_tensor: &MPSGraphTensor,
            zero_point_tensor: &MPSGraphTensor,
            data_type: MPSDataType,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        #[method_id(@__retain_semantics Other dequantizeTensor:scaleTensor:zeroPointTensor:dataType:axis:name:)]
        pub unsafe fn dequantizeTensor_scaleTensor_zeroPointTensor_dataType_axis_name(
            &self,
            tensor: &MPSGraphTensor,
            scale_tensor: &MPSGraphTensor,
            zero_point_tensor: &MPSGraphTensor,
            data_type: MPSDataType,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        #[method_id(@__retain_semantics Other dequantizeTensor:scaleTensor:zeroPointTensor:dataType:name:)]
        pub unsafe fn dequantizeTensor_scaleTensor_zeroPointTensor_dataType_name(
            &self,
            tensor: &MPSGraphTensor,
            scale_tensor: &MPSGraphTensor,
            zero_point_tensor: &MPSGraphTensor,
            data_type: MPSDataType,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        #[method_id(@__retain_semantics Other dequantizeTensor:scaleTensor:dataType:name:)]
        pub unsafe fn dequantizeTensor_scaleTensor_dataType_name(
            &self,
            tensor: &MPSGraphTensor,
            scale_tensor: &MPSGraphTensor,
            data_type: MPSDataType,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other dequantizeTensor:LUTTensor:name:)]
        pub unsafe fn dequantizeTensor_LUTTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            lut_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other dequantizeTensor:LUTTensor:axis:name:)]
        pub unsafe fn dequantizeTensor_LUTTensor_axis_name(
            &self,
            tensor: &MPSGraphTensor,
            lut_tensor: &MPSGraphTensor,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;
    }
);
