//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnconvolutionflags?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSCNNConvolutionFlags(pub NSUInteger);
impl MPSCNNConvolutionFlags {
    /// Use default options
    #[doc(alias = "MPSCNNConvolutionFlagsNone")]
    #[deprecated]
    pub const None: Self = Self(0);
}

unsafe impl Encode for MPSCNNConvolutionFlags {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MPSCNNConvolutionFlags {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnbinaryconvolutionflags?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSCNNBinaryConvolutionFlags(pub NSUInteger);
impl MPSCNNBinaryConvolutionFlags {
    /// Use default in binary convolution options
    #[doc(alias = "MPSCNNBinaryConvolutionFlagsNone")]
    pub const None: Self = Self(0);
    /// Scale the binary convolution operation using the beta-image option as detailed in MPSCNNBinaryConvolution
    #[doc(alias = "MPSCNNBinaryConvolutionFlagsUseBetaScaling")]
    pub const UseBetaScaling: Self = Self(1 << 0);
}

unsafe impl Encode for MPSCNNBinaryConvolutionFlags {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MPSCNNBinaryConvolutionFlags {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnbinaryconvolutiontype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSCNNBinaryConvolutionType(pub NSUInteger);
impl MPSCNNBinaryConvolutionType {
    /// Otherwise a normal convolution operation, except that the weights are binary values
    #[doc(alias = "MPSCNNBinaryConvolutionTypeBinaryWeights")]
    pub const BinaryWeights: Self = Self(0);
    /// Use input image binarization and the XNOR-operation to perform the actual convolution - See MPSCNNBinaryConvolution for details
    #[doc(alias = "MPSCNNBinaryConvolutionTypeXNOR")]
    pub const XNOR: Self = Self(1);
    /// Use input image binarization and the AND-operation to perform the actual convolution - See MPSCNNBinaryConvolution for details
    #[doc(alias = "MPSCNNBinaryConvolutionTypeAND")]
    pub const AND: Self = Self(2);
}

unsafe impl Encode for MPSCNNBinaryConvolutionType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MPSCNNBinaryConvolutionType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsnnconvolutionaccumulatorprecisionoption?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSNNConvolutionAccumulatorPrecisionOption(pub NSUInteger);
bitflags::bitflags! {
    impl MPSNNConvolutionAccumulatorPrecisionOption: NSUInteger {
/// Set accumulator type to half precision float.
        #[doc(alias = "MPSNNConvolutionAccumulatorPrecisionOptionHalf")]
        const Half = 0;
/// Set accumulator type to single precision float.
        #[doc(alias = "MPSNNConvolutionAccumulatorPrecisionOptionFloat")]
        const Float = 1<<0;
    }
}

unsafe impl Encode for MPSNNConvolutionAccumulatorPrecisionOption {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MPSNNConvolutionAccumulatorPrecisionOption {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsnntrainingstyle?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSNNTrainingStyle(pub NSUInteger);
bitflags::bitflags! {
    impl MPSNNTrainingStyle: NSUInteger {
/// Do not train this node, for example in transfer learning
        #[doc(alias = "MPSNNTrainingStyleUpdateDeviceNone")]
        const UpdateDeviceNone = 0;
/// The weight update pass will be called in a command buffer completion callback, with a nil command buffer
        #[doc(alias = "MPSNNTrainingStyleUpdateDeviceCPU")]
        const UpdateDeviceCPU = 1;
/// The weight update pass will be called immediately after the gradient pass is encoded, with a nonnull command buffer
        #[doc(alias = "MPSNNTrainingStyleUpdateDeviceGPU")]
        const UpdateDeviceGPU = 2;
    }
}

unsafe impl Encode for MPSNNTrainingStyle {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MPSNNTrainingStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnbatchnormalizationflags?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSCNNBatchNormalizationFlags(pub NSUInteger);
bitflags::bitflags! {
    impl MPSCNNBatchNormalizationFlags: NSUInteger {
/// Default Settings
        #[doc(alias = "MPSCNNBatchNormalizationFlagsDefault")]
        const Default = 0;
/// Statistics are calculated if another node consumes the gradient node (training). The data source is used otherwise.
        #[doc(alias = "MPSCNNBatchNormalizationFlagsCalculateStatisticsAutomatic")]
        const CalculateStatisticsAutomatic = MPSCNNBatchNormalizationFlags::Default.0;
/// Statistics are calculated always
        #[doc(alias = "MPSCNNBatchNormalizationFlagsCalculateStatisticsAlways")]
        const CalculateStatisticsAlways = 1;
/// Statistics are never calculated. Predefined values from the data source are used instead
        #[doc(alias = "MPSCNNBatchNormalizationFlagsCalculateStatisticsNever")]
        const CalculateStatisticsNever = 2;
/// Bits used for  MPSCNNBatchNormalizationFlagsCalculateStatistics
        #[doc(alias = "MPSCNNBatchNormalizationFlagsCalculateStatisticsMask")]
        const CalculateStatisticsMask = 3;
    }
}

unsafe impl Encode for MPSCNNBatchNormalizationFlags {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MPSCNNBatchNormalizationFlags {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsnnpaddingmethod?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSNNPaddingMethod(pub NSUInteger);
bitflags::bitflags! {
    impl MPSNNPaddingMethod: NSUInteger {
/// Extra padding pixels are distributed as evenly as possible to all sides
        #[doc(alias = "MPSNNPaddingMethodAlignCentered")]
        const AlignCentered = 0;
/// Extra padding pixels appear on top and left sides
        #[doc(alias = "MPSNNPaddingMethodAlignTopLeft")]
        const AlignTopLeft = 1;
/// Extra padding pixels appear on the bottom and right sides
        #[doc(alias = "MPSNNPaddingMethodAlignBottomRight")]
        const AlignBottomRight = 2;
/// Extra padding pixels are not defined.
        #[doc(alias = "MPSNNPaddingMethodAlign_reserved")]
        const Align_reserved = 3;
        #[doc(alias = "MPSNNPaddingMethodAlignMask")]
        const AlignMask = MPSNNPaddingMethod::Align_reserved.0;
/// Extra padding pixels are accumulated to top and left sides
        #[doc(alias = "MPSNNPaddingMethodAddRemainderToTopLeft")]
        const AddRemainderToTopLeft = 0<<2;
        #[doc(alias = "MPSNNPaddingMethodAddRemainderToTopRight")]
        const AddRemainderToTopRight = 1<<2;
        #[doc(alias = "MPSNNPaddingMethodAddRemainderToBottomLeft")]
        const AddRemainderToBottomLeft = 2<<2;
/// Extra padding pixels are accumulated to bottom and right sides
        #[doc(alias = "MPSNNPaddingMethodAddRemainderToBottomRight")]
        const AddRemainderToBottomRight = 3<<2;
        #[doc(alias = "MPSNNPaddingMethodAddRemainderToMask")]
        const AddRemainderToMask = MPSNNPaddingMethod::AddRemainderToBottomRight.0;
/// The result is the largest image for which *all* source pixels are valid for result pixels
        #[doc(alias = "MPSNNPaddingMethodSizeValidOnly")]
        const SizeValidOnly = 0;
/// The result is the same size as the input image (before strides)
        #[doc(alias = "MPSNNPaddingMethodSizeSame")]
        const SizeSame = 1<<4;
/// The result is the largest image for which *any* source pixel is valid for result pixels
        #[doc(alias = "MPSNNPaddingMethodSizeFull")]
        const SizeFull = 2<<4;
        #[doc(alias = "MPSNNPaddingMethodSize_reserved")]
        const Size_reserved = 3<<4;
        #[doc(alias = "MPSNNPaddingMethodCustomWhitelistForNodeFusion")]
#[deprecated]
        const CustomWhitelistForNodeFusion = 1<<13;
/// By itself, MPSNNPaddingMethodCustom will inhibit automatic fusion between nodes producing and consuming the image described by the padding policy. MPSNNPaddingMethodCustomAllowForNodeFusion signals that the custom method is benign and fusion may go ahead.
        #[doc(alias = "MPSNNPaddingMethodCustomAllowForNodeFusion")]
        const CustomAllowForNodeFusion = 1<<13;
/// Use destinationImageDescriptorForSourceImages:sourceStates:forKernel:suggestedDescriptor: to calculate padding and offset.
        #[doc(alias = "MPSNNPaddingMethodCustom")]
        const Custom = 1<<14;
        #[doc(alias = "MPSNNPaddingMethodSizeMask")]
        const SizeMask = 0x7f0;
/// The caffe framework constrains the average pooling area to the limits of the padding area in cases
/// where a pixel would read beyond the padding area. Set this bit for Caffe emulation with average pooling.
        #[doc(alias = "MPSNNPaddingMethodExcludeEdges")]
        const ExcludeEdges = 1<<15;
    }
}

unsafe impl Encode for MPSNNPaddingMethod {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MPSNNPaddingMethod {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// A method to describe how MPSCNNKernels should pad images when data outside the image is needed
    ///
    /// Different (non-Apple) CNN frameworks have different policies for how to size the result
    /// of a CNN filter and what padding to add around the edges.  Some filters such
    /// as pooling and convolution read from neighboring feature channel (pixel) values.
    /// Four predefined MPSPaddingMethods are available: MPSNNPaddingMethodValidOnly,
    /// MPSNNPaddingMethodFull, MPSNNPaddingMethodSameTL, MPSNNPaddingMethodSameBR. You
    /// may also implement your own padding definition with a block that conforms
    /// to this prototype.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsnnpadding?language=objc)
    pub unsafe trait MPSNNPadding: NSObjectProtocol + NSSecureCoding {
        /// Get the preferred padding method for the node
        #[unsafe(method(paddingMethod))]
        #[unsafe(method_family = none)]
        unsafe fn paddingMethod(&self) -> MPSNNPaddingMethod;

