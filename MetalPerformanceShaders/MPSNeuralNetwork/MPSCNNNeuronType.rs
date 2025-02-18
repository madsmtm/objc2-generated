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
    /// f(x) = x
    #[doc(alias = "MPSCNNNeuronTypeNone")]
    pub const None: Self = Self(0);
    /// f(x) = x >= 0 ? x : a * x;  rectified linear unit
    #[doc(alias = "MPSCNNNeuronTypeReLU")]
    pub const ReLU: Self = Self(1);
    /// f(x) = a * x + b
    #[doc(alias = "MPSCNNNeuronTypeLinear")]
    pub const Linear: Self = Self(2);
    /// f(x) = 1 / (1 + e^-x)
    #[doc(alias = "MPSCNNNeuronTypeSigmoid")]
    pub const Sigmoid: Self = Self(3);
    /// f(x) = clamp((x * a) + b, 0, 1)
    #[doc(alias = "MPSCNNNeuronTypeHardSigmoid")]
    pub const HardSigmoid: Self = Self(4);
    /// f(x) = a * tanh(b * x)
    #[doc(alias = "MPSCNNNeuronTypeTanH")]
    pub const TanH: Self = Self(5);
    /// f(x) = fabs(x)
    #[doc(alias = "MPSCNNNeuronTypeAbsolute")]
    pub const Absolute: Self = Self(6);
    /// f(x) = a * log(1 + e^(b * x))
    #[doc(alias = "MPSCNNNeuronTypeSoftPlus")]
    pub const SoftPlus: Self = Self(7);
    /// f(x) = x / (1 + abs(x))
    #[doc(alias = "MPSCNNNeuronTypeSoftSign")]
    pub const SoftSign: Self = Self(8);
    /// f(x) = x >= 0 ? x : a * (exp(x) - 1); exponential linear unit
    #[doc(alias = "MPSCNNNeuronTypeELU")]
    pub const ELU: Self = Self(9);
    /// Same as ReLU except parameter a is per channel; parameterized rectified linear unit
    #[doc(alias = "MPSCNNNeuronTypePReLU")]
    pub const PReLU: Self = Self(10);
    /// f(x) = min((x >= 0 ? x : a * x), b); clamped rectified liniear unit
    #[doc(alias = "MPSCNNNeuronTypeReLUN")]
    pub const ReLUN: Self = Self(11);
    /// f(x) = (a * x + b) ^ c
    #[doc(alias = "MPSCNNNeuronTypePower")]
    pub const Power: Self = Self(12);
    /// f(x) = c ^ (a * x + b)
    #[doc(alias = "MPSCNNNeuronTypeExponential")]
    pub const Exponential: Self = Self(13);
    /// f(x) = log_c(a * x + b)
    #[doc(alias = "MPSCNNNeuronTypeLogarithm")]
    pub const Logarithm: Self = Self(14);
    /// f(x) = (1.0 + erf(x * sqrt(0.5))) * 0.5 * x
    #[doc(alias = "MPSCNNNeuronTypeGeLU")]
    pub const GeLU: Self = Self(15);
    /// holds the number of MPSCNNNeuronTypes
    #[doc(alias = "MPSCNNNeuronTypeCount")]
    pub const Count: Self = Self(16);
}

unsafe impl Encode for MPSCNNNeuronType {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for MPSCNNNeuronType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
