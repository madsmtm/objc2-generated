//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpdfdictionaryref?language=objc)
pub type CGPDFDictionaryRef = *mut c_void;

extern "C-unwind" {
    pub fn CGPDFDictionaryGetCount(dict: CGPDFDictionaryRef) -> usize;
}

extern "C-unwind" {
    #[cfg(feature = "CGPDFObject")]
    pub fn CGPDFDictionaryGetObject(
        dict: CGPDFDictionaryRef,
        key: NonNull<c_char>,
        value: *mut CGPDFObjectRef,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "CGPDFObject")]
    pub fn CGPDFDictionaryGetBoolean(
        dict: CGPDFDictionaryRef,
        key: NonNull<c_char>,
        value: *mut CGPDFBoolean,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "CGPDFObject")]
    pub fn CGPDFDictionaryGetInteger(
        dict: CGPDFDictionaryRef,
        key: NonNull<c_char>,
        value: *mut CGPDFInteger,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(all(feature = "CGPDFObject", feature = "objc2-core-foundation"))]
    pub fn CGPDFDictionaryGetNumber(
        dict: CGPDFDictionaryRef,
        key: NonNull<c_char>,
        value: *mut CGPDFReal,
    ) -> bool;
}

extern "C-unwind" {
    pub fn CGPDFDictionaryGetName(
        dict: CGPDFDictionaryRef,
        key: NonNull<c_char>,
        value: *mut *mut c_char,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "CGPDFString")]
    pub fn CGPDFDictionaryGetString(
        dict: CGPDFDictionaryRef,
        key: NonNull<c_char>,
        value: *mut CGPDFStringRef,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "CGPDFArray")]
    pub fn CGPDFDictionaryGetArray(
        dict: CGPDFDictionaryRef,
        key: NonNull<c_char>,
        value: *mut CGPDFArrayRef,
    ) -> bool;
}

extern "C-unwind" {
    pub fn CGPDFDictionaryGetDictionary(
        dict: CGPDFDictionaryRef,
        key: NonNull<c_char>,
        value: *mut CGPDFDictionaryRef,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "CGPDFStream")]
    pub fn CGPDFDictionaryGetStream(
        dict: CGPDFDictionaryRef,
        key: NonNull<c_char>,
        value: *mut CGPDFStreamRef,
    ) -> bool;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpdfdictionaryapplierfunction?language=objc)
#[cfg(feature = "CGPDFObject")]
pub type CGPDFDictionaryApplierFunction =
    Option<unsafe extern "C-unwind" fn(NonNull<c_char>, CGPDFObjectRef, *mut c_void)>;

extern "C-unwind" {
    #[cfg(feature = "CGPDFObject")]
    pub fn CGPDFDictionaryApplyFunction(
        dict: CGPDFDictionaryRef,
        function: CGPDFDictionaryApplierFunction,
        info: *mut c_void,
    );
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpdfdictionaryapplierblock?language=objc)
#[cfg(all(feature = "CGPDFObject", feature = "block2"))]
pub type CGPDFDictionaryApplierBlock =
    *mut block2::Block<dyn Fn(NonNull<c_char>, CGPDFObjectRef, *mut c_void) -> bool>;

extern "C-unwind" {
    #[cfg(all(feature = "CGPDFObject", feature = "block2"))]
    pub fn CGPDFDictionaryApplyBlock(
        dict: CGPDFDictionaryRef,
        block: CGPDFDictionaryApplierBlock,
        info: *mut c_void,
    );
}