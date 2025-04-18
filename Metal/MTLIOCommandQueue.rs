//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtliopriority?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLIOPriority(pub NSInteger);
impl MTLIOPriority {
    #[doc(alias = "MTLIOPriorityHigh")]
    pub const High: Self = Self(0);
    #[doc(alias = "MTLIOPriorityNormal")]
    pub const Normal: Self = Self(1);
    #[doc(alias = "MTLIOPriorityLow")]
    pub const Low: Self = Self(2);
}

unsafe impl Encode for MTLIOPriority {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MTLIOPriority {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtliocommandqueuetype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLIOCommandQueueType(pub NSInteger);
impl MTLIOCommandQueueType {
    #[doc(alias = "MTLIOCommandQueueTypeConcurrent")]
    pub const Concurrent: Self = Self(0);
    #[doc(alias = "MTLIOCommandQueueTypeSerial")]
    pub const Serial: Self = Self(1);
}

unsafe impl Encode for MTLIOCommandQueueType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MTLIOCommandQueueType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlioerrordomain?language=objc)
    pub static MTLIOErrorDomain: &'static NSErrorDomain;
}

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlioerror?language=objc)
// NS_ERROR_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLIOError(pub NSInteger);
impl MTLIOError {
    #[doc(alias = "MTLIOErrorURLInvalid")]
    pub const URLInvalid: Self = Self(1);
    #[doc(alias = "MTLIOErrorInternal")]
    pub const Internal: Self = Self(2);
}

unsafe impl Encode for MTLIOError {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MTLIOError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// Represents a queue that schedules command buffers containing command that
    /// read from handle objects and write to MTLResource objects.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtliocommandqueue?language=objc)
    pub unsafe trait MTLIOCommandQueue: NSObjectProtocol {
        /// Inserts a barrier that ensures that all commandBuffers commited
        /// prior are completed before any commandBuffers after start execution.
        ///
        /// A serial commandQueue has implicit barriers between
        /// each commandBuffer.
        #[unsafe(method(enqueueBarrier))]
        #[unsafe(method_family = none)]
        unsafe fn enqueueBarrier(&self);

        #[cfg(feature = "MTLIOCommandBuffer")]
        /// Vends an autoreleased commandBuffer that can be used to
        /// encode  commands that read from handle objects and write to MTLResource objects.
        #[unsafe(method(commandBuffer))]
        #[unsafe(method_family = none)]
        unsafe fn commandBuffer(&self) -> Retained<ProtocolObject<dyn MTLIOCommandBuffer>>;

        #[cfg(feature = "MTLIOCommandBuffer")]
        /// Vends an autoreleased commandBuffer that can be used to
        /// encode  commands that read from handle objects and write to MTLResource objects.
        /// This commandBuffer does not retain objects referenced by the commandBuffer
        /// as an optimization.
        ///
        /// For correct execution its the application's responsibility to retain
        /// objects referenced by commands within the commandBuffer.
        #[unsafe(method(commandBufferWithUnretainedReferences))]
        #[unsafe(method_family = none)]
        unsafe fn commandBufferWithUnretainedReferences(
            &self,
        ) -> Retained<ProtocolObject<dyn MTLIOCommandBuffer>>;

        /// An optional label for this handle.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        unsafe fn label(&self) -> Option<Retained<NSString>>;

        /// Setter for [`label`][Self::label].
        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        unsafe fn setLabel(&self, label: Option<&NSString>);
    }
);

extern_protocol!(
    /// An extendible protocol that can be used to wrap the buffers vended by
    /// a custom allocator. The underlying buffer is used as scratch space for IO commands
    /// that need it.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlioscratchbuffer?language=objc)
    pub unsafe trait MTLIOScratchBuffer: NSObjectProtocol {
        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLBuffer",
            feature = "MTLResource"
        ))]
        #[unsafe(method(buffer))]
        #[unsafe(method_family = none)]
        unsafe fn buffer(&self) -> Retained<ProtocolObject<dyn MTLBuffer>>;
    }
);

