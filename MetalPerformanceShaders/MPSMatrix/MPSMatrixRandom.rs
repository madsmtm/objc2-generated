//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsmatrixrandomdistribution?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSMatrixRandomDistribution(pub NSUInteger);
bitflags::bitflags! {
    impl MPSMatrixRandomDistribution: NSUInteger {
        #[doc(alias = "MPSMatrixRandomDistributionDefault")]
        const Default = 1;
        #[doc(alias = "MPSMatrixRandomDistributionUniform")]
        const Uniform = 2;
        #[doc(alias = "MPSMatrixRandomDistributionNormal")]
        const Normal = 3;
    }
}

unsafe impl Encode for MPSMatrixRandomDistribution {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MPSMatrixRandomDistribution {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsmatrixrandomdistributiondescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPSMatrixRandomDistributionDescriptor;
);

unsafe impl NSCopying for MPSMatrixRandomDistributionDescriptor {}

unsafe impl CopyingHelper for MPSMatrixRandomDistributionDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MPSMatrixRandomDistributionDescriptor {}

extern_methods!(
    unsafe impl MPSMatrixRandomDistributionDescriptor {
        #[method(distributionType)]
        pub unsafe fn distributionType(&self) -> MPSMatrixRandomDistribution;

        #[method(setDistributionType:)]
        pub unsafe fn setDistributionType(&self, distribution_type: MPSMatrixRandomDistribution);

        #[method(minimum)]
        pub unsafe fn minimum(&self) -> c_float;

        #[method(setMinimum:)]
        pub unsafe fn setMinimum(&self, minimum: c_float);

        #[method(maximum)]
        pub unsafe fn maximum(&self) -> c_float;

        #[method(setMaximum:)]
        pub unsafe fn setMaximum(&self, maximum: c_float);

        #[method(mean)]
        pub unsafe fn mean(&self) -> c_float;

        #[method(setMean:)]
        pub unsafe fn setMean(&self, mean: c_float);

        #[method(standardDeviation)]
        pub unsafe fn standardDeviation(&self) -> c_float;

        #[method(setStandardDeviation:)]
        pub unsafe fn setStandardDeviation(&self, standard_deviation: c_float);

        #[method_id(@__retain_semantics Other uniformDistributionDescriptorWithMinimum:maximum:)]
        pub unsafe fn uniformDistributionDescriptorWithMinimum_maximum(
            minimum: c_float,
            maximum: c_float,
        ) -> Retained<MPSMatrixRandomDistributionDescriptor>;

        #[method_id(@__retain_semantics Other normalDistributionDescriptorWithMean:standardDeviation:)]
        pub unsafe fn normalDistributionDescriptorWithMean_standardDeviation(
            mean: c_float,
            standard_deviation: c_float,
        ) -> Retained<MPSMatrixRandomDistributionDescriptor>;

        #[method_id(@__retain_semantics Other normalDistributionDescriptorWithMean:standardDeviation:minimum:maximum:)]
        pub unsafe fn normalDistributionDescriptorWithMean_standardDeviation_minimum_maximum(
            mean: c_float,
            standard_deviation: c_float,
            minimum: c_float,
            maximum: c_float,
        ) -> Retained<MPSMatrixRandomDistributionDescriptor>;

        #[method_id(@__retain_semantics Other defaultDistributionDescriptor)]
        pub unsafe fn defaultDistributionDescriptor(
        ) -> Retained<MPSMatrixRandomDistributionDescriptor>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPSMatrixRandomDistributionDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsmatrixrandom?language=objc)
    #[unsafe(super(MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSKernel")]
    pub struct MPSMatrixRandom;
);

#[cfg(feature = "MPSKernel")]
unsafe impl NSCoding for MPSMatrixRandom {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSCopying for MPSMatrixRandom {}

#[cfg(feature = "MPSKernel")]
unsafe impl CopyingHelper for MPSMatrixRandom {
    type Result = Self;
}

#[cfg(feature = "MPSKernel")]
unsafe impl NSObjectProtocol for MPSMatrixRandom {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSSecureCoding for MPSMatrixRandom {}

extern_methods!(
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSMatrixRandom {
        #[cfg(feature = "MPSCoreTypes")]
        #[method(destinationDataType)]
        pub unsafe fn destinationDataType(&self) -> MPSDataType;

        #[method(distributionType)]
        pub unsafe fn distributionType(&self) -> MPSMatrixRandomDistribution;

        #[method(batchStart)]
        pub unsafe fn batchStart(&self) -> NSUInteger;

        #[method(setBatchStart:)]
        pub unsafe fn setBatchStart(&self, batch_start: NSUInteger);

        #[method(batchSize)]
        pub unsafe fn batchSize(&self) -> NSUInteger;

        #[method(setBatchSize:)]
        pub unsafe fn setBatchSize(&self, batch_size: NSUInteger);

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[cfg(feature = "MPSMatrix")]
        #[method(encodeToCommandBuffer:destinationVector:)]
        pub unsafe fn encodeToCommandBuffer_destinationVector(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            destination_vector: &MPSVector,
        );

        #[cfg(feature = "MPSMatrix")]
        #[method(encodeToCommandBuffer:destinationMatrix:)]
        pub unsafe fn encodeToCommandBuffer_destinationMatrix(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            destination_matrix: &MPSMatrix,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSMatrixRandom {
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSMatrixRandom {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsmatrixrandommtgp32?language=objc)
    #[unsafe(super(MPSMatrixRandom, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSKernel")]
    pub struct MPSMatrixRandomMTGP32;
);

#[cfg(feature = "MPSKernel")]
unsafe impl NSCoding for MPSMatrixRandomMTGP32 {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSCopying for MPSMatrixRandomMTGP32 {}

#[cfg(feature = "MPSKernel")]
unsafe impl CopyingHelper for MPSMatrixRandomMTGP32 {
    type Result = Self;
}

#[cfg(feature = "MPSKernel")]
unsafe impl NSObjectProtocol for MPSMatrixRandomMTGP32 {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSSecureCoding for MPSMatrixRandomMTGP32 {}

extern_methods!(
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSMatrixRandomMTGP32 {
        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[cfg(feature = "MPSCoreTypes")]
        #[method_id(@__retain_semantics Init initWithDevice:destinationDataType:seed:distributionDescriptor:)]
        pub unsafe fn initWithDevice_destinationDataType_seed_distributionDescriptor(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            destination_data_type: MPSDataType,
            seed: NSUInteger,
            distribution_descriptor: &MPSMatrixRandomDistributionDescriptor,
        ) -> Retained<Self>;

        #[method(synchronizeStateOnCommandBuffer:)]
        pub unsafe fn synchronizeStateOnCommandBuffer(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
        );

        #[cfg(feature = "MPSCoreTypes")]
        #[method_id(@__retain_semantics Init initWithDevice:destinationDataType:seed:)]
        pub unsafe fn initWithDevice_destinationDataType_seed(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            destination_data_type: MPSDataType,
            seed: NSUInteger,
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
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSMatrixRandomMTGP32 {
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSMatrixRandomMTGP32 {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsmatrixrandomphilox?language=objc)
    #[unsafe(super(MPSMatrixRandom, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSKernel")]
    pub struct MPSMatrixRandomPhilox;
);

#[cfg(feature = "MPSKernel")]
unsafe impl NSCoding for MPSMatrixRandomPhilox {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSCopying for MPSMatrixRandomPhilox {}

#[cfg(feature = "MPSKernel")]
unsafe impl CopyingHelper for MPSMatrixRandomPhilox {
    type Result = Self;
}

#[cfg(feature = "MPSKernel")]
unsafe impl NSObjectProtocol for MPSMatrixRandomPhilox {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSSecureCoding for MPSMatrixRandomPhilox {}

extern_methods!(
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSMatrixRandomPhilox {
        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[cfg(feature = "MPSCoreTypes")]
        #[method_id(@__retain_semantics Init initWithDevice:destinationDataType:seed:distributionDescriptor:)]
        pub unsafe fn initWithDevice_destinationDataType_seed_distributionDescriptor(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            destination_data_type: MPSDataType,
            seed: NSUInteger,
            distribution_descriptor: &MPSMatrixRandomDistributionDescriptor,
        ) -> Retained<Self>;

        #[cfg(feature = "MPSCoreTypes")]
        #[method_id(@__retain_semantics Init initWithDevice:destinationDataType:seed:)]
        pub unsafe fn initWithDevice_destinationDataType_seed(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            destination_data_type: MPSDataType,
            seed: NSUInteger,
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
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSMatrixRandomPhilox {
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSMatrixRandomPhilox {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);