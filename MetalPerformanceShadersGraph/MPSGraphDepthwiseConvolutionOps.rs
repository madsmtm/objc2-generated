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
    /// A class that defines the parameters for  a 2D-depthwise convolution operation.
    ///
    /// An `MPSGraphDepthwiseConvolution2DOpDescriptor` defines constant parameters for 2D-depthwise convolutions.
    /// Use this class with ``MPSGraph/depthwiseConvolution2DWithSourceTensor:weightsTensor:descriptor:name:``,
    /// ``MPSGraph/depthwiseConvolution2DDataGradientWithIncomingGradientTensor:weightsTensor:outputShape:descriptor:name:``,
    /// and ``MPSGraph/depthwiseConvolution2DWeightsGradientWithIncomingGradientTensor:sourceTensor:outputShape:descriptor:name:``
    /// methods.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshadersgraph/mpsgraphdepthwiseconvolution2dopdescriptor?language=objc)
    #[unsafe(super(MPSGraphObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSGraphCore")]
    pub struct MPSGraphDepthwiseConvolution2DOpDescriptor;
);

#[cfg(feature = "MPSGraphCore")]
extern_conformance!(
    unsafe impl NSCopying for MPSGraphDepthwiseConvolution2DOpDescriptor {}
);

#[cfg(feature = "MPSGraphCore")]
unsafe impl CopyingHelper for MPSGraphDepthwiseConvolution2DOpDescriptor {
    type Result = Self;
}

#[cfg(feature = "MPSGraphCore")]
extern_conformance!(
    unsafe impl NSObjectProtocol for MPSGraphDepthwiseConvolution2DOpDescriptor {}
);

