//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/security/cssm_spi_cl_funcs?language=objc)
#[cfg(all(feature = "SecAsn1Types", feature = "cssmconfig", feature = "cssmtype"))]
#[deprecated]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct cssm_spi_cl_funcs {
    pub CertCreateTemplate: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            uint32,
            *const CSSM_FIELD,
            CSSM_DATA_PTR,
        ) -> CSSM_RETURN,
    >,
    pub CertGetAllTemplateFields: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            *const SecAsn1Item,
            *mut uint32,
            *mut CSSM_FIELD_PTR,
        ) -> CSSM_RETURN,
    >,
    pub CertSign: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            CSSM_CC_HANDLE,
            *const SecAsn1Item,
            *const CSSM_FIELD,
            uint32,
            CSSM_DATA_PTR,
        ) -> CSSM_RETURN,
    >,
    pub CertVerify: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            CSSM_CC_HANDLE,
            *const SecAsn1Item,
            *const SecAsn1Item,
            *const CSSM_FIELD,
            uint32,
        ) -> CSSM_RETURN,
    >,
    pub CertVerifyWithKey: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            CSSM_CC_HANDLE,
            *const SecAsn1Item,
        ) -> CSSM_RETURN,
    >,
    pub CertGetFirstFieldValue: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            *const SecAsn1Item,
            *const SecAsn1Oid,
            CSSM_HANDLE_PTR,
            *mut uint32,
            *mut CSSM_DATA_PTR,
        ) -> CSSM_RETURN,
    >,
    pub CertGetNextFieldValue: Option<
        unsafe extern "C-unwind" fn(CSSM_CL_HANDLE, CSSM_HANDLE, *mut CSSM_DATA_PTR) -> CSSM_RETURN,
    >,
    pub CertAbortQuery:
        Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE, CSSM_HANDLE) -> CSSM_RETURN>,
    pub CertGetKeyInfo: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            *const SecAsn1Item,
            *mut CSSM_KEY_PTR,
        ) -> CSSM_RETURN,
    >,
    pub CertGetAllFields: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            *const SecAsn1Item,
            *mut uint32,
            *mut CSSM_FIELD_PTR,
        ) -> CSSM_RETURN,
    >,
    pub FreeFields: Option<
        unsafe extern "C-unwind" fn(CSSM_CL_HANDLE, uint32, *mut CSSM_FIELD_PTR) -> CSSM_RETURN,
    >,
    pub FreeFieldValue: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            *const SecAsn1Oid,
            CSSM_DATA_PTR,
        ) -> CSSM_RETURN,
    >,
    pub CertCache: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            *const SecAsn1Item,
            CSSM_HANDLE_PTR,
        ) -> CSSM_RETURN,
    >,
    pub CertGetFirstCachedFieldValue: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            CSSM_HANDLE,
            *const SecAsn1Oid,
            CSSM_HANDLE_PTR,
            *mut uint32,
            *mut CSSM_DATA_PTR,
        ) -> CSSM_RETURN,
    >,
    pub CertGetNextCachedFieldValue: Option<
        unsafe extern "C-unwind" fn(CSSM_CL_HANDLE, CSSM_HANDLE, *mut CSSM_DATA_PTR) -> CSSM_RETURN,
    >,
    pub CertAbortCache:
        Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE, CSSM_HANDLE) -> CSSM_RETURN>,
    pub CertGroupToSignedBundle: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            CSSM_CC_HANDLE,
            *const CSSM_CERTGROUP,
            *const CSSM_CERT_BUNDLE_HEADER,
            CSSM_DATA_PTR,
        ) -> CSSM_RETURN,
    >,
    pub CertGroupFromVerifiedBundle: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            CSSM_CC_HANDLE,
            *const CSSM_CERT_BUNDLE,
            *const SecAsn1Item,
            *mut CSSM_CERTGROUP_PTR,
        ) -> CSSM_RETURN,
    >,
    pub CertDescribeFormat: Option<
        unsafe extern "C-unwind" fn(CSSM_CL_HANDLE, *mut uint32, *mut CSSM_OID_PTR) -> CSSM_RETURN,
    >,
    pub CrlCreateTemplate: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            uint32,
            *const CSSM_FIELD,
            CSSM_DATA_PTR,
        ) -> CSSM_RETURN,
    >,
    pub CrlSetFields: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            uint32,
            *const CSSM_FIELD,
            *const SecAsn1Item,
            CSSM_DATA_PTR,
        ) -> CSSM_RETURN,
    >,
    pub CrlAddCert: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            CSSM_CC_HANDLE,
            *const SecAsn1Item,
            uint32,
            *const CSSM_FIELD,
            *const SecAsn1Item,
            CSSM_DATA_PTR,
        ) -> CSSM_RETURN,
    >,
    pub CrlRemoveCert: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            *const SecAsn1Item,
            *const SecAsn1Item,
            CSSM_DATA_PTR,
        ) -> CSSM_RETURN,
    >,
    pub CrlSign: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            CSSM_CC_HANDLE,
            *const SecAsn1Item,
            *const CSSM_FIELD,
            uint32,
            CSSM_DATA_PTR,
        ) -> CSSM_RETURN,
    >,
    pub CrlVerify: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            CSSM_CC_HANDLE,
            *const SecAsn1Item,
            *const SecAsn1Item,
            *const CSSM_FIELD,
            uint32,
        ) -> CSSM_RETURN,
    >,
    pub CrlVerifyWithKey: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            CSSM_CC_HANDLE,
            *const SecAsn1Item,
        ) -> CSSM_RETURN,
    >,
    pub IsCertInCrl: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            *const SecAsn1Item,
            *const SecAsn1Item,
            *mut CSSM_BOOL,
        ) -> CSSM_RETURN,
    >,
    pub CrlGetFirstFieldValue: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            *const SecAsn1Item,
            *const SecAsn1Oid,
            CSSM_HANDLE_PTR,
            *mut uint32,
            *mut CSSM_DATA_PTR,
        ) -> CSSM_RETURN,
    >,
    pub CrlGetNextFieldValue: Option<
        unsafe extern "C-unwind" fn(CSSM_CL_HANDLE, CSSM_HANDLE, *mut CSSM_DATA_PTR) -> CSSM_RETURN,
    >,
    pub CrlAbortQuery:
        Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE, CSSM_HANDLE) -> CSSM_RETURN>,
    pub CrlGetAllFields: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            *const SecAsn1Item,
            *mut uint32,
            *mut CSSM_FIELD_PTR,
        ) -> CSSM_RETURN,
    >,
    pub CrlCache: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            *const SecAsn1Item,
            CSSM_HANDLE_PTR,
        ) -> CSSM_RETURN,
    >,
    pub IsCertInCachedCrl: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            *const SecAsn1Item,
            CSSM_HANDLE,
            *mut CSSM_BOOL,
            CSSM_DATA_PTR,
        ) -> CSSM_RETURN,
    >,
    pub CrlGetFirstCachedFieldValue: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            CSSM_HANDLE,
            *const SecAsn1Item,
            *const SecAsn1Oid,
            CSSM_HANDLE_PTR,
            *mut uint32,
            *mut CSSM_DATA_PTR,
        ) -> CSSM_RETURN,
    >,
    pub CrlGetNextCachedFieldValue: Option<
        unsafe extern "C-unwind" fn(CSSM_CL_HANDLE, CSSM_HANDLE, *mut CSSM_DATA_PTR) -> CSSM_RETURN,
    >,
    pub CrlGetAllCachedRecordFields: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            CSSM_HANDLE,
            *const SecAsn1Item,
            *mut uint32,
            *mut CSSM_FIELD_PTR,
        ) -> CSSM_RETURN,
    >,
    pub CrlAbortCache:
        Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE, CSSM_HANDLE) -> CSSM_RETURN>,
    pub CrlDescribeFormat: Option<
        unsafe extern "C-unwind" fn(CSSM_CL_HANDLE, *mut uint32, *mut CSSM_OID_PTR) -> CSSM_RETURN,
    >,
    pub PassThrough: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CL_HANDLE,
            CSSM_CC_HANDLE,
            uint32,
            *const c_void,
            *mut *mut c_void,
        ) -> CSSM_RETURN,
    >,
}

