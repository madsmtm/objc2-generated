//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_methods!(
    /// MPSGraphReductionOps
    #[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
    unsafe impl MPSGraph {
        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other reductionSumWithTensor:axis:name:)]
        pub unsafe fn reductionSumWithTensor_axis_name(
            &self,
            tensor: &MPSGraphTensor,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other reductionSumWithTensor:axes:name:)]
        pub unsafe fn reductionSumWithTensor_axes_name(
            &self,
            tensor: &MPSGraphTensor,
            axes: Option<&NSArray<NSNumber>>,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other reductionMaximumWithTensor:axis:name:)]
        pub unsafe fn reductionMaximumWithTensor_axis_name(
            &self,
            tensor: &MPSGraphTensor,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other reductionMaximumWithTensor:axes:name:)]
        pub unsafe fn reductionMaximumWithTensor_axes_name(
            &self,
            tensor: &MPSGraphTensor,
            axes: Option<&NSArray<NSNumber>>,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other reductionMinimumWithTensor:axis:name:)]
        pub unsafe fn reductionMinimumWithTensor_axis_name(
            &self,
            tensor: &MPSGraphTensor,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other reductionMinimumWithTensor:axes:name:)]
        pub unsafe fn reductionMinimumWithTensor_axes_name(
            &self,
            tensor: &MPSGraphTensor,
            axes: Option<&NSArray<NSNumber>>,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other reductionMaximumPropagateNaNWithTensor:axis:name:)]
        pub unsafe fn reductionMaximumPropagateNaNWithTensor_axis_name(
            &self,
            tensor: &MPSGraphTensor,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other reductionMaximumPropagateNaNWithTensor:axes:name:)]
        pub unsafe fn reductionMaximumPropagateNaNWithTensor_axes_name(
            &self,
            tensor: &MPSGraphTensor,
            axes: Option<&NSArray<NSNumber>>,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other reductionMinimumPropagateNaNWithTensor:axis:name:)]
        pub unsafe fn reductionMinimumPropagateNaNWithTensor_axis_name(
            &self,
            tensor: &MPSGraphTensor,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other reductionMinimumPropagateNaNWithTensor:axes:name:)]
        pub unsafe fn reductionMinimumPropagateNaNWithTensor_axes_name(
            &self,
            tensor: &MPSGraphTensor,
            axes: Option<&NSArray<NSNumber>>,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other reductionProductWithTensor:axis:name:)]
        pub unsafe fn reductionProductWithTensor_axis_name(
            &self,
            tensor: &MPSGraphTensor,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other reductionProductWithTensor:axes:name:)]
        pub unsafe fn reductionProductWithTensor_axes_name(
            &self,
            tensor: &MPSGraphTensor,
            axes: Option<&NSArray<NSNumber>>,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other reductionArgMaximumWithTensor:axis:name:)]
        pub unsafe fn reductionArgMaximumWithTensor_axis_name(
            &self,
            tensor: &MPSGraphTensor,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other reductionArgMinimumWithTensor:axis:name:)]
        pub unsafe fn reductionArgMinimumWithTensor_axis_name(
            &self,
            tensor: &MPSGraphTensor,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other reductionAndWithTensor:axis:name:)]
        pub unsafe fn reductionAndWithTensor_axis_name(
            &self,
            tensor: &MPSGraphTensor,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other reductionAndWithTensor:axes:name:)]
        pub unsafe fn reductionAndWithTensor_axes_name(
            &self,
            tensor: &MPSGraphTensor,
            axes: Option<&NSArray<NSNumber>>,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other reductionOrWithTensor:axis:name:)]
        pub unsafe fn reductionOrWithTensor_axis_name(
            &self,
            tensor: &MPSGraphTensor,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other reductionOrWithTensor:axes:name:)]
        pub unsafe fn reductionOrWithTensor_axes_name(
            &self,
            tensor: &MPSGraphTensor,
            axes: Option<&NSArray<NSNumber>>,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;
    }
);
