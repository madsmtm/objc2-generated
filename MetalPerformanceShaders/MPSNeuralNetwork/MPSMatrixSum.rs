//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsmatrixsum?language=objc)
    #[unsafe(super(MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSKernel")]
    pub struct MPSMatrixSum;
);

#[cfg(feature = "MPSKernel")]
unsafe impl NSCoding for MPSMatrixSum {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSCopying for MPSMatrixSum {}

#[cfg(feature = "MPSKernel")]
unsafe impl CopyingHelper for MPSMatrixSum {
    type Result = Self;
}

#[cfg(feature = "MPSKernel")]
unsafe impl NSObjectProtocol for MPSMatrixSum {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSSecureCoding for MPSMatrixSum {}

extern_methods!(
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSMatrixSum {
        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDevice:count:rows:columns:transpose:)]
        pub unsafe fn initWithDevice_count_rows_columns_transpose(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            count: NSUInteger,
            rows: NSUInteger,
            columns: NSUInteger,
            transpose: bool,
        ) -> Retained<Self>;

        #[method(rows)]
        pub unsafe fn rows(&self) -> NSUInteger;

        #[method(columns)]
        pub unsafe fn columns(&self) -> NSUInteger;

        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;

        #[method(transpose)]
        pub unsafe fn transpose(&self) -> bool;

        #[method(resultMatrixOrigin)]
        pub unsafe fn resultMatrixOrigin(&self) -> MTLOrigin;

        #[method(setResultMatrixOrigin:)]
        pub unsafe fn setResultMatrixOrigin(&self, result_matrix_origin: MTLOrigin);

        #[cfg(feature = "MPSCNNNeuronType")]
        #[method(setNeuronType:parameterA:parameterB:parameterC:)]
        pub unsafe fn setNeuronType_parameterA_parameterB_parameterC(
            &self,
            neuron_type: MPSCNNNeuronType,
            parameter_a: c_float,
            parameter_b: c_float,
            parameter_c: c_float,
        );

        #[cfg(feature = "MPSCNNNeuronType")]
        #[method(neuronType)]
        pub unsafe fn neuronType(&self) -> MPSCNNNeuronType;

        #[method(neuronParameterA)]
        pub unsafe fn neuronParameterA(&self) -> c_float;

        #[method(neuronParameterB)]
        pub unsafe fn neuronParameterB(&self) -> c_float;

        #[method(neuronParameterC)]
        pub unsafe fn neuronParameterC(&self) -> c_float;

        #[cfg(feature = "MPSMatrix")]
        #[method(encodeToCommandBuffer:sourceMatrices:resultMatrix:scaleVector:offsetVector:biasVector:startIndex:)]
        pub unsafe fn encodeToCommandBuffer_sourceMatrices_resultMatrix_scaleVector_offsetVector_biasVector_startIndex(
            &self,
            buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source_matrices: &NSArray<MPSMatrix>,
            result_matrix: &MPSMatrix,
            scale_vector: Option<&MPSVector>,
            offset_vector: Option<&MPSVector>,
            bias_vector: Option<&MPSVector>,
            start_index: NSUInteger,
        );

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
    unsafe impl MPSMatrixSum {
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
    unsafe impl MPSMatrixSum {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
