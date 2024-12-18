//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_methods!(
    /// MPSGraphLinearAlgebraOps
    #[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
    unsafe impl MPSGraph {
        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other bandPartWithTensor:numLower:numUpper:name:)]
        pub unsafe fn bandPartWithTensor_numLower_numUpper_name(
            &self,
            input_tensor: &MPSGraphTensor,
            num_lower: NSInteger,
            num_upper: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other bandPartWithTensor:numLowerTensor:numUpperTensor:name:)]
        pub unsafe fn bandPartWithTensor_numLowerTensor_numUpperTensor_name(
            &self,
            input_tensor: &MPSGraphTensor,
            num_lower_tensor: &MPSGraphTensor,
            num_upper_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;
    }
);
