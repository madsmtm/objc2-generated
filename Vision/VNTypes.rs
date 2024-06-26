//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

pub type VNConfidence = c_float;

pub type VNAspectRatio = c_float;

pub type VNDegrees = c_float;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VNImageCropAndScaleOption(pub NSUInteger);
impl VNImageCropAndScaleOption {
    #[doc(alias = "VNImageCropAndScaleOptionCenterCrop")]
    pub const CenterCrop: Self = Self(0);
    #[doc(alias = "VNImageCropAndScaleOptionScaleFit")]
    pub const ScaleFit: Self = Self(1);
    #[doc(alias = "VNImageCropAndScaleOptionScaleFill")]
    pub const ScaleFill: Self = Self(2);
    #[doc(alias = "VNImageCropAndScaleOptionScaleFitRotate90CCW")]
    pub const ScaleFitRotate90CCW: Self = Self(0x100 + VNImageCropAndScaleOption::ScaleFit.0);
    #[doc(alias = "VNImageCropAndScaleOptionScaleFillRotate90CCW")]
    pub const ScaleFillRotate90CCW: Self = Self(0x100 + VNImageCropAndScaleOption::ScaleFill.0);
}

unsafe impl Encode for VNImageCropAndScaleOption {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for VNImageCropAndScaleOption {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_TYPED_ENUM
pub type VNComputeStage = NSString;

extern "C" {
    pub static VNComputeStageMain: Option<&'static VNComputeStage>;
}

extern "C" {
    pub static VNComputeStagePostProcessing: Option<&'static VNComputeStage>;
}

// NS_TYPED_ENUM
pub type VNBarcodeSymbology = NSString;

extern "C" {
    pub static VNBarcodeSymbologyAztec: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    pub static VNBarcodeSymbologyCode39: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    pub static VNBarcodeSymbologyCode39Checksum: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    pub static VNBarcodeSymbologyCode39FullASCII: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    pub static VNBarcodeSymbologyCode39FullASCIIChecksum: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    pub static VNBarcodeSymbologyCode93: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    pub static VNBarcodeSymbologyCode93i: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    pub static VNBarcodeSymbologyCode128: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    pub static VNBarcodeSymbologyDataMatrix: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    pub static VNBarcodeSymbologyEAN8: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    pub static VNBarcodeSymbologyEAN13: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    pub static VNBarcodeSymbologyI2of5: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    pub static VNBarcodeSymbologyI2of5Checksum: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    pub static VNBarcodeSymbologyITF14: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    pub static VNBarcodeSymbologyPDF417: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    pub static VNBarcodeSymbologyQR: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    pub static VNBarcodeSymbologyUPCE: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    pub static VNBarcodeSymbologyCodabar: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    pub static VNBarcodeSymbologyGS1DataBar: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    pub static VNBarcodeSymbologyGS1DataBarExpanded: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    pub static VNBarcodeSymbologyGS1DataBarLimited: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    pub static VNBarcodeSymbologyMicroPDF417: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    pub static VNBarcodeSymbologyMicroQR: Option<&'static VNBarcodeSymbology>;
}

extern "C" {
    pub static VNBarcodeSymbologyMSIPlessey: Option<&'static VNBarcodeSymbology>;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VNElementType(pub NSUInteger);
impl VNElementType {
    #[doc(alias = "VNElementTypeUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "VNElementTypeFloat")]
    pub const Float: Self = Self(1);
    #[doc(alias = "VNElementTypeDouble")]
    pub const Double: Self = Self(2);
}

unsafe impl Encode for VNElementType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for VNElementType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_TYPED_ENUM
pub type VNVideoProcessingOption = NSString;

extern "C" {
    pub static VNVideoProcessingOptionFrameCadence: Option<&'static VNVideoProcessingOption>;
}

extern "C" {
    pub static VNVideoProcessingOptionTimeInterval: Option<&'static VNVideoProcessingOption>;
}

// NS_CLOSED_ENUM
#[repr(isize)] // NSInteger
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VNChirality {
    #[doc(alias = "VNChiralityUnknown")]
    Unknown = 0,
    #[doc(alias = "VNChiralityLeft")]
    Left = -1,
    #[doc(alias = "VNChiralityRight")]
    Right = 1,
}

unsafe impl Encode for VNChirality {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for VNChirality {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_CLOSED_ENUM
#[repr(isize)] // NSInteger
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VNPointsClassification {
    #[doc(alias = "VNPointsClassificationDisconnected")]
    Disconnected = 0,
    #[doc(alias = "VNPointsClassificationOpenPath")]
    OpenPath = 1,
    #[doc(alias = "VNPointsClassificationClosedPath")]
    ClosedPath = 2,
}

unsafe impl Encode for VNPointsClassification {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for VNPointsClassification {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_CLOSED_ENUM
#[repr(isize)] // NSInteger
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VNBarcodeCompositeType {
    #[doc(alias = "VNBarcodeCompositeTypeNone")]
    None = 0,
    #[doc(alias = "VNBarcodeCompositeTypeLinked")]
    Linked = 1,
    #[doc(alias = "VNBarcodeCompositeTypeGS1TypeA")]
    GS1TypeA = 2,
    #[doc(alias = "VNBarcodeCompositeTypeGS1TypeB")]
    GS1TypeB = 3,
    #[doc(alias = "VNBarcodeCompositeTypeGS1TypeC")]
    GS1TypeC = 4,
}

unsafe impl Encode for VNBarcodeCompositeType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for VNBarcodeCompositeType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_TYPED_ENUM
pub type VNRecognizedPointKey = NSString;

// NS_TYPED_ENUM
pub type VNRecognizedPointGroupKey = NSString;

// NS_TYPED_ENUM
pub type VNAnimalBodyPoseObservationJointName = VNRecognizedPointKey;

extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameLeftEarTop:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameRightEarTop:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameLeftEarMiddle:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameRightEarMiddle:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameLeftEarBottom:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameRightEarBottom:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameLeftEye:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameRightEye:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameNose:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameNeck:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameLeftFrontElbow:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameRightFrontElbow:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameLeftFrontKnee:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameRightFrontKnee:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameLeftFrontPaw:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameRightFrontPaw:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameLeftBackElbow:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameRightBackElbow:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameLeftBackKnee:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameRightBackKnee:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameLeftBackPaw:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameRightBackPaw:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameTailTop:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameTailMiddle:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameTailBottom:
        Option<&'static VNAnimalBodyPoseObservationJointName>;
}

// NS_TYPED_ENUM
pub type VNAnimalBodyPoseObservationJointsGroupName = VNRecognizedPointGroupKey;

extern "C" {
    pub static VNAnimalBodyPoseObservationJointsGroupNameHead:
        Option<&'static VNAnimalBodyPoseObservationJointsGroupName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointsGroupNameTrunk:
        Option<&'static VNAnimalBodyPoseObservationJointsGroupName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointsGroupNameForelegs:
        Option<&'static VNAnimalBodyPoseObservationJointsGroupName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointsGroupNameHindlegs:
        Option<&'static VNAnimalBodyPoseObservationJointsGroupName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointsGroupNameTail:
        Option<&'static VNAnimalBodyPoseObservationJointsGroupName>;
}

extern "C" {
    pub static VNAnimalBodyPoseObservationJointsGroupNameAll:
        Option<&'static VNAnimalBodyPoseObservationJointsGroupName>;
}

// NS_TYPED_ENUM
pub type VNHumanBodyPose3DObservationJointName = VNRecognizedPointKey;

extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameRoot:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameRightHip:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameRightKnee:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameRightAnkle:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameLeftHip:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameLeftKnee:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameLeftAnkle:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameSpine:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameCenterShoulder:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameCenterHead:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameTopHead:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameLeftShoulder:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameLeftElbow:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameLeftWrist:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameRightShoulder:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameRightElbow:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameRightWrist:
        Option<&'static VNHumanBodyPose3DObservationJointName>;
}

// NS_TYPED_ENUM
pub type VNHumanBodyPose3DObservationJointsGroupName = VNRecognizedPointGroupKey;

extern "C" {
    pub static VNHumanBodyPose3DObservationJointsGroupNameHead:
        Option<&'static VNHumanBodyPose3DObservationJointsGroupName>;
}

extern "C" {
    pub static VNHumanBodyPose3DObservationJointsGroupNameTorso:
        Option<&'static VNHumanBodyPose3DObservationJointsGroupName>;
}

extern "C" {
    pub static VNHumanBodyPose3DObservationJointsGroupNameLeftArm:
        Option<&'static VNHumanBodyPose3DObservationJointsGroupName>;
}

extern "C" {
    pub static VNHumanBodyPose3DObservationJointsGroupNameRightArm:
        Option<&'static VNHumanBodyPose3DObservationJointsGroupName>;
}

extern "C" {
    pub static VNHumanBodyPose3DObservationJointsGroupNameLeftLeg:
        Option<&'static VNHumanBodyPose3DObservationJointsGroupName>;
}

extern "C" {
    pub static VNHumanBodyPose3DObservationJointsGroupNameRightLeg:
        Option<&'static VNHumanBodyPose3DObservationJointsGroupName>;
}

extern "C" {
    pub static VNHumanBodyPose3DObservationJointsGroupNameAll:
        Option<&'static VNHumanBodyPose3DObservationJointsGroupName>;
}
