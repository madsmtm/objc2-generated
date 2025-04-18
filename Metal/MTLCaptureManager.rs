//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcaptureerrordomain?language=objc)
    pub static MTLCaptureErrorDomain: &'static NSErrorDomain;
}

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcaptureerror?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLCaptureError(pub NSInteger);
impl MTLCaptureError {
    /// Capturing is not supported, maybe the destination is not supported.
    #[doc(alias = "MTLCaptureErrorNotSupported")]
    pub const NotSupported: Self = Self(1);
    /// A capture is already in progress.
    #[doc(alias = "MTLCaptureErrorAlreadyCapturing")]
    pub const AlreadyCapturing: Self = Self(2);
    /// The MTLCaptureDescriptor contains an invalid parameters.
    #[doc(alias = "MTLCaptureErrorInvalidDescriptor")]
    pub const InvalidDescriptor: Self = Self(3);
}

unsafe impl Encode for MTLCaptureError {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MTLCaptureError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// The destination where you want the GPU trace to be captured to.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcapturedestination?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLCaptureDestination(pub NSInteger);
impl MTLCaptureDestination {
    /// Capture to Developer Tools (Xcode) and stop the execution after capturing.
    #[doc(alias = "MTLCaptureDestinationDeveloperTools")]
    pub const DeveloperTools: Self = Self(1);
    /// Capture to a GPU Trace document and continue execution after capturing.
    #[doc(alias = "MTLCaptureDestinationGPUTraceDocument")]
    pub const GPUTraceDocument: Self = Self(2);
}

unsafe impl Encode for MTLCaptureDestination {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MTLCaptureDestination {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcapturedescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLCaptureDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLCaptureDescriptor {}
);

unsafe impl CopyingHelper for MTLCaptureDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLCaptureDescriptor {}
);

impl MTLCaptureDescriptor {
    extern_methods!(
        /// The object that is captured.
        ///
        /// Must be one of the following:
        ///
        /// MTLDevice captures all command queues of the device.
        ///
        /// MTLCommandQueue captures a single command queue.
        ///
        /// MTLCaptureScope captures between the next begin and end of the scope.
        #[unsafe(method(captureObject))]
        #[unsafe(method_family = none)]
        pub unsafe fn captureObject(&self) -> Option<Retained<AnyObject>>;

        /// Setter for [`captureObject`][Self::captureObject].
        #[unsafe(method(setCaptureObject:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setCaptureObject(&self, capture_object: Option<&AnyObject>);

        /// The destination you want the GPU trace to be captured to.
        #[unsafe(method(destination))]
        #[unsafe(method_family = none)]
        pub fn destination(&self) -> MTLCaptureDestination;

        /// Setter for [`destination`][Self::destination].
        #[unsafe(method(setDestination:))]
        #[unsafe(method_family = none)]
        pub fn setDestination(&self, destination: MTLCaptureDestination);

        /// URL the GPU Trace document will be captured to.
        /// Must be specified when destiation is MTLCaptureDestinationGPUTraceDocument.
        #[unsafe(method(outputURL))]
        #[unsafe(method_family = none)]
        pub fn outputURL(&self) -> Option<Retained<NSURL>>;

        /// Setter for [`outputURL`][Self::outputURL].
        #[unsafe(method(setOutputURL:))]
        #[unsafe(method_family = none)]
        pub fn setOutputURL(&self, output_url: Option<&NSURL>);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLCaptureDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}

impl DefaultRetained for MTLCaptureDescriptor {
    #[inline]
    fn default_retained() -> Retained<Self> {
        Self::new()
    }
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcapturemanager?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLCaptureManager;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLCaptureManager {}
);

impl MTLCaptureManager {
    extern_methods!(
        /// Retrieves the shared capture manager for this process. There is only one capture manager per process.
        /// The capture manager allows the user to create capture scopes and trigger captures from code.
        /// When a capture has been completed, it will be displayed in Xcode and the application will be paused.
        ///
        /// : only MTLCommandBuffers created after starting a capture and committed before stopping it are captured.
        #[unsafe(method(sharedCaptureManager))]
        #[unsafe(method_family = none)]
        pub unsafe fn sharedCaptureManager() -> Retained<MTLCaptureManager>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(feature = "MTLCaptureScope", feature = "MTLDevice"))]
        #[unsafe(method(newCaptureScopeWithDevice:))]
        #[unsafe(method_family = new)]
        pub fn newCaptureScopeWithDevice(
            &self,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<ProtocolObject<dyn MTLCaptureScope>>;

        #[cfg(all(feature = "MTLCaptureScope", feature = "MTLCommandQueue"))]
        #[unsafe(method(newCaptureScopeWithCommandQueue:))]
        #[unsafe(method_family = new)]
        pub fn newCaptureScopeWithCommandQueue(
            &self,
            command_queue: &ProtocolObject<dyn MTLCommandQueue>,
        ) -> Retained<ProtocolObject<dyn MTLCaptureScope>>;

        #[unsafe(method(supportsDestination:))]
        #[unsafe(method_family = none)]
        pub fn supportsDestination(&self, destination: MTLCaptureDestination) -> bool;

        /// Start capturing until stopCapture is called.
        ///
        /// Parameter `descriptor`: MTLCaptureDescriptor specifies the parameters.
        ///
        /// Parameter `error`: Optional error output to check why a capture could not be started.
        ///
        /// Returns: true if the capture was successfully started, otherwise false.
        ///
        /// Only MTLCommandBuffer​s created after starting and committed before stopping it are captured.
        #[unsafe(method(startCaptureWithDescriptor:error:_))]
        #[unsafe(method_family = none)]
        pub fn startCaptureWithDescriptor_error(
            &self,
            descriptor: &MTLCaptureDescriptor,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "MTLDevice")]
        #[deprecated = "Use startCaptureWithDescriptor:error: instead"]
        #[unsafe(method(startCaptureWithDevice:))]
        #[unsafe(method_family = none)]
        pub fn startCaptureWithDevice(&self, device: &ProtocolObject<dyn MTLDevice>);

