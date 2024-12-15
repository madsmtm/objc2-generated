//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "MPSImageConversion")]
#[path = "MPSImageConversion.rs"]
mod __MPSImageConversion;
#[cfg(feature = "MPSImageConvolution")]
#[path = "MPSImageConvolution.rs"]
mod __MPSImageConvolution;
#[cfg(feature = "MPSImageCopy")]
#[path = "MPSImageCopy.rs"]
mod __MPSImageCopy;
#[cfg(feature = "MPSImageDistanceTransform")]
#[path = "MPSImageDistanceTransform.rs"]
mod __MPSImageDistanceTransform;
#[cfg(feature = "MPSImageEDLines")]
#[path = "MPSImageEDLines.rs"]
mod __MPSImageEDLines;
#[cfg(feature = "MPSImageGuidedFilter")]
#[path = "MPSImageGuidedFilter.rs"]
mod __MPSImageGuidedFilter;
#[cfg(feature = "MPSImageHistogram")]
#[path = "MPSImageHistogram.rs"]
mod __MPSImageHistogram;
#[cfg(feature = "MPSImageIntegral")]
#[path = "MPSImageIntegral.rs"]
mod __MPSImageIntegral;
#[cfg(feature = "MPSImageKernel")]
#[path = "MPSImageKernel.rs"]
mod __MPSImageKernel;
#[cfg(feature = "MPSImageKeypoint")]
#[path = "MPSImageKeypoint.rs"]
mod __MPSImageKeypoint;
#[cfg(feature = "MPSImageMath")]
#[path = "MPSImageMath.rs"]
mod __MPSImageMath;
#[cfg(feature = "MPSImageMedian")]
#[path = "MPSImageMedian.rs"]
mod __MPSImageMedian;
#[cfg(feature = "MPSImageMorphology")]
#[path = "MPSImageMorphology.rs"]
mod __MPSImageMorphology;
#[cfg(feature = "MPSImageReduce")]
#[path = "MPSImageReduce.rs"]
mod __MPSImageReduce;
#[cfg(feature = "MPSImageResampling")]
#[path = "MPSImageResampling.rs"]
mod __MPSImageResampling;
#[cfg(feature = "MPSImageStatistics")]
#[path = "MPSImageStatistics.rs"]
mod __MPSImageStatistics;
#[cfg(feature = "MPSImageThreshold")]
#[path = "MPSImageThreshold.rs"]
mod __MPSImageThreshold;
#[cfg(feature = "MPSImageTranspose")]
#[path = "MPSImageTranspose.rs"]
mod __MPSImageTranspose;
#[cfg(feature = "MPSImageTypes")]
#[path = "MPSImageTypes.rs"]
mod __MPSImageTypes;