        /// A human readable string that describes the padding policy. Useful for verbose debugging support.
        #[optional]
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        unsafe fn label(&self) -> Retained<NSString>;

        #[cfg(all(
            feature = "MPSCore",
            feature = "MPSImage",
            feature = "MPSKernel",
            feature = "MPSState"
        ))]
        /// Determine padding and sizing of result images
        ///
        /// A MPSNNPaddingMethod must both return a valid MPSImageDescriptor
        /// and set the MPSKernel.offset to the correct value.  This is a
        /// required feature if the MPSNNPaddingMethodCustom bit is set in
        /// the paddingMethod.
        ///
        /// Some code that may prove helpful:
        ///
        ///
        /// ```text
        ///                   const int centeringPolicy = 0;  // When kernelSize is even: 0 pad bottom right. 1 pad top left.    Centers the kernel for even sized kernels.
        ///
        ///                   typedef enum Style{
        ///                       StyleValidOnly = -1,
        ///                       StyleSame = 0,
        ///                       StyleFull = 1
        ///                   }Style;
        ///
        ///                   // Typical destination size in one dimension for forward filters (most filters)
        ///                   static int DestSize( int sourceSize, int stride, int filterWindowSize, Style style ){
        ///                       sourceSize += style * (filterWindowSize - 1);       // adjust how many pixels we are allowed to read
        ///                       return (sourceSize + stride - 1) / stride;          // sourceSize / stride, round up
        ///                   }
        ///
        ///                   // Typical destination size in one dimension for reverse filters (e.g. convolution transpose)
        ///                   static int DestSizeReverse( int sourceSize, int stride, int filterWindowSize, Style style ){
        ///                       return (sourceSize-1) * stride +        // center tap for the last N-1 results. Take stride into account
        ///                               1 +                             // center tap for the first result
        ///                               style * (filterWindowSize-1);   // add or subtract (or ignore) the filter extent
        ///                   }
        ///
        ///                   // Find the MPSOffset in one dimension
        ///                   static int Offset( int sourceSize, int stride, int filterWindowSize, Style style ){
        ///                       // The correction needed to adjust from position of left edge to center per MPSOffset definition
        ///                       int correction = filterWindowSize / 2;
        ///
        ///                       // exit if all we want is to start consuming pixels at the left edge of the image.
        ///                       if( 0 )
        ///                           return correction;
        ///
        ///                       // Center the area consumed in the source image:
        ///                       // Calculate the size of the destination image
        ///                       int destSize = DestSize( sourceSize, stride, filterWindowSize, style ); // use DestSizeReverse here instead as appropriate
        ///
        ///                       // calculate extent of pixels we need to read in source to populate the destination
        ///                       int readSize = (destSize-1) * stride + filterWindowSize;
        ///
        ///                       // calculate number of missing pixels in source
        ///                       int extraSize = readSize - sourceSize;
        ///
        ///                       // number of missing pixels on left side
        ///                       int leftExtraPixels = (extraSize + centeringPolicy) / 2;
        ///
        ///                       // account for the fact that the offset is based on the center pixel, not the left edge
        ///                       return correction - leftExtraPixels;
        ///                   }
        /// ```
        ///
        ///
        /// Parameter `sourceImages`: The list of source images to be used
        ///
        /// Parameter `sourceStates`: The list of source states to be used
        ///
        /// Parameter `kernel`: The MPSKernel the padding method will be applied to. Set the kernel.offset
        ///
        /// Parameter `inDescriptor`: MPS will prepare a starting guess based on the padding policy (exclusive of
        /// MPSNNPaddingMethodCustom) set for the object. You should adjust the offset
        /// and image size accordingly. It is on an autoreleasepool.
        ///
        ///
        /// Returns: The MPSImageDescriptor to use to make a MPSImage to capture the results from the filter.
        /// The MPSImageDescriptor is assumed to be on an autoreleasepool. Your method must also set the
        /// kernel.offset property.
        #[optional]
        #[unsafe(method(destinationImageDescriptorForSourceImages:sourceStates:forKernel:suggestedDescriptor:))]
        #[unsafe(method_family = none)]
        unsafe fn destinationImageDescriptorForSourceImages_sourceStates_forKernel_suggestedDescriptor(
            &self,
            source_images: &NSArray<MPSImage>,
            source_states: Option<&NSArray<MPSState>>,
            kernel: &MPSKernel,
            in_descriptor: &MPSImageDescriptor,
        ) -> Retained<MPSImageDescriptor>;

