//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A symbolic representation of a compute operation.
    ///
    /// `NSCopy` will take a refrence, this is so `NSDictionary` can work with the tensor.
    /// All operations are created, owned and destroyed by the graph.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshadersgraph/mpsgraphoperation?language=objc)
    #[unsafe(super(MPSGraphObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSGraphCore")]
    pub struct MPSGraphOperation;
);

#[cfg(feature = "MPSGraphCore")]
unsafe impl NSCopying for MPSGraphOperation {}

#[cfg(feature = "MPSGraphCore")]
unsafe impl CopyingHelper for MPSGraphOperation {
    type Result = Self;
}

#[cfg(feature = "MPSGraphCore")]
unsafe impl NSObjectProtocol for MPSGraphOperation {}

extern_methods!(
    #[cfg(feature = "MPSGraphCore")]
    unsafe impl MPSGraphOperation {
        #[cfg(feature = "MPSGraphTensor")]
        /// The input tensors of the operation.
        #[method_id(@__retain_semantics Other inputTensors)]
        pub unsafe fn inputTensors(&self) -> Retained<NSArray<MPSGraphTensor>>;

        #[cfg(feature = "MPSGraphTensor")]
        /// The output tensors of the operation.
        #[method_id(@__retain_semantics Other outputTensors)]
        pub unsafe fn outputTensors(&self) -> Retained<NSArray<MPSGraphTensor>>;

        /// The set of operations guaranteed to execute before this operation.
        #[method_id(@__retain_semantics Other controlDependencies)]
        pub unsafe fn controlDependencies(&self) -> Retained<NSArray<MPSGraphOperation>>;

        #[cfg(feature = "MPSGraph")]
        /// The graph on which the operation is defined.
        #[method_id(@__retain_semantics Other graph)]
        pub unsafe fn graph(&self) -> Retained<MPSGraph>;

        /// Name of the operation.
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        /// Unavailable, please utilize graph methods to create and initialize operations.
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MPSGraphCore")]
    unsafe impl MPSGraphOperation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);