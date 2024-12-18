//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnlossdatadescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPSCNNLossDataDescriptor;
);

unsafe impl NSCopying for MPSCNNLossDataDescriptor {}

unsafe impl CopyingHelper for MPSCNNLossDataDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MPSCNNLossDataDescriptor {}

extern_methods!(
    unsafe impl MPSCNNLossDataDescriptor {
        #[cfg(feature = "MPSImage")]
        #[method(layout)]
        pub unsafe fn layout(&self) -> MPSDataLayout;

        #[method(size)]
        pub unsafe fn size(&self) -> MTLSize;

        #[method(bytesPerRow)]
        pub unsafe fn bytesPerRow(&self) -> NSUInteger;

        #[method(setBytesPerRow:)]
        pub unsafe fn setBytesPerRow(&self, bytes_per_row: NSUInteger);

        #[method(bytesPerImage)]
        pub unsafe fn bytesPerImage(&self) -> NSUInteger;

        #[method(setBytesPerImage:)]
        pub unsafe fn setBytesPerImage(&self, bytes_per_image: NSUInteger);

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "MPSImage")]
        #[method_id(@__retain_semantics Other cnnLossDataDescriptorWithData:layout:size:)]
        pub unsafe fn cnnLossDataDescriptorWithData_layout_size(
            data: &NSData,
            layout: MPSDataLayout,
            size: MTLSize,
        ) -> Option<Retained<MPSCNNLossDataDescriptor>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPSCNNLossDataDescriptor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnlosslabels?language=objc)
    #[unsafe(super(MPSState, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSState")]
    pub struct MPSCNNLossLabels;
);

#[cfg(feature = "MPSState")]
unsafe impl NSObjectProtocol for MPSCNNLossLabels {}