#[cfg(all(
    feature = "MPSImageConversion",
    feature = "MPSImageKernel",
    feature = "MPSKernel"
))]
pub use self::__MPSImageConversion::MPSImageConversion;
#[cfg(all(
    feature = "MPSImageConvolution",
    feature = "MPSImageKernel",
    feature = "MPSKernel"
))]
pub use self::__MPSImageConvolution::MPSImageBox;
#[cfg(all(
    feature = "MPSImageConvolution",
    feature = "MPSImageKernel",
    feature = "MPSKernel"
))]
pub use self::__MPSImageConvolution::MPSImageCanny;
#[cfg(all(
    feature = "MPSImageConvolution",
    feature = "MPSImageKernel",
    feature = "MPSKernel"
))]
pub use self::__MPSImageConvolution::MPSImageConvolution;
#[cfg(all(
    feature = "MPSImageConvolution",
    feature = "MPSImageKernel",
    feature = "MPSKernel"
))]
pub use self::__MPSImageConvolution::MPSImageGaussianBlur;
#[cfg(all(
    feature = "MPSImageConvolution",
    feature = "MPSImageKernel",
    feature = "MPSKernel"
))]
pub use self::__MPSImageConvolution::MPSImageGaussianPyramid;
#[cfg(all(
    feature = "MPSImageConvolution",
    feature = "MPSImageKernel",
    feature = "MPSKernel"
))]
pub use self::__MPSImageConvolution::MPSImageLaplacian;
#[cfg(all(
    feature = "MPSImageConvolution",
    feature = "MPSImageKernel",
    feature = "MPSKernel"
))]
pub use self::__MPSImageConvolution::MPSImageLaplacianPyramid;
#[cfg(all(
    feature = "MPSImageConvolution",
    feature = "MPSImageKernel",
    feature = "MPSKernel"
))]
pub use self::__MPSImageConvolution::MPSImageLaplacianPyramidAdd;
#[cfg(all(
    feature = "MPSImageConvolution",
    feature = "MPSImageKernel",
    feature = "MPSKernel"
))]
pub use self::__MPSImageConvolution::MPSImageLaplacianPyramidSubtract;
#[cfg(all(
    feature = "MPSImageConvolution",
    feature = "MPSImageKernel",
    feature = "MPSKernel"
))]
pub use self::__MPSImageConvolution::MPSImagePyramid;
#[cfg(all(
    feature = "MPSImageConvolution",
    feature = "MPSImageKernel",
    feature = "MPSKernel"
))]
pub use self::__MPSImageConvolution::MPSImageSobel;
#[cfg(all(
    feature = "MPSImageConvolution",
    feature = "MPSImageKernel",
    feature = "MPSKernel"
))]
pub use self::__MPSImageConvolution::MPSImageTent;
#[cfg(all(feature = "MPSImageCopy", feature = "MPSKernel"))]
pub use self::__MPSImageCopy::MPSImageCopyToMatrix;
#[cfg(all(feature = "MPSImageCopy", feature = "MPSKernel"))]
pub use self::__MPSImageCopy::MPSMatrixCopyToImage;
#[cfg(all(
    feature = "MPSImageDistanceTransform",
    feature = "MPSImageKernel",
    feature = "MPSKernel"
))]
pub use self::__MPSImageDistanceTransform::MPSImageEuclideanDistanceTransform;
#[cfg(all(feature = "MPSImageEDLines", feature = "MPSKernel"))]
pub use self::__MPSImageEDLines::MPSImageEDLines;
#[cfg(all(feature = "MPSImageGuidedFilter", feature = "MPSKernel"))]
pub use self::__MPSImageGuidedFilter::MPSImageGuidedFilter;
#[cfg(all(feature = "MPSImageHistogram", feature = "MPSKernel"))]
pub use self::__MPSImageHistogram::MPSImageHistogram;
#[cfg(all(
    feature = "MPSImageHistogram",
    feature = "MPSImageKernel",
    feature = "MPSKernel"
))]
pub use self::__MPSImageHistogram::MPSImageHistogramEqualization;
#[cfg(all(
    feature = "MPSImageHistogram",
    feature = "MPSImageKernel",
    feature = "MPSKernel"
))]
pub use self::__MPSImageHistogram::MPSImageHistogramSpecification;
#[cfg(all(feature = "MPSImageHistogram", feature = "MPSKernel"))]
pub use self::__MPSImageHistogram::MPSImageNormalizedHistogram;
#[cfg(all(
    feature = "MPSImageIntegral",
    feature = "MPSImageKernel",
    feature = "MPSKernel"
))]
pub use self::__MPSImageIntegral::MPSImageIntegral;
#[cfg(all(
    feature = "MPSImageIntegral",
    feature = "MPSImageKernel",
    feature = "MPSKernel"
))]
pub use self::__MPSImageIntegral::MPSImageIntegralOfSquares;
#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
pub use self::__MPSImageKernel::MPSBinaryImageKernel;
#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel", feature = "block2"))]
pub use self::__MPSImageKernel::MPSCopyAllocator;
#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
pub use self::__MPSImageKernel::MPSUnaryImageKernel;
#[cfg(all(feature = "MPSImageKeypoint", feature = "MPSKernel"))]
pub use self::__MPSImageKeypoint::MPSImageFindKeypoints;
#[cfg(feature = "MPSImageKeypoint")]
pub use self::__MPSImageKeypoint::MPSImageKeypointRangeInfo;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageMath",
    feature = "MPSKernel"
))]
pub use self::__MPSImageMath::MPSImageAdd;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageMath",
    feature = "MPSKernel"
))]
pub use self::__MPSImageMath::MPSImageArithmetic;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageMath",
    feature = "MPSKernel"
))]
pub use self::__MPSImageMath::MPSImageDivide;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageMath",
    feature = "MPSKernel"
))]
pub use self::__MPSImageMath::MPSImageMultiply;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageMath",
    feature = "MPSKernel"
))]
pub use self::__MPSImageMath::MPSImageSubtract;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageMedian",
    feature = "MPSKernel"
))]
pub use self::__MPSImageMedian::MPSImageMedian;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageMorphology",
    feature = "MPSKernel"
))]
pub use self::__MPSImageMorphology::MPSImageAreaMax;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageMorphology",
    feature = "MPSKernel"
))]
pub use self::__MPSImageMorphology::MPSImageAreaMin;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageMorphology",
    feature = "MPSKernel"
))]
pub use self::__MPSImageMorphology::MPSImageDilate;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageMorphology",
    feature = "MPSKernel"
))]
pub use self::__MPSImageMorphology::MPSImageErode;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageReduce",
    feature = "MPSKernel"
))]
pub use self::__MPSImageReduce::MPSImageReduceColumnMax;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageReduce",
    feature = "MPSKernel"
))]
pub use self::__MPSImageReduce::MPSImageReduceColumnMean;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageReduce",
    feature = "MPSKernel"
))]
pub use self::__MPSImageReduce::MPSImageReduceColumnMin;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageReduce",
    feature = "MPSKernel"
))]
pub use self::__MPSImageReduce::MPSImageReduceColumnSum;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageReduce",
    feature = "MPSKernel"
))]
pub use self::__MPSImageReduce::MPSImageReduceRowMax;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageReduce",
    feature = "MPSKernel"
))]
pub use self::__MPSImageReduce::MPSImageReduceRowMean;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageReduce",
    feature = "MPSKernel"
))]
pub use self::__MPSImageReduce::MPSImageReduceRowMin;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageReduce",
    feature = "MPSKernel"
))]
pub use self::__MPSImageReduce::MPSImageReduceRowSum;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageReduce",
    feature = "MPSKernel"
))]
pub use self::__MPSImageReduce::MPSImageReduceUnary;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageResampling",
    feature = "MPSKernel"
))]
pub use self::__MPSImageResampling::MPSImageBilinearScale;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageResampling",
    feature = "MPSKernel"
))]
pub use self::__MPSImageResampling::MPSImageLanczosScale;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageResampling",
    feature = "MPSKernel"
))]
pub use self::__MPSImageResampling::MPSImageScale;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageStatistics",
    feature = "MPSKernel"
))]
pub use self::__MPSImageStatistics::MPSImageStatisticsMean;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageStatistics",
    feature = "MPSKernel"
))]
pub use self::__MPSImageStatistics::MPSImageStatisticsMeanAndVariance;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageStatistics",
    feature = "MPSKernel"
))]
pub use self::__MPSImageStatistics::MPSImageStatisticsMinAndMax;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageThreshold",
    feature = "MPSKernel"
))]
pub use self::__MPSImageThreshold::MPSImageThresholdBinary;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageThreshold",
    feature = "MPSKernel"
))]
pub use self::__MPSImageThreshold::MPSImageThresholdBinaryInverse;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageThreshold",
    feature = "MPSKernel"
))]
pub use self::__MPSImageThreshold::MPSImageThresholdToZero;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageThreshold",
    feature = "MPSKernel"
))]
pub use self::__MPSImageThreshold::MPSImageThresholdToZeroInverse;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageThreshold",
    feature = "MPSKernel"
))]
pub use self::__MPSImageThreshold::MPSImageThresholdTruncate;
#[cfg(all(
    feature = "MPSImageKernel",
    feature = "MPSImageTranspose",
    feature = "MPSKernel"
))]
pub use self::__MPSImageTranspose::MPSImageTranspose;
#[cfg(feature = "MPSImageTypes")]
pub use self::__MPSImageTypes::MPSAlphaType;