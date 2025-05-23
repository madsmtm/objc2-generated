//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::cell::UnsafeCell;
use core::marker::{PhantomData, PhantomPinned};
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfuuid?language=objc)
#[repr(C)]
pub struct CFUUID {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

cf_type!(
    unsafe impl CFUUID {}
);
#[cfg(feature = "objc2")]
cf_objc2_type!(
    unsafe impl RefEncode<"__CFUUID"> for CFUUID {}
);

/// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfuuidbytes?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CFUUIDBytes {
    pub byte0: u8,
    pub byte1: u8,
    pub byte2: u8,
    pub byte3: u8,
    pub byte4: u8,
    pub byte5: u8,
    pub byte6: u8,
    pub byte7: u8,
    pub byte8: u8,
    pub byte9: u8,
    pub byte10: u8,
    pub byte11: u8,
    pub byte12: u8,
    pub byte13: u8,
    pub byte14: u8,
    pub byte15: u8,
}

#[cfg(feature = "objc2")]
unsafe impl Encode for CFUUIDBytes {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <u8>::ENCODING,
            <u8>::ENCODING,
            <u8>::ENCODING,
            <u8>::ENCODING,
            <u8>::ENCODING,
            <u8>::ENCODING,
            <u8>::ENCODING,
            <u8>::ENCODING,
            <u8>::ENCODING,
            <u8>::ENCODING,
            <u8>::ENCODING,
            <u8>::ENCODING,
            <u8>::ENCODING,
            <u8>::ENCODING,
            <u8>::ENCODING,
            <u8>::ENCODING,
        ],
    );
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CFUUIDBytes {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

unsafe impl ConcreteType for CFUUID {
    #[doc(alias = "CFUUIDGetTypeID")]
    #[inline]
    fn type_id() -> CFTypeID {
        extern "C-unwind" {
            fn CFUUIDGetTypeID() -> CFTypeID;
        }
        unsafe { CFUUIDGetTypeID() }
    }
}

impl CFUUID {
    #[doc(alias = "CFUUIDCreate")]
    #[inline]
    pub fn new(alloc: Option<&CFAllocator>) -> Option<CFRetained<CFUUID>> {
        extern "C-unwind" {
            fn CFUUIDCreate(alloc: Option<&CFAllocator>) -> Option<NonNull<CFUUID>>;
        }
        let ret = unsafe { CFUUIDCreate(alloc) };
        ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
    }

    #[doc(alias = "CFUUIDCreateWithBytes")]
    #[inline]
    pub fn with_bytes(
        alloc: Option<&CFAllocator>,
        byte0: u8,
        byte1: u8,
        byte2: u8,
        byte3: u8,
        byte4: u8,
        byte5: u8,
        byte6: u8,
        byte7: u8,
        byte8: u8,
        byte9: u8,
        byte10: u8,
        byte11: u8,
        byte12: u8,
        byte13: u8,
        byte14: u8,
        byte15: u8,
    ) -> Option<CFRetained<CFUUID>> {
        extern "C-unwind" {
            fn CFUUIDCreateWithBytes(
                alloc: Option<&CFAllocator>,
                byte0: u8,
                byte1: u8,
                byte2: u8,
                byte3: u8,
                byte4: u8,
                byte5: u8,
                byte6: u8,
                byte7: u8,
                byte8: u8,
                byte9: u8,
                byte10: u8,
                byte11: u8,
                byte12: u8,
                byte13: u8,
                byte14: u8,
                byte15: u8,
            ) -> Option<NonNull<CFUUID>>;
        }
        let ret = unsafe {
            CFUUIDCreateWithBytes(
                alloc, byte0, byte1, byte2, byte3, byte4, byte5, byte6, byte7, byte8, byte9,
                byte10, byte11, byte12, byte13, byte14, byte15,
            )
        };
        ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
    }

    #[doc(alias = "CFUUIDCreateFromString")]
    #[inline]
    pub fn from_string(
        alloc: Option<&CFAllocator>,
        uuid_str: Option<&CFString>,
    ) -> Option<CFRetained<CFUUID>> {
        extern "C-unwind" {
            fn CFUUIDCreateFromString(
                alloc: Option<&CFAllocator>,
                uuid_str: Option<&CFString>,
            ) -> Option<NonNull<CFUUID>>;
        }
        let ret = unsafe { CFUUIDCreateFromString(alloc, uuid_str) };
        ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
    }

    #[doc(alias = "CFUUIDCreateString")]
    #[inline]
    pub fn new_string(
        alloc: Option<&CFAllocator>,
        uuid: Option<&CFUUID>,
    ) -> Option<CFRetained<CFString>> {
        extern "C-unwind" {
            fn CFUUIDCreateString(
                alloc: Option<&CFAllocator>,
                uuid: Option<&CFUUID>,
            ) -> Option<NonNull<CFString>>;
        }
        let ret = unsafe { CFUUIDCreateString(alloc, uuid) };
        ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
    }

    #[doc(alias = "CFUUIDGetConstantUUIDWithBytes")]
    #[inline]
    pub fn constant_uuid_with_bytes(
        alloc: Option<&CFAllocator>,
        byte0: u8,
        byte1: u8,
        byte2: u8,
        byte3: u8,
        byte4: u8,
        byte5: u8,
        byte6: u8,
        byte7: u8,
        byte8: u8,
        byte9: u8,
        byte10: u8,
        byte11: u8,
        byte12: u8,
        byte13: u8,
        byte14: u8,
        byte15: u8,
    ) -> Option<CFRetained<CFUUID>> {
        extern "C-unwind" {
            fn CFUUIDGetConstantUUIDWithBytes(
                alloc: Option<&CFAllocator>,
                byte0: u8,
                byte1: u8,
                byte2: u8,
                byte3: u8,
                byte4: u8,
                byte5: u8,
                byte6: u8,
                byte7: u8,
                byte8: u8,
                byte9: u8,
                byte10: u8,
                byte11: u8,
                byte12: u8,
                byte13: u8,
                byte14: u8,
                byte15: u8,
            ) -> Option<NonNull<CFUUID>>;
        }
        let ret = unsafe {
            CFUUIDGetConstantUUIDWithBytes(
                alloc, byte0, byte1, byte2, byte3, byte4, byte5, byte6, byte7, byte8, byte9,
                byte10, byte11, byte12, byte13, byte14, byte15,
            )
        };
        ret.map(|ret| unsafe { CFRetained::retain(ret) })
    }

    #[doc(alias = "CFUUIDGetUUIDBytes")]
    #[inline]
    pub fn uuid_bytes(self: &CFUUID) -> CFUUIDBytes {
        extern "C-unwind" {
            fn CFUUIDGetUUIDBytes(uuid: &CFUUID) -> CFUUIDBytes;
        }
        unsafe { CFUUIDGetUUIDBytes(self) }
    }

    #[doc(alias = "CFUUIDCreateFromUUIDBytes")]
    #[inline]
    pub fn from_uuid_bytes(
        alloc: Option<&CFAllocator>,
        bytes: CFUUIDBytes,
    ) -> Option<CFRetained<CFUUID>> {
        extern "C-unwind" {
            fn CFUUIDCreateFromUUIDBytes(
                alloc: Option<&CFAllocator>,
                bytes: CFUUIDBytes,
            ) -> Option<NonNull<CFUUID>>;
        }
        let ret = unsafe { CFUUIDCreateFromUUIDBytes(alloc, bytes) };
        ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
    }
}

#[deprecated = "renamed to `CFUUID::new`"]
#[inline]
pub extern "C-unwind" fn CFUUIDCreate(alloc: Option<&CFAllocator>) -> Option<CFRetained<CFUUID>> {
    extern "C-unwind" {
        fn CFUUIDCreate(alloc: Option<&CFAllocator>) -> Option<NonNull<CFUUID>>;
    }
    let ret = unsafe { CFUUIDCreate(alloc) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[deprecated = "renamed to `CFUUID::with_bytes`"]
#[inline]
pub extern "C-unwind" fn CFUUIDCreateWithBytes(
    alloc: Option<&CFAllocator>,
    byte0: u8,
    byte1: u8,
    byte2: u8,
    byte3: u8,
    byte4: u8,
    byte5: u8,
    byte6: u8,
    byte7: u8,
    byte8: u8,
    byte9: u8,
    byte10: u8,
    byte11: u8,
    byte12: u8,
    byte13: u8,
    byte14: u8,
    byte15: u8,
) -> Option<CFRetained<CFUUID>> {
    extern "C-unwind" {
        fn CFUUIDCreateWithBytes(
            alloc: Option<&CFAllocator>,
            byte0: u8,
            byte1: u8,
            byte2: u8,
            byte3: u8,
            byte4: u8,
            byte5: u8,
            byte6: u8,
            byte7: u8,
            byte8: u8,
            byte9: u8,
            byte10: u8,
            byte11: u8,
            byte12: u8,
            byte13: u8,
            byte14: u8,
            byte15: u8,
        ) -> Option<NonNull<CFUUID>>;
    }
    let ret = unsafe {
        CFUUIDCreateWithBytes(
            alloc, byte0, byte1, byte2, byte3, byte4, byte5, byte6, byte7, byte8, byte9, byte10,
            byte11, byte12, byte13, byte14, byte15,
        )
    };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[deprecated = "renamed to `CFUUID::from_string`"]
#[inline]
pub extern "C-unwind" fn CFUUIDCreateFromString(
    alloc: Option<&CFAllocator>,
    uuid_str: Option<&CFString>,
) -> Option<CFRetained<CFUUID>> {
    extern "C-unwind" {
        fn CFUUIDCreateFromString(
            alloc: Option<&CFAllocator>,
            uuid_str: Option<&CFString>,
        ) -> Option<NonNull<CFUUID>>;
    }
    let ret = unsafe { CFUUIDCreateFromString(alloc, uuid_str) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[deprecated = "renamed to `CFUUID::new_string`"]
#[inline]
pub extern "C-unwind" fn CFUUIDCreateString(
    alloc: Option<&CFAllocator>,
    uuid: Option<&CFUUID>,
) -> Option<CFRetained<CFString>> {
    extern "C-unwind" {
        fn CFUUIDCreateString(
            alloc: Option<&CFAllocator>,
            uuid: Option<&CFUUID>,
        ) -> Option<NonNull<CFString>>;
    }
    let ret = unsafe { CFUUIDCreateString(alloc, uuid) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

#[deprecated = "renamed to `CFUUID::constant_uuid_with_bytes`"]
#[inline]
pub extern "C-unwind" fn CFUUIDGetConstantUUIDWithBytes(
    alloc: Option<&CFAllocator>,
    byte0: u8,
    byte1: u8,
    byte2: u8,
    byte3: u8,
    byte4: u8,
    byte5: u8,
    byte6: u8,
    byte7: u8,
    byte8: u8,
    byte9: u8,
    byte10: u8,
    byte11: u8,
    byte12: u8,
    byte13: u8,
    byte14: u8,
    byte15: u8,
) -> Option<CFRetained<CFUUID>> {
    extern "C-unwind" {
        fn CFUUIDGetConstantUUIDWithBytes(
            alloc: Option<&CFAllocator>,
            byte0: u8,
            byte1: u8,
            byte2: u8,
            byte3: u8,
            byte4: u8,
            byte5: u8,
            byte6: u8,
            byte7: u8,
            byte8: u8,
            byte9: u8,
            byte10: u8,
            byte11: u8,
            byte12: u8,
            byte13: u8,
            byte14: u8,
            byte15: u8,
        ) -> Option<NonNull<CFUUID>>;
    }
    let ret = unsafe {
        CFUUIDGetConstantUUIDWithBytes(
            alloc, byte0, byte1, byte2, byte3, byte4, byte5, byte6, byte7, byte8, byte9, byte10,
            byte11, byte12, byte13, byte14, byte15,
        )
    };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

#[deprecated = "renamed to `CFUUID::uuid_bytes`"]
#[inline]
pub extern "C-unwind" fn CFUUIDGetUUIDBytes(uuid: &CFUUID) -> CFUUIDBytes {
    extern "C-unwind" {
        fn CFUUIDGetUUIDBytes(uuid: &CFUUID) -> CFUUIDBytes;
    }
    unsafe { CFUUIDGetUUIDBytes(uuid) }
}

#[deprecated = "renamed to `CFUUID::from_uuid_bytes`"]
#[inline]
pub extern "C-unwind" fn CFUUIDCreateFromUUIDBytes(
    alloc: Option<&CFAllocator>,
    bytes: CFUUIDBytes,
) -> Option<CFRetained<CFUUID>> {
    extern "C-unwind" {
        fn CFUUIDCreateFromUUIDBytes(
            alloc: Option<&CFAllocator>,
            bytes: CFUUIDBytes,
        ) -> Option<NonNull<CFUUID>>;
    }
    let ret = unsafe { CFUUIDCreateFromUUIDBytes(alloc, bytes) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}
