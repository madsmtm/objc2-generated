//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/security/cssm_spi_csp_funcs?language=objc)
#[cfg(all(
    feature = "SecAsn1Types",
    feature = "cssmconfig",
    feature = "cssmkrapi",
    feature = "cssmspi",
    feature = "cssmtype"
))]
#[deprecated]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct cssm_spi_csp_funcs {
    pub EventNotify: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CONTEXT_EVENT,
            CSSM_CC_HANDLE,
            *const CSSM_CONTEXT,
        ) -> CSSM_RETURN,
    >,
    pub QuerySize: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const CSSM_CONTEXT,
            CSSM_BOOL,
            uint32,
            CSSM_QUERY_SIZE_DATA_PTR,
        ) -> CSSM_RETURN,
    >,
    pub SignData: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const CSSM_CONTEXT,
            *const SecAsn1Item,
            uint32,
            CSSM_ALGORITHMS,
            CSSM_DATA_PTR,
        ) -> CSSM_RETURN,
    >,
    pub SignDataInit: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const CSSM_CONTEXT,
        ) -> CSSM_RETURN,
    >,
    pub SignDataUpdate: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const SecAsn1Item,
            uint32,
        ) -> CSSM_RETURN,
    >,
    pub SignDataFinal: Option<
        unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE, CSSM_CC_HANDLE, CSSM_DATA_PTR) -> CSSM_RETURN,
    >,
    pub VerifyData: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const CSSM_CONTEXT,
            *const SecAsn1Item,
            uint32,
            CSSM_ALGORITHMS,
            *const SecAsn1Item,
        ) -> CSSM_RETURN,
    >,
    pub VerifyDataInit: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const CSSM_CONTEXT,
        ) -> CSSM_RETURN,
    >,
    pub VerifyDataUpdate: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const SecAsn1Item,
            uint32,
        ) -> CSSM_RETURN,
    >,
    pub VerifyDataFinal: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const SecAsn1Item,
        ) -> CSSM_RETURN,
    >,
    pub DigestData: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const CSSM_CONTEXT,
            *const SecAsn1Item,
            uint32,
            CSSM_DATA_PTR,
        ) -> CSSM_RETURN,
    >,
    pub DigestDataInit: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const CSSM_CONTEXT,
        ) -> CSSM_RETURN,
    >,
    pub DigestDataUpdate: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const SecAsn1Item,
            uint32,
        ) -> CSSM_RETURN,
    >,
    pub DigestDataClone: Option<
        unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE, CSSM_CC_HANDLE, CSSM_CC_HANDLE) -> CSSM_RETURN,
    >,
    pub DigestDataFinal: Option<
        unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE, CSSM_CC_HANDLE, CSSM_DATA_PTR) -> CSSM_RETURN,
    >,
    pub GenerateMac: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const CSSM_CONTEXT,
            *const SecAsn1Item,
            uint32,
            CSSM_DATA_PTR,
        ) -> CSSM_RETURN,
    >,
    pub GenerateMacInit: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const CSSM_CONTEXT,
        ) -> CSSM_RETURN,
    >,
    pub GenerateMacUpdate: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const SecAsn1Item,
            uint32,
        ) -> CSSM_RETURN,
    >,
    pub GenerateMacFinal: Option<
        unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE, CSSM_CC_HANDLE, CSSM_DATA_PTR) -> CSSM_RETURN,
    >,
    pub VerifyMac: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const CSSM_CONTEXT,
            *const SecAsn1Item,
            uint32,
            *const SecAsn1Item,
        ) -> CSSM_RETURN,
    >,
    pub VerifyMacInit: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const CSSM_CONTEXT,
        ) -> CSSM_RETURN,
    >,
    pub VerifyMacUpdate: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const SecAsn1Item,
            uint32,
        ) -> CSSM_RETURN,
    >,
    pub VerifyMacFinal: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const SecAsn1Item,
        ) -> CSSM_RETURN,
    >,
    pub EncryptData: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const CSSM_CONTEXT,
            *const SecAsn1Item,
            uint32,
            CSSM_DATA_PTR,
            uint32,
            *mut CSSM_SIZE,
            CSSM_DATA_PTR,
            CSSM_PRIVILEGE,
        ) -> CSSM_RETURN,
    >,
    pub EncryptDataInit: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const CSSM_CONTEXT,
            CSSM_PRIVILEGE,
        ) -> CSSM_RETURN,
    >,
    pub EncryptDataUpdate: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const SecAsn1Item,
            uint32,
            CSSM_DATA_PTR,
            uint32,
            *mut CSSM_SIZE,
        ) -> CSSM_RETURN,
    >,
    pub EncryptDataFinal: Option<
        unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE, CSSM_CC_HANDLE, CSSM_DATA_PTR) -> CSSM_RETURN,
    >,
    pub DecryptData: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const CSSM_CONTEXT,
            *const SecAsn1Item,
            uint32,
            CSSM_DATA_PTR,
            uint32,
            *mut CSSM_SIZE,
            CSSM_DATA_PTR,
            CSSM_PRIVILEGE,
        ) -> CSSM_RETURN,
    >,
    pub DecryptDataInit: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const CSSM_CONTEXT,
            CSSM_PRIVILEGE,
        ) -> CSSM_RETURN,
    >,
    pub DecryptDataUpdate: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const SecAsn1Item,
            uint32,
            CSSM_DATA_PTR,
            uint32,
            *mut CSSM_SIZE,
        ) -> CSSM_RETURN,
    >,
    pub DecryptDataFinal: Option<
        unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE, CSSM_CC_HANDLE, CSSM_DATA_PTR) -> CSSM_RETURN,
    >,
    pub QueryKeySizeInBits: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const CSSM_CONTEXT,
            *const CSSM_KEY,
            CSSM_KEY_SIZE_PTR,
        ) -> CSSM_RETURN,
    >,
    pub GenerateKey: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const CSSM_CONTEXT,
            uint32,
            uint32,
            *const SecAsn1Item,
            *const CSSM_RESOURCE_CONTROL_CONTEXT,
            CSSM_KEY_PTR,
            CSSM_PRIVILEGE,
        ) -> CSSM_RETURN,
    >,
    pub GenerateKeyPair: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const CSSM_CONTEXT,
            uint32,
            uint32,
            *const SecAsn1Item,
            CSSM_KEY_PTR,
            uint32,
            uint32,
            *const SecAsn1Item,
            *const CSSM_RESOURCE_CONTROL_CONTEXT,
            CSSM_KEY_PTR,
            CSSM_PRIVILEGE,
        ) -> CSSM_RETURN,
    >,
    pub GenerateRandom: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const CSSM_CONTEXT,
            CSSM_DATA_PTR,
        ) -> CSSM_RETURN,
    >,
    pub GenerateAlgorithmParams: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const CSSM_CONTEXT,
            uint32,
            CSSM_DATA_PTR,
            *mut uint32,
            *mut CSSM_CONTEXT_ATTRIBUTE_PTR,
        ) -> CSSM_RETURN,
    >,
    pub WrapKey: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const CSSM_CONTEXT,
            *const CSSM_ACCESS_CREDENTIALS,
            *const CSSM_KEY,
            *const SecAsn1Item,
            CSSM_WRAP_KEY_PTR,
            CSSM_PRIVILEGE,
        ) -> CSSM_RETURN,
    >,
    pub UnwrapKey: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const CSSM_CONTEXT,
            *const CSSM_KEY,
            *const CSSM_WRAP_KEY,
            uint32,
            uint32,
            *const SecAsn1Item,
            *const CSSM_RESOURCE_CONTROL_CONTEXT,
            CSSM_KEY_PTR,
            CSSM_DATA_PTR,
            CSSM_PRIVILEGE,
        ) -> CSSM_RETURN,
    >,
    pub DeriveKey: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const CSSM_CONTEXT,
            CSSM_DATA_PTR,
            uint32,
            uint32,
            *const SecAsn1Item,
            *const CSSM_RESOURCE_CONTROL_CONTEXT,
            CSSM_KEY_PTR,
        ) -> CSSM_RETURN,
    >,
    pub FreeKey: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            *const CSSM_ACCESS_CREDENTIALS,
            CSSM_KEY_PTR,
            CSSM_BOOL,
        ) -> CSSM_RETURN,
    >,
    pub PassThrough: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_CC_HANDLE,
            *const CSSM_CONTEXT,
            uint32,
            *const c_void,
            *mut *mut c_void,
        ) -> CSSM_RETURN,
    >,
    pub Login: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            *const CSSM_ACCESS_CREDENTIALS,
            *const SecAsn1Item,
            *const c_void,
        ) -> CSSM_RETURN,
    >,
    pub Logout: Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE) -> CSSM_RETURN>,
    pub ChangeLoginAcl: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            *const CSSM_ACCESS_CREDENTIALS,
            *const CSSM_ACL_EDIT,
        ) -> CSSM_RETURN,
    >,
    pub ObtainPrivateKeyFromPublicKey: Option<
        unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE, *const CSSM_KEY, CSSM_KEY_PTR) -> CSSM_RETURN,
    >,
    pub RetrieveUniqueId:
        Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE, CSSM_DATA_PTR) -> CSSM_RETURN>,
    pub RetrieveCounter:
        Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE, CSSM_DATA_PTR) -> CSSM_RETURN>,
    pub VerifyDevice:
        Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE, *const SecAsn1Item) -> CSSM_RETURN>,
    pub GetTimeValue: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            CSSM_ALGORITHMS,
            *mut SecAsn1Item,
        ) -> CSSM_RETURN,
    >,
    pub GetOperationalStatistics: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            *mut CSSM_CSP_OPERATIONAL_STATISTICS,
        ) -> CSSM_RETURN,
    >,
    pub GetLoginAcl: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            *const CSSM_STRING,
            *mut uint32,
            *mut CSSM_ACL_ENTRY_INFO_PTR,
        ) -> CSSM_RETURN,
    >,
    pub GetKeyAcl: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            *const CSSM_KEY,
            *const CSSM_STRING,
            *mut uint32,
            *mut CSSM_ACL_ENTRY_INFO_PTR,
        ) -> CSSM_RETURN,
    >,
    pub ChangeKeyAcl: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            *const CSSM_ACCESS_CREDENTIALS,
            *const CSSM_ACL_EDIT,
            *const CSSM_KEY,
        ) -> CSSM_RETURN,
    >,
    pub GetKeyOwner: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            *const CSSM_KEY,
            CSSM_ACL_OWNER_PROTOTYPE_PTR,
        ) -> CSSM_RETURN,
    >,
    pub ChangeKeyOwner: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            *const CSSM_ACCESS_CREDENTIALS,
            *const CSSM_KEY,
            *const CSSM_ACL_OWNER_PROTOTYPE,
        ) -> CSSM_RETURN,
    >,
    pub GetLoginOwner: Option<
        unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE, CSSM_ACL_OWNER_PROTOTYPE_PTR) -> CSSM_RETURN,
    >,
    pub ChangeLoginOwner: Option<
        unsafe extern "C-unwind" fn(
            CSSM_CSP_HANDLE,
            *const CSSM_ACCESS_CREDENTIALS,
            *const CSSM_ACL_OWNER_PROTOTYPE,
        ) -> CSSM_RETURN,
    >,
}