extern_protocol!(
    /// An extendible protocol that can implement a custom allocator passed
    /// to the queue descriptor.
    ///
    /// If provided, the queue will call newScratchBufferWithMinimumSize
    /// when it needs scratch storage for IO commands. When the commands that use the memory
    /// complete they return the storage by dealloc'ing the MTLIOScratchBuffer objects (where
    /// the application can optionally pool the memory for use by future commands.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlioscratchbufferallocator?language=objc)
    pub unsafe trait MTLIOScratchBufferAllocator: NSObjectProtocol {
        /// This method is called when additional scratch memory is required by a load command.
        /// The scratch buffer returned should NOT be an autoreleased object.
        ///
        /// Scratch memory is needed for cases where a texture is being copied to. minimumSize
        /// is the smallest buffer that will allow the command to execute, however a larger buffer can be provided and
        /// susequent commands will be able to use it, thus avoiding the need for an additional callback. Returning nil
        /// from the function will result in the load command being skipped and the commandBuffer getting cancelled.
        #[unsafe(method(newScratchBufferWithMinimumSize:))]
        #[unsafe(method_family = new)]
        unsafe fn newScratchBufferWithMinimumSize(
            &self,
            minimum_size: NSUInteger,
        ) -> Option<Retained<ProtocolObject<dyn MTLIOScratchBuffer>>>;
    }
);

extern_class!(
    /// Represents a descriptor to create a MTLIOCommandQueue.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtliocommandqueuedescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLIOCommandQueueDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLIOCommandQueueDescriptor {}
);

unsafe impl CopyingHelper for MTLIOCommandQueueDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLIOCommandQueueDescriptor {}
);

impl MTLIOCommandQueueDescriptor {
    extern_methods!(
        /// The maximum number of commandBuffers that can be in flight at a given time for the queue.
        #[unsafe(method(maxCommandBufferCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn maxCommandBufferCount(&self) -> NSUInteger;

        /// Setter for [`maxCommandBufferCount`][Self::maxCommandBufferCount].
        #[unsafe(method(setMaxCommandBufferCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMaxCommandBufferCount(&self, max_command_buffer_count: NSUInteger);

        /// The priority of the commands executed by this queue.
        #[unsafe(method(priority))]
        #[unsafe(method_family = none)]
        pub unsafe fn priority(&self) -> MTLIOPriority;

        /// Setter for [`priority`][Self::priority].
        #[unsafe(method(setPriority:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setPriority(&self, priority: MTLIOPriority);

        /// The type (serial or concurrent) of the queue.
        #[unsafe(method(type))]
        #[unsafe(method_family = none)]
        pub unsafe fn r#type(&self) -> MTLIOCommandQueueType;

        /// Setter for [`type`][Self::type].
        #[unsafe(method(setType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setType(&self, r#type: MTLIOCommandQueueType);

        /// The maximum number of IO commands that can be in flight at a given time for the queue.
        ///
        /// A zero value defaults to the system dependent maximum value, a smaller number can be
        /// provided to bound the utilization of the storage device.
        #[unsafe(method(maxCommandsInFlight))]
        #[unsafe(method_family = none)]
        pub unsafe fn maxCommandsInFlight(&self) -> NSUInteger;

        /// Setter for [`maxCommandsInFlight`][Self::maxCommandsInFlight].
        #[unsafe(method(setMaxCommandsInFlight:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMaxCommandsInFlight(&self, max_commands_in_flight: NSUInteger);

        /// An optional property that allows setting a custom allocator for scratch buffers by the queue.
        ///
        /// An application can manage scratch buffers manually by implemeting a class  conforming
        /// to the MTLIOScratchBufferAllocator protocol and creating an instance that is passed in here.
        #[unsafe(method(scratchBufferAllocator))]
        #[unsafe(method_family = none)]
        pub unsafe fn scratchBufferAllocator(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLIOScratchBufferAllocator>>>;

        /// Setter for [`scratchBufferAllocator`][Self::scratchBufferAllocator].
        #[unsafe(method(setScratchBufferAllocator:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setScratchBufferAllocator(
            &self,
            scratch_buffer_allocator: Option<&ProtocolObject<dyn MTLIOScratchBufferAllocator>>,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLIOCommandQueueDescriptor {
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
    /// Represents a  file (raw or compressed) that can be used as a source
    /// for load commands encoded in a MTLIOCommandBuffer.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtliofilehandle?language=objc)
    pub unsafe trait MTLIOFileHandle: NSObjectProtocol {
        /// An optional label for this handle.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        unsafe fn label(&self) -> Option<Retained<NSString>>;

        /// Setter for [`label`][Self::label].
        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        unsafe fn setLabel(&self, label: Option<&NSString>);
    }
);
