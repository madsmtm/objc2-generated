//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

extern "C-unwind" {
    #[cfg(all(feature = "CMSync", feature = "objc2-core-foundation"))]
    pub fn CMAudioDeviceClockCreate(
        allocator: CFAllocatorRef,
        device_uid: CFStringRef,
        clock_out: NonNull<CMClockRef>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(all(feature = "CMSync", feature = "objc2-core-foundation"))]
    pub fn CMAudioDeviceClockSetAudioDeviceUID(
        clock: CMClockRef,
        device_uid: CFStringRef,
    ) -> OSStatus;
}
