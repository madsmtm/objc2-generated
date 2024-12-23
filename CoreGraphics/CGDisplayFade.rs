//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgdisplayfadereservationtoken?language=objc)
pub type CGDisplayFadeReservationToken = u32;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgdisplayblendfraction?language=objc)
pub type CGDisplayBlendFraction = c_float;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgdisplayfadeinterval?language=objc)
pub type CGDisplayFadeInterval = c_float;

extern "C-unwind" {
    #[cfg(all(feature = "CGDisplayConfiguration", feature = "CGError"))]
    pub fn CGConfigureDisplayFadeEffect(
        config: CGDisplayConfigRef,
        fade_out_seconds: CGDisplayFadeInterval,
        fade_in_seconds: CGDisplayFadeInterval,
        fade_red: c_float,
        fade_green: c_float,
        fade_blue: c_float,
    ) -> CGError;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgdisplayreservationinterval?language=objc)
pub type CGDisplayReservationInterval = c_float;

extern "C-unwind" {
    #[cfg(feature = "CGError")]
    pub fn CGAcquireDisplayFadeReservation(
        seconds: CGDisplayReservationInterval,
        token: *mut CGDisplayFadeReservationToken,
    ) -> CGError;
}

extern "C-unwind" {
    #[cfg(feature = "CGError")]
    pub fn CGReleaseDisplayFadeReservation(token: CGDisplayFadeReservationToken) -> CGError;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGError", feature = "libc"))]
    pub fn CGDisplayFade(
        token: CGDisplayFadeReservationToken,
        duration: CGDisplayFadeInterval,
        start_blend: CGDisplayBlendFraction,
        end_blend: CGDisplayBlendFraction,
        red_blend: c_float,
        green_blend: c_float,
        blue_blend: c_float,
        synchronous: libc::boolean_t,
    ) -> CGError;
}

extern "C-unwind" {
    #[cfg(feature = "libc")]
    #[deprecated = "No longer supported"]
    pub fn CGDisplayFadeOperationInProgress() -> libc::boolean_t;
}
