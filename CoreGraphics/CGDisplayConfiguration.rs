//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgdisplayconfigref?language=objc)
pub type CGDisplayConfigRef = *mut c_void;

extern "C-unwind" {
    #[cfg(feature = "CGError")]
    pub fn CGBeginDisplayConfiguration(config: *mut CGDisplayConfigRef) -> CGError;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGDirectDisplay", feature = "CGError"))]
    pub fn CGConfigureDisplayOrigin(
        config: CGDisplayConfigRef,
        display: CGDirectDisplayID,
        x: i32,
        y: i32,
    ) -> CGError;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CGDirectDisplay",
        feature = "CGError",
        feature = "objc2-core-foundation"
    ))]
    pub fn CGConfigureDisplayWithDisplayMode(
        config: CGDisplayConfigRef,
        display: CGDirectDisplayID,
        mode: CGDisplayModeRef,
        options: CFDictionaryRef,
    ) -> CGError;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGDirectDisplay", feature = "CGError", feature = "libc"))]
    pub fn CGConfigureDisplayStereoOperation(
        config: CGDisplayConfigRef,
        display: CGDirectDisplayID,
        stereo: libc::boolean_t,
        force_blue_line: libc::boolean_t,
    ) -> CGError;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGDirectDisplay", feature = "CGError"))]
    pub fn CGConfigureDisplayMirrorOfDisplay(
        config: CGDisplayConfigRef,
        display: CGDirectDisplayID,
        master: CGDirectDisplayID,
    ) -> CGError;
}

