//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_metal::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshadersgraph/mpsgraphdevicetype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSGraphDeviceType(pub u32);
impl MPSGraphDeviceType {
    #[doc(alias = "MPSGraphDeviceTypeMetal")]
    pub const Metal: Self = Self(0);
}

unsafe impl Encode for MPSGraphDeviceType {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for MPSGraphDeviceType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshadersgraph/mpsgraphdevice?language=objc)
    #[unsafe(super(MPSGraphObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSGraphCore")]
    pub struct MPSGraphDevice;
);

#[cfg(feature = "MPSGraphCore")]
unsafe impl NSObjectProtocol for MPSGraphDevice {}

extern_methods!(
    #[cfg(feature = "MPSGraphCore")]
    unsafe impl MPSGraphDevice {
        #[method(type)]
        pub unsafe fn r#type(&self) -> MPSGraphDeviceType;

        #[method_id(@__retain_semantics Other metalDevice)]
        pub unsafe fn metalDevice(&self) -> Option<Retained<ProtocolObject<dyn MTLDevice>>>;

        #[method_id(@__retain_semantics Other deviceWithMTLDevice:)]
        pub unsafe fn deviceWithMTLDevice(
            metal_device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MPSGraphCore")]
    unsafe impl MPSGraphDevice {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
