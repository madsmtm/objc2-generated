//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/ciimageaccumulator?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIImageAccumulator;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for CIImageAccumulator {}
);

impl CIImageAccumulator {
    extern_methods!(
        #[cfg(all(feature = "CIImage", feature = "objc2-core-foundation"))]
        #[unsafe(method(imageAccumulatorWithExtent:format:))]
        #[unsafe(method_family = none)]
        pub unsafe fn imageAccumulatorWithExtent_format(
            extent: CGRect,
            format: CIFormat,
        ) -> Option<Retained<Self>>;

        #[cfg(all(
            feature = "CIImage",
            feature = "objc2-core-foundation",
            feature = "objc2-core-graphics"
        ))]
        #[unsafe(method(imageAccumulatorWithExtent:format:colorSpace:))]
        #[unsafe(method_family = none)]
        pub unsafe fn imageAccumulatorWithExtent_format_colorSpace(
            extent: CGRect,
            format: CIFormat,
            color_space: &CGColorSpace,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "CIImage", feature = "objc2-core-foundation"))]
        #[unsafe(method(initWithExtent:format:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithExtent_format(
            this: Allocated<Self>,
            extent: CGRect,
            format: CIFormat,
        ) -> Option<Retained<Self>>;

        #[cfg(all(
            feature = "CIImage",
            feature = "objc2-core-foundation",
            feature = "objc2-core-graphics"
        ))]
        #[unsafe(method(initWithExtent:format:colorSpace:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithExtent_format_colorSpace(
            this: Allocated<Self>,
            extent: CGRect,
            format: CIFormat,
            color_space: &CGColorSpace,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[unsafe(method(extent))]
        #[unsafe(method_family = none)]
        pub unsafe fn extent(&self) -> CGRect;

        #[cfg(feature = "CIImage")]
        #[unsafe(method(format))]
        #[unsafe(method_family = none)]
        pub unsafe fn format(&self) -> CIFormat;

        #[cfg(feature = "CIImage")]
        #[unsafe(method(image))]
        #[unsafe(method_family = none)]
        pub unsafe fn image(&self) -> Retained<CIImage>;

        #[cfg(feature = "CIImage")]
        #[unsafe(method(setImage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setImage(&self, image: &CIImage);

        #[cfg(all(feature = "CIImage", feature = "objc2-core-foundation"))]
        #[unsafe(method(setImage:dirtyRect:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setImage_dirtyRect(&self, image: &CIImage, dirty_rect: CGRect);

        #[unsafe(method(clear))]
        #[unsafe(method_family = none)]
        pub unsafe fn clear(&self);
    );
}

/// Methods declared on superclass `NSObject`.
impl CIImageAccumulator {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