#[cfg(feature = "MPSGraphCore")]
impl MPSGraphDepthwiseConvolution2DOpDescriptor {
    extern_methods!(
        /// The stride for the x dimension.
        ///
        /// Default value: 1.
        #[unsafe(method(strideInX))]
        #[unsafe(method_family = none)]
        pub unsafe fn strideInX(&self) -> NSUInteger;

        /// Setter for [`strideInX`][Self::strideInX].
        #[unsafe(method(setStrideInX:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setStrideInX(&self, stride_in_x: NSUInteger);

        /// The stride for the y dimension.
        ///
        /// Default value: 1.
        #[unsafe(method(strideInY))]
        #[unsafe(method_family = none)]
        pub unsafe fn strideInY(&self) -> NSUInteger;

        /// Setter for [`strideInY`][Self::strideInY].
        #[unsafe(method(setStrideInY:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setStrideInY(&self, stride_in_y: NSUInteger);

        /// The dilation rate for the x dimension.
        ///
        /// Default value: 1.
        #[unsafe(method(dilationRateInX))]
        #[unsafe(method_family = none)]
        pub unsafe fn dilationRateInX(&self) -> NSUInteger;

        /// Setter for [`dilationRateInX`][Self::dilationRateInX].
        #[unsafe(method(setDilationRateInX:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDilationRateInX(&self, dilation_rate_in_x: NSUInteger);

        /// The dilation rate for the y dimension.
        ///
        /// Default value: 1.
        #[unsafe(method(dilationRateInY))]
        #[unsafe(method_family = none)]
        pub unsafe fn dilationRateInY(&self) -> NSUInteger;

        /// Setter for [`dilationRateInY`][Self::dilationRateInY].
        #[unsafe(method(setDilationRateInY:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDilationRateInY(&self, dilation_rate_in_y: NSUInteger);

        /// The explicit padding value for the x dimension the operation adds before the data.
        ///
        /// Default value: 0.
        #[unsafe(method(paddingLeft))]
        #[unsafe(method_family = none)]
        pub unsafe fn paddingLeft(&self) -> NSUInteger;

        /// Setter for [`paddingLeft`][Self::paddingLeft].
        #[unsafe(method(setPaddingLeft:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPaddingLeft(&self, padding_left: NSUInteger);

        /// The explicit padding value for the x dimension operation adds after the data.
        ///
        /// Default value: 0.
        #[unsafe(method(paddingRight))]
        #[unsafe(method_family = none)]
        pub unsafe fn paddingRight(&self) -> NSUInteger;

        /// Setter for [`paddingRight`][Self::paddingRight].
        #[unsafe(method(setPaddingRight:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPaddingRight(&self, padding_right: NSUInteger);

        /// The explicit padding value for the y dimension operation adds before the data.
        ///
        /// Default value: 0.
        #[unsafe(method(paddingTop))]
        #[unsafe(method_family = none)]
        pub unsafe fn paddingTop(&self) -> NSUInteger;

        /// Setter for [`paddingTop`][Self::paddingTop].
        #[unsafe(method(setPaddingTop:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPaddingTop(&self, padding_top: NSUInteger);

        /// The explicit padding value for the y dimension operation adds after the data.
        ///
        /// Default value: 0.
        #[unsafe(method(paddingBottom))]
        #[unsafe(method_family = none)]
        pub unsafe fn paddingBottom(&self) -> NSUInteger;

        /// Setter for [`paddingBottom`][Self::paddingBottom].
        #[unsafe(method(setPaddingBottom:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPaddingBottom(&self, padding_bottom: NSUInteger);

        /// The padding style for the operation.
        ///
        /// Default value is `MPSGraphPaddingStyleExplicit`.
        #[unsafe(method(paddingStyle))]
        #[unsafe(method_family = none)]
        pub unsafe fn paddingStyle(&self) -> MPSGraphPaddingStyle;

        /// Setter for [`paddingStyle`][Self::paddingStyle].
        #[unsafe(method(setPaddingStyle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPaddingStyle(&self, padding_style: MPSGraphPaddingStyle);

        /// The data layout of the input data in the forward pass.
        ///
        /// See: ``MPSGraphTensorNamedDataLayout``.
        #[unsafe(method(dataLayout))]
        #[unsafe(method_family = none)]
        pub unsafe fn dataLayout(&self) -> MPSGraphTensorNamedDataLayout;

        /// Setter for [`dataLayout`][Self::dataLayout].
        #[unsafe(method(setDataLayout:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDataLayout(&self, data_layout: MPSGraphTensorNamedDataLayout);

        /// The data layout of the weights.
        ///
        /// NOTE: 'O' index is channel multiplier index. See: ``MPSGraphTensorNamedDataLayout``.
        #[unsafe(method(weightsLayout))]
        #[unsafe(method_family = none)]
        pub unsafe fn weightsLayout(&self) -> MPSGraphTensorNamedDataLayout;

        /// Setter for [`weightsLayout`][Self::weightsLayout].
        #[unsafe(method(setWeightsLayout:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setWeightsLayout(&self, weights_layout: MPSGraphTensorNamedDataLayout);

        /// Creates a 2D-depthwise convolution descriptor with given values.
        ///
        /// - Parameters:
        /// - strideInX: See `strideInX` property.
        /// - strideInY: See `strideInY` property.
        /// - dilationRateInX: See `dilationRateInX` property.
        /// - dilationRateInY: See `dilationRateInY` property.
        /// - paddingLeft: See `paddingLeft` property.
        /// - paddingRight: See `paddingRight` property.
        /// - paddingTop: See `paddingTop` property.
        /// - paddingBottom: See `paddingBottom` property.
        /// - paddingStyle: See `paddingStyle` property.
        /// - dataLayout: See `dataLayout` property.
        /// - weightsLayout: See `weightsLayout` property.
        /// - Returns: The descriptor on autoreleasepool.
        #[unsafe(method(descriptorWithStrideInX:strideInY:dilationRateInX:dilationRateInY:paddingLeft:paddingRight:paddingTop:paddingBottom:paddingStyle:dataLayout:weightsLayout:))]
        #[unsafe(method_family = none)]
        pub unsafe fn descriptorWithStrideInX_strideInY_dilationRateInX_dilationRateInY_paddingLeft_paddingRight_paddingTop_paddingBottom_paddingStyle_dataLayout_weightsLayout(
            stride_in_x: NSUInteger,
            stride_in_y: NSUInteger,
            dilation_rate_in_x: NSUInteger,
            dilation_rate_in_y: NSUInteger,
            padding_left: NSUInteger,
            padding_right: NSUInteger,
            padding_top: NSUInteger,
            padding_bottom: NSUInteger,
            padding_style: MPSGraphPaddingStyle,
            data_layout: MPSGraphTensorNamedDataLayout,
            weights_layout: MPSGraphTensorNamedDataLayout,
        ) -> Option<Retained<Self>>;

        /// Creates a 2D-depthwise convolution descriptor with given properties and default values.
        ///
        /// - Parameters:
        /// - dataLayout: See `dataLayout` property.
        /// - weightsLayout: See `weightsLayout` property.
        /// - Returns: The descriptor on autoreleasepool.
        #[unsafe(method(descriptorWithDataLayout:weightsLayout:))]
        #[unsafe(method_family = none)]
        pub unsafe fn descriptorWithDataLayout_weightsLayout(
            data_layout: MPSGraphTensorNamedDataLayout,
            weights_layout: MPSGraphTensorNamedDataLayout,
        ) -> Option<Retained<Self>>;

        /// Sets the explicit padding values.
        ///
        /// Note: this method also sets `paddingStyle` to `MPSGraphPaddingStyleExplicit` (see ``MPSGraphPaddingStyle``).
        ///
        /// - Parameters:
        /// - paddingLeft: See `paddingLeft` property.
        /// - paddingRight: See `paddingRight` property.
        /// - paddingTop: See `paddingTop` property.
        /// - paddingBottom: See `paddingBottom` property.
        #[unsafe(method(setExplicitPaddingWithPaddingLeft:paddingRight:paddingTop:paddingBottom:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setExplicitPaddingWithPaddingLeft_paddingRight_paddingTop_paddingBottom(
            &self,
            padding_left: NSUInteger,
            padding_right: NSUInteger,
            padding_top: NSUInteger,
            padding_bottom: NSUInteger,
        );
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "MPSGraphCore")]
impl MPSGraphDepthwiseConvolution2DOpDescriptor {
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
    /// The class that defines the parameters for a 3D-depthwise convolution operation.
    ///
    /// A `MPSGraphDepthwiseConvolution3DOpDescriptor` defines constant parameters for 3D depthwise convolutions.
    /// Use this class with ``MPSGraph/depthwiseConvolution3DWithSourceTensor:weightsTensor:descriptor:name:``,
    /// ``MPSGraph/depthwiseConvolution3DDataGradientWithIncomingGradientTensor:weightsTensor:outputShape:descriptor:name:``
    /// and ``MPSGraph/depthwiseConvolution3DWeightsGradientWithIncomingGradientTensor:sourceTensor:outputShape:descriptor:name:``
    /// methods.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshadersgraph/mpsgraphdepthwiseconvolution3dopdescriptor?language=objc)
    #[unsafe(super(MPSGraphObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSGraphCore")]
    pub struct MPSGraphDepthwiseConvolution3DOpDescriptor;
);

