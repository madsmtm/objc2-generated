//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// Common counters that, when present, are expected to have similar meanings across
/// different implementations.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncounter?language=objc)
// NS_TYPED_ENUM
pub type MTLCommonCounter = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncountertimestamp?language=objc)
    pub static MTLCommonCounterTimestamp: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncountertessellationinputpatches?language=objc)
    pub static MTLCommonCounterTessellationInputPatches: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncountervertexinvocations?language=objc)
    pub static MTLCommonCounterVertexInvocations: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncounterposttessellationvertexinvocations?language=objc)
    pub static MTLCommonCounterPostTessellationVertexInvocations: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncounterclipperinvocations?language=objc)
    pub static MTLCommonCounterClipperInvocations: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncounterclipperprimitivesout?language=objc)
    pub static MTLCommonCounterClipperPrimitivesOut: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncounterfragmentinvocations?language=objc)
    pub static MTLCommonCounterFragmentInvocations: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncounterfragmentspassed?language=objc)
    pub static MTLCommonCounterFragmentsPassed: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncountercomputekernelinvocations?language=objc)
    pub static MTLCommonCounterComputeKernelInvocations: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncountertotalcycles?language=objc)
    pub static MTLCommonCounterTotalCycles: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncountervertexcycles?language=objc)
    pub static MTLCommonCounterVertexCycles: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncountertessellationcycles?language=objc)
    pub static MTLCommonCounterTessellationCycles: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncounterposttessellationvertexcycles?language=objc)
    pub static MTLCommonCounterPostTessellationVertexCycles: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncounterfragmentcycles?language=objc)
    pub static MTLCommonCounterFragmentCycles: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncounterrendertargetwritecycles?language=objc)
    pub static MTLCommonCounterRenderTargetWriteCycles: &'static MTLCommonCounter;
}

/// Common counter set names.
///
/// Each of these common counter sets has a defined structure type.  Implementations
/// may omit some of the counters from these sets.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncounterset?language=objc)
// NS_TYPED_ENUM
pub type MTLCommonCounterSet = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncountersettimestamp?language=objc)
    pub static MTLCommonCounterSetTimestamp: &'static MTLCommonCounterSet;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncountersetstageutilization?language=objc)
    pub static MTLCommonCounterSetStageUtilization: &'static MTLCommonCounterSet;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncountersetstatistic?language=objc)
    pub static MTLCommonCounterSetStatistic: &'static MTLCommonCounterSet;
}

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcounterresulttimestamp?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLCounterResultTimestamp {
    pub timestamp: u64,
}

unsafe impl Encode for MTLCounterResultTimestamp {
    const ENCODING: Encoding = Encoding::Struct("?", &[<u64>::ENCODING]);
}