extern_methods!(
    #[cfg(feature = "MPSState")]
    unsafe impl MPSCNNLossLabels {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:labelsDescriptor:)]
        pub unsafe fn initWithDevice_labelsDescriptor(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            labels_descriptor: &MPSCNNLossDataDescriptor,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:lossImageSize:labelsDescriptor:weightsDescriptor:)]
        pub unsafe fn initWithDevice_lossImageSize_labelsDescriptor_weightsDescriptor(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            loss_image_size: MTLSize,
            labels_descriptor: &MPSCNNLossDataDescriptor,
            weights_descriptor: Option<&MPSCNNLossDataDescriptor>,
        ) -> Retained<Self>;

        #[cfg(feature = "MPSImage")]
        #[method_id(@__retain_semantics Init initWithDevice:lossImageSize:labelsImage:weightsImage:)]
        pub unsafe fn initWithDevice_lossImageSize_labelsImage_weightsImage(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            loss_image_size: MTLSize,
            labels_image: &MPSImage,
            weights_image: Option<&MPSImage>,
        ) -> Retained<Self>;

        #[cfg(feature = "MPSImage")]
        #[method_id(@__retain_semantics Other lossImage)]
        pub unsafe fn lossImage(&self) -> Retained<MPSImage>;

        #[cfg(feature = "MPSImage")]
        #[method_id(@__retain_semantics Other labelsImage)]
        pub unsafe fn labelsImage(&self) -> Retained<MPSImage>;

        #[cfg(feature = "MPSImage")]
        #[method_id(@__retain_semantics Other weightsImage)]
        pub unsafe fn weightsImage(&self) -> Retained<MPSImage>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSState`
    #[cfg(feature = "MPSState")]
    unsafe impl MPSCNNLossLabels {
        #[method_id(@__retain_semantics Other temporaryStateWithCommandBuffer:bufferSize:)]
        pub unsafe fn temporaryStateWithCommandBuffer_bufferSize(
            cmd_buf: &ProtocolObject<dyn MTLCommandBuffer>,
            buffer_size: usize,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other temporaryStateWithCommandBuffer:textureDescriptor:)]
        pub unsafe fn temporaryStateWithCommandBuffer_textureDescriptor(
            cmd_buf: &ProtocolObject<dyn MTLCommandBuffer>,
            descriptor: &MTLTextureDescriptor,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other temporaryStateWithCommandBuffer:)]
        pub unsafe fn temporaryStateWithCommandBuffer(
            cmd_buf: &ProtocolObject<dyn MTLCommandBuffer>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:bufferSize:)]
        pub unsafe fn initWithDevice_bufferSize(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            buffer_size: usize,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:textureDescriptor:)]
        pub unsafe fn initWithDevice_textureDescriptor(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            descriptor: &MTLTextureDescriptor,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithResource:)]
        pub unsafe fn initWithResource(
            this: Allocated<Self>,
            resource: Option<&ProtocolObject<dyn MTLResource>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:resourceList:)]
        pub unsafe fn initWithDevice_resourceList(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            resource_list: &MPSStateResourceList,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other temporaryStateWithCommandBuffer:resourceList:)]
        pub unsafe fn temporaryStateWithCommandBuffer_resourceList(
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            resource_list: &MPSStateResourceList,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithResources:)]
        pub unsafe fn initWithResources(
            this: Allocated<Self>,
            resources: Option<&NSArray<ProtocolObject<dyn MTLResource>>>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MPSState")]
    unsafe impl MPSCNNLossLabels {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnlosslabelsbatch?language=objc)
#[cfg(feature = "MPSState")]
pub type MPSCNNLossLabelsBatch = NSArray<MPSCNNLossLabels>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnlossdescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPSCNNLossDescriptor;
);

unsafe impl NSCopying for MPSCNNLossDescriptor {}

unsafe impl CopyingHelper for MPSCNNLossDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MPSCNNLossDescriptor {}

extern_methods!(
    unsafe impl MPSCNNLossDescriptor {
        #[cfg(feature = "MPSCNNTypes")]
        #[method(lossType)]
        pub unsafe fn lossType(&self) -> MPSCNNLossType;

        #[cfg(feature = "MPSCNNTypes")]
        #[method(setLossType:)]
        pub unsafe fn setLossType(&self, loss_type: MPSCNNLossType);

        #[cfg(feature = "MPSCNNTypes")]
        #[method(reductionType)]
        pub unsafe fn reductionType(&self) -> MPSCNNReductionType;

        #[cfg(feature = "MPSCNNTypes")]
        #[method(setReductionType:)]
        pub unsafe fn setReductionType(&self, reduction_type: MPSCNNReductionType);

        #[method(reduceAcrossBatch)]
        pub unsafe fn reduceAcrossBatch(&self) -> bool;

        #[method(setReduceAcrossBatch:)]
        pub unsafe fn setReduceAcrossBatch(&self, reduce_across_batch: bool);

        #[method(weight)]
        pub unsafe fn weight(&self) -> c_float;

        #[method(setWeight:)]
        pub unsafe fn setWeight(&self, weight: c_float);

        #[method(labelSmoothing)]
        pub unsafe fn labelSmoothing(&self) -> c_float;

        #[method(setLabelSmoothing:)]
        pub unsafe fn setLabelSmoothing(&self, label_smoothing: c_float);

        #[method(numberOfClasses)]
        pub unsafe fn numberOfClasses(&self) -> NSUInteger;

        #[method(setNumberOfClasses:)]
        pub unsafe fn setNumberOfClasses(&self, number_of_classes: NSUInteger);

        #[method(epsilon)]
        pub unsafe fn epsilon(&self) -> c_float;

        #[method(setEpsilon:)]
        pub unsafe fn setEpsilon(&self, epsilon: c_float);

        #[method(delta)]
        pub unsafe fn delta(&self) -> c_float;

        #[method(setDelta:)]
        pub unsafe fn setDelta(&self, delta: c_float);

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "MPSCNNTypes")]
        #[method_id(@__retain_semantics Other cnnLossDescriptorWithType:reductionType:)]
        pub unsafe fn cnnLossDescriptorWithType_reductionType(
            loss_type: MPSCNNLossType,
            reduction_type: MPSCNNReductionType,
        ) -> Retained<MPSCNNLossDescriptor>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPSCNNLossDescriptor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnloss?language=objc)
    #[unsafe(super(MPSCNNKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNLoss;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNLoss {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNLoss {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNLoss {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNLoss {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNLoss {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNLoss {
        #[cfg(feature = "MPSCNNTypes")]
        #[method(lossType)]
        pub unsafe fn lossType(&self) -> MPSCNNLossType;

        #[cfg(feature = "MPSCNNTypes")]
        #[method(reductionType)]
        pub unsafe fn reductionType(&self) -> MPSCNNReductionType;

        #[method(weight)]
        pub unsafe fn weight(&self) -> c_float;

        #[method(labelSmoothing)]
        pub unsafe fn labelSmoothing(&self) -> c_float;

        #[method(numberOfClasses)]
        pub unsafe fn numberOfClasses(&self) -> NSUInteger;

        #[method(epsilon)]
        pub unsafe fn epsilon(&self) -> c_float;

        #[method(delta)]
        pub unsafe fn delta(&self) -> c_float;

        #[method(reduceAcrossBatch)]
        pub unsafe fn reduceAcrossBatch(&self) -> bool;

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:lossDescriptor:)]
        pub unsafe fn initWithDevice_lossDescriptor(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            loss_descriptor: &MPSCNNLossDescriptor,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "MPSImage", feature = "MPSState"))]
        #[method(encodeToCommandBuffer:sourceImage:labels:destinationImage:)]
        pub unsafe fn encodeToCommandBuffer_sourceImage_labels_destinationImage(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_image: &MPSImage,
            labels: &MPSCNNLossLabels,
            destination_image: &MPSImage,
        );

        #[cfg(all(feature = "MPSImage", feature = "MPSState"))]
        #[method_id(@__retain_semantics Other encodeToCommandBuffer:sourceImage:labels:)]
        pub unsafe fn encodeToCommandBuffer_sourceImage_labels(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_image: &MPSImage,
            labels: &MPSCNNLossLabels,
        ) -> Retained<MPSImage>;

        #[cfg(all(feature = "MPSImage", feature = "MPSNDArray", feature = "MPSState"))]
        #[method(encodeBatchToCommandBuffer:sourceImages:labels:destinationImages:)]
        pub unsafe fn encodeBatchToCommandBuffer_sourceImages_labels_destinationImages(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_image: &MPSImageBatch,
            labels: &MPSCNNLossLabelsBatch,
            destination_image: &MPSImageBatch,
        );

        #[cfg(all(feature = "MPSImage", feature = "MPSNDArray", feature = "MPSState"))]
        #[method_id(@__retain_semantics Other encodeBatchToCommandBuffer:sourceImages:labels:)]
        pub unsafe fn encodeBatchToCommandBuffer_sourceImages_labels(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_image: &MPSImageBatch,
            labels: &MPSCNNLossLabelsBatch,
        ) -> Retained<MPSImageBatch>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNLoss {
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNLoss {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnyololossdescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPSCNNYOLOLossDescriptor;
);

unsafe impl NSCopying for MPSCNNYOLOLossDescriptor {}

unsafe impl CopyingHelper for MPSCNNYOLOLossDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MPSCNNYOLOLossDescriptor {}

extern_methods!(
    unsafe impl MPSCNNYOLOLossDescriptor {
        #[method_id(@__retain_semantics Other XYLossDescriptor)]
        pub unsafe fn XYLossDescriptor(&self) -> Retained<MPSCNNLossDescriptor>;

        #[method(setXYLossDescriptor:)]
        pub unsafe fn setXYLossDescriptor(&self, xy_loss_descriptor: &MPSCNNLossDescriptor);

        #[method_id(@__retain_semantics Other WHLossDescriptor)]
        pub unsafe fn WHLossDescriptor(&self) -> Retained<MPSCNNLossDescriptor>;

        #[method(setWHLossDescriptor:)]
        pub unsafe fn setWHLossDescriptor(&self, wh_loss_descriptor: &MPSCNNLossDescriptor);

        #[method_id(@__retain_semantics Other confidenceLossDescriptor)]
        pub unsafe fn confidenceLossDescriptor(&self) -> Retained<MPSCNNLossDescriptor>;

        #[method(setConfidenceLossDescriptor:)]
        pub unsafe fn setConfidenceLossDescriptor(
            &self,
            confidence_loss_descriptor: &MPSCNNLossDescriptor,
        );

        #[method_id(@__retain_semantics Other classesLossDescriptor)]
        pub unsafe fn classesLossDescriptor(&self) -> Retained<MPSCNNLossDescriptor>;

        #[method(setClassesLossDescriptor:)]
        pub unsafe fn setClassesLossDescriptor(
            &self,
            classes_loss_descriptor: &MPSCNNLossDescriptor,
        );

        #[cfg(feature = "MPSCNNTypes")]
        #[method(reductionType)]
        pub unsafe fn reductionType(&self) -> MPSCNNReductionType;

        #[cfg(feature = "MPSCNNTypes")]
        #[method(setReductionType:)]
        pub unsafe fn setReductionType(&self, reduction_type: MPSCNNReductionType);

        #[method(reduceAcrossBatch)]
        pub unsafe fn reduceAcrossBatch(&self) -> bool;

        #[method(setReduceAcrossBatch:)]
        pub unsafe fn setReduceAcrossBatch(&self, reduce_across_batch: bool);

        #[method(rescore)]
        pub unsafe fn rescore(&self) -> bool;

        #[method(setRescore:)]
        pub unsafe fn setRescore(&self, rescore: bool);

        #[method(scaleXY)]
        pub unsafe fn scaleXY(&self) -> c_float;

        #[method(setScaleXY:)]
        pub unsafe fn setScaleXY(&self, scale_xy: c_float);

        #[method(scaleWH)]
        pub unsafe fn scaleWH(&self) -> c_float;

        #[method(setScaleWH:)]
        pub unsafe fn setScaleWH(&self, scale_wh: c_float);

        #[method(scaleNoObject)]
        pub unsafe fn scaleNoObject(&self) -> c_float;

        #[method(setScaleNoObject:)]
        pub unsafe fn setScaleNoObject(&self, scale_no_object: c_float);

        #[method(scaleObject)]
        pub unsafe fn scaleObject(&self) -> c_float;

        #[method(setScaleObject:)]
        pub unsafe fn setScaleObject(&self, scale_object: c_float);

        #[method(scaleClass)]
        pub unsafe fn scaleClass(&self) -> c_float;

        #[method(setScaleClass:)]
        pub unsafe fn setScaleClass(&self, scale_class: c_float);

        #[method(minIOUForObjectPresence)]
        pub unsafe fn minIOUForObjectPresence(&self) -> c_float;

        #[method(setMinIOUForObjectPresence:)]
        pub unsafe fn setMinIOUForObjectPresence(&self, min_iou_for_object_presence: c_float);

        #[method(maxIOUForObjectAbsence)]
        pub unsafe fn maxIOUForObjectAbsence(&self) -> c_float;

        #[method(setMaxIOUForObjectAbsence:)]
        pub unsafe fn setMaxIOUForObjectAbsence(&self, max_iou_for_object_absence: c_float);

        #[method(numberOfAnchorBoxes)]
        pub unsafe fn numberOfAnchorBoxes(&self) -> NSUInteger;

        #[method(setNumberOfAnchorBoxes:)]
        pub unsafe fn setNumberOfAnchorBoxes(&self, number_of_anchor_boxes: NSUInteger);

        #[method_id(@__retain_semantics Other anchorBoxes)]
        pub unsafe fn anchorBoxes(&self) -> Retained<NSData>;

        #[method(setAnchorBoxes:)]
        pub unsafe fn setAnchorBoxes(&self, anchor_boxes: &NSData);

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "MPSCNNTypes")]
        #[method_id(@__retain_semantics Other cnnLossDescriptorWithXYLossType:WHLossType:confidenceLossType:classesLossType:reductionType:anchorBoxes:numberOfAnchorBoxes:)]
        pub unsafe fn cnnLossDescriptorWithXYLossType_WHLossType_confidenceLossType_classesLossType_reductionType_anchorBoxes_numberOfAnchorBoxes(
            xy_loss_type: MPSCNNLossType,
            wh_loss_type: MPSCNNLossType,
            confidence_loss_type: MPSCNNLossType,
            classes_loss_type: MPSCNNLossType,
            reduction_type: MPSCNNReductionType,
            anchor_boxes: &NSData,
            number_of_anchor_boxes: NSUInteger,
        ) -> Retained<MPSCNNYOLOLossDescriptor>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPSCNNYOLOLossDescriptor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnyololoss?language=objc)
    #[unsafe(super(MPSCNNKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNYOLOLoss;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNYOLOLoss {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNYOLOLoss {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNYOLOLoss {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNYOLOLoss {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNYOLOLoss {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNYOLOLoss {
        #[method_id(@__retain_semantics Other lossXY)]
        pub unsafe fn lossXY(&self) -> Retained<MPSCNNLoss>;

        #[method_id(@__retain_semantics Other lossWH)]
        pub unsafe fn lossWH(&self) -> Retained<MPSCNNLoss>;

        #[method_id(@__retain_semantics Other lossConfidence)]
        pub unsafe fn lossConfidence(&self) -> Retained<MPSCNNLoss>;

        #[method_id(@__retain_semantics Other lossClasses)]
        pub unsafe fn lossClasses(&self) -> Retained<MPSCNNLoss>;

        #[method(scaleXY)]
        pub unsafe fn scaleXY(&self) -> c_float;

        #[method(scaleWH)]
        pub unsafe fn scaleWH(&self) -> c_float;

        #[method(scaleNoObject)]
        pub unsafe fn scaleNoObject(&self) -> c_float;

        #[method(scaleObject)]
        pub unsafe fn scaleObject(&self) -> c_float;

        #[method(scaleClass)]
        pub unsafe fn scaleClass(&self) -> c_float;

        #[method(minIOUForObjectPresence)]
        pub unsafe fn minIOUForObjectPresence(&self) -> c_float;

        #[method(maxIOUForObjectAbsence)]
        pub unsafe fn maxIOUForObjectAbsence(&self) -> c_float;

        #[cfg(feature = "MPSCNNTypes")]
        #[method(reductionType)]
        pub unsafe fn reductionType(&self) -> MPSCNNReductionType;

        #[method(numberOfAnchorBoxes)]
        pub unsafe fn numberOfAnchorBoxes(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other anchorBoxes)]
        pub unsafe fn anchorBoxes(&self) -> Retained<NSData>;

        #[method(reduceAcrossBatch)]
        pub unsafe fn reduceAcrossBatch(&self) -> bool;

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:lossDescriptor:)]
        pub unsafe fn initWithDevice_lossDescriptor(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            loss_descriptor: &MPSCNNYOLOLossDescriptor,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "MPSImage", feature = "MPSState"))]
        #[method(encodeToCommandBuffer:sourceImage:labels:destinationImage:)]
        pub unsafe fn encodeToCommandBuffer_sourceImage_labels_destinationImage(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_image: &MPSImage,
            labels: &MPSCNNLossLabels,
            destination_image: &MPSImage,
        );

        #[cfg(all(feature = "MPSImage", feature = "MPSState"))]
        #[method_id(@__retain_semantics Other encodeToCommandBuffer:sourceImage:labels:)]
        pub unsafe fn encodeToCommandBuffer_sourceImage_labels(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_image: &MPSImage,
            labels: &MPSCNNLossLabels,
        ) -> Retained<MPSImage>;

        #[cfg(all(feature = "MPSImage", feature = "MPSNDArray", feature = "MPSState"))]
        #[method(encodeBatchToCommandBuffer:sourceImages:labels:destinationImages:)]
        pub unsafe fn encodeBatchToCommandBuffer_sourceImages_labels_destinationImages(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_image: &MPSImageBatch,
            labels: &MPSCNNLossLabelsBatch,
            destination_image: &MPSImageBatch,
        );

        #[cfg(all(feature = "MPSImage", feature = "MPSNDArray", feature = "MPSState"))]
        #[method_id(@__retain_semantics Other encodeBatchToCommandBuffer:sourceImages:labels:)]
        pub unsafe fn encodeBatchToCommandBuffer_sourceImages_labels(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_image: &MPSImageBatch,
            labels: &MPSCNNLossLabelsBatch,
        ) -> Retained<MPSImageBatch>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNYOLOLoss {
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNYOLOLoss {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsnnforwardloss?language=objc)
    #[unsafe(super(MPSCNNKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSNNForwardLoss;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSNNForwardLoss {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSNNForwardLoss {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSNNForwardLoss {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSNNForwardLoss {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSNNForwardLoss {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSNNForwardLoss {
        #[cfg(feature = "MPSCNNTypes")]
        #[method(lossType)]
        pub unsafe fn lossType(&self) -> MPSCNNLossType;

        #[cfg(feature = "MPSCNNTypes")]
        #[method(reductionType)]
        pub unsafe fn reductionType(&self) -> MPSCNNReductionType;

        #[method(reduceAcrossBatch)]
        pub unsafe fn reduceAcrossBatch(&self) -> bool;

        #[method(numberOfClasses)]
        pub unsafe fn numberOfClasses(&self) -> NSUInteger;

        #[method(weight)]
        pub unsafe fn weight(&self) -> c_float;

        #[method(setWeight:)]
        pub unsafe fn setWeight(&self, weight: c_float);

        #[method(labelSmoothing)]
        pub unsafe fn labelSmoothing(&self) -> c_float;

        #[method(setLabelSmoothing:)]
        pub unsafe fn setLabelSmoothing(&self, label_smoothing: c_float);

        #[method(epsilon)]
        pub unsafe fn epsilon(&self) -> c_float;

        #[method(setEpsilon:)]
        pub unsafe fn setEpsilon(&self, epsilon: c_float);

        #[method(delta)]
        pub unsafe fn delta(&self) -> c_float;

        #[method(setDelta:)]
        pub unsafe fn setDelta(&self, delta: c_float);

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:lossDescriptor:)]
        pub unsafe fn initWithDevice_lossDescriptor(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            loss_descriptor: &MPSCNNLossDescriptor,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "MPSImage", feature = "MPSNDArray", feature = "MPSState"))]
        #[method(encodeBatchToCommandBuffer:sourceImages:labels:weights:destinationStates:destinationImages:)]
        pub unsafe fn encodeBatchToCommandBuffer_sourceImages_labels_weights_destinationStates_destinationImages(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_images: &MPSImageBatch,
            labels: &MPSImageBatch,
            weights: Option<&MPSImageBatch>,
            destination_states: Option<&MPSStateBatch>,
            destination_images: &MPSImageBatch,
        );

        #[cfg(all(feature = "MPSImage", feature = "MPSNDArray", feature = "MPSState"))]
        #[method_id(@__retain_semantics Other encodeBatchToCommandBuffer:sourceImages:labels:weights:destinationStates:destinationStateIsTemporary:)]
        pub unsafe fn encodeBatchToCommandBuffer_sourceImages_labels_weights_destinationStates_destinationStateIsTemporary(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_images: &MPSImageBatch,
            labels: &MPSImageBatch,
            weights: Option<&MPSImageBatch>,
            out_states: Option<&mut Option<Retained<MPSStateBatch>>>,
            is_temporary: bool,
        ) -> Retained<MPSImageBatch>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSNNForwardLoss {
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSNNForwardLoss {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsnnlossgradient?language=objc)
    #[unsafe(super(MPSCNNBinaryKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSNNLossGradient;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSNNLossGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSNNLossGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSNNLossGradient {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSNNLossGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSNNLossGradient {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSNNLossGradient {
        #[cfg(feature = "MPSCNNTypes")]
        #[method(lossType)]
        pub unsafe fn lossType(&self) -> MPSCNNLossType;

        #[cfg(feature = "MPSCNNTypes")]
        #[method(reductionType)]
        pub unsafe fn reductionType(&self) -> MPSCNNReductionType;

        #[method(reduceAcrossBatch)]
        pub unsafe fn reduceAcrossBatch(&self) -> bool;

        #[method(numberOfClasses)]
        pub unsafe fn numberOfClasses(&self) -> NSUInteger;

        #[method(weight)]
        pub unsafe fn weight(&self) -> c_float;

        #[method(setWeight:)]
        pub unsafe fn setWeight(&self, weight: c_float);

        #[method(labelSmoothing)]
        pub unsafe fn labelSmoothing(&self) -> c_float;

        #[method(setLabelSmoothing:)]
        pub unsafe fn setLabelSmoothing(&self, label_smoothing: c_float);

        #[method(epsilon)]
        pub unsafe fn epsilon(&self) -> c_float;

        #[method(setEpsilon:)]
        pub unsafe fn setEpsilon(&self, epsilon: c_float);

        #[method(delta)]
        pub unsafe fn delta(&self) -> c_float;

        #[method(setDelta:)]
        pub unsafe fn setDelta(&self, delta: c_float);

        #[method(computeLabelGradients)]
        pub unsafe fn computeLabelGradients(&self) -> bool;

        #[method(setComputeLabelGradients:)]
        pub unsafe fn setComputeLabelGradients(&self, compute_label_gradients: bool);

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:lossDescriptor:)]
        pub unsafe fn initWithDevice_lossDescriptor(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            loss_descriptor: &MPSCNNLossDescriptor,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "MPSImage", feature = "MPSNDArray", feature = "MPSState"))]
        #[method_id(@__retain_semantics Other encodeBatchToCommandBuffer:sourceGradients:sourceImages:labels:weights:sourceStates:)]
        pub unsafe fn encodeBatchToCommandBuffer_sourceGradients_sourceImages_labels_weights_sourceStates(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_gradients: &MPSImageBatch,
            source_images: &MPSImageBatch,
            labels: &MPSImageBatch,
            weights: Option<&MPSImageBatch>,
            source_states: Option<&MPSStateBatch>,
        ) -> Retained<MPSImageBatch>;

        #[cfg(all(feature = "MPSImage", feature = "MPSNDArray", feature = "MPSState"))]
        #[method(encodeBatchToCommandBuffer:sourceGradients:sourceImages:labels:weights:sourceStates:destinationGradients:)]
        pub unsafe fn encodeBatchToCommandBuffer_sourceGradients_sourceImages_labels_weights_sourceStates_destinationGradients(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_gradients: &MPSImageBatch,
            source_images: &MPSImageBatch,
            labels: &MPSImageBatch,
            weights: Option<&MPSImageBatch>,
            source_states: Option<&MPSStateBatch>,
            destination_gradients: &MPSImageBatch,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSNNLossGradient {
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSNNLossGradient {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsnninitialgradient?language=objc)
    #[unsafe(super(MPSCNNKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSNNInitialGradient;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSNNInitialGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSNNInitialGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSNNInitialGradient {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSNNInitialGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSNNInitialGradient {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSNNInitialGradient {
        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSCNNKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSNNInitialGradient {
        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSNNInitialGradient {
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSNNInitialGradient {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
