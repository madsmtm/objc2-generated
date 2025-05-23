//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/security/sec_key_import_export_params_version?language=objc)
pub const SEC_KEY_IMPORT_EXPORT_PARAMS_VERSION: c_uint = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/security/secexternalformat?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SecExternalFormat(pub u32);
impl SecExternalFormat {
    #[doc(alias = "kSecFormatUnknown")]
    pub const FormatUnknown: Self = Self(0);
    #[doc(alias = "kSecFormatOpenSSL")]
    pub const FormatOpenSSL: Self = Self(1);
    #[doc(alias = "kSecFormatSSH")]
    pub const FormatSSH: Self = Self(2);
    #[doc(alias = "kSecFormatBSAFE")]
    pub const FormatBSAFE: Self = Self(3);
    #[doc(alias = "kSecFormatRawKey")]
    pub const FormatRawKey: Self = Self(4);
    #[doc(alias = "kSecFormatWrappedPKCS8")]
    pub const FormatWrappedPKCS8: Self = Self(5);
    #[doc(alias = "kSecFormatWrappedOpenSSL")]
    pub const FormatWrappedOpenSSL: Self = Self(6);
    #[doc(alias = "kSecFormatWrappedSSH")]
    pub const FormatWrappedSSH: Self = Self(7);
    #[doc(alias = "kSecFormatWrappedLSH")]
    pub const FormatWrappedLSH: Self = Self(8);
    #[doc(alias = "kSecFormatX509Cert")]
    pub const FormatX509Cert: Self = Self(9);
    #[doc(alias = "kSecFormatPEMSequence")]
    pub const FormatPEMSequence: Self = Self(10);
    #[doc(alias = "kSecFormatPKCS7")]
    pub const FormatPKCS7: Self = Self(11);
    #[doc(alias = "kSecFormatPKCS12")]
    pub const FormatPKCS12: Self = Self(12);
    #[doc(alias = "kSecFormatNetscapeCertSequence")]
    pub const FormatNetscapeCertSequence: Self = Self(13);
    #[doc(alias = "kSecFormatSSHv2")]
    pub const FormatSSHv2: Self = Self(14);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for SecExternalFormat {
    const ENCODING: Encoding = u32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for SecExternalFormat {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/security/secexternalitemtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SecExternalItemType(pub u32);
impl SecExternalItemType {
    #[doc(alias = "kSecItemTypeUnknown")]
    pub const ItemTypeUnknown: Self = Self(0);
    #[doc(alias = "kSecItemTypePrivateKey")]
    pub const ItemTypePrivateKey: Self = Self(1);
    #[doc(alias = "kSecItemTypePublicKey")]
    pub const ItemTypePublicKey: Self = Self(2);
    #[doc(alias = "kSecItemTypeSessionKey")]
    pub const ItemTypeSessionKey: Self = Self(3);
    #[doc(alias = "kSecItemTypeCertificate")]
    pub const ItemTypeCertificate: Self = Self(4);
    #[doc(alias = "kSecItemTypeAggregate")]
    pub const ItemTypeAggregate: Self = Self(5);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for SecExternalItemType {
    const ENCODING: Encoding = u32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for SecExternalItemType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/security/secitemimportexportflags?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SecItemImportExportFlags(pub u32);
bitflags::bitflags! {
    impl SecItemImportExportFlags: u32 {
        #[doc(alias = "kSecItemPemArmour")]
        const PemArmour = 0x00000001;
    }
}

#[cfg(feature = "objc2")]
unsafe impl Encode for SecItemImportExportFlags {
    const ENCODING: Encoding = u32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for SecItemImportExportFlags {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/security/seckeyimportexportflags?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SecKeyImportExportFlags(pub u32);
bitflags::bitflags! {
    impl SecKeyImportExportFlags: u32 {
        #[doc(alias = "kSecKeyImportOnlyOne")]
        const ImportOnlyOne = 0x00000001;
        #[doc(alias = "kSecKeySecurePassphrase")]
        const SecurePassphrase = 0x00000002;
        #[doc(alias = "kSecKeyNoAccessControl")]
        const NoAccessControl = 0x00000004;
    }
}

#[cfg(feature = "objc2")]
unsafe impl Encode for SecKeyImportExportFlags {
    const ENCODING: Encoding = u32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for SecKeyImportExportFlags {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/security/seckeyimportexportparameters?language=objc)
#[cfg(all(feature = "SecBase", feature = "cssmconfig", feature = "cssmtype"))]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecKeyImportExportParameters {
    pub version: u32,
    pub flags: SecKeyImportExportFlags,
    pub passphrase: *const CFType,
    pub alertTitle: NonNull<CFString>,
    pub alertPrompt: NonNull<CFString>,
    pub accessRef: *mut SecAccess,
    pub keyUsage: CSSM_KEYUSE,
    pub keyAttributes: CSSM_KEYATTR_FLAGS,
}

#[cfg(all(
    feature = "SecBase",
    feature = "cssmconfig",
    feature = "cssmtype",
    feature = "objc2"
))]
unsafe impl Encode for SecKeyImportExportParameters {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <u32>::ENCODING,
            <SecKeyImportExportFlags>::ENCODING,
            <*const CFType>::ENCODING,
            <NonNull<CFString>>::ENCODING,
            <NonNull<CFString>>::ENCODING,
            <*mut SecAccess>::ENCODING,
            <CSSM_KEYUSE>::ENCODING,
            <CSSM_KEYATTR_FLAGS>::ENCODING,
        ],
    );
}

#[cfg(all(
    feature = "SecBase",
    feature = "cssmconfig",
    feature = "cssmtype",
    feature = "objc2"
))]
unsafe impl RefEncode for SecKeyImportExportParameters {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/security/secitemimportexportkeyparameters?language=objc)
#[cfg(feature = "SecBase")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecItemImportExportKeyParameters {
    pub version: u32,
    pub flags: SecKeyImportExportFlags,
    pub passphrase: *const CFType,
    pub alertTitle: *const CFString,
    pub alertPrompt: *const CFString,
    pub accessRef: *mut SecAccess,
    pub keyUsage: *const CFArray,
    pub keyAttributes: *const CFArray,
}

