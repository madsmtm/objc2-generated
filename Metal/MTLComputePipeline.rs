//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcomputepipelinereflection?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLComputePipelineReflection;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLComputePipelineReflection {}
);

impl MTLComputePipelineReflection {
    extern_methods!(
        #[cfg(feature = "MTLArgument")]
        #[unsafe(method(bindings))]
        #[unsafe(method_family = none)]
        pub unsafe fn bindings(&self) -> Retained<NSArray<ProtocolObject<dyn MTLBinding>>>;

        #[cfg(feature = "MTLArgument")]
        #[deprecated]
        #[unsafe(method(arguments))]
        #[unsafe(method_family = none)]
        pub fn arguments(&self) -> Retained<NSArray<MTLArgument>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLComputePipelineReflection {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLComputePipelineDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLComputePipelineDescriptor {}
);

unsafe impl CopyingHelper for MTLComputePipelineDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLComputePipelineDescriptor {}
);

impl MTLComputePipelineDescriptor {
    extern_methods!(
        /// A string to help identify this object.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        pub fn label(&self) -> Option<Retained<NSString>>;

        /// Setter for [`label`][Self::label].
        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        pub fn setLabel(&self, label: Option<&NSString>);

        #[cfg(feature = "MTLLibrary")]
        /// The function to use with the MTLComputePipelineState
        #[unsafe(method(computeFunction))]
        #[unsafe(method_family = none)]
        pub fn computeFunction(&self) -> Option<Retained<ProtocolObject<dyn MTLFunction>>>;

        #[cfg(feature = "MTLLibrary")]
        /// Setter for [`computeFunction`][Self::computeFunction].
        #[unsafe(method(setComputeFunction:))]
        #[unsafe(method_family = none)]
        pub fn setComputeFunction(
            &self,
            compute_function: Option<&ProtocolObject<dyn MTLFunction>>,
        );

        /// An optimization flag, set if the thread group size will always be a multiple of thread execution width
        #[unsafe(method(threadGroupSizeIsMultipleOfThreadExecutionWidth))]
        #[unsafe(method_family = none)]
        pub fn threadGroupSizeIsMultipleOfThreadExecutionWidth(&self) -> bool;

        /// Setter for [`threadGroupSizeIsMultipleOfThreadExecutionWidth`][Self::threadGroupSizeIsMultipleOfThreadExecutionWidth].
        #[unsafe(method(setThreadGroupSizeIsMultipleOfThreadExecutionWidth:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setThreadGroupSizeIsMultipleOfThreadExecutionWidth(
            &self,
            thread_group_size_is_multiple_of_thread_execution_width: bool,
        );

        /// Optional property. Set the maxTotalThreadsPerThreadgroup. If it is not set, returns zero.
        #[unsafe(method(maxTotalThreadsPerThreadgroup))]
        #[unsafe(method_family = none)]
        pub fn maxTotalThreadsPerThreadgroup(&self) -> NSUInteger;

        /// Setter for [`maxTotalThreadsPerThreadgroup`][Self::maxTotalThreadsPerThreadgroup].
        #[unsafe(method(setMaxTotalThreadsPerThreadgroup:))]
        #[unsafe(method_family = none)]
        pub fn setMaxTotalThreadsPerThreadgroup(
            &self,
            max_total_threads_per_threadgroup: NSUInteger,
        );

        #[cfg(feature = "MTLStageInputOutputDescriptor")]
        /// An MTLStageInputOutputDescriptor to fetch data from buffers
        #[unsafe(method(stageInputDescriptor))]
        #[unsafe(method_family = none)]
        pub fn stageInputDescriptor(&self) -> Option<Retained<MTLStageInputOutputDescriptor>>;

        #[cfg(feature = "MTLStageInputOutputDescriptor")]
        /// Setter for [`stageInputDescriptor`][Self::stageInputDescriptor].
        #[unsafe(method(setStageInputDescriptor:))]
        #[unsafe(method_family = none)]
        pub fn setStageInputDescriptor(
            &self,
            stage_input_descriptor: Option<&MTLStageInputOutputDescriptor>,
        );

