//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshadersgraph/mpsgraphnonmaximumsuppressioncoordinatemode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSGraphNonMaximumSuppressionCoordinateMode(pub NSUInteger);
impl MPSGraphNonMaximumSuppressionCoordinateMode {
    #[doc(alias = "MPSGraphNonMaximumSuppressionCoordinateModeCornersHeightFirst")]
    pub const CornersHeightFirst: Self = Self(0);
    #[doc(alias = "MPSGraphNonMaximumSuppressionCoordinateModeCornersWidthFirst")]
    pub const CornersWidthFirst: Self = Self(1);
    #[doc(alias = "MPSGraphNonMaximumSuppressionCoordinateModeCentersHeightFirst")]
    pub const CentersHeightFirst: Self = Self(2);
    #[doc(alias = "MPSGraphNonMaximumSuppressionCoordinateModeCentersWidthFirst")]
    pub const CentersWidthFirst: Self = Self(3);
}

unsafe impl Encode for MPSGraphNonMaximumSuppressionCoordinateMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MPSGraphNonMaximumSuppressionCoordinateMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// MPSGraphNonMaximumSuppressionOps
    #[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
    unsafe impl MPSGraph {
        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other nonMaximumSuppressionWithBoxesTensor:scoresTensor:IOUThreshold:scoreThreshold:perClassSuppression:coordinateMode:name:)]
        pub unsafe fn nonMaximumSuppressionWithBoxesTensor_scoresTensor_IOUThreshold_scoreThreshold_perClassSuppression_coordinateMode_name(
            &self,
            boxes_tensor: &MPSGraphTensor,
            scores_tensor: &MPSGraphTensor,
            iou_threshold: c_float,
            score_threshold: c_float,
            per_class_suppression: bool,
            coordinate_mode: MPSGraphNonMaximumSuppressionCoordinateMode,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other nonMaximumSuppressionWithBoxesTensor:scoresTensor:classIndicesTensor:IOUThreshold:scoreThreshold:perClassSuppression:coordinateMode:name:)]
        pub unsafe fn nonMaximumSuppressionWithBoxesTensor_scoresTensor_classIndicesTensor_IOUThreshold_scoreThreshold_perClassSuppression_coordinateMode_name(
            &self,
            boxes_tensor: &MPSGraphTensor,
            scores_tensor: &MPSGraphTensor,
            class_indices_tensor: &MPSGraphTensor,
            iou_threshold: c_float,
            score_threshold: c_float,
            per_class_suppression: bool,
            coordinate_mode: MPSGraphNonMaximumSuppressionCoordinateMode,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;
    }
);
