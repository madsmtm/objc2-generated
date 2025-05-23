//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// The MLCPoolingDescriptor specifies a pooling descriptor.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcpoolingdescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct MLCPoolingDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MLCPoolingDescriptor {}
);

unsafe impl CopyingHelper for MLCPoolingDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MLCPoolingDescriptor {}
);

impl MLCPoolingDescriptor {
    extern_methods!(
        #[cfg(feature = "MLCTypes")]
        /// The pooling operation
        #[deprecated]
        #[unsafe(method(poolingType))]
        #[unsafe(method_family = none)]
        pub unsafe fn poolingType(&self) -> MLCPoolingType;

        /// The pooling kernel size in x.
        #[deprecated]
        #[unsafe(method(kernelWidth))]
        #[unsafe(method_family = none)]
        pub unsafe fn kernelWidth(&self) -> NSUInteger;

        /// The pooling kernel size in y.
        #[deprecated]
        #[unsafe(method(kernelHeight))]
        #[unsafe(method_family = none)]
        pub unsafe fn kernelHeight(&self) -> NSUInteger;

        /// The stride of the kernel in x.
        #[deprecated]
        #[unsafe(method(strideInX))]
        #[unsafe(method_family = none)]
        pub unsafe fn strideInX(&self) -> NSUInteger;

        /// The stride of the kernel in y.
        #[deprecated]
        #[unsafe(method(strideInY))]
        #[unsafe(method_family = none)]
        pub unsafe fn strideInY(&self) -> NSUInteger;

        /// The dilation rate i.e. stride of elements in the kernel in x.
        #[deprecated]
        #[unsafe(method(dilationRateInX))]
        #[unsafe(method_family = none)]
        pub unsafe fn dilationRateInX(&self) -> NSUInteger;

        /// The dilation rate i.e. stride of elements in the kernel in y.
        #[deprecated]
        #[unsafe(method(dilationRateInY))]
        #[unsafe(method_family = none)]
        pub unsafe fn dilationRateInY(&self) -> NSUInteger;

        #[cfg(feature = "MLCTypes")]
        /// The padding policy to use.
        #[deprecated]
        #[unsafe(method(paddingPolicy))]
        #[unsafe(method_family = none)]
        pub unsafe fn paddingPolicy(&self) -> MLCPaddingPolicy;

        /// The padding size in x (left and right) to use if paddingPolicy is MLCPaddingPolicyUsePaddingSize
        #[deprecated]
        #[unsafe(method(paddingSizeInX))]
        #[unsafe(method_family = none)]
        pub unsafe fn paddingSizeInX(&self) -> NSUInteger;

        /// The padding size in y (top and bottom) to use if paddingPolicy is MLCPaddingPolicyUsePaddingSize
        #[deprecated]
        #[unsafe(method(paddingSizeInY))]
        #[unsafe(method_family = none)]
        pub unsafe fn paddingSizeInY(&self) -> NSUInteger;

        /// Include the zero-padding in the averaging calculation if true.  Used only with average pooling.
        #[deprecated]
        #[unsafe(method(countIncludesPadding))]
        #[unsafe(method_family = none)]
        pub unsafe fn countIncludesPadding(&self) -> bool;

        #[deprecated]
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[deprecated]
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        /// Create a MLCPoolingDescriptor object
        ///
        /// Parameter `poolingType`: The pooling function
        ///
        /// Parameter `kernelSize`: The kernel sizes in x and y
        ///
        /// Parameter `stride`: The kernel strides in x and y
        ///
        /// Returns: A new MLCPoolingDescriptor object.
        #[deprecated]
        #[unsafe(method(poolingDescriptorWithType:kernelSize:stride:))]
        #[unsafe(method_family = none)]
        pub unsafe fn poolingDescriptorWithType_kernelSize_stride(
            pooling_type: MLCPoolingType,
            kernel_size: NSUInteger,
            stride: NSUInteger,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        /// Create a MLCPoolingDescriptor object for a max pooling function
        ///
        /// Parameter `kernelSizes`: The kernel sizes in x and y
        ///
        /// Parameter `strides`: The kernel strides in x and y
        ///
        /// Parameter `paddingPolicy`: The padding policy
        ///
        /// Parameter `paddingSizes`: The padding sizes in x and y if padding policy is MLCPaddingPolicyUsePaddingSIze
        ///
        /// Returns: A new MLCPoolingDescriptor object.
        #[deprecated]
        #[unsafe(method(maxPoolingDescriptorWithKernelSizes:strides:paddingPolicy:paddingSizes:))]
        #[unsafe(method_family = none)]
        pub unsafe fn maxPoolingDescriptorWithKernelSizes_strides_paddingPolicy_paddingSizes(
            kernel_sizes: &NSArray<NSNumber>,
            strides: &NSArray<NSNumber>,
            padding_policy: MLCPaddingPolicy,
            padding_sizes: Option<&NSArray<NSNumber>>,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        /// Create a MLCPoolingDescriptor object for a max pooling function
        ///
        /// Parameter `kernelSizes`: The kernel sizes in x and y
        ///
        /// Parameter `strides`: The kernel strides in x and y
        ///
        /// Parameter `dilationRates`: The kernel dilation rates in x and y
        ///
        /// Parameter `paddingPolicy`: The padding policy
        ///
        /// Parameter `paddingSizes`: The padding sizes in x and y if padding policy is MLCPaddingPolicyUsePaddingSIze
        ///
        /// Returns: A new MLCPoolingDescriptor object.
        #[deprecated]
        #[unsafe(method(maxPoolingDescriptorWithKernelSizes:strides:dilationRates:paddingPolicy:paddingSizes:))]
        #[unsafe(method_family = none)]
        pub unsafe fn maxPoolingDescriptorWithKernelSizes_strides_dilationRates_paddingPolicy_paddingSizes(
            kernel_sizes: &NSArray<NSNumber>,
            strides: &NSArray<NSNumber>,
            dilation_rates: &NSArray<NSNumber>,
            padding_policy: MLCPaddingPolicy,
            padding_sizes: Option<&NSArray<NSNumber>>,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        /// Create a MLCPoolingDescriptor object for an average pooling function
        ///
        /// Parameter `kernelSizes`: The kernel sizes in x and y
        ///
        /// Parameter `strides`: The kernel strides in x and y
        ///
        /// Parameter `paddingPolicy`: The padding policy
        ///
        /// Parameter `paddingSizes`: The padding sizes in x and y if padding policy is MLCPaddingPolicyUsePaddingSIze
        ///
        /// Parameter `countIncludesPadding`: Whether to include zero padding in the averaging calculation
        ///
        /// Returns: A new MLCPoolingDescriptor object.
        #[deprecated]
        #[unsafe(method(averagePoolingDescriptorWithKernelSizes:strides:paddingPolicy:paddingSizes:countIncludesPadding:))]
        #[unsafe(method_family = none)]
        pub unsafe fn averagePoolingDescriptorWithKernelSizes_strides_paddingPolicy_paddingSizes_countIncludesPadding(
            kernel_sizes: &NSArray<NSNumber>,
            strides: &NSArray<NSNumber>,
            padding_policy: MLCPaddingPolicy,
            padding_sizes: Option<&NSArray<NSNumber>>,
            count_includes_padding: bool,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        /// Create a MLCPoolingDescriptor object for an average pooling function
        ///
        /// Parameter `kernelSizes`: The kernel sizes in x and y
        ///
        /// Parameter `strides`: The kernel strides in x and y
        ///
        /// Parameter `dilationRates`: The kernel dilation rates in x and y
        ///
        /// Parameter `paddingPolicy`: The padding policy
        ///
        /// Parameter `paddingSizes`: The padding sizes in x and y if padding policy is MLCPaddingPolicyUsePaddingSIze
        ///
        /// Parameter `countIncludesPadding`: Whether to include zero padding in the averaging calculation
        ///
        /// Returns: A new MLCPoolingDescriptor object.
        #[deprecated]
        #[unsafe(method(averagePoolingDescriptorWithKernelSizes:strides:dilationRates:paddingPolicy:paddingSizes:countIncludesPadding:))]
        #[unsafe(method_family = none)]
        pub unsafe fn averagePoolingDescriptorWithKernelSizes_strides_dilationRates_paddingPolicy_paddingSizes_countIncludesPadding(
            kernel_sizes: &NSArray<NSNumber>,
            strides: &NSArray<NSNumber>,
            dilation_rates: &NSArray<NSNumber>,
            padding_policy: MLCPaddingPolicy,
            padding_sizes: Option<&NSArray<NSNumber>>,
            count_includes_padding: bool,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        /// Create a MLCPoolingDescriptor object for a L2 norm pooling function
        ///
        /// Parameter `kernelSizes`: The kernel sizes in x and y
        ///
        /// Parameter `strides`: The kernel strides in x and y
        ///
        /// Parameter `paddingPolicy`: The padding policy
        ///
        /// Parameter `paddingSizes`: The padding sizes in x and y if padding policy is MLCPaddingPolicyUsePaddingSIze
        ///
        /// Returns: A new MLCPoolingDescriptor object.
        #[deprecated]
        #[unsafe(method(l2NormPoolingDescriptorWithKernelSizes:strides:paddingPolicy:paddingSizes:))]
        #[unsafe(method_family = none)]
        pub unsafe fn l2NormPoolingDescriptorWithKernelSizes_strides_paddingPolicy_paddingSizes(
            kernel_sizes: &NSArray<NSNumber>,
            strides: &NSArray<NSNumber>,
            padding_policy: MLCPaddingPolicy,
            padding_sizes: Option<&NSArray<NSNumber>>,
        ) -> Retained<Self>;

        #[cfg(feature = "MLCTypes")]
        /// Create a MLCPoolingDescriptor object for a L2 norm pooling function
        ///
        /// Parameter `kernelSizes`: The kernel sizes in x and y
        ///
        /// Parameter `strides`: The kernel strides in x and y
        ///
        /// Parameter `dilationRates`: The kernel dilation rates in x and y
        ///
        /// Parameter `paddingPolicy`: The padding policy
        ///
        /// Parameter `paddingSizes`: The padding sizes in x and y if padding policy is MLCPaddingPolicyUsePaddingSIze
        ///
        /// Returns: A new MLCPoolingDescriptor object.
        #[deprecated]
        #[unsafe(method(l2NormPoolingDescriptorWithKernelSizes:strides:dilationRates:paddingPolicy:paddingSizes:))]
        #[unsafe(method_family = none)]
        pub unsafe fn l2NormPoolingDescriptorWithKernelSizes_strides_dilationRates_paddingPolicy_paddingSizes(
            kernel_sizes: &NSArray<NSNumber>,
            strides: &NSArray<NSNumber>,
            dilation_rates: &NSArray<NSNumber>,
            padding_policy: MLCPaddingPolicy,
            padding_sizes: Option<&NSArray<NSNumber>>,
        ) -> Retained<Self>;
    );
}