        #[cfg(feature = "MTLPipeline")]
        /// Optional properties for each buffer binding used by the compute function.
        #[unsafe(method(buffers))]
        #[unsafe(method_family = none)]
        pub fn buffers(&self) -> Retained<MTLPipelineBufferDescriptorArray>;

        /// This flag makes this pipeline usable with indirect command buffers.
        #[unsafe(method(supportIndirectCommandBuffers))]
        #[unsafe(method_family = none)]
        pub fn supportIndirectCommandBuffers(&self) -> bool;

        /// Setter for [`supportIndirectCommandBuffers`][Self::supportIndirectCommandBuffers].
        #[unsafe(method(setSupportIndirectCommandBuffers:))]
        #[unsafe(method_family = none)]
        pub fn setSupportIndirectCommandBuffers(&self, support_indirect_command_buffers: bool);

        #[cfg(feature = "MTLDynamicLibrary")]
        /// The set of MTLDynamicLibrary to use to resolve external symbols before considering symbols from dependent MTLDynamicLibrary.
        ///
        /// Typical workflows use the libraries property of MTLCompileOptions to record dependent libraries at compile time without having to use insertLibraries.
        /// This property can be used to override symbols from dependent libraries for experimentation or evaluating alternative implementations.
        /// It can also be used to provide dynamic libraries that are dynamically created (for example, from source) that have no stable installName that can be used to automatically load from the file system.
        ///
        /// See: MTLDynamicLibrary
        #[deprecated]
        #[unsafe(method(insertLibraries))]
        #[unsafe(method_family = none)]
        pub fn insertLibraries(
            &self,
        ) -> Option<Retained<NSArray<ProtocolObject<dyn MTLDynamicLibrary>>>>;

        #[cfg(feature = "MTLDynamicLibrary")]
        /// Setter for [`insertLibraries`][Self::insertLibraries].
        #[deprecated]
        #[unsafe(method(setInsertLibraries:))]
        #[unsafe(method_family = none)]
        pub fn setInsertLibraries(
            &self,
            insert_libraries: Option<&NSArray<ProtocolObject<dyn MTLDynamicLibrary>>>,
        );

        #[cfg(feature = "MTLDynamicLibrary")]
        /// The set of MTLDynamicLibrary to use to resolve external symbols before considering symbols from dependent MTLDynamicLibrary.
        ///
        /// Typical workflows use the libraries property of MTLCompileOptions to record dependent libraries at compile time without having to use preloadedLibraries.
        /// This property can be used to override symbols from dependent libraries for experimentation or evaluating alternative implementations.
        /// It can also be used to provide dynamic libraries that are dynamically created (for example, from source) that have no stable installName that can be used to automatically load from the file system.
        ///
        /// See: MTLDynamicLibrary
        #[unsafe(method(preloadedLibraries))]
        #[unsafe(method_family = none)]
        pub fn preloadedLibraries(
            &self,
        ) -> Retained<NSArray<ProtocolObject<dyn MTLDynamicLibrary>>>;

        #[cfg(feature = "MTLDynamicLibrary")]
        /// Setter for [`preloadedLibraries`][Self::preloadedLibraries].
        #[unsafe(method(setPreloadedLibraries:))]
        #[unsafe(method_family = none)]
        pub fn setPreloadedLibraries(
            &self,
            preloaded_libraries: &NSArray<ProtocolObject<dyn MTLDynamicLibrary>>,
        );

        #[cfg(feature = "MTLBinaryArchive")]
        /// The set of MTLBinaryArchive to search for compiled code when creating the pipeline state.
        ///
        /// Accelerate pipeline state creation by providing archives of compiled code such that no compilation needs to happen on the fast path.
        ///
        /// See: MTLBinaryArchive
        #[unsafe(method(binaryArchives))]
        #[unsafe(method_family = none)]
        pub fn binaryArchives(
            &self,
        ) -> Option<Retained<NSArray<ProtocolObject<dyn MTLBinaryArchive>>>>;

