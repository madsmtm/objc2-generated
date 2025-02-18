//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/videotoolbox/vtint32point?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VTInt32Point {
    pub x: i32,
    pub y: i32,
}

#[cfg(feature = "objc2")]
unsafe impl Encode for VTInt32Point {
    const ENCODING: Encoding =
        Encoding::Struct("VTInt32Point", &[<i32>::ENCODING, <i32>::ENCODING]);
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for VTInt32Point {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/videotoolbox/vtint32size?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VTInt32Size {
    pub width: i32,
    pub height: i32,
}

#[cfg(feature = "objc2")]
unsafe impl Encode for VTInt32Size {
    const ENCODING: Encoding = Encoding::Struct("VTInt32Size", &[<i32>::ENCODING, <i32>::ENCODING]);
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for VTInt32Size {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