#[cfg(all(feature = "SecBase", feature = "objc2"))]
unsafe impl Encode for SecItemImportExportKeyParameters {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <u32>::ENCODING,
            <SecKeyImportExportFlags>::ENCODING,
            <*const CFType>::ENCODING,
            <*const CFString>::ENCODING,
            <*const CFString>::ENCODING,
            <*mut SecAccess>::ENCODING,
            <*const CFArray>::ENCODING,
            <*const CFArray>::ENCODING,
        ],
    );
}

#[cfg(all(feature = "SecBase", feature = "objc2"))]
unsafe impl RefEncode for SecItemImportExportKeyParameters {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(feature = "SecBase")]
impl SecKeychainItem {
    #[doc(alias = "SecKeychainItemExport")]
    #[cfg(all(feature = "SecBase", feature = "cssmconfig", feature = "cssmtype"))]
    #[deprecated]
    #[inline]
    pub unsafe fn export(
        keychain_item_or_array: &CFType,
        output_format: SecExternalFormat,
        flags: SecItemImportExportFlags,
        key_params: *const SecKeyImportExportParameters,
        exported_data: NonNull<*const CFData>,
    ) -> OSStatus {
        extern "C-unwind" {
            fn SecKeychainItemExport(
                keychain_item_or_array: &CFType,
                output_format: SecExternalFormat,
                flags: SecItemImportExportFlags,
                key_params: *const SecKeyImportExportParameters,
                exported_data: NonNull<*const CFData>,
            ) -> OSStatus;
        }
        unsafe {
            SecKeychainItemExport(
                keychain_item_or_array,
                output_format,
                flags,
                key_params,
                exported_data,
            )
        }
    }
}

extern "C-unwind" {
    #[cfg(feature = "SecBase")]
    pub fn SecItemExport(
        sec_item_or_array: &CFType,
        output_format: SecExternalFormat,
        flags: SecItemImportExportFlags,
        key_params: *const SecItemImportExportKeyParameters,
        exported_data: NonNull<*const CFData>,
    ) -> OSStatus;
}

#[cfg(feature = "SecBase")]
impl SecKeychainItem {
    #[doc(alias = "SecKeychainItemImport")]
    #[cfg(all(feature = "SecBase", feature = "cssmconfig", feature = "cssmtype"))]
    #[deprecated]
    #[inline]
    pub unsafe fn import(
        imported_data: &CFData,
        file_name_or_extension: Option<&CFString>,
        input_format: *mut SecExternalFormat,
        item_type: *mut SecExternalItemType,
        flags: SecItemImportExportFlags,
        key_params: *const SecKeyImportExportParameters,
        import_keychain: Option<&SecKeychain>,
        out_items: *mut *const CFArray,
    ) -> OSStatus {
        extern "C-unwind" {
            fn SecKeychainItemImport(
                imported_data: &CFData,
                file_name_or_extension: Option<&CFString>,
                input_format: *mut SecExternalFormat,
                item_type: *mut SecExternalItemType,
                flags: SecItemImportExportFlags,
                key_params: *const SecKeyImportExportParameters,
                import_keychain: Option<&SecKeychain>,
                out_items: *mut *const CFArray,
            ) -> OSStatus;
        }
        unsafe {
            SecKeychainItemImport(
                imported_data,
                file_name_or_extension,
                input_format,
                item_type,
                flags,
                key_params,
                import_keychain,
                out_items,
            )
        }
    }
}