#[cfg(all(
    feature = "SecAsn1Types",
    feature = "cssmconfig",
    feature = "cssmkrapi",
    feature = "cssmspi",
    feature = "cssmtype",
    feature = "objc2"
))]
unsafe impl Encode for cssm_spi_csp_funcs {
    const ENCODING: Encoding = Encoding::Struct("cssm_spi_csp_funcs", &[
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CONTEXT_EVENT,CSSM_CC_HANDLE,*const CSSM_CONTEXT,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const CSSM_CONTEXT,CSSM_BOOL,uint32,CSSM_QUERY_SIZE_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const CSSM_CONTEXT,*const SecAsn1Item,uint32,CSSM_ALGORITHMS,CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const CSSM_CONTEXT,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const SecAsn1Item,uint32,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const CSSM_CONTEXT,*const SecAsn1Item,uint32,CSSM_ALGORITHMS,*const SecAsn1Item,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const CSSM_CONTEXT,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const SecAsn1Item,uint32,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const SecAsn1Item,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const CSSM_CONTEXT,*const SecAsn1Item,uint32,CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const CSSM_CONTEXT,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const SecAsn1Item,uint32,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,CSSM_CC_HANDLE,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const CSSM_CONTEXT,*const SecAsn1Item,uint32,CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const CSSM_CONTEXT,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const SecAsn1Item,uint32,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const CSSM_CONTEXT,*const SecAsn1Item,uint32,*const SecAsn1Item,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const CSSM_CONTEXT,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const SecAsn1Item,uint32,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const SecAsn1Item,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const CSSM_CONTEXT,*const SecAsn1Item,uint32,CSSM_DATA_PTR,uint32,*mut CSSM_SIZE,CSSM_DATA_PTR,CSSM_PRIVILEGE,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const CSSM_CONTEXT,CSSM_PRIVILEGE,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const SecAsn1Item,uint32,CSSM_DATA_PTR,uint32,*mut CSSM_SIZE,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const CSSM_CONTEXT,*const SecAsn1Item,uint32,CSSM_DATA_PTR,uint32,*mut CSSM_SIZE,CSSM_DATA_PTR,CSSM_PRIVILEGE,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const CSSM_CONTEXT,CSSM_PRIVILEGE,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const SecAsn1Item,uint32,CSSM_DATA_PTR,uint32,*mut CSSM_SIZE,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const CSSM_CONTEXT,*const CSSM_KEY,CSSM_KEY_SIZE_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const CSSM_CONTEXT,uint32,uint32,*const SecAsn1Item,*const CSSM_RESOURCE_CONTROL_CONTEXT,CSSM_KEY_PTR,CSSM_PRIVILEGE,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const CSSM_CONTEXT,uint32,uint32,*const SecAsn1Item,CSSM_KEY_PTR,uint32,uint32,*const SecAsn1Item,*const CSSM_RESOURCE_CONTROL_CONTEXT,CSSM_KEY_PTR,CSSM_PRIVILEGE,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const CSSM_CONTEXT,CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const CSSM_CONTEXT,uint32,CSSM_DATA_PTR,*mut uint32,*mut CSSM_CONTEXT_ATTRIBUTE_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const CSSM_CONTEXT,*const CSSM_ACCESS_CREDENTIALS,*const CSSM_KEY,*const SecAsn1Item,CSSM_WRAP_KEY_PTR,CSSM_PRIVILEGE,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const CSSM_CONTEXT,*const CSSM_KEY,*const CSSM_WRAP_KEY,uint32,uint32,*const SecAsn1Item,*const CSSM_RESOURCE_CONTROL_CONTEXT,CSSM_KEY_PTR,CSSM_DATA_PTR,CSSM_PRIVILEGE,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const CSSM_CONTEXT,CSSM_DATA_PTR,uint32,uint32,*const SecAsn1Item,*const CSSM_RESOURCE_CONTROL_CONTEXT,CSSM_KEY_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,*const CSSM_ACCESS_CREDENTIALS,CSSM_KEY_PTR,CSSM_BOOL,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_CC_HANDLE,*const CSSM_CONTEXT,uint32,*const c_void,*mut *mut c_void,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,*const CSSM_ACCESS_CREDENTIALS,*const SecAsn1Item,*const c_void,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,*const CSSM_ACCESS_CREDENTIALS,*const CSSM_ACL_EDIT,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,*const CSSM_KEY,CSSM_KEY_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_DATA_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,*const SecAsn1Item,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_ALGORITHMS,*mut SecAsn1Item,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,*mut CSSM_CSP_OPERATIONAL_STATISTICS,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,*const CSSM_STRING,*mut uint32,*mut CSSM_ACL_ENTRY_INFO_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,*const CSSM_KEY,*const CSSM_STRING,*mut uint32,*mut CSSM_ACL_ENTRY_INFO_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,*const CSSM_ACCESS_CREDENTIALS,*const CSSM_ACL_EDIT,*const CSSM_KEY,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,*const CSSM_KEY,CSSM_ACL_OWNER_PROTOTYPE_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,*const CSSM_ACCESS_CREDENTIALS,*const CSSM_KEY,*const CSSM_ACL_OWNER_PROTOTYPE,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,CSSM_ACL_OWNER_PROTOTYPE_PTR,) -> CSSM_RETURN>>::ENCODING,
        <Option<unsafe extern "C-unwind" fn(CSSM_CSP_HANDLE,*const CSSM_ACCESS_CREDENTIALS,*const CSSM_ACL_OWNER_PROTOTYPE,) -> CSSM_RETURN>>::ENCODING,
    ]);
}

#[cfg(all(
    feature = "SecAsn1Types",
    feature = "cssmconfig",
    feature = "cssmkrapi",
    feature = "cssmspi",
    feature = "cssmtype",
    feature = "objc2"
))]
unsafe impl RefEncode for cssm_spi_csp_funcs {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/security/cssm_spi_csp_funcs?language=objc)
#[cfg(all(
    feature = "SecAsn1Types",
    feature = "cssmconfig",
    feature = "cssmkrapi",
    feature = "cssmspi",
    feature = "cssmtype"
))]
pub type CSSM_SPI_CSP_FUNCS = cssm_spi_csp_funcs;

/// [Apple's documentation](https://developer.apple.com/documentation/security/cssm_spi_csp_funcs_ptr?language=objc)
#[cfg(all(
    feature = "SecAsn1Types",
    feature = "cssmconfig",
    feature = "cssmkrapi",
    feature = "cssmspi",
    feature = "cssmtype"
))]
pub type CSSM_SPI_CSP_FUNCS_PTR = *mut cssm_spi_csp_funcs;