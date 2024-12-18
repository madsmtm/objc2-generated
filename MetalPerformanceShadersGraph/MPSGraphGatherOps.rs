//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_methods!(
    /// GatherNDOps
    #[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
    unsafe impl MPSGraph {
        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other gatherNDWithUpdatesTensor:indicesTensor:batchDimensions:name:)]
        pub unsafe fn gatherNDWithUpdatesTensor_indicesTensor_batchDimensions_name(
            &self,
            updates_tensor: &MPSGraphTensor,
            indices_tensor: &MPSGraphTensor,
            batch_dimensions: NSUInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;
    }
);

extern_methods!(
    /// GatherOps
    #[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
    unsafe impl MPSGraph {
        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other gatherWithUpdatesTensor:indicesTensor:axis:batchDimensions:name:)]
        pub unsafe fn gatherWithUpdatesTensor_indicesTensor_axis_batchDimensions_name(
            &self,
            updates_tensor: &MPSGraphTensor,
            indices_tensor: &MPSGraphTensor,
            axis: NSUInteger,
            batch_dimensions: NSUInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;
    }
);

extern_methods!(
    /// MPSGraphGatherAlongAxisOps
    #[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
    unsafe impl MPSGraph {
        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other gatherAlongAxis:withUpdatesTensor:indicesTensor:name:)]
        pub unsafe fn gatherAlongAxis_withUpdatesTensor_indicesTensor_name(
            &self,
            axis: NSInteger,
            updates_tensor: &MPSGraphTensor,
            indices_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other gatherAlongAxisTensor:withUpdatesTensor:indicesTensor:name:)]
        pub unsafe fn gatherAlongAxisTensor_withUpdatesTensor_indicesTensor_name(
            &self,
            axis_tensor: &MPSGraphTensor,
            updates_tensor: &MPSGraphTensor,
            indices_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;
    }
);
