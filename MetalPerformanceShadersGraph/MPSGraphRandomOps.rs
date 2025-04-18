//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-metal-performance-shaders")]
use objc2_metal_performance_shaders::*;

use crate::*;

/// The distributions supported by random operations.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshadersgraph/mpsgraphrandomdistribution?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSGraphRandomDistribution(pub u64);
impl MPSGraphRandomDistribution {
    /// The uniform distribution, with samples drawn uniformly from [min, max) for float types, and [min, max] for integer types.
    #[doc(alias = "MPSGraphRandomDistributionUniform")]
    pub const Uniform: Self = Self(0);
    /// The normal distribution defined by mean and standard deviation.
    #[doc(alias = "MPSGraphRandomDistributionNormal")]
    pub const Normal: Self = Self(1);
    /// The normal distribution defined by mean and standard deviation, truncated to the range [min, max)
    #[doc(alias = "MPSGraphRandomDistributionTruncatedNormal")]
    pub const TruncatedNormal: Self = Self(2);
}

unsafe impl Encode for MPSGraphRandomDistribution {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MPSGraphRandomDistribution {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// The sampling method to use when generating values in the normal distribution.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshadersgraph/mpsgraphrandomnormalsamplingmethod?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSGraphRandomNormalSamplingMethod(pub u64);
impl MPSGraphRandomNormalSamplingMethod {
    /// Use inverse erf to convert uniform values to values in the normal distribution
    #[doc(alias = "MPSGraphRandomNormalSamplingInvCDF")]
    pub const InvCDF: Self = Self(0);
    /// Use Box Muller transform to convert uniform values to values in the normal distribution. For bounded distributions this is a rejection sampling method.
    #[doc(alias = "MPSGraphRandomNormalSamplingBoxMuller")]
    pub const BoxMuller: Self = Self(1);
}

unsafe impl Encode for MPSGraphRandomNormalSamplingMethod {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MPSGraphRandomNormalSamplingMethod {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// A class that describes the random operation.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshadersgraph/mpsgraphrandomopdescriptor?language=objc)
    #[unsafe(super(MPSGraphObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSGraphCore")]
    pub struct MPSGraphRandomOpDescriptor;
);

#[cfg(feature = "MPSGraphCore")]
extern_conformance!(
    unsafe impl NSCopying for MPSGraphRandomOpDescriptor {}
);

#[cfg(feature = "MPSGraphCore")]
unsafe impl CopyingHelper for MPSGraphRandomOpDescriptor {
    type Result = Self;
}

#[cfg(feature = "MPSGraphCore")]
extern_conformance!(
    unsafe impl NSObjectProtocol for MPSGraphRandomOpDescriptor {}
);

#[cfg(feature = "MPSGraphCore")]
impl MPSGraphRandomOpDescriptor {
    extern_methods!(
        /// The type of distribution to draw samples from. See MPSGraphRandomDistribution.
        #[unsafe(method(distribution))]
        #[unsafe(method_family = none)]
        pub unsafe fn distribution(&self) -> MPSGraphRandomDistribution;

        /// Setter for [`distribution`][Self::distribution].
        #[unsafe(method(setDistribution:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDistribution(&self, distribution: MPSGraphRandomDistribution);

        #[cfg(feature = "objc2-metal-performance-shaders")]
        /// The data type of the generated result values.
        ///
        /// When sampling from the uniform distribution, valid types are MPSDataTypeFloat16,
        /// MPSDataTypeFloat32, and MPSDataTypeInt32.
        /// When sampling from the normal or truncated normal distribution, valid types are
        /// MPSDataTypeFloat16 and MPSDataTypeFloat32.
        #[unsafe(method(dataType))]
        #[unsafe(method_family = none)]
        pub unsafe fn dataType(&self) -> MPSDataType;

        #[cfg(feature = "objc2-metal-performance-shaders")]
        /// Setter for [`dataType`][Self::dataType].
        #[unsafe(method(setDataType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDataType(&self, data_type: MPSDataType);

        /// The lower range of the distribution.
        ///
        /// This value is used for Uniform distributions with float data types and Truncated Normal disributions.
        /// Defaults to 0 for uniform distributions and -2 for normal distributions.
        #[unsafe(method(min))]
        #[unsafe(method_family = none)]
        pub unsafe fn min(&self) -> c_float;

        /// Setter for [`min`][Self::min].
        #[unsafe(method(setMin:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMin(&self, min: c_float);

        /// The upper range of the distribution.
        ///
        /// This value is used for Uniform distributions with float data types and Truncated Normal disributions.
        /// Defaults to 1 for uniform distributions and 2 for normal distributions.
        #[unsafe(method(max))]
        #[unsafe(method_family = none)]
        pub unsafe fn max(&self) -> c_float;

        /// Setter for [`max`][Self::max].
        #[unsafe(method(setMax:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMax(&self, max: c_float);

        /// The lower range of the distribution.
        ///
        /// This value is used for Uniform with integer data types
        /// Defaults to 0.
        #[unsafe(method(minInteger))]
        #[unsafe(method_family = none)]
        pub unsafe fn minInteger(&self) -> NSInteger;

        /// Setter for [`minInteger`][Self::minInteger].
        #[unsafe(method(setMinInteger:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMinInteger(&self, min_integer: NSInteger);

        /// The upper range of the distribution.
        ///
        /// This value is used for Uniform with integer data types
        /// Defaults to INT32_MAX for uniform distributions and 0 for normal distributions.
        #[unsafe(method(maxInteger))]
        #[unsafe(method_family = none)]
        pub unsafe fn maxInteger(&self) -> NSInteger;

        /// Setter for [`maxInteger`][Self::maxInteger].
        #[unsafe(method(setMaxInteger:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMaxInteger(&self, max_integer: NSInteger);

        /// The mean of the distribution.
        ///
        /// This value is used for Normal and Truncated Normal disributions.
        /// Defaults to 0.
        #[unsafe(method(mean))]
        #[unsafe(method_family = none)]
        pub unsafe fn mean(&self) -> c_float;

        /// Setter for [`mean`][Self::mean].
        #[unsafe(method(setMean:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMean(&self, mean: c_float);

        /// The standard deviation of the distribution.
        ///
        /// This value is used for Normal and Truncated Normal disributions.
        /// For Truncated Normal distribution this defines the standard deviation parameter of the underlying Normal distribution, that is the width
        /// of the Gaussian, not the true standard deviation of the truncated distribution which typically differs from the standard deviation of the
        /// original Normal distribution.
        /// Defaults to 0 for uniform distributions and 1 for normal distributions.
        #[unsafe(method(standardDeviation))]
        #[unsafe(method_family = none)]
        pub unsafe fn standardDeviation(&self) -> c_float;

        /// Setter for [`standardDeviation`][Self::standardDeviation].
        #[unsafe(method(setStandardDeviation:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setStandardDeviation(&self, standard_deviation: c_float);

        /// The sampling method of the distribution.
        ///
        /// This value is used for Normal and Truncated Normal disributions. See MPSGraphRandomNormalSamplingMethod.
        /// Defaults to MPSGraphRandomNormalSamplingInvCDF.
        #[unsafe(method(samplingMethod))]
        #[unsafe(method_family = none)]
        pub unsafe fn samplingMethod(&self) -> MPSGraphRandomNormalSamplingMethod;

        /// Setter for [`samplingMethod`][Self::samplingMethod].
        #[unsafe(method(setSamplingMethod:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSamplingMethod(&self, sampling_method: MPSGraphRandomNormalSamplingMethod);

        #[cfg(feature = "objc2-metal-performance-shaders")]
        /// Class method to initialize a distribution descriptor.
        #[unsafe(method(descriptorWithDistribution:dataType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn descriptorWithDistribution_dataType(
            distribution: MPSGraphRandomDistribution,
            data_type: MPSDataType,
        ) -> Option<Retained<Self>>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(feature = "MPSGraphCore")]
impl MPSGraphRandomOpDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

/// MPSGraphRandomOps.
#[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
impl MPSGraph {
    extern_methods!(
        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a tensor representing state using the Philox algorithm with given counter and key values.
        ///
        /// Generates random numbers using the Philox counter-based algorithm, for further details see:
        /// John K. Salmon, Mark A. Moraes, Ron O. Dror, and David E. Shaw. Parallel Random Numbers: As Easy as 1, 2, 3.
        /// A stateTensor generated with this API can be used in MPSGraph Random APIs which accept a stateTensor. The
        /// updated stateTensor is returned alongside the random values, and can be fed to the following random layer. In
        /// most use cases, a stateTensor should only need to be initialized once at the start of the graph. A stateTensor can
        /// be set as a target tensor of an MPSGraph execution to obtain a stateTensor serialized as an NDArray. This can be
        /// used as input to a placeholder in the graph to avoid ever needing to have a state intilization layer in an MPSGraph.
        /// This can allow for a continued stream through multiple executions of a single MPSGraph by having the final
        /// stateTensor as a target tensor passed into the following MPSGraph execution as a placeholder input. This may be
        /// helpful for training graphs in particular.
        /// ```md
        /// MPSGraph *graph = [MPSGraph new];
        /// MPSGraphTensor *stateTensor = [graph randomPhiloxStateTensorWithSeed: seed name: nil];
        /// NSArray
        /// <MPSGraphTensor
        /// *> *resultTensors0 = [graph randomUniformTensorWithShape:
        ///
        /// - Parameters:
        /// - seed: Initial counter and key values will be generated using seed.
        /// - name: Name for the operation
        /// - Returns: An MPSGraphTensor representing a random state, to be passed as an input to a random op.
        #[unsafe(method(randomPhiloxStateTensorWithSeed:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn randomPhiloxStateTensorWithSeed_name(
            &self,
            seed: NSUInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a tensor representing state using the Philox algorithm with given counter and key values.
        ///
        /// See randomPhiloxStateTensorWithSeed.
        ///
        /// - Parameters:
        /// - counterLow: The value to initilaize lower 64 bits of counter to. Philox utilizes a 128 bit counter
        /// - counterHigh: The value to initilaize upper 64 bits of counter to. Philox utilizes a 128 bit counter
        /// - key: The value to initialize the key to in Philox algorithm.
        /// - name: Name for the operation
        /// - Returns: An MPSGraphTensor representing a random state, to be passed as an input to a random op.
        #[unsafe(method(randomPhiloxStateTensorWithCounterLow:counterHigh:key:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn randomPhiloxStateTensorWithCounterLow_counterHigh_key_name(
            &self,
            counter_low: NSUInteger,
            counter_high: NSUInteger,
            key: NSUInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        /// Creates a Random op of type matching distribution in descriptor and returns random values.
        ///
        /// Returns a tensor of provided shape of random values in the distribution specified. Uses a random seed value
        /// to initalize state. No state is preserved, and subsequent calls are not guaranteed to result in a unique stream of
        /// random values.
        ///
        /// - Parameters:
        /// - shape: The shape of the tensor generated
        /// - descriptor: The descriptor of the distribution. See MPSGraphRandomOpDescriptor.
        /// - name: The name for the operation.
        /// - Returns: An MPSGraphTensor of shape containing random values in the defined range.
        #[unsafe(method(randomTensorWithShape:descriptor:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn randomTensorWithShape_descriptor_name(
            &self,
            shape: &MPSShape,
            descriptor: &MPSGraphRandomOpDescriptor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a Random op of type matching distribution in descriptor and returns random values.
        ///
        /// Returns a tensor of provided shape of random values in the distribution specified. Uses a random seed value
        /// to initalize state. No state is preserved, and subsequent calls are not guaranteed to result in a unique stream of
        /// random values.
        ///
        /// - Parameters:
        /// - shapeTensor: 1D Int32 or Int64 tensor. The shape of the tensor generated
        /// - descriptor: The descriptor of the distribution. See MPSGraphRandomOpDescriptor.
        /// - name: The name for the operation.
        /// - Returns: An MPSGraphTensor of shape containing random values in the defined range.
        #[unsafe(method(randomTensorWithShapeTensor:descriptor:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn randomTensorWithShapeTensor_descriptor_name(
            &self,
            shape_tensor: &MPSGraphTensor,
            descriptor: &MPSGraphRandomOpDescriptor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        /// Creates a Random op of type matching distribution in descriptor and returns random values.
        ///
        /// Returns a tensor of provided shape of random values in the distribution specified. Uses the provided seed value
        /// to initalize state. No state is preserved, and all calls with equal seed yield an identical stream of random values.
        ///
        /// - Parameters:
        /// - shape: The shape of the tensor generated
        /// - descriptor: The descriptor of the distribution. See MPSGraphRandomOpDescriptor.
        /// - seed: The seed to use to initialize state. All calls with equal seed yield an identical stream of random values.
        /// - name: The name for the operation.
        /// - Returns: An MPSGraphTensor of shape containing random values in the defined range.
        #[unsafe(method(randomTensorWithShape:descriptor:seed:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn randomTensorWithShape_descriptor_seed_name(
            &self,
            shape: &MPSShape,
            descriptor: &MPSGraphRandomOpDescriptor,
            seed: NSUInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a Random op of type matching distribution in descriptor and returns random values.
        ///
        /// Returns a tensor of provided shape of random values in the distribution specified. Uses the provided seed value
        /// to initalize state. No state is preserved, and all calls with equal seed yield an identical stream of random values.
        ///
        /// - Parameters:
        /// - shapeTensor: 1D Int32 or Int64 tensor. The shape of the tensor generated
        /// - descriptor: The descriptor of the distribution. See MPSGraphRandomOpDescriptor.
        /// - seed: The seed to use to initialize state. All calls with equal seed yield an identical stream of random values.
        /// - name: The name for the operation.
        /// - Returns: An MPSGraphTensor of shape containing random values in the defined range.
        #[unsafe(method(randomTensorWithShapeTensor:descriptor:seed:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn randomTensorWithShapeTensor_descriptor_seed_name(
            &self,
            shape_tensor: &MPSGraphTensor,
            descriptor: &MPSGraphRandomOpDescriptor,
            seed: NSUInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        /// Creates a Random op of type matching distribution in descriptor, and returns random values and updated state.
        ///
        /// Returns an array of 2 tensors, where the first is of provided shape of random values in the distribution specified,
        /// and the second is the updated state tensor.
        /// Uses the provided state to define a stream of random values. No state is preserved, and all calls with equal state
        /// yield an identical stream of random values. The initial stateTensor provided should be created using the MPSGraph
        /// randomPhiloxStateTensor APIs. The resulting stateTensor from this op can be passed as an argument to the following
        /// random calls to continue sampling from the stream.
        ///
        /// - Parameters:
        /// - shape: The shape of the tensor generated
        /// - descriptor: The descriptor of the distribution. See MPSGraphRandomOpDescriptor.
        /// - state: The state to define a stream of random values. All calls with equal state yield an identical stream of random values.
        /// - name: The name for the operation.
        /// - Returns: An array of MPSGraphTensor of size 2. The first MPSGraphTensor is of shape containing random values in the defined range.
        /// The second MPSGraphTensor is the updated state tensor.
        #[unsafe(method(randomTensorWithShape:descriptor:stateTensor:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn randomTensorWithShape_descriptor_stateTensor_name(
            &self,
            shape: &MPSShape,
            descriptor: &MPSGraphRandomOpDescriptor,
            state: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<NSArray<MPSGraphTensor>>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a Random op of type matching distribution in descriptor, and returns random values and updated state.
        ///
        /// Returns an array of 2 tensors, where the first is of provided shape of random values in the distribution specified,
        /// and the second is the updated state tensor.
        /// Uses the provided state to define a stream of random values. No state is preserved, and all calls with equal state
        /// yield an identical stream of random values. The initial stateTensor provided should be created using the MPSGraph
        /// randomPhiloxStateTensor APIs. The resulting stateTensor from this op can be passed as an argument to the following
        /// random calls to continue sampling from the stream.
        ///
        /// - Parameters:
        /// - shapeTensor: 1D Int32 or Int64 tensor. The shape of the tensor generated.
        /// - descriptor: The descriptor of the distribution. See MPSGraphRandomOpDescriptor.
        /// - state: The state to define a stream of random values. All calls with equal state yield an identical stream of random values.
        /// - name: The name for the operation.
        /// - Returns: An array of MPSGraphTensor of size 2. The first MPSGraphTensor is of shape containing random values in the defined range.
        /// The second MPSGraphTensor is the updated state tensor.
        #[unsafe(method(randomTensorWithShapeTensor:descriptor:stateTensor:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn randomTensorWithShapeTensor_descriptor_stateTensor_name(
            &self,
            shape_tensor: &MPSGraphTensor,
            descriptor: &MPSGraphRandomOpDescriptor,
            state: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<NSArray<MPSGraphTensor>>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        /// Creates a RandomUniform operation and returns random uniform values
        ///
        /// Returns a tensor of provided shape of random uniform values in the range [0.0, 1.0). Uses a random seed value
        /// to initalize state. No state is preserved, and subsequent calls are not guaranteed to result in a unique stream of
        /// random values.
        ///
        /// - Parameters:
        /// - shape: The shape of the tensor generated
        /// - name: The name for the operation.
        /// - Returns: An MPSGraphTensor of shape containing random values in the defined range.
        #[unsafe(method(randomUniformTensorWithShape:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn randomUniformTensorWithShape_name(
            &self,
            shape: &MPSShape,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a RandomUniform operation and returns random uniform values
        ///
        /// Returns a tensor of provided shape of random uniform values in the range [0.0, 1.0). Uses a random seed value
        /// to initalize state. No state is preserved, and subsequent calls are not guaranteed to result in a unique stream of
        /// random values.
        ///
        /// - Parameters:
        /// - shapeTensor: 1D Int32 or Int64 tensor. The shape of the tensor generated
        /// - name: The name for the operation.
        /// - Returns: An MPSGraphTensor of shape containing random values in the defined range.
        #[unsafe(method(randomUniformTensorWithShapeTensor:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn randomUniformTensorWithShapeTensor_name(
            &self,
            shape_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        /// Creates a RandomUniform operation and returns random uniform values
        ///
        /// Returns a tensor of provided shape of random uniform values in the range [0.0, 1.0). Uses the provided seed value
        /// to initalize state. No state is preserved, and all calls with equal seed yield an identical stream of random values.
        ///
        /// - Parameters:
        /// - shape: The shape of the tensor generated
        /// - seed: The seed to use to initialize state. All calls with equal seed yield an identical stream of random values.
        /// - name: The name for the operation.
        /// - Returns: An MPSGraphTensor of shape containing random values in the defined range.
        #[unsafe(method(randomUniformTensorWithShape:seed:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn randomUniformTensorWithShape_seed_name(
            &self,
            shape: &MPSShape,
            seed: NSUInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a RandomUniform operation and returns random uniform values
        ///
        /// Returns a tensor of provided shape of random uniform values in the range [0.0, 1.0). Uses the provided seed value
        /// to initalize state. No state is preserved, and all calls with equal seed yield an identical stream of random values.
        ///
        /// - Parameters:
        /// - shapeTensor: 1D Int32 or Int64 tensor. The shape of the tensor generated
        /// - seed: The seed to use to initialize state. All calls with equal seed yield an identical stream of random values.
        /// - name: The name for the operation.
        /// - Returns: An MPSGraphTensor of shape containing random values in the defined range.
        #[unsafe(method(randomUniformTensorWithShapeTensor:seed:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn randomUniformTensorWithShapeTensor_seed_name(
            &self,
            shape_tensor: &MPSGraphTensor,
            seed: NSUInteger,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(all(
            feature = "MPSGraphTensor",
            feature = "objc2-metal-performance-shaders"
        ))]
        /// Creates a RandomUniform operation and returns random uniform values and updated state
        ///
        /// Returns an array of 2 tensors, where the first is a tensor of provided shape of random uniform values in the range
        /// [0.0, 1.0), and the second is the updated state tensor.
        /// The provided state is used to define a stream of random values. No state is preserved, and all calls with equal state
        /// yield an identical stream of random values. The initial stateTensor provided should be created using the MPSGraph
        /// randomPhiloxStateTensor APIs. The resulting stateTensor from this op can be passed as an argument to the following
        /// random calls to continue sampling from the stream.
        ///
        /// - Parameters:
        /// - shape: The shape of the tensor generated
        /// - state: The state to define a stream of random values. All calls with equal state yield an identical stream of random values.
        /// - name: The name for the operation.
        /// - Returns: An array of MPSGraphTensor of size 2. The first MPSGraphTensor is of shape containing random values in the defined range.
        /// The second MPSGraphTensor is the updated state tensor.
        #[unsafe(method(randomUniformTensorWithShape:stateTensor:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn randomUniformTensorWithShape_stateTensor_name(
            &self,
            shape: &MPSShape,
            state: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<NSArray<MPSGraphTensor>>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a RandomUniform operation and returns random uniform values and updated state
        ///
        /// Returns an array of 2 tensors, where the first is a tensor of provided shape of random uniform values in the range
        /// [0.0, 1.0), and the second is the updated state tensor.
        /// The provided state is used to define a stream of random values. No state is preserved, and all calls with equal state
        /// yield an identical stream of random values. The initial stateTensor provided should be created using the MPSGraph
        /// randomPhiloxStateTensor APIs. The resulting stateTensor from this op can be passed as an argument to the following
        /// random calls to continue sampling from the stream.
        ///
        /// - Parameters:
        /// - shapeTensor: 1D Int32 or Int64 tensor. The shape of the tensor generated
        /// - state: The state to define a stream of random values. All calls with equal state yield an identical stream of random values.
        /// - name: The name for the operation.
        /// - Returns: An array of MPSGraphTensor of size 2. The first MPSGraphTensor is of shape containing random values in the defined range.
        /// The second MPSGraphTensor is the updated state tensor.
        #[unsafe(method(randomUniformTensorWithShapeTensor:stateTensor:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn randomUniformTensorWithShapeTensor_stateTensor_name(
            &self,
            shape_tensor: &MPSGraphTensor,
            state: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<NSArray<MPSGraphTensor>>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a dropout operation and returns the result
        ///
        /// Removes values in the `tensor` with a percentage chance equal to `rate`. Removed values are set to 0
        ///
        /// - Parameters:
        /// - tensor: Input tensor
        /// - rate: The rate of values to be set to 0
        /// - name: The name for the operation.
        /// - Returns: A valid MPSGraphTensor object
        #[unsafe(method(dropoutTensor:rate:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn dropoutTensor_rate_name(
            &self,
            tensor: &MPSGraphTensor,
            rate: c_double,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        /// Creates a dropout operation and returns the result
        ///
        /// Removes values in the `tensor` with a percentage chance equal to `rate`. Removed values are set to 0
        ///
        /// - Parameters:
        /// - tensor: Input tensor
        /// - rate: The rate of values to be set to 0
        /// - name: The name for the operation.
        /// - Returns: A valid MPSGraphTensor object
        #[unsafe(method(dropoutTensor:rateTensor:name:))]
        #[unsafe(method_family = none)]
        pub unsafe fn dropoutTensor_rateTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            rate: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;
    );
}
