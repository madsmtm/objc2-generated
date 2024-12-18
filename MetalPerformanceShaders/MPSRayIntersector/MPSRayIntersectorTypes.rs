//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsraypackedorigindirection?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPSRayPackedOriginDirection {
    pub origin: MPSPackedFloat3,
    pub direction: MPSPackedFloat3,
}

unsafe impl Encode for MPSRayPackedOriginDirection {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[<MPSPackedFloat3>::ENCODING, <MPSPackedFloat3>::ENCODING],
    );
}

unsafe impl RefEncode for MPSRayPackedOriginDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsrayoriginmindistancedirectionmaxdistance?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPSRayOriginMinDistanceDirectionMaxDistance {
    pub origin: MPSPackedFloat3,
    pub minDistance: c_float,
    pub direction: MPSPackedFloat3,
    pub maxDistance: c_float,
}

unsafe impl Encode for MPSRayOriginMinDistanceDirectionMaxDistance {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <MPSPackedFloat3>::ENCODING,
            <c_float>::ENCODING,
            <MPSPackedFloat3>::ENCODING,
            <c_float>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MPSRayOriginMinDistanceDirectionMaxDistance {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsrayoriginmaskdirectionmaxdistance?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPSRayOriginMaskDirectionMaxDistance {
    pub origin: MPSPackedFloat3,
    pub mask: c_uint,
    pub direction: MPSPackedFloat3,
    pub maxDistance: c_float,
}

unsafe impl Encode for MPSRayOriginMaskDirectionMaxDistance {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <MPSPackedFloat3>::ENCODING,
            <c_uint>::ENCODING,
            <MPSPackedFloat3>::ENCODING,
            <c_float>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MPSRayOriginMaskDirectionMaxDistance {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsintersectiondistance?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPSIntersectionDistance {
    pub distance: c_float,
}

unsafe impl Encode for MPSIntersectionDistance {
    const ENCODING: Encoding = Encoding::Struct("?", &[<c_float>::ENCODING]);
}

unsafe impl RefEncode for MPSIntersectionDistance {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsintersectiondistanceprimitiveindex?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPSIntersectionDistancePrimitiveIndex {
    pub distance: c_float,
    pub primitiveIndex: c_uint,
}

unsafe impl Encode for MPSIntersectionDistancePrimitiveIndex {
    const ENCODING: Encoding = Encoding::Struct("?", &[<c_float>::ENCODING, <c_uint>::ENCODING]);
}

unsafe impl RefEncode for MPSIntersectionDistancePrimitiveIndex {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsintersectiondistanceprimitiveindexbufferindex?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPSIntersectionDistancePrimitiveIndexBufferIndex {
    pub distance: c_float,
    pub primitiveIndex: c_uint,
    pub bufferIndex: c_uint,
}

unsafe impl Encode for MPSIntersectionDistancePrimitiveIndexBufferIndex {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[<c_float>::ENCODING, <c_uint>::ENCODING, <c_uint>::ENCODING],
    );
}

unsafe impl RefEncode for MPSIntersectionDistancePrimitiveIndexBufferIndex {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsintersectiondistanceprimitiveindexinstanceindex?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPSIntersectionDistancePrimitiveIndexInstanceIndex {
    pub distance: c_float,
    pub primitiveIndex: c_uint,
    pub instanceIndex: c_uint,
}

unsafe impl Encode for MPSIntersectionDistancePrimitiveIndexInstanceIndex {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[<c_float>::ENCODING, <c_uint>::ENCODING, <c_uint>::ENCODING],
    );
}

unsafe impl RefEncode for MPSIntersectionDistancePrimitiveIndexInstanceIndex {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsintersectiondistanceprimitiveindexbufferindexinstanceindex?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPSIntersectionDistancePrimitiveIndexBufferIndexInstanceIndex {
    pub distance: c_float,
    pub primitiveIndex: c_uint,
    pub bufferIndex: c_uint,
    pub instanceIndex: c_uint,
}

unsafe impl Encode for MPSIntersectionDistancePrimitiveIndexBufferIndexInstanceIndex {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <c_float>::ENCODING,
            <c_uint>::ENCODING,
            <c_uint>::ENCODING,
            <c_uint>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MPSIntersectionDistancePrimitiveIndexBufferIndexInstanceIndex {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
