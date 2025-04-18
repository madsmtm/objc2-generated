//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::cell::UnsafeCell;
use core::ffi::*;
use core::marker::{PhantomData, PhantomPinned};
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpdfboolean?language=objc)
pub type CGPDFBoolean = c_uchar;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpdfinteger?language=objc)
pub type CGPDFInteger = c_long;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpdfreal?language=objc)
pub type CGPDFReal = CGFloat;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpdfobject?language=objc)
#[repr(C)]
#[derive(Debug)]
pub struct CGPDFObject {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CGPDFObject {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Encoding::Struct("CGPDFObject", &[]));
}

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpdfobjectref?language=objc)
pub type CGPDFObjectRef = *mut CGPDFObject;

/// [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgpdfobjecttype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CGPDFObjectType(pub i32);
impl CGPDFObjectType {
    #[doc(alias = "kCGPDFObjectTypeNull")]
    pub const Null: Self = Self(1);
    #[doc(alias = "kCGPDFObjectTypeBoolean")]
    pub const Boolean: Self = Self(2);
    #[doc(alias = "kCGPDFObjectTypeInteger")]
    pub const Integer: Self = Self(3);
    #[doc(alias = "kCGPDFObjectTypeReal")]
    pub const Real: Self = Self(4);
    #[doc(alias = "kCGPDFObjectTypeName")]
    pub const Name: Self = Self(5);
    #[doc(alias = "kCGPDFObjectTypeString")]
    pub const String: Self = Self(6);
    #[doc(alias = "kCGPDFObjectTypeArray")]
    pub const Array: Self = Self(7);
    #[doc(alias = "kCGPDFObjectTypeDictionary")]
    pub const Dictionary: Self = Self(8);
    #[doc(alias = "kCGPDFObjectTypeStream")]
    pub const Stream: Self = Self(9);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CGPDFObjectType {
    const ENCODING: Encoding = i32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CGPDFObjectType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

impl CGPDFObject {
    #[doc(alias = "CGPDFObjectGetType")]
    #[inline]
    pub unsafe fn r#type(object: CGPDFObjectRef) -> CGPDFObjectType {
        extern "C-unwind" {
            fn CGPDFObjectGetType(object: CGPDFObjectRef) -> CGPDFObjectType;
        }
        unsafe { CGPDFObjectGetType(object) }
    }

    #[doc(alias = "CGPDFObjectGetValue")]
    #[inline]
    pub unsafe fn value(
        object: CGPDFObjectRef,
        r#type: CGPDFObjectType,
        value: *mut c_void,
    ) -> bool {
        extern "C-unwind" {
            fn CGPDFObjectGetValue(
                object: CGPDFObjectRef,
                r#type: CGPDFObjectType,
                value: *mut c_void,
            ) -> bool;
        }
        unsafe { CGPDFObjectGetValue(object, r#type, value) }
    }
}

extern "C-unwind" {
    #[deprecated = "renamed to `CGPDFObject::type`"]
    pub fn CGPDFObjectGetType(object: CGPDFObjectRef) -> CGPDFObjectType;
}

extern "C-unwind" {
    #[deprecated = "renamed to `CGPDFObject::value`"]
    pub fn CGPDFObjectGetValue(
        object: CGPDFObjectRef,
        r#type: CGPDFObjectType,
        value: *mut c_void,
    ) -> bool;
}