#[cfg(all(
    feature = "SecAsn1Types",
    feature = "cssmconfig",
    feature = "cssmtype",
    feature = "objc2"
))]
unsafe impl Encode for cssm_spi_cl_funcs {
    const ENCODING: Encoding = Encoding::Struct("cssm_spi_cl_funcs", &[
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,uint32,*const CSSM_FIELD,CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,*const SecAsn1Item,*mut uint32,*mut CSSM_FIELD_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,CSSM_CC_HANDLE,*const SecAsn1Item,*const CSSM_FIELD,uint32,CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,CSSM_CC_HANDLE,*const SecAsn1Item,*const SecAsn1Item,*const CSSM_FIELD,uint32,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,CSSM_CC_HANDLE,*const SecAsn1Item,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,*const SecAsn1Item,*const SecAsn1Oid,CSSM_HANDLE_PTR,*mut uint32,*mut CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,CSSM_HANDLE,*mut CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,CSSM_HANDLE,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,*const SecAsn1Item,*mut CSSM_KEY_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,*const SecAsn1Item,*mut uint32,*mut CSSM_FIELD_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,uint32,*mut CSSM_FIELD_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,*const SecAsn1Oid,CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,*const SecAsn1Item,CSSM_HANDLE_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,CSSM_HANDLE,*const SecAsn1Oid,CSSM_HANDLE_PTR,*mut uint32,*mut CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,CSSM_HANDLE,*mut CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,CSSM_HANDLE,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,CSSM_CC_HANDLE,*const CSSM_CERTGROUP,*const CSSM_CERT_BUNDLE_HEADER,CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,CSSM_CC_HANDLE,*const CSSM_CERT_BUNDLE,*const SecAsn1Item,*mut CSSM_CERTGROUP_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,*mut uint32,*mut CSSM_OID_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,uint32,*const CSSM_FIELD,CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,uint32,*const CSSM_FIELD,*const SecAsn1Item,CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,CSSM_CC_HANDLE,*const SecAsn1Item,uint32,*const CSSM_FIELD,*const SecAsn1Item,CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,*const SecAsn1Item,*const SecAsn1Item,CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,CSSM_CC_HANDLE,*const SecAsn1Item,*const CSSM_FIELD,uint32,CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,CSSM_CC_HANDLE,*const SecAsn1Item,*const SecAsn1Item,*const CSSM_FIELD,uint32,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,CSSM_CC_HANDLE,*const SecAsn1Item,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,*const SecAsn1Item,*const SecAsn1Item,*mut CSSM_BOOL,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,*const SecAsn1Item,*const SecAsn1Oid,CSSM_HANDLE_PTR,*mut uint32,*mut CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,CSSM_HANDLE,*mut CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,CSSM_HANDLE,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,*const SecAsn1Item,*mut uint32,*mut CSSM_FIELD_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,*const SecAsn1Item,CSSM_HANDLE_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,*const SecAsn1Item,CSSM_HANDLE,*mut CSSM_BOOL,CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,CSSM_HANDLE,*const SecAsn1Item,*const SecAsn1Oid,CSSM_HANDLE_PTR,*mut uint32,*mut CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,CSSM_HANDLE,*mut CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,CSSM_HANDLE,*const SecAsn1Item,*mut uint32,*mut CSSM_FIELD_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,CSSM_HANDLE,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,*mut uint32,*mut CSSM_OID_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CL_HANDLE,CSSM_CC_HANDLE,uint32,*const c_void,*mut *mut c_void,) -> CSSM_RETURN>>::ENCODING,
    ]);
}

#[cfg(all(
    feature = "SecAsn1Types",
    feature = "cssmconfig",
    feature = "cssmtype",
    feature = "objc2"
))]
unsafe impl RefEncode for cssm_spi_cl_funcs {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/security/cssm_spi_cl_funcs?language=objc)
#[cfg(all(feature = "SecAsn1Types", feature = "cssmconfig", feature = "cssmtype"))]
pub type CSSM_SPI_CL_FUNCS = cssm_spi_cl_funcs;

/// [Apple's documentation](https://developer.apple.com/documentation/security/cssm_spi_cl_funcs_ptr?language=objc)
#[cfg(all(feature = "SecAsn1Types", feature = "cssmconfig", feature = "cssmtype"))]
pub type CSSM_SPI_CL_FUNCS_PTR = *mut cssm_spi_cl_funcs;