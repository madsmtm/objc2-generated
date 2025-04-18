//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// Represents a NeuralEngine compute device for inference of machine learning models.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coreml/mlneuralenginecomputedevice?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLNeuralEngineComputeDevice;
);

unsafe impl Send for MLNeuralEngineComputeDevice {}

unsafe impl Sync for MLNeuralEngineComputeDevice {}

#[cfg(feature = "MLComputeDeviceProtocol")]
extern_conformance!(
    unsafe impl MLComputeDeviceProtocol for MLNeuralEngineComputeDevice {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MLNeuralEngineComputeDevice {}
);

impl MLNeuralEngineComputeDevice {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        /// The total number of cores in the NeuralEngine.
        #[unsafe(method(totalCoreCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn totalCoreCount(&self) -> NSInteger;
    );
}
