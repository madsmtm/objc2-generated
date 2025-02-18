//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/security/cssm_spi_ac_funcs?language=objc)
#[cfg(all(feature = "cssmconfig", feature = "cssmtype"))]
#[deprecated]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct cssm_spi_ac_funcs {
    pub AuthCompute: Option<
        unsafe extern "C-unwind" fn(
            CSSM_AC_HANDLE,
            *const CSSM_TUPLEGROUP,
            *const CSSM_TUPLEGROUP,
            uint32,
            *const CSSM_LIST,
            *const CSSM_LIST,
            *const CSSM_LIST,
            CSSM_TUPLEGROUP_PTR,
        ) -> CSSM_RETURN,
    >,
    pub PassThrough: Option<
        unsafe extern "C-unwind" fn(
            CSSM_AC_HANDLE,
            CSSM_TP_HANDLE,
            CSSM_CL_HANDLE,
            CSSM_CC_HANDLE,
            *const CSSM_DL_DB_LIST,
            uint32,
            *const c_void,
            *mut *mut c_void,
        ) -> CSSM_RETURN,
    >,
}

#[cfg(all(feature = "cssmconfig", feature = "cssmtype", feature = "objc2"))]
unsafe impl Encode for cssm_spi_ac_funcs {
    const ENCODING: Encoding = Encoding::Struct(
        "cssm_spi_ac_funcs",
        &[
            <Option<
                unsafe extern "C-unwind" fn(
                    CSSM_AC_HANDLE,
                    *const CSSM_TUPLEGROUP,
                    *const CSSM_TUPLEGROUP,
                    uint32,
                    *const CSSM_LIST,
                    *const CSSM_LIST,
                    *const CSSM_LIST,
                    CSSM_TUPLEGROUP_PTR,
                ) -> CSSM_RETURN,
            >>::ENCODING,
            <Option<
                unsafe extern "C-unwind" fn(
                    CSSM_AC_HANDLE,
                    CSSM_TP_HANDLE,
                    CSSM_CL_HANDLE,
                    CSSM_CC_HANDLE,
                    *const CSSM_DL_DB_LIST,
                    uint32,
                    *const c_void,
                    *mut *mut c_void,
                ) -> CSSM_RETURN,
            >>::ENCODING,
        ],
    );
}

#[cfg(all(feature = "cssmconfig", feature = "cssmtype", feature = "objc2"))]
unsafe impl RefEncode for cssm_spi_ac_funcs {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/security/cssm_spi_ac_funcs?language=objc)
#[cfg(all(feature = "cssmconfig", feature = "cssmtype"))]
pub type CSSM_SPI_AC_FUNCS = cssm_spi_ac_funcs;

/// [Apple's documentation](https://developer.apple.com/documentation/security/cssm_spi_ac_funcs_ptr?language=objc)
#[cfg(all(feature = "cssmconfig", feature = "cssmtype"))]
pub type CSSM_SPI_AC_FUNCS_PTR = *mut cssm_spi_ac_funcs;
