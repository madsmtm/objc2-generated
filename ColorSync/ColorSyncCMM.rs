//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::cell::UnsafeCell;
use core::ffi::*;
use core::marker::{PhantomData, PhantomPinned};
use core::ptr::NonNull;
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/colorsync/colorsynccmm?language=objc)
#[repr(C)]
pub struct ColorSyncCMM {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

cf_type!(
    #[encoding_name = "ColorSyncCMM"]
    unsafe impl ColorSyncCMM {}
);

unsafe impl ConcreteType for ColorSyncCMM {
    #[doc(alias = "ColorSyncCMMGetTypeID")]
    #[inline]
    fn type_id() -> CFTypeID {
        extern "C-unwind" {
            fn ColorSyncCMMGetTypeID() -> CFTypeID;
        }
        unsafe { ColorSyncCMMGetTypeID() }
    }
}

#[inline]
pub unsafe extern "C-unwind" fn ColorSyncCMMCreate(
    cmm_bundle: &CFBundle,
) -> Option<CFRetained<ColorSyncCMM>> {
    extern "C-unwind" {
        fn ColorSyncCMMCreate(cmm_bundle: &CFBundle) -> *mut ColorSyncCMM;
    }
    let ret = unsafe { ColorSyncCMMCreate(cmm_bundle) };
    NonNull::new(ret).map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[inline]
pub unsafe extern "C-unwind" fn ColorSyncCMMGetBundle(
    param1: &ColorSyncCMM,
) -> Option<CFRetained<CFBundle>> {
    extern "C-unwind" {
        fn ColorSyncCMMGetBundle(param1: &ColorSyncCMM) -> *mut CFBundle;
    }
    let ret = unsafe { ColorSyncCMMGetBundle(param1) };
    NonNull::new(ret).map(|ret| unsafe { CFRetained::retain(ret) })
}

#[inline]
pub unsafe extern "C-unwind" fn ColorSyncCMMCopyLocalizedName(
    param1: &ColorSyncCMM,
) -> Option<CFRetained<CFString>> {
    extern "C-unwind" {
        fn ColorSyncCMMCopyLocalizedName(param1: &ColorSyncCMM) -> *mut CFString;
    }
    let ret = unsafe { ColorSyncCMMCopyLocalizedName(param1) };
    NonNull::new(ret).map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[inline]
pub unsafe extern "C-unwind" fn ColorSyncCMMCopyCMMIdentifier(
    param1: &ColorSyncCMM,
) -> Option<CFRetained<CFString>> {
    extern "C-unwind" {
        fn ColorSyncCMMCopyCMMIdentifier(param1: &ColorSyncCMM) -> *mut CFString;
    }
    let ret = unsafe { ColorSyncCMMCopyCMMIdentifier(param1) };
    NonNull::new(ret).map(|ret| unsafe { CFRetained::from_raw(ret) })
}

/// [Apple's documentation](https://developer.apple.com/documentation/colorsync/colorsynccmmiteratecallback?language=objc)
pub type ColorSyncCMMIterateCallback =
    Option<unsafe extern "C-unwind" fn(NonNull<ColorSyncCMM>, NonNull<c_void>) -> bool>;

extern "C-unwind" {
    pub fn ColorSyncIterateInstalledCMMs(
        call_back: ColorSyncCMMIterateCallback,
        user_info: *mut c_void,
    );
}

/// [Apple's documentation](https://developer.apple.com/documentation/colorsync/cmminitializelinkprofileproc?language=objc)
#[cfg(feature = "ColorSyncProfile")]
pub type CMMInitializeLinkProfileProc = Option<
    unsafe extern "C-unwind" fn(
        NonNull<ColorSyncMutableProfile>,
        NonNull<CFArray>,
        *mut CFDictionary,
    ) -> bool,
>;

/// [Apple's documentation](https://developer.apple.com/documentation/colorsync/cmminitializetransformproc?language=objc)
#[cfg(feature = "ColorSyncTransform")]
pub type CMMInitializeTransformProc = Option<
    unsafe extern "C-unwind" fn(
        NonNull<ColorSyncTransform>,
        NonNull<CFArray>,
        *mut CFDictionary,
    ) -> bool,
>;

/// [Apple's documentation](https://developer.apple.com/documentation/colorsync/cmmapplytransformproc?language=objc)
#[cfg(feature = "ColorSyncTransform")]
pub type CMMApplyTransformProc = Option<
    unsafe extern "C-unwind" fn(
        NonNull<ColorSyncTransform>,
        usize,
        usize,
        usize,
        NonNull<NonNull<c_void>>,
        ColorSyncDataDepth,
        ColorSyncDataLayout,
        usize,
        usize,
        NonNull<NonNull<c_void>>,
        ColorSyncDataDepth,
        ColorSyncDataLayout,
        usize,
        *mut CFDictionary,
    ) -> bool,
>;

/// [Apple's documentation](https://developer.apple.com/documentation/colorsync/cmmcreatetransformpropertyproc?language=objc)
#[cfg(feature = "ColorSyncTransform")]
pub type CMMCreateTransformPropertyProc = Option<
    unsafe extern "C-unwind" fn(
        *mut ColorSyncTransform,
        NonNull<CFType>,
        *mut CFDictionary,
    ) -> *mut CFType,
>;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/colorsync/kcmminitializelinkprofileprocname?language=objc)
    pub static kCMMInitializeLinkProfileProcName: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/colorsync/kcmminitializetransformprocname?language=objc)
    pub static kCMMInitializeTransformProcName: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/colorsync/kcmmapplytransformprocname?language=objc)
    pub static kCMMApplyTransformProcName: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/colorsync/kcmmcreatetransformpropertyprocname?language=objc)
    pub static kCMMCreateTransformPropertyProcName: &'static CFString;
}