extern "C-unwind" {
    #[cfg(feature = "SecBase")]
    pub fn SecItemImport(
        imported_data: &CFData,
        file_name_or_extension: Option<&CFString>,
        input_format: *mut SecExternalFormat,
        item_type: *mut SecExternalItemType,
        flags: SecItemImportExportFlags,
        key_params: *const SecItemImportExportKeyParameters,
        import_keychain: Option<&SecKeychain>,
        out_items: *mut *const CFArray,
    ) -> OSStatus;
}

extern "C" {
    /// Predefined key constants used when passing dictionary-based arguments to import/export functions.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/security/ksecimportexportpassphrase?language=objc)
    pub static kSecImportExportPassphrase: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/security/ksecimportexportkeychain?language=objc)
    pub static kSecImportExportKeychain: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/security/ksecimportexportaccess?language=objc)
    pub static kSecImportExportAccess: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/security/ksecimporttomemoryonly?language=objc)
    pub static kSecImportToMemoryOnly: &'static CFString;
}

extern "C" {
    /// Predefined key constants used to pass back a CFArray with a
    /// CFDictionary per item.
    ///
    ///
    /// This implementation specific identifier cannot be expected to have
    /// any format.
    ///
    /// the SHA-1 digest of the public key.
    ///
    ///
    /// certificates.  Not guaranteed to successfully evaluate.
    ///
    /// certificates for this item's identity
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/security/ksecimportitemlabel?language=objc)
    pub static kSecImportItemLabel: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/security/ksecimportitemkeyid?language=objc)
    pub static kSecImportItemKeyID: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/security/ksecimportitemtrust?language=objc)
    pub static kSecImportItemTrust: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/security/ksecimportitemcertchain?language=objc)
    pub static kSecImportItemCertChain: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/security/ksecimportitemidentity?language=objc)
    pub static kSecImportItemIdentity: &'static CFString;
}

extern "C-unwind" {
    /// Imports the contents of a PKCS12 formatted blob.
    ///
    /// Parameter `pkcs12_data`: The PKCS#12 formatted data to be imported.
    ///
    /// Parameter `options`: A dictionary containing import options. A
    /// kSecImportExportPassphrase entry is required at minimum. Only password-based
    /// PKCS12 blobs are currently supported.
    ///
    /// Parameter `items`: On return, an array containing a dictionary for every item
    /// extracted. Use kSecImportItem constants to access specific elements of
    /// these dictionaries. Your code must CFRelease the array when it is no longer
    /// needed.
    ///
    /// Returns: errSecSuccess in case of success. errSecDecode means either the
    /// blob can't be read or it is malformed. errSecAuthFailed means an
    /// incorrect password was supplied, or data in the container is damaged.
    ///
    /// The normal behavior of this function is to import items into process
    /// memory on iOS, and into the default keychain on macOS. You can modify this behavior
    /// with entries in the options dictionary. To specify a file-based keychain and
    /// legacy access control on macOS, provide kSecImportExportKeychain with a SecKeychainRef
    /// value, and/or kSecImportExportAccess with a SecAccessRef value. In macOS 14 and later,
    /// it is possible to specify the data protection keychain instead of a file-based keychain
    /// by including kSecUseDataProtectionKeychain with a value of kCFBooleanTrue. Starting with
    /// macOS 15 and iOS 18, kSecImportToMemoryOnly (with a value of kCFBooleanTrue) allows you
    /// to skip importing to the keychain on macOS and explicitly specify iOS behavior.
    pub fn SecPKCS12Import(
        pkcs12_data: &CFData,
        options: &CFDictionary,
        items: NonNull<*const CFArray>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(all(feature = "SecBase", feature = "cssmconfig", feature = "cssmtype"))]
    #[deprecated = "renamed to `SecKeychainItem::export`"]
    pub fn SecKeychainItemExport(
        keychain_item_or_array: &CFType,
        output_format: SecExternalFormat,
        flags: SecItemImportExportFlags,
        key_params: *const SecKeyImportExportParameters,
        exported_data: NonNull<*const CFData>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(all(feature = "SecBase", feature = "cssmconfig", feature = "cssmtype"))]
    #[deprecated = "renamed to `SecKeychainItem::import`"]
    pub fn SecKeychainItemImport(
        imported_data: &CFData,
        file_name_or_extension: Option<&CFString>,
        input_format: *mut SecExternalFormat,
        item_type: *mut SecExternalItemType,
        flags: SecItemImportExportFlags,
        key_params: *const SecKeyImportExportParameters,
        import_keychain: Option<&SecKeychain>,
        out_items: *mut *const CFArray,
    ) -> OSStatus;
}
