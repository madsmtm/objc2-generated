//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;
use objc2_foundation::*;

use crate::*;

#[cfg(feature = "objc2-core-graphics")]
#[inline]
pub unsafe extern "C-unwind" fn AXNameFromColor(color: &CGColor) -> Retained<NSString> {
    extern "C-unwind" {
        fn AXNameFromColor(color: &CGColor) -> *mut NSString;
    }
    let ret = unsafe { AXNameFromColor(color) };
    unsafe { Retained::retain_autoreleased(ret) }
        .expect("function was marked as returning non-null, but actually returned NULL")
}
