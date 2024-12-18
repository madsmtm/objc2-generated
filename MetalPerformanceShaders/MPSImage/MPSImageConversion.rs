//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimageconversion?language=objc)
    #[unsafe(super(MPSUnaryImageKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    pub struct MPSImageConversion;
);

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSImageConversion {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSImageConversion {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSImageConversion {
    type Result = Self;
}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSImageConversion {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSImageConversion {}

extern_methods!(
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageConversion {
        #[cfg(feature = "MPSImageTypes")]
        #[method(sourceAlpha)]
        pub unsafe fn sourceAlpha(&self) -> MPSAlphaType;

        #[cfg(feature = "MPSImageTypes")]
        #[method(destinationAlpha)]
        pub unsafe fn destinationAlpha(&self) -> MPSAlphaType;

        #[cfg(all(
            feature = "MPSImageTypes",
            feature = "objc2-core-foundation",
            feature = "objc2-core-graphics"
        ))]
        #[method_id(@__retain_semantics Init initWithDevice:srcAlpha:destAlpha:backgroundColor:conversionInfo:)]
        pub unsafe fn initWithDevice_srcAlpha_destAlpha_backgroundColor_conversionInfo(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            src_alpha: MPSAlphaType,
            dest_alpha: MPSAlphaType,
            background_color: *mut CGFloat,
            conversion_info: CGColorConversionInfoRef,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSUnaryImageKernel`
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageConversion {
        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

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
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageConversion {
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageConversion {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
