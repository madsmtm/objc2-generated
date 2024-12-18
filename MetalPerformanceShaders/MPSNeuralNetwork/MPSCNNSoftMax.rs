//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnsoftmax?language=objc)
    #[unsafe(super(MPSCNNKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNSoftMax;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNSoftMax {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNSoftMax {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNSoftMax {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNSoftMax {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNSoftMax {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNSoftMax {}
);

extern_methods!(
    /// Methods declared on superclass `MPSCNNKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNSoftMax {
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
    unsafe impl MPSCNNSoftMax {
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
    unsafe impl MPSCNNSoftMax {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnsoftmaxgradient?language=objc)
    #[unsafe(super(MPSCNNGradientKernel, MPSCNNBinaryKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNSoftMaxGradient;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNSoftMaxGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNSoftMaxGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNSoftMaxGradient {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNSoftMaxGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNSoftMaxGradient {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNSoftMaxGradient {
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
    unsafe impl MPSCNNSoftMaxGradient {
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
    unsafe impl MPSCNNSoftMaxGradient {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnlogsoftmax?language=objc)
    #[unsafe(super(MPSCNNKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNLogSoftMax;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNLogSoftMax {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNLogSoftMax {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNLogSoftMax {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNLogSoftMax {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNLogSoftMax {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNLogSoftMax {}
);

extern_methods!(
    /// Methods declared on superclass `MPSCNNKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNLogSoftMax {
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
    unsafe impl MPSCNNLogSoftMax {
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
    unsafe impl MPSCNNLogSoftMax {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnlogsoftmaxgradient?language=objc)
    #[unsafe(super(MPSCNNGradientKernel, MPSCNNBinaryKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNLogSoftMaxGradient;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNLogSoftMaxGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNLogSoftMaxGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNLogSoftMaxGradient {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNLogSoftMaxGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNLogSoftMaxGradient {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNLogSoftMaxGradient {
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
    unsafe impl MPSCNNLogSoftMaxGradient {
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
    unsafe impl MPSCNNLogSoftMaxGradient {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
