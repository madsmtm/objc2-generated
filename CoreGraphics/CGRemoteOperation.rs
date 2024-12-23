//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgeventerr?language=objc)
#[cfg(feature = "CGError")]
pub type CGEventErr = CGError;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgbuttoncount?language=objc)
pub type CGButtonCount = u32;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgwheelcount?language=objc)
pub type CGWheelCount = u32;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgcharcode?language=objc)
pub type CGCharCode = u16;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgkeycode?language=objc)
pub type CGKeyCode = u16;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgscreenrefreshcallback?language=objc)
#[cfg(feature = "objc2-core-foundation")]
pub type CGScreenRefreshCallback =
    Option<unsafe extern "C-unwind" fn(u32, NonNull<CGRect>, *mut c_void)>;

extern "C-unwind" {
    #[cfg(all(feature = "CGError", feature = "objc2-core-foundation"))]
    #[deprecated = "No longer supported"]
    pub fn CGRegisterScreenRefreshCallback(
        callback: CGScreenRefreshCallback,
        user_info: *mut c_void,
    ) -> CGError;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated = "No longer supported"]
    pub fn CGUnregisterScreenRefreshCallback(
        callback: CGScreenRefreshCallback,
        user_info: *mut c_void,
    );
}

extern "C-unwind" {
    #[cfg(all(feature = "CGError", feature = "objc2-core-foundation"))]
    #[deprecated = "No longer supported"]
    pub fn CGWaitForScreenRefreshRects(rects: *mut *mut CGRect, count: *mut u32) -> CGError;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgscreenupdateoperation?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CGScreenUpdateOperation(pub u32);
bitflags::bitflags! {
    impl CGScreenUpdateOperation: u32 {
        const kCGScreenUpdateOperationRefresh = 0;
        const kCGScreenUpdateOperationMove = 1<<0;
        const kCGScreenUpdateOperationReducedDirtyRectangleCount = 1<<31;
    }
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CGScreenUpdateOperation {
    const ENCODING: Encoding = u32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CGScreenUpdateOperation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgscreenupdatemovedelta?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CGScreenUpdateMoveDelta {
    pub dX: i32,
    pub dY: i32,
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CGScreenUpdateMoveDelta {
    const ENCODING: Encoding = Encoding::Struct(
        "CGScreenUpdateMoveDelta",
        &[<i32>::ENCODING, <i32>::ENCODING],
    );
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CGScreenUpdateMoveDelta {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgscreenupdatemovecallback?language=objc)
#[cfg(feature = "objc2-core-foundation")]
pub type CGScreenUpdateMoveCallback = Option<
    unsafe extern "C-unwind" fn(CGScreenUpdateMoveDelta, usize, NonNull<CGRect>, *mut c_void),
>;

extern "C-unwind" {
    #[cfg(all(feature = "CGError", feature = "objc2-core-foundation"))]
    #[deprecated = "No longer supported"]
    pub fn CGScreenRegisterMoveCallback(
        callback: CGScreenUpdateMoveCallback,
        user_info: *mut c_void,
    ) -> CGError;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated = "No longer supported"]
    pub fn CGScreenUnregisterMoveCallback(
        callback: CGScreenUpdateMoveCallback,
        user_info: *mut c_void,
    );
}

extern "C-unwind" {
    #[cfg(all(feature = "CGError", feature = "objc2-core-foundation"))]
    #[deprecated = "No longer supported"]
    pub fn CGWaitForScreenUpdateRects(
        requested_operations: CGScreenUpdateOperation,
        current_operation: *mut CGScreenUpdateOperation,
        rects: *mut *mut CGRect,
        rect_count: *mut usize,
        delta: *mut CGScreenUpdateMoveDelta,
    ) -> CGError;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated = "No longer supported"]
    pub fn CGReleaseScreenRefreshRects(rects: *mut CGRect);
}

extern "C-unwind" {
    #[cfg(feature = "libc")]
    #[deprecated = "No longer supported"]
    pub fn CGCursorIsVisible() -> libc::boolean_t;
}

extern "C-unwind" {
    #[cfg(feature = "libc")]
    #[deprecated = "No longer supported"]
    pub fn CGCursorIsDrawnInFramebuffer() -> libc::boolean_t;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGError", feature = "objc2-core-foundation"))]
    pub fn CGWarpMouseCursorPosition(new_cursor_position: CGPoint) -> CGError;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGError", feature = "libc"))]
    pub fn CGAssociateMouseAndMouseCursorPosition(connected: libc::boolean_t) -> CGError;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CGWindowServerCreateServerPort() -> CFMachPortRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGError", feature = "libc"))]
    #[deprecated = "No longer supported"]
    pub fn CGEnableEventStateCombining(combine_state: libc::boolean_t) -> CGError;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGError", feature = "libc"))]
    #[deprecated = "No longer supported"]
    pub fn CGInhibitLocalEvents(inhibit: libc::boolean_t) -> CGError;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGError", feature = "libc"))]
    #[deprecated = "No longer supported"]
    pub fn CGPostKeyboardEvent(
        key_char: CGCharCode,
        virtual_key: CGKeyCode,
        key_down: libc::boolean_t,
    ) -> CGError;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgeventfiltermask?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CGEventFilterMask(pub u32);
bitflags::bitflags! {
    impl CGEventFilterMask: u32 {
        const kCGEventFilterMaskPermitLocalMouseEvents = 0x00000001;
        const kCGEventFilterMaskPermitLocalKeyboardEvents = 0x00000002;
        const kCGEventFilterMaskPermitSystemDefinedEvents = 0x00000004;
    }
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CGEventFilterMask {
    const ENCODING: Encoding = u32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CGEventFilterMask {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgeventsuppressionstate?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CGEventSuppressionState(pub u32);
impl CGEventSuppressionState {
    pub const kCGEventSuppressionStateSuppressionInterval: Self = Self(0);
    pub const kCGEventSuppressionStateRemoteMouseDrag: Self = Self(1);
    pub const kCGNumberOfEventSuppressionStates: Self = Self(2);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CGEventSuppressionState {
    const ENCODING: Encoding = u32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CGEventSuppressionState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(feature = "CGError")]
    #[deprecated = "No longer supported"]
    pub fn CGSetLocalEventsFilterDuringSuppressionState(
        filter: CGEventFilterMask,
        state: CGEventSuppressionState,
    ) -> CGError;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGError", feature = "objc2-core-foundation"))]
    #[deprecated = "No longer supported"]
    pub fn CGSetLocalEventsSuppressionInterval(seconds: CFTimeInterval) -> CGError;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated = "No longer supported"]
    pub fn CGWindowServerCFMachPort() -> CFMachPortRef;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgrectcount?language=objc)
pub type CGRectCount = u32;
