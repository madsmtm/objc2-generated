//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use objc2::__framework_prelude::*;

use crate::*;

extern "C-unwind" {
    #[deprecated = "Use MPVolumeView to present volume controls."]
    pub fn MPVolumeSettingsAlertShow();
}

extern "C-unwind" {
    #[deprecated = "Use MPVolumeView to present volume controls."]
    pub fn MPVolumeSettingsAlertHide();
}

#[deprecated = "Use MPVolumeView to present volume controls."]
#[inline]
pub unsafe extern "C-unwind" fn MPVolumeSettingsAlertIsVisible() -> bool {
    extern "C-unwind" {
        fn MPVolumeSettingsAlertIsVisible() -> Bool;
    }
    unsafe { MPVolumeSettingsAlertIsVisible() }.as_bool()
}