        #[cfg(feature = "MTLBinaryArchive")]
        /// Setter for [`binaryArchives`][Self::binaryArchives].
        #[unsafe(method(setBinaryArchives:))]
        #[unsafe(method_family = none)]
        pub fn setBinaryArchives(
            &self,
            binary_archives: Option<&NSArray<ProtocolObject<dyn MTLBinaryArchive>>>,
        );

        /// Restore all compute pipeline descriptor properties to their default values.
        #[unsafe(method(reset))]
        #[unsafe(method_family = none)]
        pub fn reset(&self);

        #[cfg(feature = "MTLLinkedFunctions")]
        /// The set of functions to be linked with the pipeline state and accessed from the compute function.
        ///
        /// See: MTLLinkedFunctions
        #[unsafe(method(linkedFunctions))]
        #[unsafe(method_family = none)]
        pub fn linkedFunctions(&self) -> Option<Retained<MTLLinkedFunctions>>;

        #[cfg(feature = "MTLLinkedFunctions")]
        /// Setter for [`linkedFunctions`][Self::linkedFunctions].
        #[unsafe(method(setLinkedFunctions:))]
        #[unsafe(method_family = none)]
        pub fn setLinkedFunctions(&self, linked_functions: Option<&MTLLinkedFunctions>);

        /// This flag makes this pipeline support creating a new pipeline by adding binary functions.
        #[unsafe(method(supportAddingBinaryFunctions))]
        #[unsafe(method_family = none)]
        pub fn supportAddingBinaryFunctions(&self) -> bool;

        /// Setter for [`supportAddingBinaryFunctions`][Self::supportAddingBinaryFunctions].
        #[unsafe(method(setSupportAddingBinaryFunctions:))]
        #[unsafe(method_family = none)]
        pub fn setSupportAddingBinaryFunctions(&self, support_adding_binary_functions: bool);

        /// The maximum depth of the call stack in stack frames from the kernel. Defaults to 1 additional stack frame.
        #[unsafe(method(maxCallStackDepth))]
        #[unsafe(method_family = none)]
        pub fn maxCallStackDepth(&self) -> NSUInteger;

        /// Setter for [`maxCallStackDepth`][Self::maxCallStackDepth].
        #[unsafe(method(setMaxCallStackDepth:))]
        #[unsafe(method_family = none)]
        pub fn setMaxCallStackDepth(&self, max_call_stack_depth: NSUInteger);

        #[cfg(feature = "MTLPipeline")]
        /// Toggle that determines whether Metal Shader Validation should be enabled or disabled for the pipeline.
        ///
        /// The value can be overridden using `MTL_SHADER_VALIDATION_ENABLE_PIPELINES` or `MTL_SHADER_VALIDATION_DISABLE_PIPELINES` Environment Variables.
        #[unsafe(method(shaderValidation))]
        #[unsafe(method_family = none)]
        pub unsafe fn shaderValidation(&self) -> MTLShaderValidation;