        #[cfg(feature = "MTLCommandQueue")]
        #[deprecated = "Use startCaptureWithDescriptor:error: instead"]
        #[unsafe(method(startCaptureWithCommandQueue:))]
        #[unsafe(method_family = none)]
        pub fn startCaptureWithCommandQueue(
            &self,
            command_queue: &ProtocolObject<dyn MTLCommandQueue>,
        );

        #[cfg(feature = "MTLCaptureScope")]
        #[deprecated = "Use startCaptureWithDescriptor:error: instead"]
        #[unsafe(method(startCaptureWithScope:))]
        #[unsafe(method_family = none)]
        pub fn startCaptureWithScope(&self, capture_scope: &ProtocolObject<dyn MTLCaptureScope>);

        #[unsafe(method(stopCapture))]
        #[unsafe(method_family = none)]
        pub fn stopCapture(&self);

        #[cfg(feature = "MTLCaptureScope")]
        #[unsafe(method(defaultCaptureScope))]
        #[unsafe(method_family = none)]
        pub fn defaultCaptureScope(&self) -> Option<Retained<ProtocolObject<dyn MTLCaptureScope>>>;

        #[cfg(feature = "MTLCaptureScope")]
        /// Setter for [`defaultCaptureScope`][Self::defaultCaptureScope].
        #[unsafe(method(setDefaultCaptureScope:))]
        #[unsafe(method_family = none)]
        pub fn setDefaultCaptureScope(
            &self,
            default_capture_scope: Option<&ProtocolObject<dyn MTLCaptureScope>>,
        );

        #[unsafe(method(isCapturing))]
        #[unsafe(method_family = none)]
        pub fn isCapturing(&self) -> bool;
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLCaptureManager {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
