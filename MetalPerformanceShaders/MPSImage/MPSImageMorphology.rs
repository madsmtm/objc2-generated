//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimageareamax?language=objc)
    #[unsafe(super(MPSUnaryImageKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    pub struct MPSImageAreaMax;
);

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSImageAreaMax {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSImageAreaMax {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSImageAreaMax {
    type Result = Self;
}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSImageAreaMax {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSImageAreaMax {}

extern_methods!(
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageAreaMax {
        #[method(kernelHeight)]
        pub unsafe fn kernelHeight(&self) -> NSUInteger;

        #[method(kernelWidth)]
        pub unsafe fn kernelWidth(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Init initWithDevice:kernelWidth:kernelHeight:)]
        pub unsafe fn initWithDevice_kernelWidth_kernelHeight(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageAreaMax {
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
    unsafe impl MPSImageAreaMax {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimageareamin?language=objc)
    #[unsafe(super(MPSImageAreaMax, MPSUnaryImageKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    pub struct MPSImageAreaMin;
);

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSImageAreaMin {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSImageAreaMin {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSImageAreaMin {
    type Result = Self;
}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSImageAreaMin {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSImageAreaMin {}

extern_methods!(
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageAreaMin {}
);

extern_methods!(
    /// Methods declared on superclass `MPSImageAreaMax`
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageAreaMin {
        #[method_id(@__retain_semantics Init initWithDevice:kernelWidth:kernelHeight:)]
        pub unsafe fn initWithDevice_kernelWidth_kernelHeight(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageAreaMin {
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
    unsafe impl MPSImageAreaMin {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimagedilate?language=objc)
    #[unsafe(super(MPSUnaryImageKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    pub struct MPSImageDilate;
);

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSImageDilate {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSImageDilate {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSImageDilate {
    type Result = Self;
}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSImageDilate {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSImageDilate {}

extern_methods!(
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageDilate {
        #[method(kernelHeight)]
        pub unsafe fn kernelHeight(&self) -> NSUInteger;

        #[method(kernelWidth)]
        pub unsafe fn kernelWidth(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Init initWithDevice:kernelWidth:kernelHeight:values:)]
        pub unsafe fn initWithDevice_kernelWidth_kernelHeight_values(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
            values: NonNull<c_float>,
        ) -> Retained<Self>;

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
    unsafe impl MPSImageDilate {
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
    unsafe impl MPSImageDilate {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimageerode?language=objc)
    #[unsafe(super(MPSImageDilate, MPSUnaryImageKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    pub struct MPSImageErode;
);

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSImageErode {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSImageErode {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSImageErode {
    type Result = Self;
}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSImageErode {}

#[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSImageErode {}

extern_methods!(
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageErode {}
);

extern_methods!(
    /// Methods declared on superclass `MPSImageDilate`
    #[cfg(all(feature = "MPSImageKernel", feature = "MPSKernel"))]
    unsafe impl MPSImageErode {
        #[method_id(@__retain_semantics Init initWithDevice:kernelWidth:kernelHeight:values:)]
        pub unsafe fn initWithDevice_kernelWidth_kernelHeight_values(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
            values: NonNull<c_float>,
        ) -> Retained<Self>;

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
    unsafe impl MPSImageErode {
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
    unsafe impl MPSImageErode {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);