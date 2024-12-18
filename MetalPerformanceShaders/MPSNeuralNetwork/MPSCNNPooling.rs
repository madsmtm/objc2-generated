//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnpooling?language=objc)
    #[unsafe(super(MPSCNNKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNPooling;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNPooling {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNPooling {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNPooling {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNPooling {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNPooling {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNPooling {
        #[method_id(@__retain_semantics Init initWithDevice:kernelWidth:kernelHeight:)]
        pub unsafe fn initWithDevice_kernelWidth_kernelHeight(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:kernelWidth:kernelHeight:strideInPixelsX:strideInPixelsY:)]
        pub unsafe fn initWithDevice_kernelWidth_kernelHeight_strideInPixelsX_strideInPixelsY(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
            stride_in_pixels_x: NSUInteger,
            stride_in_pixels_y: NSUInteger,
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
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNPooling {
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
    unsafe impl MPSCNNPooling {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnpoolingmax?language=objc)
    #[unsafe(super(MPSCNNPooling, MPSCNNKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNPoolingMax;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNPoolingMax {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNPoolingMax {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNPoolingMax {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNPoolingMax {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNPoolingMax {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNPoolingMax {
        #[method_id(@__retain_semantics Init initWithDevice:kernelWidth:kernelHeight:strideInPixelsX:strideInPixelsY:)]
        pub unsafe fn initWithDevice_kernelWidth_kernelHeight_strideInPixelsX_strideInPixelsY(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
            stride_in_pixels_x: NSUInteger,
            stride_in_pixels_y: NSUInteger,
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
    /// Methods declared on superclass `MPSCNNPooling`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNPoolingMax {
        #[method_id(@__retain_semantics Init initWithDevice:kernelWidth:kernelHeight:)]
        pub unsafe fn initWithDevice_kernelWidth_kernelHeight(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNPoolingMax {
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
    unsafe impl MPSCNNPoolingMax {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnpoolingaverage?language=objc)
    #[unsafe(super(MPSCNNPooling, MPSCNNKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNPoolingAverage;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNPoolingAverage {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNPoolingAverage {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNPoolingAverage {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNPoolingAverage {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNPoolingAverage {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNPoolingAverage {
        #[method(zeroPadSizeX)]
        pub unsafe fn zeroPadSizeX(&self) -> NSUInteger;

        #[method(setZeroPadSizeX:)]
        pub unsafe fn setZeroPadSizeX(&self, zero_pad_size_x: NSUInteger);

        #[method(zeroPadSizeY)]
        pub unsafe fn zeroPadSizeY(&self) -> NSUInteger;

        #[method(setZeroPadSizeY:)]
        pub unsafe fn setZeroPadSizeY(&self, zero_pad_size_y: NSUInteger);

        #[method_id(@__retain_semantics Init initWithDevice:kernelWidth:kernelHeight:strideInPixelsX:strideInPixelsY:)]
        pub unsafe fn initWithDevice_kernelWidth_kernelHeight_strideInPixelsX_strideInPixelsY(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
            stride_in_pixels_x: NSUInteger,
            stride_in_pixels_y: NSUInteger,
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
    /// Methods declared on superclass `MPSCNNPooling`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNPoolingAverage {
        #[method_id(@__retain_semantics Init initWithDevice:kernelWidth:kernelHeight:)]
        pub unsafe fn initWithDevice_kernelWidth_kernelHeight(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNPoolingAverage {
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
    unsafe impl MPSCNNPoolingAverage {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnpoolingl2norm?language=objc)
    #[unsafe(super(MPSCNNPooling, MPSCNNKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNPoolingL2Norm;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNPoolingL2Norm {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNPoolingL2Norm {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNPoolingL2Norm {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNPoolingL2Norm {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNPoolingL2Norm {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNPoolingL2Norm {
        #[method_id(@__retain_semantics Init initWithDevice:kernelWidth:kernelHeight:strideInPixelsX:strideInPixelsY:)]
        pub unsafe fn initWithDevice_kernelWidth_kernelHeight_strideInPixelsX_strideInPixelsY(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
            stride_in_pixels_x: NSUInteger,
            stride_in_pixels_y: NSUInteger,
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
    /// Methods declared on superclass `MPSCNNPooling`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNPoolingL2Norm {
        #[method_id(@__retain_semantics Init initWithDevice:kernelWidth:kernelHeight:)]
        pub unsafe fn initWithDevice_kernelWidth_kernelHeight(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNPoolingL2Norm {
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
    unsafe impl MPSCNNPoolingL2Norm {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnndilatedpoolingmax?language=objc)
    #[unsafe(super(MPSCNNPooling, MPSCNNKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNDilatedPoolingMax;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNDilatedPoolingMax {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNDilatedPoolingMax {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNDilatedPoolingMax {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNDilatedPoolingMax {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNDilatedPoolingMax {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNDilatedPoolingMax {
        #[method(dilationRateX)]
        pub unsafe fn dilationRateX(&self) -> NSUInteger;

        #[method(dilationRateY)]
        pub unsafe fn dilationRateY(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Init initWithDevice:kernelWidth:kernelHeight:dilationRateX:dilationRateY:strideInPixelsX:strideInPixelsY:)]
        pub unsafe fn initWithDevice_kernelWidth_kernelHeight_dilationRateX_dilationRateY_strideInPixelsX_strideInPixelsY(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
            dilation_rate_x: NSUInteger,
            dilation_rate_y: NSUInteger,
            stride_in_pixels_x: NSUInteger,
            stride_in_pixels_y: NSUInteger,
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
    /// Methods declared on superclass `MPSCNNPooling`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNDilatedPoolingMax {
        #[method_id(@__retain_semantics Init initWithDevice:kernelWidth:kernelHeight:)]
        pub unsafe fn initWithDevice_kernelWidth_kernelHeight(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:kernelWidth:kernelHeight:strideInPixelsX:strideInPixelsY:)]
        pub unsafe fn initWithDevice_kernelWidth_kernelHeight_strideInPixelsX_strideInPixelsY(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
            stride_in_pixels_x: NSUInteger,
            stride_in_pixels_y: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNDilatedPoolingMax {
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
    unsafe impl MPSCNNDilatedPoolingMax {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnpoolinggradient?language=objc)
    #[unsafe(super(MPSCNNGradientKernel, MPSCNNBinaryKernel, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNPoolingGradient;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNPoolingGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNPoolingGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNPoolingGradient {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNPoolingGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNPoolingGradient {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNPoolingGradient {
        #[method(sourceSize)]
        pub unsafe fn sourceSize(&self) -> MTLSize;

        #[method(setSourceSize:)]
        pub unsafe fn setSourceSize(&self, source_size: MTLSize);

        #[method_id(@__retain_semantics Init initWithDevice:kernelWidth:kernelHeight:)]
        pub unsafe fn initWithDevice_kernelWidth_kernelHeight(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:kernelWidth:kernelHeight:strideInPixelsX:strideInPixelsY:)]
        pub unsafe fn initWithDevice_kernelWidth_kernelHeight_strideInPixelsX_strideInPixelsY(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
            stride_in_pixels_x: NSUInteger,
            stride_in_pixels_y: NSUInteger,
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
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNPoolingGradient {
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
    unsafe impl MPSCNNPoolingGradient {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnpoolingaveragegradient?language=objc)
    #[unsafe(super(
        MPSCNNPoolingGradient,
        MPSCNNGradientKernel,
        MPSCNNBinaryKernel,
        MPSKernel,
        NSObject
    ))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNPoolingAverageGradient;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNPoolingAverageGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNPoolingAverageGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNPoolingAverageGradient {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNPoolingAverageGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNPoolingAverageGradient {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNPoolingAverageGradient {
        #[method(zeroPadSizeX)]
        pub unsafe fn zeroPadSizeX(&self) -> NSUInteger;

        #[method(setZeroPadSizeX:)]
        pub unsafe fn setZeroPadSizeX(&self, zero_pad_size_x: NSUInteger);

        #[method(zeroPadSizeY)]
        pub unsafe fn zeroPadSizeY(&self) -> NSUInteger;

        #[method(setZeroPadSizeY:)]
        pub unsafe fn setZeroPadSizeY(&self, zero_pad_size_y: NSUInteger);

        #[method_id(@__retain_semantics Init initWithDevice:kernelWidth:kernelHeight:strideInPixelsX:strideInPixelsY:)]
        pub unsafe fn initWithDevice_kernelWidth_kernelHeight_strideInPixelsX_strideInPixelsY(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
            stride_in_pixels_x: NSUInteger,
            stride_in_pixels_y: NSUInteger,
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
    /// Methods declared on superclass `MPSCNNPoolingGradient`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNPoolingAverageGradient {
        #[method_id(@__retain_semantics Init initWithDevice:kernelWidth:kernelHeight:)]
        pub unsafe fn initWithDevice_kernelWidth_kernelHeight(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNPoolingAverageGradient {
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
    unsafe impl MPSCNNPoolingAverageGradient {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnpoolingmaxgradient?language=objc)
    #[unsafe(super(
        MPSCNNPoolingGradient,
        MPSCNNGradientKernel,
        MPSCNNBinaryKernel,
        MPSKernel,
        NSObject
    ))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNPoolingMaxGradient;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNPoolingMaxGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNPoolingMaxGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNPoolingMaxGradient {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNPoolingMaxGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNPoolingMaxGradient {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNPoolingMaxGradient {
        #[method_id(@__retain_semantics Init initWithDevice:kernelWidth:kernelHeight:strideInPixelsX:strideInPixelsY:)]
        pub unsafe fn initWithDevice_kernelWidth_kernelHeight_strideInPixelsX_strideInPixelsY(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
            stride_in_pixels_x: NSUInteger,
            stride_in_pixels_y: NSUInteger,
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
    /// Methods declared on superclass `MPSCNNPoolingGradient`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNPoolingMaxGradient {
        #[method_id(@__retain_semantics Init initWithDevice:kernelWidth:kernelHeight:)]
        pub unsafe fn initWithDevice_kernelWidth_kernelHeight(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNPoolingMaxGradient {
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
    unsafe impl MPSCNNPoolingMaxGradient {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnpoolingl2normgradient?language=objc)
    #[unsafe(super(
        MPSCNNPoolingGradient,
        MPSCNNGradientKernel,
        MPSCNNBinaryKernel,
        MPSKernel,
        NSObject
    ))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNPoolingL2NormGradient;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNPoolingL2NormGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNPoolingL2NormGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNPoolingL2NormGradient {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNPoolingL2NormGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNPoolingL2NormGradient {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNPoolingL2NormGradient {
        #[method_id(@__retain_semantics Init initWithDevice:kernelWidth:kernelHeight:strideInPixelsX:strideInPixelsY:)]
        pub unsafe fn initWithDevice_kernelWidth_kernelHeight_strideInPixelsX_strideInPixelsY(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
            stride_in_pixels_x: NSUInteger,
            stride_in_pixels_y: NSUInteger,
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
    /// Methods declared on superclass `MPSCNNPoolingGradient`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNPoolingL2NormGradient {
        #[method_id(@__retain_semantics Init initWithDevice:kernelWidth:kernelHeight:)]
        pub unsafe fn initWithDevice_kernelWidth_kernelHeight(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNPoolingL2NormGradient {
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
    unsafe impl MPSCNNPoolingL2NormGradient {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnndilatedpoolingmaxgradient?language=objc)
    #[unsafe(super(
        MPSCNNPoolingGradient,
        MPSCNNGradientKernel,
        MPSCNNBinaryKernel,
        MPSKernel,
        NSObject
    ))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    pub struct MPSCNNDilatedPoolingMaxGradient;
);

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSCNNDilatedPoolingMaxGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSCNNDilatedPoolingMaxGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSCNNDilatedPoolingMaxGradient {
    type Result = Self;
}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSCNNDilatedPoolingMaxGradient {}

#[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSCNNDilatedPoolingMaxGradient {}

extern_methods!(
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNDilatedPoolingMaxGradient {
        #[method_id(@__retain_semantics Init initWithDevice:kernelWidth:kernelHeight:dilationRateX:dilationRateY:strideInPixelsX:strideInPixelsY:)]
        pub unsafe fn initWithDevice_kernelWidth_kernelHeight_dilationRateX_dilationRateY_strideInPixelsX_strideInPixelsY(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
            dilation_rate_x: NSUInteger,
            dilation_rate_y: NSUInteger,
            stride_in_pixels_x: NSUInteger,
            stride_in_pixels_y: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:kernelWidth:kernelHeight:strideInPixelsX:strideInPixelsY:)]
        pub unsafe fn initWithDevice_kernelWidth_kernelHeight_strideInPixelsX_strideInPixelsY(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
            stride_in_pixels_x: NSUInteger,
            stride_in_pixels_y: NSUInteger,
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
    /// Methods declared on superclass `MPSCNNPoolingGradient`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNDilatedPoolingMaxGradient {
        #[method_id(@__retain_semantics Init initWithDevice:kernelWidth:kernelHeight:)]
        pub unsafe fn initWithDevice_kernelWidth_kernelHeight(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            kernel_width: NSUInteger,
            kernel_height: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSCNNKernel", feature = "MPSKernel"))]
    unsafe impl MPSCNNDilatedPoolingMaxGradient {
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
    unsafe impl MPSCNNDilatedPoolingMaxGradient {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