extern "C-unwind" {
    #[cfg(feature = "CGError")]
    pub fn CGCancelDisplayConfiguration(config: CGDisplayConfigRef) -> CGError;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgconfigureoption?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CGConfigureOption(pub u32);
bitflags::bitflags! {
    impl CGConfigureOption: u32 {
        const kCGConfigureForAppOnly = 0;
        const kCGConfigureForSession = 1;
        const kCGConfigurePermanently = 2;
    }
}

unsafe impl Encode for CGConfigureOption {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for CGConfigureOption {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(feature = "CGError")]
    pub fn CGCompleteDisplayConfiguration(
        config: CGDisplayConfigRef,
        option: CGConfigureOption,
    ) -> CGError;
}

extern "C-unwind" {
    pub fn CGRestorePermanentDisplayConfiguration();
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgdisplaychangesummaryflags?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CGDisplayChangeSummaryFlags(pub u32);
bitflags::bitflags! {
    impl CGDisplayChangeSummaryFlags: u32 {
        const kCGDisplayBeginConfigurationFlag = 1<<0;
        const kCGDisplayMovedFlag = 1<<1;
        const kCGDisplaySetMainFlag = 1<<2;
        const kCGDisplaySetModeFlag = 1<<3;
        const kCGDisplayAddFlag = 1<<4;
        const kCGDisplayRemoveFlag = 1<<5;
        const kCGDisplayEnabledFlag = 1<<8;
        const kCGDisplayDisabledFlag = 1<<9;
        const kCGDisplayMirrorFlag = 1<<10;
        const kCGDisplayUnMirrorFlag = 1<<11;
        const kCGDisplayDesktopShapeChangedFlag = 1<<12;
    }
}

unsafe impl Encode for CGDisplayChangeSummaryFlags {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for CGDisplayChangeSummaryFlags {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgdisplayreconfigurationcallback?language=objc)
#[cfg(feature = "CGDirectDisplay")]
pub type CGDisplayReconfigurationCallBack = Option<
    unsafe extern "C-unwind" fn(CGDirectDisplayID, CGDisplayChangeSummaryFlags, *mut c_void),
>;

extern "C-unwind" {
    #[cfg(all(feature = "CGDirectDisplay", feature = "CGError"))]
    pub fn CGDisplayRegisterReconfigurationCallback(
        callback: CGDisplayReconfigurationCallBack,
        user_info: *mut c_void,
    ) -> CGError;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGDirectDisplay", feature = "CGError"))]
    pub fn CGDisplayRemoveReconfigurationCallback(
        callback: CGDisplayReconfigurationCallBack,
        user_info: *mut c_void,
    ) -> CGError;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGDirectDisplay", feature = "CGError", feature = "libc"))]
    pub fn CGDisplaySetStereoOperation(
        display: CGDirectDisplayID,
        stereo: libc::boolean_t,
        force_blue_line: libc::boolean_t,
        option: CGConfigureOption,
    ) -> CGError;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGDirectDisplay", feature = "libc"))]
    pub fn CGDisplayIsActive(display: CGDirectDisplayID) -> libc::boolean_t;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGDirectDisplay", feature = "libc"))]
    pub fn CGDisplayIsAsleep(display: CGDirectDisplayID) -> libc::boolean_t;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGDirectDisplay", feature = "libc"))]
    pub fn CGDisplayIsOnline(display: CGDirectDisplayID) -> libc::boolean_t;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGDirectDisplay", feature = "libc"))]
    pub fn CGDisplayIsMain(display: CGDirectDisplayID) -> libc::boolean_t;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGDirectDisplay", feature = "libc"))]
    pub fn CGDisplayIsBuiltin(display: CGDirectDisplayID) -> libc::boolean_t;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGDirectDisplay", feature = "libc"))]
    pub fn CGDisplayIsInMirrorSet(display: CGDirectDisplayID) -> libc::boolean_t;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGDirectDisplay", feature = "libc"))]
    pub fn CGDisplayIsAlwaysInMirrorSet(display: CGDirectDisplayID) -> libc::boolean_t;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGDirectDisplay", feature = "libc"))]
    pub fn CGDisplayIsInHWMirrorSet(display: CGDirectDisplayID) -> libc::boolean_t;
}

extern "C-unwind" {
    #[cfg(feature = "CGDirectDisplay")]
    pub fn CGDisplayMirrorsDisplay(display: CGDirectDisplayID) -> CGDirectDisplayID;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGDirectDisplay", feature = "libc"))]
    pub fn CGDisplayUsesOpenGLAcceleration(display: CGDirectDisplayID) -> libc::boolean_t;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGDirectDisplay", feature = "libc"))]
    pub fn CGDisplayIsStereo(display: CGDirectDisplayID) -> libc::boolean_t;
}

extern "C-unwind" {
    #[cfg(feature = "CGDirectDisplay")]
    pub fn CGDisplayPrimaryDisplay(display: CGDirectDisplayID) -> CGDirectDisplayID;
}

extern "C-unwind" {
    #[cfg(feature = "CGDirectDisplay")]
    pub fn CGDisplayUnitNumber(display: CGDirectDisplayID) -> u32;
}

extern "C-unwind" {
    #[cfg(feature = "CGDirectDisplay")]
    pub fn CGDisplayVendorNumber(display: CGDirectDisplayID) -> u32;
}

extern "C-unwind" {
    #[cfg(feature = "CGDirectDisplay")]
    pub fn CGDisplayModelNumber(display: CGDirectDisplayID) -> u32;
}

extern "C-unwind" {
    #[cfg(feature = "CGDirectDisplay")]
    pub fn CGDisplaySerialNumber(display: CGDirectDisplayID) -> u32;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGDirectDisplay", feature = "objc2-core-foundation"))]
    pub fn CGDisplayScreenSize(display: CGDirectDisplayID) -> CGSize;
}

extern "C-unwind" {
    #[cfg(feature = "CGDirectDisplay")]
    pub fn CGDisplayRotation(display: CGDirectDisplayID) -> c_double;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGColorSpace", feature = "CGDirectDisplay"))]
    pub fn CGDisplayCopyColorSpace(display: CGDirectDisplayID) -> CGColorSpaceRef;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CGDirectDisplay",
        feature = "CGError",
        feature = "objc2-core-foundation"
    ))]
    #[deprecated = "No longer supported"]
    pub fn CGConfigureDisplayMode(
        config: CGDisplayConfigRef,
        display: CGDirectDisplayID,
        mode: CFDictionaryRef,
    ) -> CGError;
}