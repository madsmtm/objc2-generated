//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// Virtio Traditional Memory Balloon Device
    ///
    /// This is a primitive device for managing guest memory.
    /// This device enables memory transfer between the host and the guest as specified by the Virtio specification, which allows the guest to adapt changes
    /// in allowance of underlying physical memory.
    ///
    /// To request a memory balloon device operation for the memory transfer, write the targeted memory size for the virtual machine to the targetVirtualMachineMemorySize property.
    /// When the value written to targetVirtualMachineMemorySize is less than the current value, memory will be taken away from the guest and given to the host by the amount
    /// determined by the difference between those two values. Similarly, when the value written to targetVirtualMachineMemorySize is greater than the current value, memory will be
    /// given back to the guest by the amount determined by the difference between those two values.
    ///
    /// Note that any changes to targetVirtualMachineMemorySize is a request to trigger a memory balloon operation. The actual changes in memory only happen after the guest operating
    /// system handles the request, if at all.
    ///
    /// The targetVirtualMachineMemorySize property is initialized to VZVirtualMachineConfiguration.memorySize. The acceptable values for the targetVirtualMachineMemorySize
    /// property range from VZVirtualMachineConfiguration.minimumAllowedMemorySize to VZVirtualMachineConfiguration.memorySize, and must be a multiple of 1 megabyte
    /// (1024 * 1024 bytes). If those constraints aren't satisfied, targetVirtualMachineMemorySize will be rounded down to the nearest multiple of 1 megabyte, clamped to
    /// VZVirtualMachineConfiguration.minimumAllowedMemorySize and VZVirtualMachineConfiguration.memorySize respectively.
    ///
    /// For optimal performance, it is strongly recommended to perform a memory compaction operation in the guest (e.g. echo 1 > /proc/sys/vm/compact_memory) before invoking the device.
    /// This helps to minimize memory fragmentation in order for the memory allocation/deallocation process to be more effective.
    ///
    /// This device is created through instantiating a VZVirtioTraditionalMemoryBalloonDeviceConfiguration in a VZVirtualMachineConfiguration and is available in the
    /// VZVirtualMachine.memoryBalloonDevices property.
    ///
    /// See: VZVirtioTraditionalMemoryBalloonDeviceConfiguration
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzvirtiotraditionalmemoryballoondevice?language=objc)
    #[unsafe(super(VZMemoryBalloonDevice, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZMemoryBalloonDevice")]
    pub struct VZVirtioTraditionalMemoryBalloonDevice;
);

#[cfg(feature = "VZMemoryBalloonDevice")]
extern_conformance!(
    unsafe impl NSObjectProtocol for VZVirtioTraditionalMemoryBalloonDevice {}
);

#[cfg(feature = "VZMemoryBalloonDevice")]
impl VZVirtioTraditionalMemoryBalloonDevice {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Target memory size for the virtual machine in bytes.
        ///
        /// The targetVirtualMachineMemorySize must be a multiple of 1 megabyte (1024 * 1024 bytes) between VZVirtualMachineConfiguration.minimumAllowedMemorySize
        /// and VZVirtualMachineConfiguration.memorySize. If those constraints aren't satisfied, targetVirtualMachineMemorySize will be rounded down to the nearest multiple of
        /// 1 megabyte, clamped to VZVirtualMachineConfiguration.minimumAllowedMemorySize and VZVirtualMachineConfiguration.memorySize respectively.
        ///
        /// The targetVirtualMachineMemorySize represents the amount of physical memory to be made available to the guest.
        ///
        /// See: VZVirtualMachineConfiguration.minimumAllowedMemorySize
        ///
        /// See: VZVirtualMachineConfiguration.memorySize
        #[unsafe(method(targetVirtualMachineMemorySize))]
        #[unsafe(method_family = none)]
        pub unsafe fn targetVirtualMachineMemorySize(&self) -> u64;

        /// Setter for [`targetVirtualMachineMemorySize`][Self::targetVirtualMachineMemorySize].
        #[unsafe(method(setTargetVirtualMachineMemorySize:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTargetVirtualMachineMemorySize(
            &self,
            target_virtual_machine_memory_size: u64,
        );
    );
}
