//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsnnslice?language=objc)
    #[unsafe(super(MPSCNNKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSNNSlice;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSNNSlice {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSNNSlice {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSNNSlice {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSNNSlice {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSNNSlice {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSNNSlice {
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
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSNNSlice {
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
    unsafe impl MPSNNSlice {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);