//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// The MLCYOLOLossDescriptor specifies a YOLO loss filter descriptor.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcyololossdescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct MLCYOLOLossDescriptor;
);

unsafe impl NSCopying for MLCYOLOLossDescriptor {}

unsafe impl CopyingHelper for MLCYOLOLossDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MLCYOLOLossDescriptor {}

extern_methods!(
    unsafe impl MLCYOLOLossDescriptor {
        /// number of anchor boxes used to detect object per grid cell
        #[deprecated]
        #[method(anchorBoxCount)]
        pub unsafe fn anchorBoxCount(&self) -> NSUInteger;

        /// `NSData`containing the width and height for
        /// `anchorBoxCount`anchor boxes
        /// This
        /// `NSData`should have 2 floating-point values per anchor box which represent the width
        /// and height of the anchor box.
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(anchorBoxes)]
        pub unsafe fn anchorBoxes(&self) -> Retained<NSData>;

        /// Rescore pertains to multiplying the confidence groundTruth with IOU (intersection over union)
        /// of predicted bounding box and the groundTruth boundingBox.  The default is YES
        #[deprecated]
        #[method(shouldRescore)]
        pub unsafe fn shouldRescore(&self) -> bool;

        /// Setter for [`shouldRescore`][Self::shouldRescore].
        #[deprecated]
        #[method(setShouldRescore:)]
        pub unsafe fn setShouldRescore(&self, should_rescore: bool);

        /// The scale factor for spatial position loss and loss gradient.  The default is 10.0
        #[deprecated]
        #[method(scaleSpatialPositionLoss)]
        pub unsafe fn scaleSpatialPositionLoss(&self) -> c_float;

        /// Setter for [`scaleSpatialPositionLoss`][Self::scaleSpatialPositionLoss].
        #[deprecated]
        #[method(setScaleSpatialPositionLoss:)]
        pub unsafe fn setScaleSpatialPositionLoss(&self, scale_spatial_position_loss: c_float);

        /// The scale factor for spatial size loss and loss gradient.  The default is 10.0
        #[deprecated]
        #[method(scaleSpatialSizeLoss)]
        pub unsafe fn scaleSpatialSizeLoss(&self) -> c_float;

        /// Setter for [`scaleSpatialSizeLoss`][Self::scaleSpatialSizeLoss].
        #[deprecated]
        #[method(setScaleSpatialSizeLoss:)]
        pub unsafe fn setScaleSpatialSizeLoss(&self, scale_spatial_size_loss: c_float);

        /// The scale factor for no object confidence loss and loss gradient.  The default is 5.0
        #[deprecated]
        #[method(scaleNoObjectConfidenceLoss)]
        pub unsafe fn scaleNoObjectConfidenceLoss(&self) -> c_float;

        /// Setter for [`scaleNoObjectConfidenceLoss`][Self::scaleNoObjectConfidenceLoss].
        #[deprecated]
        #[method(setScaleNoObjectConfidenceLoss:)]
        pub unsafe fn setScaleNoObjectConfidenceLoss(
            &self,
            scale_no_object_confidence_loss: c_float,
        );

        /// The scale factor for object confidence loss and loss gradient.  The default is 100.0
        #[deprecated]
        #[method(scaleObjectConfidenceLoss)]
        pub unsafe fn scaleObjectConfidenceLoss(&self) -> c_float;

        /// Setter for [`scaleObjectConfidenceLoss`][Self::scaleObjectConfidenceLoss].
        #[deprecated]
        #[method(setScaleObjectConfidenceLoss:)]
        pub unsafe fn setScaleObjectConfidenceLoss(&self, scale_object_confidence_loss: c_float);

        /// The scale factor for no object classes loss and loss gradient.  The default is 2.0
        #[deprecated]
        #[method(scaleClassLoss)]
        pub unsafe fn scaleClassLoss(&self) -> c_float;

        /// Setter for [`scaleClassLoss`][Self::scaleClassLoss].
        #[deprecated]
        #[method(setScaleClassLoss:)]
        pub unsafe fn setScaleClassLoss(&self, scale_class_loss: c_float);

        /// If the prediction IOU with groundTruth is higher than this
        /// value we consider it a confident object presence, The default is 0.7
        #[deprecated]
        #[method(minimumIOUForObjectPresence)]
        pub unsafe fn minimumIOUForObjectPresence(&self) -> c_float;

        /// Setter for [`minimumIOUForObjectPresence`][Self::minimumIOUForObjectPresence].
        #[deprecated]
        #[method(setMinimumIOUForObjectPresence:)]
        pub unsafe fn setMinimumIOUForObjectPresence(
            &self,
            minimum_iou_for_object_presence: c_float,
        );

        /// If the prediction IOU with groundTruth is lower than this
        /// value we consider it a confident object absence.  The default is 0.3
        #[deprecated]
        #[method(maximumIOUForObjectAbsence)]
        pub unsafe fn maximumIOUForObjectAbsence(&self) -> c_float;

        /// Setter for [`maximumIOUForObjectAbsence`][Self::maximumIOUForObjectAbsence].
        #[deprecated]
        #[method(setMaximumIOUForObjectAbsence:)]
        pub unsafe fn setMaximumIOUForObjectAbsence(&self, maximum_iou_for_object_absence: c_float);

        #[deprecated]
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;

        #[deprecated]
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Create a YOLO loss descriptor object
        ///
        /// Parameter `anchorBoxes`: The anchor box data
        ///
        /// Parameter `anchorBoxCount`: The number of anchor boxes
        ///
        /// Returns: A new MLCYOLOLossDescriptor object.
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(descriptorWithAnchorBoxes:anchorBoxCount:)]
        pub unsafe fn descriptorWithAnchorBoxes_anchorBoxCount(
            anchor_boxes: &NSData,
            anchor_box_count: NSUInteger,
        ) -> Retained<Self>;
    }
);