#[cfg(feature = "MPSGraphCore")]
extern_conformance!(
    unsafe impl NSCopying for MPSGraphDepthwiseConvolution3DOpDescriptor {}
);

#[cfg(feature = "MPSGraphCore")]
unsafe impl CopyingHelper for MPSGraphDepthwiseConvolution3DOpDescriptor {
    type Result = Self;
}

#[cfg(feature = "MPSGraphCore")]
extern_conformance!(
    unsafe impl NSObjectProtocol for MPSGraphDepthwiseConvolution3DOpDescriptor {}
);

#[cfg(feature = "MPSGraphCore")]
impl MPSGraphDepthwiseConvolution3DOpDescriptor {
    extern_methods!(
        /// The strides for spatial dimensions.
        ///
        /// Must be three numbers, one for each spatial dimension, fastest running index last.
        /// Default value: `
        /// @
        /// [
        /// @
        /// 1,
        /// @
        /// 1,
        /// @
        /// 1 ]`
        #[unsafe(method(strides))]
        #[unsafe(method_family = none)]
        pub unsafe fn strides(&self) -> Retained<NSArray<NSNumber>>;

        /// Setter for [`strides`][Self::strides].
        #[unsafe(method(setStrides:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setStrides(&self, strides: &NSArray<NSNumber>);

        /// The dilation rates for spatial dimensions.
        ///
        /// Must be three numbers, one for each spatial dimension, fastest running index last.
        /// Default value: `
        /// @
        /// [
        /// @
        /// 1,
        /// @
        /// 1,
        /// @
        /// 1 ]`
        #[unsafe(method(dilationRates))]
        #[unsafe(method_family = none)]
        pub unsafe fn dilationRates(&self) -> Retained<NSArray<NSNumber>>;

        /// Setter for [`dilationRates`][Self::dilationRates].
        #[unsafe(method(setDilationRates:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDilationRates(&self, dilation_rates: &NSArray<NSNumber>);

        /// The padding values for spatial dimensions.
        ///
        /// Must be six numbers, two for each spatial dimension.
        /// For example `paddingValues[0]` defines the explicit padding
        /// amount before the first spatial dimension (slowest running index of spatial dimensions),
        /// `paddingValues[1]` defines the padding amount after the first spatial dimension etc.
        /// Use only with `paddingStyle = MPSGraphPaddingStyleExplicit`.
        /// Default value: `
        /// @
        /// [
        /// @
        /// 0,
        /// @
        /// 0,
        /// @
        /// 0,
        /// @
        /// 0,
        /// @
        /// 0,
        /// @
        /// 0 ]`
        #[unsafe(method(paddingValues))]
        #[unsafe(method_family = none)]
        pub unsafe fn paddingValues(&self) -> Retained<NSArray<NSNumber>>;

        /// Setter for [`paddingValues`][Self::paddingValues].
        #[unsafe(method(setPaddingValues:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPaddingValues(&self, padding_values: &NSArray<NSNumber>);

        /// The padding style for the operation.
        ///
        /// Default value: `MPSGraphPaddingStyleExplicit`.
        #[unsafe(method(paddingStyle))]
        #[unsafe(method_family = none)]
        pub unsafe fn paddingStyle(&self) -> MPSGraphPaddingStyle;

        /// Setter for [`paddingStyle`][Self::paddingStyle].
        #[unsafe(method(setPaddingStyle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPaddingStyle(&self, padding_style: MPSGraphPaddingStyle);

        /// The axis that contains the channels in the input and the weights, within
        /// the 4D tile of the last dimensions.
        ///
        /// For example the value of `-1` corresponds to `NDHWC`, `NHWC` layouts. This allows the placement
        /// of the channel index anywhere within the last 4 dimensions of the tensor. In case your
        /// weights are in a different layout you can bring them to the same layout
        /// as inputs using transposes or permutations.
        /// Default value: `-4`, corresponds to `NCDHW` and `CDHW` layouts.
        #[unsafe(method(channelDimensionIndex))]
        #[unsafe(method_family = none)]
        pub unsafe fn channelDimensionIndex(&self) -> NSInteger;

        /// Setter for [`channelDimensionIndex`][Self::channelDimensionIndex].
        #[unsafe(method(setChannelDimensionIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setChannelDimensionIndex(&self, channel_dimension_index: NSInteger);

        /// Creates a 3D depthwise convolution descriptor with given values.
        ///
        /// - Parameters:
        /// - strides: See `strides` property.
        /// - dilationRates: See `dilationRates` property.
        /// - paddingValues: See `paddingValues` property.
        /// - paddingStyle: See `paddingStyle` property.
        /// - Returns: The descriptor on autoreleasepool.
        #[unsafe(method(descriptorWithStrides:dilationRates:paddingValues:paddingStyle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn descriptorWithStrides_dilationRates_paddingValues_paddingStyle(
            strides: &NSArray<NSNumber>,
            dilation_rates: &NSArray<NSNumber>,
            padding_values: &NSArray<NSNumber>,
            padding_style: MPSGraphPaddingStyle,
        ) -> Option<Retained<Self>>;