        /// Make a "inverted" padding policy suitable for a training gradient pass.
        #[optional]
        #[unsafe(method(inverse))]
        #[unsafe(method_family = none)]
        unsafe fn inverse(&self) -> Option<Retained<Self>>;
    }
);

extern_class!(
    /// This class provides some pre-rolled padding policies for common tasks
    ///
    /// You are, of course, welcome to write your own class that conforms to
    /// The MPSNNPadding protocol and use that instead.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsnndefaultpadding?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPSNNDefaultPadding;
);

extern_conformance!(
    unsafe impl MPSNNPadding for MPSNNDefaultPadding {}
);

extern_conformance!(
    unsafe impl NSCoding for MPSNNDefaultPadding {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MPSNNDefaultPadding {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for MPSNNDefaultPadding {}
);

impl MPSNNDefaultPadding {
    extern_methods!(
        /// Fetch a well known object that implements a non-custom padding method
        ///
        /// For custom padding methods, you will need to implement an object that conforms
        /// to the full MPSNNPadding protocol, including NSSecureCoding.
        ///
        /// Parameter `method`: A MPSNNPaddingMethod
        ///
        /// Returns: An object that implements
        /// <MPSNNPadding
        /// > for use with MPSNNGraphNodes.
        #[unsafe(method(paddingWithMethod:))]
        #[unsafe(method_family = none)]
        pub unsafe fn paddingWithMethod(method: MPSNNPaddingMethod) -> Retained<Self>;

        /// A padding policy that attempts to reproduce TensorFlow behavior for average pooling
        ///
        /// Most TensorFlow padding is covered by the standard MPSNNPaddingMethod encodings.
        /// You can use +paddingWithMethod to get quick access to MPSNNPadding objects, when
        /// default filter behavior isn't enough. (It often is.)  However, the edging for
        /// max pooling in TensorFlow is a bit unusual.
        ///
        /// This padding method attempts to reproduce TensorFlow padding for average pooling.
        /// In addition to setting MPSNNPaddingMethodSizeSame | MPSNNPaddingMethodAlignCentered |
        /// MPSNNPaddingMethodAddRemainderToBottomRight, it also configures the filter to run with
        /// MPSImageEdgeModeClamp, which (as a special case for average pooling only), normalizes the
        /// sum of contributing samples to the area of valid contributing pixels only.
        ///
        ///
        /// ```text
        ///                       // Sample implementation for the tensorflowPoolingPaddingPolicy returned
        ///                        -(MPSNNPaddingMethod) paddingMethod{ return MPSNNPaddingMethodCustom | MPSNNPaddingMethodSizeSame; }
        ///
        ///                        -(MPSImageDescriptor * __nonnull) destinationImageDescriptorForSourceImages: (NSArray <MPSImage *> *__nonnull) sourceImages
        ///                                                                                       sourceStates: (NSArray <MPSState *> * __nullable) sourceStates
        ///                                                                                          forKernel: (MPSKernel * __nonnull) kernel
        ///                                                                                suggestedDescriptor: (MPSImageDescriptor * __nonnull) inDescriptor
        ///                        {
        ///
        ///                           ((MPSCNNKernel *)kernel).edgeMode = MPSImageEdgeModeClamp;
        ///
        ///                           return inDescriptor;
        ///                        }
        /// ```
        #[unsafe(method(paddingForTensorflowAveragePooling))]
        #[unsafe(method_family = none)]
        pub unsafe fn paddingForTensorflowAveragePooling() -> Retained<Self>;

        /// Typical pooling padding policy for valid only mode
        #[unsafe(method(paddingForTensorflowAveragePoolingValidOnly))]
        #[unsafe(method_family = none)]
        pub unsafe fn paddingForTensorflowAveragePoolingValidOnly() -> Retained<Self>;

        /// Human readable description of what the padding policy does
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        pub unsafe fn label(&self) -> Retained<NSString>;
    );
}

/// Methods declared on superclass `NSObject`.
impl MPSNNDefaultPadding {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_protocol!(
    /// MPSStates conforming to this protocol contain information about a image size elsewhere in the graph
    ///
    /// In some graphs a sequence of operations are done, then they are undone ins a series of 'reverse'
    /// operations. Examples might be pooling vs pooling gradient / upsampling,  or convolution vs. convolution transpose.
    /// In such cases, the 'reverse' pass generally is converting from a smaller image to a larger image,
    /// and there is insufficient information to do this correctly. Several answers exist and we don't know
    /// which is correct.
    ///
    /// As an example, consider trying to 'undo' integer division with a multiplication. The expression c = a/b
    /// is incomplete because there is also a remainder, which may constitute information lost. If we want to
    /// reconstitute a based on c and b, we need to use a = c * b + remainder, not just a = c*b.  Similarly, when
    /// undoing a downsizing operation, we need the original size to find which answer in the range of
    /// a = c*b + [0,b-1] is the right one.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimagesizeencodingstate?language=objc)
    pub unsafe trait MPSImageSizeEncodingState: NSObjectProtocol {
        /// The width of the source image passed to MPSCNNConvolution encode call.
        #[unsafe(method(sourceWidth))]
        #[unsafe(method_family = none)]
        unsafe fn sourceWidth(&self) -> NSUInteger;

        /// The height of the source image passed to MPSCNNConvolution encode call.
        #[unsafe(method(sourceHeight))]
        #[unsafe(method_family = none)]
        unsafe fn sourceHeight(&self) -> NSUInteger;
    }
);
