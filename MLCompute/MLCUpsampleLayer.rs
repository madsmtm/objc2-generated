//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// An upsample layer
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/mlcompute/mlcupsamplelayer?language=objc)
    #[unsafe(super(MLCLayer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MLCLayer")]
    #[deprecated]
    pub struct MLCUpsampleLayer;
);

#[cfg(feature = "MLCLayer")]
unsafe impl NSObjectProtocol for MLCUpsampleLayer {}

extern_methods!(
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCUpsampleLayer {
        /// A NSArray
        /// <NSNumber
        /// *> representing just the width if number of entries in shape array is 1 or
        /// the height followed by width of result tensor if the number of entries in shape array is 2.
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(shape)]
        pub unsafe fn shape(&self) -> Retained<NSArray<NSNumber>>;

        #[cfg(feature = "MLCTypes")]
        /// The sampling mode to use when performing the upsample.
        #[deprecated]
        #[method(sampleMode)]
        pub unsafe fn sampleMode(&self) -> MLCSampleMode;

        /// A boolean that specifies whether the corner pixels of the source and result tensors are aligned.
        ///
        /// If True, the corner pixels of the source and result tensors are aligned, and thus preserving the values at those pixels.
        /// This only has effect when mode is 'bilinear'. Default is NO.
        #[deprecated]
        #[method(alignsCorners)]
        pub unsafe fn alignsCorners(&self) -> bool;

        /// Create an upsample layer
        ///
        /// Parameter `shape`: A NSArray
        /// <NSNumber
        /// *> representing the dimensions of the result tensor
        ///
        /// Returns: A new upsample layer.
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(layerWithShape:)]
        pub unsafe fn layerWithShape(shape: &NSArray<NSNumber>) -> Option<Retained<Self>>;

        #[cfg(feature = "MLCTypes")]
        /// Create an upsample layer
        ///
        /// Parameter `shape`: A NSArray
        /// <NSNumber
        /// *> representing the dimensions of the result tensor
        ///
        /// Parameter `sampleMode`: The upsampling algorithm to use.  Default is nearest.
        ///
        /// Parameter `alignsCorners`: Whether the corner pixels of the input and output tensors are aligned or not.
        ///
        /// Returns: A new upsample layer.
        #[deprecated]
        #[unsafe(method_family(none))]
        #[method_id(layerWithShape:sampleMode:alignsCorners:)]
        pub unsafe fn layerWithShape_sampleMode_alignsCorners(
            shape: &NSArray<NSNumber>,
            sample_mode: MLCSampleMode,
            aligns_corners: bool,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MLCLayer`
    #[cfg(feature = "MLCLayer")]
    unsafe impl MLCUpsampleLayer {
        #[deprecated]
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;

        #[deprecated]
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);