unsafe impl RefEncode for MTLCounterResultTimestamp {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcounterresultstageutilization?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLCounterResultStageUtilization {
    pub totalCycles: u64,
    pub vertexCycles: u64,
    pub tessellationCycles: u64,
    pub postTessellationVertexCycles: u64,
    pub fragmentCycles: u64,
    pub renderTargetCycles: u64,
}

unsafe impl Encode for MTLCounterResultStageUtilization {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTLCounterResultStageUtilization {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcounterresultstatistic?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLCounterResultStatistic {
    pub tessellationInputPatches: u64,
    pub vertexInvocations: u64,
    pub postTessellationVertexInvocations: u64,
    pub clipperInvocations: u64,
    pub clipperPrimitivesOut: u64,
    pub fragmentInvocations: u64,
    pub fragmentsPassed: u64,
    pub computeKernelInvocations: u64,
}

unsafe impl Encode for MTLCounterResultStatistic {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTLCounterResultStatistic {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// A descriptor for a single counter.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcounter?language=objc)
    pub unsafe trait MTLCounter: NSObjectProtocol {
        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        unsafe fn name(&self) -> Retained<NSString>;
    }
);

extern_protocol!(
    /// A collection of MTLCounters that the device can capture in
    /// a single pass.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcounterset?language=objc)
    pub unsafe trait MTLCounterSet: NSObjectProtocol {
        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        unsafe fn name(&self) -> Retained<NSString>;

        /// The counters array contains all the counters that will be written
        /// when a counter sample is collected.  Counters that do not appear in this array
        /// will not be written to the resolved buffer when the samples are resolved, even if
        /// they appear in the corresponding resolved counter structure.  Instead
        /// MTLCounterErrorValue will be written in the resolved buffer.
        #[unsafe(method(counters))]
        #[unsafe(method_family = none)]
        unsafe fn counters(&self) -> Retained<NSArray<ProtocolObject<dyn MTLCounter>>>;
    }
);

extern_class!(
    /// Object to represent the counter state.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcountersamplebufferdescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLCounterSampleBufferDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLCounterSampleBufferDescriptor {}
);

unsafe impl CopyingHelper for MTLCounterSampleBufferDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLCounterSampleBufferDescriptor {}
);

impl MTLCounterSampleBufferDescriptor {
    extern_methods!(
        #[unsafe(method(counterSet))]
        #[unsafe(method_family = none)]
        pub unsafe fn counterSet(&self) -> Option<Retained<ProtocolObject<dyn MTLCounterSet>>>;

        /// Setter for [`counterSet`][Self::counterSet].
        #[unsafe(method(setCounterSet:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCounterSet(&self, counter_set: Option<&ProtocolObject<dyn MTLCounterSet>>);

        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        pub unsafe fn label(&self) -> Retained<NSString>;

        /// Setter for [`label`][Self::label].
        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLabel(&self, label: &NSString);

        #[cfg(feature = "MTLResource")]
        /// MTLStorageModeShared and MTLStorageModePrivate may be used.
        #[unsafe(method(storageMode))]
        #[unsafe(method_family = none)]
        pub unsafe fn storageMode(&self) -> MTLStorageMode;

        #[cfg(feature = "MTLResource")]
        /// Setter for [`storageMode`][Self::storageMode].
        #[unsafe(method(setStorageMode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setStorageMode(&self, storage_mode: MTLStorageMode);

        /// counter sample buffer.
        #[unsafe(method(sampleCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn sampleCount(&self) -> NSUInteger;

        /// Setter for [`sampleCount`][Self::sampleCount].
        #[unsafe(method(setSampleCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setSampleCount(&self, sample_count: NSUInteger);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLCounterSampleBufferDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_protocol!(
    /// The Counter Sample Buffer contains opaque counter samples as well
    /// as state needed to request a sample from the API.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcountersamplebuffer?language=objc)
    pub unsafe trait MTLCounterSampleBuffer: NSObjectProtocol {
        #[cfg(feature = "MTLDevice")]
        /// to use the sample buffer with this device.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        unsafe fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// property of the descriptor that is used to create the sample buffer.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        unsafe fn label(&self) -> Retained<NSString>;

        #[unsafe(method(sampleCount))]
        #[unsafe(method_family = none)]
        unsafe fn sampleCount(&self) -> NSUInteger;

        /// Resolve the counters from the sample buffer to an NSData containing
        /// the counter values.  This may only be used with sample buffers that have
        /// MTLStorageModeShared.
        ///
        /// Parameter `range`: The range of indices in the sample buffer to resolve.
        ///
        /// Returns: The resolved samples.
        ///
        /// Samples that encountered an error during resolve will be set to
        /// MTLCounterErrorValue.
        #[unsafe(method(resolveCounterRange:))]
        #[unsafe(method_family = none)]
        unsafe fn resolveCounterRange(&self, range: NSRange) -> Option<Retained<NSData>>;
    }
);

extern "C" {
    /// NSErrors raised when creating a counter sample buffer.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcountererrordomain?language=objc)
    pub static MTLCounterErrorDomain: &'static NSErrorDomain;
}

/// There wasn't enough memory available to allocate the counter sample buffer.
///
///
/// Invalid parameter passed while creating counter sample buffer.
///
///
/// There was some other error in allocating the counter sample buffer.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcountersamplebuffererror?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLCounterSampleBufferError(pub NSInteger);
impl MTLCounterSampleBufferError {
    #[doc(alias = "MTLCounterSampleBufferErrorOutOfMemory")]
    pub const OutOfMemory: Self = Self(0);
    #[doc(alias = "MTLCounterSampleBufferErrorInvalid")]
    pub const Invalid: Self = Self(1);
    #[doc(alias = "MTLCounterSampleBufferErrorInternal")]
    pub const Internal: Self = Self(2);
}

unsafe impl Encode for MTLCounterSampleBufferError {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MTLCounterSampleBufferError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
