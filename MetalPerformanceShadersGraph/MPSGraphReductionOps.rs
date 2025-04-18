//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// MPSGraphReductionOps.
#[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
impl MPSGraph {
    extern_methods!(
        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a reduction sum operation and returns the result tensor.
        ///
        /// - Parameters:
        /// - tensor: input tensor
        /// - axis: axis of reduction
        /// - name: name for the operation
        /// - Returns: A valid MPSGraphTensor object.
        #[unsafe(method(reductionSumWithTensor:axis:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn reductionSumWithTensor_axis_name(
            &self,
            tensor: &MPSGraphTensor,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a reduction sum operation and returns the result tensor.
        ///
        /// - Parameters:
        /// - tensor: input tensor
        /// - axes: axes of reduction
        /// - name: name for the operation
        /// - Returns: A valid MPSGraphTensor object.
        #[unsafe(method(reductionSumWithTensor:axes:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn reductionSumWithTensor_axes_name(
            &self,
            tensor: &MPSGraphTensor,
            axes: Option<&NSArray<NSNumber>>,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a reduction max operation and returns the result tensor.
        ///
        /// - Parameters:
        /// - tensor: input tensor
        /// - axis: axis of reduction
        /// - name: name for the operation
        /// - Returns: A valid MPSGraphTensor object.
        #[unsafe(method(reductionMaximumWithTensor:axis:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn reductionMaximumWithTensor_axis_name(
            &self,
            tensor: &MPSGraphTensor,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a reduction max operation and returns the result tensor.
        ///
        /// - Parameters:
        /// - tensor: input tensor
        /// - axes: axes of reduction
        /// - name: name for the operation
        /// - Returns: A valid MPSGraphTensor object.
        #[unsafe(method(reductionMaximumWithTensor:axes:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn reductionMaximumWithTensor_axes_name(
            &self,
            tensor: &MPSGraphTensor,
            axes: Option<&NSArray<NSNumber>>,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a reduction minimum operation and returns the result tensor.
        ///
        /// - Parameters:
        /// - tensor: input tensor
        /// - axis: axis of reduction
        /// - name: name for the operation
        /// - Returns: A valid MPSGraphTensor object.
        #[unsafe(method(reductionMinimumWithTensor:axis:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn reductionMinimumWithTensor_axis_name(
            &self,
            tensor: &MPSGraphTensor,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a reduction min operation and returns the result tensor.
        ///
        /// - Parameters:
        /// - tensor: input tensor
        /// - axes: axes of reduction
        /// - name: name for the operation
        /// - Returns: A valid MPSGraphTensor object.
        #[unsafe(method(reductionMinimumWithTensor:axes:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn reductionMinimumWithTensor_axes_name(
            &self,
            tensor: &MPSGraphTensor,
            axes: Option<&NSArray<NSNumber>>,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a reduction max propagate NaN operation and returns the result tensor.
        ///
        /// - Parameters:
        /// - tensor: input tensor
        /// - axis: axis of reduction
        /// - name: name for the operation
        /// - Returns: A valid MPSGraphTensor object.
        #[unsafe(method(reductionMaximumPropagateNaNWithTensor:axis:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn reductionMaximumPropagateNaNWithTensor_axis_name(
            &self,
            tensor: &MPSGraphTensor,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a reduction max propagate NaN operation and returns the result tensor.
        ///
        /// - Parameters:
        /// - tensor: input tensor
        /// - axes: axes of reduction
        /// - name: name for the operation
        /// - Returns: A valid MPSGraphTensor object.
        #[unsafe(method(reductionMaximumPropagateNaNWithTensor:axes:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn reductionMaximumPropagateNaNWithTensor_axes_name(
            &self,
            tensor: &MPSGraphTensor,
            axes: Option<&NSArray<NSNumber>>,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a reduction min propagate NaN operation and returns the result tensor.
        ///
        /// - Parameters:
        /// - tensor: input tensor
        /// - axis: axis of reduction
        /// - name: name for the operation
        /// - Returns: A valid MPSGraphTensor object.
        #[unsafe(method(reductionMinimumPropagateNaNWithTensor:axis:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn reductionMinimumPropagateNaNWithTensor_axis_name(
            &self,
            tensor: &MPSGraphTensor,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a reduction min propagate NaN operation and returns the result tensor.
        ///
        /// - Parameters:
        /// - tensor: input tensor
        /// - axes: axes of reduction
        /// - name: name for the operation
        /// - Returns: A valid MPSGraphTensor object.
        #[unsafe(method(reductionMinimumPropagateNaNWithTensor:axes:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn reductionMinimumPropagateNaNWithTensor_axes_name(
            &self,
            tensor: &MPSGraphTensor,
            axes: Option<&NSArray<NSNumber>>,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a reduction product operation and returns the result tensor.
        ///
        /// - Parameters:
        /// - tensor: input tensor
        /// - axis: axis of reduction
        /// - name: name for the operation
        /// - Returns: A valid MPSGraphTensor object.
        #[unsafe(method(reductionProductWithTensor:axis:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn reductionProductWithTensor_axis_name(
            &self,
            tensor: &MPSGraphTensor,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a reduction product operation and returns the result tensor.
        ///
        /// - Parameters:
        /// - tensor: input tensor
        /// - axes: axes of reduction
        /// - name: name for the operation
        /// - Returns: A valid MPSGraphTensor object.
        #[unsafe(method(reductionProductWithTensor:axes:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn reductionProductWithTensor_axes_name(
            &self,
            tensor: &MPSGraphTensor,
            axes: Option<&NSArray<NSNumber>>,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a reduction argMax operation and returns the result tensor.
        ///
        /// - Parameters:
        /// - tensor: input tensor
        /// - axis: axis of reduction
        /// - name: name for the operation
        /// - Returns: A valid MPSGraphTensor object.
        #[unsafe(method(reductionArgMaximumWithTensor:axis:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn reductionArgMaximumWithTensor_axis_name(
            &self,
            tensor: &MPSGraphTensor,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a reduction argMin operation and returns the result tensor.
        ///
        /// - Parameters:
        /// - tensor: input tensor
        /// - axis: axis of reduction
        /// - name: name for the operation
        /// - Returns: A valid MPSGraphTensor object.
        #[unsafe(method(reductionArgMinimumWithTensor:axis:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn reductionArgMinimumWithTensor_axis_name(
            &self,
            tensor: &MPSGraphTensor,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a reduction and operation and returns the result tensor.
        ///
        /// - Parameters:
        /// - tensor: input tensor
        /// - axis: axis of reduction
        /// - name: name for the operation
        /// - Returns: A valid MPSGraphTensor object.
        #[unsafe(method(reductionAndWithTensor:axis:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn reductionAndWithTensor_axis_name(
            &self,
            tensor: &MPSGraphTensor,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a reduction and operation and returns the result tensor.
        ///
        /// - Parameters:
        /// - tensor: input tensor
        /// - axes: axes of reduction
        /// - name: name for the operation
        /// - Returns: A valid MPSGraphTensor object.
        #[unsafe(method(reductionAndWithTensor:axes:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn reductionAndWithTensor_axes_name(
            &self,
            tensor: &MPSGraphTensor,
            axes: Option<&NSArray<NSNumber>>,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a reduction or operation and returns the result tensor.
        ///
        /// - Parameters:
        /// - tensor: input tensor
        /// - axis: axis of reduction
        /// - name: name for the operation
        /// - Returns: A valid MPSGraphTensor object.
        #[unsafe(method(reductionOrWithTensor:axis:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn reductionOrWithTensor_axis_name(
            &self,
            tensor: &MPSGraphTensor,
            axis: NSInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a reduction or operation and returns the result tensor.
        ///
        /// - Parameters:
        /// - tensor: input tensor
        /// - axes: axes of reduction
        /// - name: name for the operation
        /// - Returns: A valid MPSGraphTensor object.
        #[unsafe(method(reductionOrWithTensor:axes:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn reductionOrWithTensor_axes_name(
            &self,
            tensor: &MPSGraphTensor,
            axes: Option<&NSArray<NSNumber>>,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;
    );
}
