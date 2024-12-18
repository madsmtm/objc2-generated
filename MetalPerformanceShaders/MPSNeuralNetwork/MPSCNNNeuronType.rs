//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnneurontype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSCNNNeuronType(pub i32);
impl MPSCNNNeuronType {
    #[doc(alias = "MPSCNNNeuronTypeNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "MPSCNNNeuronTypeReLU")]
    pub const ReLU: Self = Self(1);
    #[doc(alias = "MPSCNNNeuronTypeLinear")]
    pub const Linear: Self = Self(2);
    #[doc(alias = "MPSCNNNeuronTypeSigmoid")]
    pub const Sigmoid: Self = Self(3);
    #[doc(alias = "MPSCNNNeuronTypeHardSigmoid")]
    pub const HardSigmoid: Self = Self(4);
    #[doc(alias = "MPSCNNNeuronTypeTanH")]
    pub const TanH: Self = Self(5);
    #[doc(alias = "MPSCNNNeuronTypeAbsolute")]
    pub const Absolute: Self = Self(6);
    #[doc(alias = "MPSCNNNeuronTypeSoftPlus")]
    pub const SoftPlus: Self = Self(7);
    #[doc(alias = "MPSCNNNeuronTypeSoftSign")]
    pub const SoftSign: Self = Self(8);
    #[doc(alias = "MPSCNNNeuronTypeELU")]
    pub const ELU: Self = Self(9);
    #[doc(alias = "MPSCNNNeuronTypePReLU")]
    pub const PReLU: Self = Self(10);
    #[doc(alias = "MPSCNNNeuronTypeReLUN")]
    pub const ReLUN: Self = Self(11);
    #[doc(alias = "MPSCNNNeuronTypePower")]
    pub const Power: Self = Self(12);
    #[doc(alias = "MPSCNNNeuronTypeExponential")]
    pub const Exponential: Self = Self(13);
    #[doc(alias = "MPSCNNNeuronTypeLogarithm")]
    pub const Logarithm: Self = Self(14);
    #[doc(alias = "MPSCNNNeuronTypeGeLU")]
    pub const GeLU: Self = Self(15);
    #[doc(alias = "MPSCNNNeuronTypeCount")]
    pub const Count: Self = Self(16);
}

unsafe impl Encode for MPSCNNNeuronType {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for MPSCNNNeuronType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
