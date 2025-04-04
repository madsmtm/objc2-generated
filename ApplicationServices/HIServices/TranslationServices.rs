//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::cell::UnsafeCell;
use core::ffi::*;
use core::marker::{PhantomData, PhantomPinned};
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/translation?language=objc)
#[repr(C)]
pub struct Translation {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

cf_type!(
    unsafe impl Translation {}
);
#[cfg(feature = "objc2")]
cf_objc2_type!(
    unsafe impl RefEncode<"OpaqueTranslationRef"> for Translation {}
);

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/badtranslationreferr?language=objc)
pub const badTranslationRefErr: c_int = -3031;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/translationflags?language=objc)
pub type TranslationFlags = OptionBits;

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/ktranslationdatatranslation?language=objc)
pub const kTranslationDataTranslation: c_uint = 1 << 0;
/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/ktranslationfiletranslation?language=objc)
pub const kTranslationFileTranslation: c_uint = 1 << 1;

unsafe impl ConcreteType for Translation {
    #[doc(alias = "TranslationGetTypeID")]
    #[inline]
    fn type_id() -> CFTypeID {
        extern "C-unwind" {
            fn TranslationGetTypeID() -> CFTypeID;
        }
        unsafe { TranslationGetTypeID() }
    }
}

extern "C-unwind" {
    pub fn TranslationCreate(
        in_source_type: Option<&CFString>,
        in_destination_type: Option<&CFString>,
        in_translation_flags: TranslationFlags,
        out_translation: *mut *mut Translation,
    ) -> OSStatus;
}

extern "C-unwind" {
    pub fn TranslationCreateWithSourceArray(
        in_source_types: Option<&CFArray>,
        in_translation_flags: TranslationFlags,
        out_destination_types: *mut *const CFArray,
        out_translations: *mut *const CFDictionary,
    ) -> OSStatus;
}

extern "C-unwind" {
    pub fn TranslationPerformForData(
        in_translation: &Translation,
        in_source_data: Option<&CFData>,
        out_destination_data: *mut *const CFData,
    ) -> OSStatus;
}

extern "C-unwind" {
    pub fn TranslationPerformForURL(
        in_translation: &Translation,
        in_source_url: Option<&CFURL>,
        in_destination_url: Option<&CFURL>,
        out_translated_url: *mut *const CFURL,
    ) -> OSStatus;
}

extern "C-unwind" {
    pub fn TranslationCopySourceType(
        in_translation: &Translation,
        out_source_type: *mut *const CFString,
    ) -> OSStatus;
}

extern "C-unwind" {
    pub fn TranslationCopyDestinationType(
        in_translation: &Translation,
        out_destination_type: *mut *const CFString,
    ) -> OSStatus;
}

extern "C-unwind" {
    pub fn TranslationGetTranslationFlags(
        in_translation: &Translation,
        out_translation_flags: *mut TranslationFlags,
    ) -> OSStatus;
}