        /// Creates a 3D depthwise convolution descriptor with default values.
        ///
        /// - Parameters:
        /// - paddingStyle: See `paddingStyle` property.
        /// - Returns: The descriptor on autoreleasepool.
        #[unsafe(method(descriptorWithPaddingStyle:))]
        #[unsafe(method_family = none)]
        pub unsafe fn descriptorWithPaddingStyle(
            padding_style: MPSGraphPaddingStyle,
        ) -> Option<Retained<Self>>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "MPSGraphCore")]
impl MPSGraphDepthwiseConvolution3DOpDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

/// MPSGraphDepthwiseConvolutionOps.
#[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
impl MPSGraph {
    extern_methods!(
        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a 2D-depthwise convolution operation and returns the result tensor.
        ///
        /// - Parameters:
        /// - source: A 2D Image source as tensor - must be of rank=4. The layout is defined by `descriptor.dataLayout`.
        /// - weights: The weights tensor, must be rank=4. The layout is defined by `descriptor.weightsLayout`.
        /// - descriptor: The descriptor object that specifies strides, dilation rates, paddings and layouts.
        /// - name: The name for the operation.
        /// - Returns: A valid MPSGraphTensor object
        #[unsafe(method(depthwiseConvolution2DWithSourceTensor:weightsTensor:descriptor:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn depthwiseConvolution2DWithSourceTensor_weightsTensor_descriptor_name(
            &self,
            source: &MPSGraphTensor,
            weights: &MPSGraphTensor,
            descriptor: &MPSGraphDepthwiseConvolution2DOpDescriptor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        /// Creates a 2D-depthwise convolution gradient for data operation and returns the result tensor.
        ///
        /// - Parameters:
        /// - incomingGradient: A 2D input gradient tensor - must be of rank=4. The layout is defined by `descriptor.dataLayout`.
        /// - weights: The weights tensor, must be rank=4. The layout is defined by `descriptor.weightsLayout`.
        /// - outputShape: The shape of the οutput tensor (and therefore input tensor of forward pass).
        /// - descriptor: The descriptor object that specifies strides, dilation rates, paddings and layouts.
        /// - name: The name for the operation.
        /// - Returns: A valid MPSGraphTensor object
        #[unsafe(method(depthwiseConvolution2DDataGradientWithIncomingGradientTensor:weightsTensor:outputShape:descriptor:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn depthwiseConvolution2DDataGradientWithIncomingGradientTensor_weightsTensor_outputShape_descriptor_name(
            &self,
            incoming_gradient: &MPSGraphTensor,
            weights: &MPSGraphTensor,
            output_shape: &MPSShape,
            descriptor: &MPSGraphDepthwiseConvolution2DOpDescriptor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        /// Creates a 2D-depthwise convolution gradient for weights operation and returns the result tensor.
        ///
        /// - Parameters:
        /// - incomingGradient: A 2D input gradient tensor - must be of rank=4. The layout is defined by `descriptor.dataLayout`.
        /// - source: A 2D Image source as tensor - must be of rank=4. The layout is defined by `descriptor.dataLayout`.
        /// - outputShape: The shape of the οutput tensor (and therefore weight tensor of forward pass).
        /// - descriptor: The descriptor object that specifies strides, dilation rates, paddings and layouts.
        /// - name: The name for the operation.
        /// - Returns: A valid MPSGraphTensor object
        #[unsafe(method(depthwiseConvolution2DWeightsGradientWithIncomingGradientTensor:sourceTensor:outputShape:descriptor:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn depthwiseConvolution2DWeightsGradientWithIncomingGradientTensor_sourceTensor_outputShape_descriptor_name(
            &self,
            incoming_gradient: &MPSGraphTensor,
            source: &MPSGraphTensor,
            output_shape: &MPSShape,
            descriptor: &MPSGraphDepthwiseConvolution2DOpDescriptor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a 3D depthwise convolution operation and returns the result tensor.
        ///
        /// Works exactly like depthwise convolution2D, but in three dimensions. Supports different layouts with
        /// the ``MPSGraphDepthwiseConvolution3DOpDescriptor/channelDimensionIndex`` property.
        /// If your weights need a different layout add a permute operation on them before this operation.
        ///
        /// - Parameters:
        /// - source: A 3D Image source as tensor - must be at least rank=4 (CDHW when channelDimensionIndex = -4).
        /// - weights: The weights tensor, must be rank=4 - axes are interpreted as CDHW when channelDimensionIndex = -4 .
        /// - descriptor: The descriptor object that specifies strides, dilation rates and paddings.
        /// - name: The name for the operation.
        /// - Returns: A valid MPSGraphTensor object
        #[unsafe(method(depthwiseConvolution3DWithSourceTensor:weightsTensor:descriptor:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn depthwiseConvolution3DWithSourceTensor_weightsTensor_descriptor_name(
            &self,
            source: &MPSGraphTensor,
            weights: &MPSGraphTensor,
            descriptor: &MPSGraphDepthwiseConvolution3DOpDescriptor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        /// Creates a 3D depthwise convolution gradient for data operation and returns the result tensor.
        ///
        /// - Parameters:
        /// - incomingGradient: A 3D input gradient tensor - must be at least rank=4 (CDHW).
        /// - weights: The weights tensor, must be rank=4 - axes are interpreted as CDHW.
        /// - outputShape: The shape of the οutput tensor (and therefore input tensor of forward pass).
        /// - descriptor: The descriptor object that  specifies strides, dilation rates and paddings.
        /// - name: The name for the operation.
        /// - Returns: A valid MPSGraphTensor object
        #[unsafe(method(depthwiseConvolution3DDataGradientWithIncomingGradientTensor:weightsTensor:outputShape:descriptor:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn depthwiseConvolution3DDataGradientWithIncomingGradientTensor_weightsTensor_outputShape_descriptor_name(
            &self,
            incoming_gradient: &MPSGraphTensor,
            weights: &MPSGraphTensor,
            output_shape: Option<&MPSShape>,
            descriptor: &MPSGraphDepthwiseConvolution3DOpDescriptor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        /// Creates a 3D depthwise convolution gradient for weights operation and returns the result tensor.
        ///
        /// - Parameters:
        /// - incomingGradient: A 3D input gradient tensor - must be at least rank=4 (NCDHW).
        /// - source: The forward pass 3D Image source as tensor - must be at least rank=4 (NCDHW).
        /// - outputShape: The shape of the οutput tensor (and therefore weight tensor of forward pass).
        /// - descriptor: The descriptor object that specifies strides, dilation rates and paddings.
        /// - name: The name for the operation.
        /// - Returns: A valid MPSGraphTensor object
        #[unsafe(method(depthwiseConvolution3DWeightsGradientWithIncomingGradientTensor:sourceTensor:outputShape:descriptor:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn depthwiseConvolution3DWeightsGradientWithIncomingGradientTensor_sourceTensor_outputShape_descriptor_name(
            &self,
            incoming_gradient: &MPSGraphTensor,
            source: &MPSGraphTensor,
            output_shape: &MPSShape,
            descriptor: &MPSGraphDepthwiseConvolution3DOpDescriptor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;
    );
}
