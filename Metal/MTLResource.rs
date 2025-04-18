//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// Options for setPurgeable call.
///
///
/// The contents of this resource may not be discarded.
///
///
/// The contents of this resource may be discarded.
///
///
/// The contents of this are discarded.
///
///
/// The purgeabelity state is not changed.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlpurgeablestate?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLPurgeableState(pub NSUInteger);
impl MTLPurgeableState {
    #[doc(alias = "MTLPurgeableStateKeepCurrent")]
    pub const KeepCurrent: Self = Self(1);
    #[doc(alias = "MTLPurgeableStateNonVolatile")]
    pub const NonVolatile: Self = Self(2);
    #[doc(alias = "MTLPurgeableStateVolatile")]
    pub const Volatile: Self = Self(3);
    #[doc(alias = "MTLPurgeableStateEmpty")]
    pub const Empty: Self = Self(4);
}

unsafe impl Encode for MTLPurgeableState {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLPurgeableState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Describes what CPU cache mode is used for the CPU's mapping of a texture resource.
///
/// The default cache mode for the system.
///
///
/// Write combined memory is optimized for resources that the CPU will write into, but never read.  On some implementations, writes may bypass caches avoiding cache pollution, and reads perform very poorly.
///
///
/// Applications should only investigate changing the cache mode if writing to normally cached buffers is known to cause performance issues due to cache pollution, as write combined memory can have surprising performance pitfalls.  Another approach is to use non-temporal stores to normally cached memory (STNP on ARMv8, _mm_stream_* on x86_64).
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcpucachemode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLCPUCacheMode(pub NSUInteger);
impl MTLCPUCacheMode {
    #[doc(alias = "MTLCPUCacheModeDefaultCache")]
    pub const DefaultCache: Self = Self(0);
    #[doc(alias = "MTLCPUCacheModeWriteCombined")]
    pub const WriteCombined: Self = Self(1);
}

unsafe impl Encode for MTLCPUCacheMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLCPUCacheMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Describes location and CPU mapping of MTLTexture.
///
/// In this mode, CPU and device will nominally both use the same underlying memory when accessing the contents of the texture resource.
/// However, coherency is only guaranteed at command buffer boundaries to minimize the required flushing of CPU and GPU caches.
/// This is the default storage mode for iOS Textures.
///
///
/// This mode relaxes the coherency requirements and requires that the developer make explicit requests to maintain
/// coherency between a CPU and GPU version of the texture resource.  In order for CPU to access up to date GPU results,
/// first, a blit synchronizations must be completed (see synchronize methods of MTLBlitCommandEncoder).
/// Blit overhead is only incurred if GPU has modified the resource.
/// This is the default storage mode for OS X Textures.
///
///
/// This mode allows the texture resource data to be kept entirely to GPU (or driver) private memory that will never be accessed by the CPU directly, so no
/// conherency of any kind must be maintained.
///
///
/// This mode allows creation of resources that do not have a GPU or CPU memory backing, but do have on-chip storage for TBDR
/// devices. The contents of the on-chip storage is undefined and does not persist, but its configuration is controlled by the
/// MTLTexture descriptor. Textures created with MTLStorageModeMemoryless dont have an IOAccelResource at any point in their
/// lifetime. The only way to populate such resource is to perform rendering operations on it. Blit operations are disallowed.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlstoragemode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLStorageMode(pub NSUInteger);
impl MTLStorageMode {
    #[doc(alias = "MTLStorageModeShared")]
    pub const Shared: Self = Self(0);
    #[doc(alias = "MTLStorageModeManaged")]
    pub const Managed: Self = Self(1);
    #[doc(alias = "MTLStorageModePrivate")]
    pub const Private: Self = Self(2);
    #[doc(alias = "MTLStorageModeMemoryless")]
    pub const Memoryless: Self = Self(3);
}

unsafe impl Encode for MTLStorageMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLStorageMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Describes how hazard tracking is performed.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlhazardtrackingmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLHazardTrackingMode(pub NSUInteger);
impl MTLHazardTrackingMode {
    #[doc(alias = "MTLHazardTrackingModeDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "MTLHazardTrackingModeUntracked")]
    pub const Untracked: Self = Self(1);
    #[doc(alias = "MTLHazardTrackingModeTracked")]
    pub const Tracked: Self = Self(2);
}

unsafe impl Encode for MTLHazardTrackingMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLHazardTrackingMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlresourceoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLResourceOptions(pub NSUInteger);
bitflags::bitflags! {
    impl MTLResourceOptions: NSUInteger {
        #[doc(alias = "MTLResourceCPUCacheModeDefaultCache")]
        const CPUCacheModeDefaultCache = MTLCPUCacheMode::DefaultCache.0<<MTLResourceCPUCacheModeShift;
        #[doc(alias = "MTLResourceCPUCacheModeWriteCombined")]
        const CPUCacheModeWriteCombined = MTLCPUCacheMode::WriteCombined.0<<MTLResourceCPUCacheModeShift;
        #[doc(alias = "MTLResourceStorageModeShared")]
        const StorageModeShared = MTLStorageMode::Shared.0<<MTLResourceStorageModeShift;
        #[doc(alias = "MTLResourceStorageModeManaged")]
        const StorageModeManaged = MTLStorageMode::Managed.0<<MTLResourceStorageModeShift;
        #[doc(alias = "MTLResourceStorageModePrivate")]
        const StorageModePrivate = MTLStorageMode::Private.0<<MTLResourceStorageModeShift;
        #[doc(alias = "MTLResourceStorageModeMemoryless")]
        const StorageModeMemoryless = MTLStorageMode::Memoryless.0<<MTLResourceStorageModeShift;
        #[doc(alias = "MTLResourceHazardTrackingModeDefault")]
        const HazardTrackingModeDefault = MTLHazardTrackingMode::Default.0<<MTLResourceHazardTrackingModeShift;
        #[doc(alias = "MTLResourceHazardTrackingModeUntracked")]
        const HazardTrackingModeUntracked = MTLHazardTrackingMode::Untracked.0<<MTLResourceHazardTrackingModeShift;
        #[doc(alias = "MTLResourceHazardTrackingModeTracked")]
        const HazardTrackingModeTracked = MTLHazardTrackingMode::Tracked.0<<MTLResourceHazardTrackingModeShift;
        #[doc(alias = "MTLResourceOptionCPUCacheModeDefault")]
#[deprecated]
        const OptionCPUCacheModeDefault = MTLResourceOptions::CPUCacheModeDefaultCache.0;
        #[doc(alias = "MTLResourceOptionCPUCacheModeWriteCombined")]
#[deprecated]
        const OptionCPUCacheModeWriteCombined = MTLResourceOptions::CPUCacheModeWriteCombined.0;
    }
}

unsafe impl Encode for MTLResourceOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLResourceOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// Common APIs available for MTLBuffer and MTLTexture instances
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlresource?language=objc)
    #[cfg(feature = "MTLAllocation")]
    pub unsafe trait MTLResource: MTLAllocation {
        /// A string to help identify this object.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        fn label(&self) -> Option<Retained<NSString>>;

        /// Setter for [`label`][Self::label].
        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        fn setLabel(&self, label: Option<&NSString>);

        #[cfg(feature = "MTLDevice")]
        /// The device this resource was created against.  This resource can only be used with this device.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// The cache mode used for the CPU mapping for this resource
        #[unsafe(method(cpuCacheMode))]
        #[unsafe(method_family = none)]
        fn cpuCacheMode(&self) -> MTLCPUCacheMode;

        /// The resource storage mode used for the CPU mapping for this resource
        #[unsafe(method(storageMode))]
        #[unsafe(method_family = none)]
        fn storageMode(&self) -> MTLStorageMode;

        /// Whether or not the resource is hazard tracked.
        ///
        /// This value can be either MTLHazardTrackingModeUntracked or MTLHazardTrackingModeTracked.
        /// Resources created from heaps are by default untracked, whereas resources created from the device are by default tracked.
        #[unsafe(method(hazardTrackingMode))]
        #[unsafe(method_family = none)]
        fn hazardTrackingMode(&self) -> MTLHazardTrackingMode;

        /// A packed tuple of the storageMode, cpuCacheMode and hazardTrackingMode properties.
        #[unsafe(method(resourceOptions))]
        #[unsafe(method_family = none)]
        fn resourceOptions(&self) -> MTLResourceOptions;

        /// Set (or query) the purgeability state of a resource
        ///
        /// Synchronously set the purgeability state of a resource and return what the prior (or current) state is.
        /// FIXME: If the device is keeping a cached copy of the resource, both the shared copy and cached copy are made purgeable.  Any access to the resource by either the CPU or device will be undefined.
        #[unsafe(method(setPurgeableState:))]
        #[unsafe(method_family = none)]
        fn setPurgeableState(&self, state: MTLPurgeableState) -> MTLPurgeableState;

        #[cfg(feature = "MTLHeap")]
        /// The heap from which this resouce was created.
        ///
        /// Nil when this resource is not backed by a heap.
        #[unsafe(method(heap))]
        #[unsafe(method_family = none)]
        fn heap(&self) -> Option<Retained<ProtocolObject<dyn MTLHeap>>>;

        /// The offset inside the heap at which this resource was created.
        ///
        /// Zero when this resource was not created on a heap with MTLHeapTypePlacement.
        #[unsafe(method(heapOffset))]
        #[unsafe(method_family = none)]
        fn heapOffset(&self) -> NSUInteger;

        /// The size in bytes occupied by this resource
        #[unsafe(method(allocatedSize))]
        #[unsafe(method_family = none)]
        fn allocatedSize(&self) -> NSUInteger;

        /// Allow future heap sub-allocations to alias against this resource's memory.
        ///
        /// It is illegal to call this method on a non heap-based resource.
        /// It is also illegal to call this method on texture views created from heap-based textures.
        /// The debug layer will raise an exception. Calling this method on textures sub-allocated
        /// from Buffers backed by heap memory has no effect.
        /// Once a resource is made aliasable, the decision cannot be reverted.
        #[unsafe(method(makeAliasable))]
        #[unsafe(method_family = none)]
        unsafe fn makeAliasable(&self);

        /// Returns whether future heap sub-allocations may alias against this resource's memory.
        ///
        /// Returns: YES if
        /// <st
        /// >makeAliasable
        /// </st
        /// > was previously successfully called on this resource. NO otherwise.
        /// If resource is sub-allocated from other resource created on the heap, isAliasable returns
        /// aliasing state of that base resource. Also returns NO when storage mode is memoryless.
        #[unsafe(method(isAliasable))]
        #[unsafe(method_family = none)]
        fn isAliasable(&self) -> bool;
    }
);