        #[cfg(feature = "MTLPipeline")]
        /// Setter for [`shaderValidation`][Self::shaderValidation].
        #[unsafe(method(setShaderValidation:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setShaderValidation(&self, shader_validation: MTLShaderValidation);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLComputePipelineDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}

impl DefaultRetained for MTLComputePipelineDescriptor {
    #[inline]
    fn default_retained() -> Retained<Self> {
        Self::new()
    }
}

extern_protocol!(
    /// A handle to compiled code for a compute function.
    ///
    /// MTLComputePipelineState is a single compute function.  It can only be used with the device that it was created against.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate?language=objc)
    pub unsafe trait MTLComputePipelineState: NSObjectProtocol {
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        fn label(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "MTLDevice")]
        /// The device this resource was created against.  This resource can only be used with this device.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// The maximum total number of threads that can be in a single compute threadgroup.
        #[unsafe(method(maxTotalThreadsPerThreadgroup))]
        #[unsafe(method_family = none)]
        fn maxTotalThreadsPerThreadgroup(&self) -> NSUInteger;

        /// For most efficient execution, the threadgroup size should be a multiple of this when executing the kernel.
        #[unsafe(method(threadExecutionWidth))]
        #[unsafe(method_family = none)]
        fn threadExecutionWidth(&self) -> NSUInteger;

        /// The length in bytes of threadgroup memory that is statically allocated.
        #[unsafe(method(staticThreadgroupMemoryLength))]
        #[unsafe(method_family = none)]
        fn staticThreadgroupMemoryLength(&self) -> NSUInteger;

        #[cfg(feature = "MTLTypes")]
        /// Returns imageblock memory length for given image block dimensions.
        #[unsafe(method(imageblockMemoryLengthForDimensions:))]
        #[unsafe(method_family = none)]
        unsafe fn imageblockMemoryLengthForDimensions(
            &self,
            imageblock_dimensions: MTLSize,
        ) -> NSUInteger;

        /// Tells whether this pipeline state is usable through an Indirect Command Buffer.
        #[unsafe(method(supportIndirectCommandBuffers))]
        #[unsafe(method_family = none)]
        fn supportIndirectCommandBuffers(&self) -> bool;

        #[cfg(feature = "MTLTypes")]
        /// Handle of the GPU resource suitable for storing in an Argument Buffer
        #[unsafe(method(gpuResourceID))]
        #[unsafe(method_family = none)]
        unsafe fn gpuResourceID(&self) -> MTLResourceID;

        #[cfg(all(feature = "MTLFunctionHandle", feature = "MTLLibrary"))]
        /// Get the function handle for the specified function from the pipeline state.
        #[unsafe(method(functionHandleWithFunction:))]
        #[unsafe(method_family = none)]
        fn functionHandleWithFunction(
            &self,
            function: &ProtocolObject<dyn MTLFunction>,
        ) -> Option<Retained<ProtocolObject<dyn MTLFunctionHandle>>>;

        #[cfg(feature = "MTLLibrary")]
        /// Allocate a new compute pipeline state by adding binary functions to this pipeline state.
        #[unsafe(method(newComputePipelineStateWithAdditionalBinaryFunctions:error:_))]
        #[unsafe(method_family = new)]
        fn newComputePipelineStateWithAdditionalBinaryFunctions_error(
            &self,
            functions: &NSArray<ProtocolObject<dyn MTLFunction>>,
        ) -> Result<Retained<ProtocolObject<dyn MTLComputePipelineState>>, Retained<NSError>>;

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLResource",
            feature = "MTLVisibleFunctionTable"
        ))]
        /// Allocate a visible function table for the pipeline with the provided descriptor.
        #[unsafe(method(newVisibleFunctionTableWithDescriptor:))]
        #[unsafe(method_family = new)]
        fn newVisibleFunctionTableWithDescriptor(
            &self,
            descriptor: &MTLVisibleFunctionTableDescriptor,
        ) -> Option<Retained<ProtocolObject<dyn MTLVisibleFunctionTable>>>;

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLIntersectionFunctionTable",
            feature = "MTLResource"
        ))]
        /// Allocate an intersection function table for the pipeline with the provided descriptor.
        #[unsafe(method(newIntersectionFunctionTableWithDescriptor:))]
        #[unsafe(method_family = new)]
        fn newIntersectionFunctionTableWithDescriptor(
            &self,
            descriptor: &MTLIntersectionFunctionTableDescriptor,
        ) -> Option<Retained<ProtocolObject<dyn MTLIntersectionFunctionTable>>>;

        #[cfg(feature = "MTLPipeline")]
        /// Current state of Shader Validation for the pipeline.
        #[unsafe(method(shaderValidation))]
        #[unsafe(method_family = none)]
        unsafe fn shaderValidation(&self) -> MTLShaderValidation;
    }
);
