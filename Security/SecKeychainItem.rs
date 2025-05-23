//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
use objc2_core_foundation::*;

use crate::*;

/// Specifies a keychain item's class code.
///
///
///
/// Note: AppleShare passwords are no longer used by macOS, starting in Leopard (10.5). Use of this item class is deprecated in OS X 10.9 and later; kSecInternetPasswordItemClass should be used instead when storing or looking up passwords for an Apple Filing Protocol (AFP) server.
///
///
///
///
///
/// The SecItemClass enumeration defines constants your application can use to specify the type of the keychain item you wish to create, dispose, add, delete, update, copy, or locate. You can also use these constants with the tag constant SecItemAttr.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/security/secitemclass?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SecItemClass(pub FourCharCode);
impl SecItemClass {
    #[doc(alias = "kSecInternetPasswordItemClass")]
    pub const InternetPasswordItemClass: Self = Self(0x696e6574);
    #[doc(alias = "kSecGenericPasswordItemClass")]
    pub const GenericPasswordItemClass: Self = Self(0x67656e70);
    #[doc(alias = "kSecAppleSharePasswordItemClass")]
    #[deprecated]
    pub const AppleSharePasswordItemClass: Self = Self(0x61736870);
    #[doc(alias = "kSecCertificateItemClass")]
    pub const CertificateItemClass: Self = Self(0x80001000);
    #[doc(alias = "kSecPublicKeyItemClass")]
    pub const PublicKeyItemClass: Self = Self(0x0000000F);
    #[doc(alias = "kSecPrivateKeyItemClass")]
    pub const PrivateKeyItemClass: Self = Self(0x00000010);
    #[doc(alias = "kSecSymmetricKeyItemClass")]
    pub const SymmetricKeyItemClass: Self = Self(0x00000011);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for SecItemClass {
    const ENCODING: Encoding = FourCharCode::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for SecItemClass {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Specifies keychain item attributes.
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
/// To obtain information about a certificate, use the CDSA Certificate Library (CL) API. To obtain information about a key, use the SecKeyGetCSSMKey function and the CDSA Cryptographic Service Provider (CSP) API.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/security/secitemattr?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SecItemAttr(pub FourCharCode);
impl SecItemAttr {
    #[doc(alias = "kSecCreationDateItemAttr")]
    pub const CreationDateItemAttr: Self = Self(0x63646174);
    #[doc(alias = "kSecModDateItemAttr")]
    pub const ModDateItemAttr: Self = Self(0x6d646174);
    #[doc(alias = "kSecDescriptionItemAttr")]
    pub const DescriptionItemAttr: Self = Self(0x64657363);
    #[doc(alias = "kSecCommentItemAttr")]
    pub const CommentItemAttr: Self = Self(0x69636d74);
    #[doc(alias = "kSecCreatorItemAttr")]
    pub const CreatorItemAttr: Self = Self(0x63727472);
    #[doc(alias = "kSecTypeItemAttr")]
    pub const TypeItemAttr: Self = Self(0x74797065);
    #[doc(alias = "kSecScriptCodeItemAttr")]
    pub const ScriptCodeItemAttr: Self = Self(0x73637270);
    #[doc(alias = "kSecLabelItemAttr")]
    pub const LabelItemAttr: Self = Self(0x6c61626c);
    #[doc(alias = "kSecInvisibleItemAttr")]
    pub const InvisibleItemAttr: Self = Self(0x696e7669);
    #[doc(alias = "kSecNegativeItemAttr")]
    pub const NegativeItemAttr: Self = Self(0x6e656761);
    #[doc(alias = "kSecCustomIconItemAttr")]
    pub const CustomIconItemAttr: Self = Self(0x63757369);
    #[doc(alias = "kSecAccountItemAttr")]
    pub const AccountItemAttr: Self = Self(0x61636374);
    #[doc(alias = "kSecServiceItemAttr")]
    pub const ServiceItemAttr: Self = Self(0x73766365);
    #[doc(alias = "kSecGenericItemAttr")]
    pub const GenericItemAttr: Self = Self(0x67656e61);
    #[doc(alias = "kSecSecurityDomainItemAttr")]
    pub const SecurityDomainItemAttr: Self = Self(0x73646d6e);
    #[doc(alias = "kSecServerItemAttr")]
    pub const ServerItemAttr: Self = Self(0x73727672);
    #[doc(alias = "kSecAuthenticationTypeItemAttr")]
    pub const AuthenticationTypeItemAttr: Self = Self(0x61747970);
    #[doc(alias = "kSecPortItemAttr")]
    pub const PortItemAttr: Self = Self(0x706f7274);
    #[doc(alias = "kSecPathItemAttr")]
    pub const PathItemAttr: Self = Self(0x70617468);
    #[doc(alias = "kSecVolumeItemAttr")]
    pub const VolumeItemAttr: Self = Self(0x766c6d65);
    #[doc(alias = "kSecAddressItemAttr")]
    pub const AddressItemAttr: Self = Self(0x61646472);
    #[doc(alias = "kSecSignatureItemAttr")]
    pub const SignatureItemAttr: Self = Self(0x73736967);
    #[doc(alias = "kSecProtocolItemAttr")]
    pub const ProtocolItemAttr: Self = Self(0x7074636c);
    #[doc(alias = "kSecCertificateType")]
    pub const CertificateType: Self = Self(0x63747970);
    #[doc(alias = "kSecCertificateEncoding")]
    pub const CertificateEncoding: Self = Self(0x63656e63);
    #[doc(alias = "kSecCrlType")]
    pub const CrlType: Self = Self(0x63727470);
    #[doc(alias = "kSecCrlEncoding")]
    pub const CrlEncoding: Self = Self(0x63726e63);
    #[doc(alias = "kSecAlias")]
    pub const Alias: Self = Self(0x616c6973);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for SecItemAttr {
    const ENCODING: Encoding = FourCharCode::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for SecItemAttr {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(feature = "SecBase")]
unsafe impl ConcreteType for SecKeychainItem {
    /// Returns the type identifier of SecKeychainItem instances.
    ///
    /// Returns: The CFTypeID of SecKeychainItem instances.
    #[doc(alias = "SecKeychainItemGetTypeID")]
    #[inline]
    fn type_id() -> CFTypeID {
        extern "C-unwind" {
            fn SecKeychainItemGetTypeID() -> CFTypeID;
        }
        unsafe { SecKeychainItemGetTypeID() }
    }
}

#[cfg(feature = "SecBase")]
impl SecKeychainItem {
    /// Updates an existing keychain item after changing its attributes or data.
    ///
    /// Parameter `itemRef`: A reference to the keychain item to modify.
    ///
    /// Parameter `attrList`: The list of attributes to modify, along with their new values. Pass NULL if you don't need to modify any attributes.
    ///
    /// Parameter `length`: The length of the buffer pointed to by data.
    ///
    /// Parameter `data`: Pointer to a buffer containing the data to store. Pass NULL if you don't need to modify the data.
    ///
    /// Returns: A result code. See "Security Error Codes" (SecBase.h).
    ///
    /// The keychain item is written to the keychain's permanent data store. If the keychain item has not previously been added to a keychain, a call to the SecKeychainItemModifyContent function does nothing and returns errSecSuccess.
    #[doc(alias = "SecKeychainItemModifyAttributesAndData")]
    #[cfg(feature = "SecBase")]
    #[deprecated = "SecKeychain is deprecated"]
    #[inline]
    pub unsafe fn modify_attributes_and_data(
        self: &SecKeychainItem,
        attr_list: *const SecKeychainAttributeList,
        length: u32,
        data: *const c_void,
    ) -> OSStatus {
        extern "C-unwind" {
            fn SecKeychainItemModifyAttributesAndData(
                item_ref: &SecKeychainItem,
                attr_list: *const SecKeychainAttributeList,
                length: u32,
                data: *const c_void,
            ) -> OSStatus;
        }
        unsafe { SecKeychainItemModifyAttributesAndData(self, attr_list, length, data) }
    }

    /// Creates a new keychain item from the supplied parameters.
    ///
    /// Parameter `itemClass`: A constant identifying the class of item to create.
    ///
    /// Parameter `attrList`: The list of attributes of the item to create.
    ///
    /// Parameter `length`: The length of the buffer pointed to by data.
    ///
    /// Parameter `data`: A pointer to a buffer containing the data to store.
    ///
    /// Parameter `initialAccess`: A reference to the access for this keychain item.
    ///
    /// Parameter `keychainRef`: A reference to the keychain in which to add the item.
    ///
    /// Parameter `itemRef`: On return, a pointer to a reference to the newly created keychain item (optional). When the item reference is no longer required, call CFRelease to deallocate memory occupied by the item.
    ///
    /// Returns: A result code. See "Security Error Codes" (SecBase.h). In addition, errSecParam (-50) may be returned if not enough valid parameters are supplied, or errSecAllocate (-108) if there is not enough memory in the current heap zone to create the object.
    #[doc(alias = "SecKeychainItemCreateFromContent")]
    #[cfg(feature = "SecBase")]
    #[deprecated = "SecKeychain is deprecated"]
    #[inline]
    pub unsafe fn create_from_content(
        item_class: SecItemClass,
        attr_list: NonNull<SecKeychainAttributeList>,
        length: u32,
        data: *const c_void,
        keychain_ref: Option<&SecKeychain>,
        initial_access: Option<&SecAccess>,
        item_ref: *mut *mut SecKeychainItem,
    ) -> OSStatus {
        extern "C-unwind" {
            fn SecKeychainItemCreateFromContent(
                item_class: SecItemClass,
                attr_list: NonNull<SecKeychainAttributeList>,
                length: u32,
                data: *const c_void,
                keychain_ref: Option<&SecKeychain>,
                initial_access: Option<&SecAccess>,
                item_ref: *mut *mut SecKeychainItem,
            ) -> OSStatus;
        }
        unsafe {
            SecKeychainItemCreateFromContent(
                item_class,
                attr_list,
                length,
                data,
                keychain_ref,
                initial_access,
                item_ref,
            )
        }
    }

    /// Updates an existing keychain item after changing its attributes or data. This call should only be used in conjunction with SecKeychainItemCopyContent().
    ///
    /// Parameter `itemRef`: A reference to the keychain item to modify.
    ///
    /// Parameter `attrList`: The list of attributes to modify, along with their new values. Pass NULL if you don't need to modify any attributes.
    ///
    /// Parameter `length`: The length of the buffer pointed to by data.
    ///
    /// Parameter `data`: A pointer to a buffer containing the data to store. Pass NULL if you don't need to modify the data.
    ///
    /// Returns: A result code.  See "Security Error Codes" (SecBase.h).
    #[doc(alias = "SecKeychainItemModifyContent")]
    #[cfg(feature = "SecBase")]
    #[deprecated = "SecKeychain is deprecated"]
    #[inline]
    pub unsafe fn modify_content(
        self: &SecKeychainItem,
        attr_list: *const SecKeychainAttributeList,
        length: u32,
        data: *const c_void,
    ) -> OSStatus {
        extern "C-unwind" {
            fn SecKeychainItemModifyContent(
                item_ref: &SecKeychainItem,
                attr_list: *const SecKeychainAttributeList,
                length: u32,
                data: *const c_void,
            ) -> OSStatus;
        }
        unsafe { SecKeychainItemModifyContent(self, attr_list, length, data) }
    }

    /// Copies the data and/or attributes stored in the given keychain item. It is recommended that you use SecKeychainItemCopyAttributesAndData(). You must call SecKeychainItemFreeContent when you no longer need the attributes and data. If you want to modify the attributes returned here, use SecKeychainModifyContent().
    ///
    /// Parameter `itemRef`: A reference to the keychain item to modify.
    ///
    /// Parameter `itemClass`: On return, the item's class. Pass NULL if you don't require this information.
    ///
    /// Parameter `attrList`: On input, the list of attributes to retrieve. On output, the attributes are filled in. Pass NULL if you don't need to retrieve any attributes. You must call SecKeychainItemFreeContent when you no longer need the attributes.
    ///
    /// Parameter `length`: On return, the length of the buffer pointed to by outData.
    ///
    /// Parameter `outData`: On return, a pointer to a buffer containing the data in this item. Pass NULL if you don't need to retrieve the data. You must call SecKeychainItemFreeContent when you no longer need the data.
    ///
    /// Returns: A result code. See "Security Error Codes" (SecBase.h). In addition, errSecParam (-50) may be returned if not enough valid parameters are supplied.
    #[doc(alias = "SecKeychainItemCopyContent")]
    #[cfg(feature = "SecBase")]
    #[deprecated = "SecKeychain is deprecated"]
    #[inline]
    pub unsafe fn copy_content(
        self: &SecKeychainItem,
        item_class: *mut SecItemClass,
        attr_list: *mut SecKeychainAttributeList,
        length: *mut u32,
        out_data: *mut *mut c_void,
    ) -> OSStatus {
        extern "C-unwind" {
            fn SecKeychainItemCopyContent(
                item_ref: &SecKeychainItem,
                item_class: *mut SecItemClass,
                attr_list: *mut SecKeychainAttributeList,
                length: *mut u32,
                out_data: *mut *mut c_void,
            ) -> OSStatus;
        }
        unsafe { SecKeychainItemCopyContent(self, item_class, attr_list, length, out_data) }
    }

    /// Releases the memory used by the keychain attribute list and the keychain data retrieved in a previous call to SecKeychainItemCopyContent.
    ///
    /// Parameter `attrList`: A pointer to the attribute list to release. Pass NULL to ignore this parameter.
    ///
    /// Parameter `data`: A pointer to the data buffer to release. Pass NULL to ignore this parameter.
    #[doc(alias = "SecKeychainItemFreeContent")]
    #[cfg(feature = "SecBase")]
    #[deprecated = "SecKeychain is deprecated"]
    #[inline]
    pub unsafe fn free_content(
        attr_list: *mut SecKeychainAttributeList,
        data: *mut c_void,
    ) -> OSStatus {
        extern "C-unwind" {
            fn SecKeychainItemFreeContent(
                attr_list: *mut SecKeychainAttributeList,
                data: *mut c_void,
            ) -> OSStatus;
        }
        unsafe { SecKeychainItemFreeContent(attr_list, data) }
    }

    /// Copies the data and/or attributes stored in the given keychain item. You must call SecKeychainItemFreeAttributesAndData when you no longer need the attributes and data. If you want to modify the attributes returned here, use SecKeychainModifyAttributesAndData.
    ///
    /// Parameter `itemRef`: A reference to the keychain item to copy.
    ///
    /// Parameter `info`: A list of tags and formats of the attributes you wish to retrieve. Pass NULL if you don't need to retrieve any attributes. You can call SecKeychainAttributeInfoForItemID to obtain a list with all possible attribute tags and formats for the item's class.
    ///
    /// Parameter `itemClass`: On return, the item's class. Pass NULL if you don't require this information.
    ///
    /// Parameter `attrList`: On return, a pointer to the list of retrieved attributes. Pass NULL if you don't need to retrieve any attributes. You must call SecKeychainItemFreeAttributesAndData when you no longer need this list.
    ///
    /// Parameter `length`: On return, the length of the buffer pointed to by outData.
    ///
    /// Parameter `outData`: On return, a pointer to a buffer containing the data in this item. Pass NULL if you don't need to retrieve the data. You must call SecKeychainItemFreeAttributesAndData when you no longer need the data.
    ///
    /// Returns: A result code. See "Security Error Codes" (SecBase.h). In addition, errSecParam (-50) may be returned if not enough valid parameters are supplied.
    #[doc(alias = "SecKeychainItemCopyAttributesAndData")]
    #[cfg(feature = "SecBase")]
    #[deprecated = "SecKeychain is deprecated"]
    #[inline]
    pub unsafe fn copy_attributes_and_data(
        self: &SecKeychainItem,
        info: *mut SecKeychainAttributeInfo,
        item_class: *mut SecItemClass,
        attr_list: *mut *mut SecKeychainAttributeList,
        length: *mut u32,
        out_data: *mut *mut c_void,
    ) -> OSStatus {
        extern "C-unwind" {
            fn SecKeychainItemCopyAttributesAndData(
                item_ref: &SecKeychainItem,
                info: *mut SecKeychainAttributeInfo,
                item_class: *mut SecItemClass,
                attr_list: *mut *mut SecKeychainAttributeList,
                length: *mut u32,
                out_data: *mut *mut c_void,
            ) -> OSStatus;
        }
        unsafe {
            SecKeychainItemCopyAttributesAndData(
                self, info, item_class, attr_list, length, out_data,
            )
        }
    }

    /// Releases the memory used by the keychain attribute list and the keychain data retrieved in a previous call to SecKeychainItemCopyAttributesAndData.
    ///
    /// Parameter `attrList`: A pointer to the attribute list to release. Pass NULL to ignore this parameter.
    ///
    /// Parameter `data`: A pointer to the data buffer to release. Pass NULL to ignore this parameter.
    ///
    /// Returns: A result code. See "Security Error Codes" (SecBase.h).
    #[doc(alias = "SecKeychainItemFreeAttributesAndData")]
    #[cfg(feature = "SecBase")]
    #[deprecated = "SecKeychain is deprecated"]
    #[inline]
    pub unsafe fn free_attributes_and_data(
        attr_list: *mut SecKeychainAttributeList,
        data: *mut c_void,
    ) -> OSStatus {
        extern "C-unwind" {
            fn SecKeychainItemFreeAttributesAndData(
                attr_list: *mut SecKeychainAttributeList,
                data: *mut c_void,
            ) -> OSStatus;
        }
        unsafe { SecKeychainItemFreeAttributesAndData(attr_list, data) }
    }

    /// Deletes a keychain item from the default keychain's permanent data store.
    ///
    /// Parameter `itemRef`: A keychain item reference of the item to delete.
    ///
    /// Returns: A result code. See "Security Error Codes" (SecBase.h).
    ///
    /// If itemRef has not previously been added to the keychain, SecKeychainItemDelete does nothing and returns errSecSuccess. IMPORTANT: SecKeychainItemDelete does not dispose the memory occupied by the item reference itself; use the CFRelease function when you are completely finished with an item.
    #[doc(alias = "SecKeychainItemDelete")]
    #[cfg(feature = "SecBase")]
    #[deprecated = "SecKeychain is deprecated"]
    #[inline]
    pub unsafe fn delete(self: &SecKeychainItem) -> OSStatus {
        extern "C-unwind" {
            fn SecKeychainItemDelete(item_ref: &SecKeychainItem) -> OSStatus;
        }
        unsafe { SecKeychainItemDelete(self) }
    }

    /// Copies an existing keychain reference from a keychain item.
    ///
    /// Parameter `itemRef`: A keychain item reference.
    ///
    /// Parameter `keychainRef`: On return, the keychain reference for the specified item. Release this reference by calling the CFRelease function.
    ///
    /// Returns: A result code. See "Security Error Codes" (SecBase.h).
    #[doc(alias = "SecKeychainItemCopyKeychain")]
    #[cfg(feature = "SecBase")]
    #[deprecated = "SecKeychain is deprecated"]
    #[inline]
    pub unsafe fn copy_keychain(
        self: &SecKeychainItem,
        keychain_ref: NonNull<*mut SecKeychain>,
    ) -> OSStatus {
        extern "C-unwind" {
            fn SecKeychainItemCopyKeychain(
                item_ref: &SecKeychainItem,
                keychain_ref: NonNull<*mut SecKeychain>,
            ) -> OSStatus;
        }
        unsafe { SecKeychainItemCopyKeychain(self, keychain_ref) }
    }

    /// Copies a keychain item.
    ///
    /// Parameter `itemRef`: A reference to the keychain item to copy.
    ///
    /// Parameter `destKeychainRef`: A reference to the keychain in which to insert the copied keychain item.
    ///
    /// Parameter `initialAccess`: The initial access for the copied keychain item.
    ///
    /// Parameter `itemCopy`: On return, a reference to the copied keychain item.
    ///
    /// Returns: A result code. See "Security Error Codes" (SecBase.h).
    #[doc(alias = "SecKeychainItemCreateCopy")]
    #[cfg(feature = "SecBase")]
    #[deprecated = "SecKeychain is deprecated"]
    #[inline]
    pub unsafe fn create_copy(
        self: &SecKeychainItem,
        dest_keychain_ref: Option<&SecKeychain>,
        initial_access: Option<&SecAccess>,
        item_copy: NonNull<*mut SecKeychainItem>,
    ) -> OSStatus {
        extern "C-unwind" {
            fn SecKeychainItemCreateCopy(
                item_ref: &SecKeychainItem,
                dest_keychain_ref: Option<&SecKeychain>,
                initial_access: Option<&SecAccess>,
                item_copy: NonNull<*mut SecKeychainItem>,
            ) -> OSStatus;
        }
        unsafe { SecKeychainItemCreateCopy(self, dest_keychain_ref, initial_access, item_copy) }
    }

    /// Returns a CFDataRef which can be used as a persistent reference to the given keychain item. The data obtained can be turned back into a SecKeychainItemRef later by calling SecKeychainItemCopyFromPersistentReference().
    ///
    /// Parameter `itemRef`: A reference to a keychain item.
    ///
    /// Parameter `persistentItemRef`: On return, a CFDataRef containing a persistent reference. You must release this data reference by calling the CFRelease function.
    ///
    /// Returns: A result code. See "Security Error Codes" (SecBase.h).
    #[doc(alias = "SecKeychainItemCreatePersistentReference")]
    #[cfg(feature = "SecBase")]
    #[deprecated = "SecKeychain is deprecated"]
    #[inline]
    pub unsafe fn create_persistent_reference(
        self: &SecKeychainItem,
        persistent_item_ref: NonNull<*const CFData>,
    ) -> OSStatus {
        extern "C-unwind" {
            fn SecKeychainItemCreatePersistentReference(
                item_ref: &SecKeychainItem,
                persistent_item_ref: NonNull<*const CFData>,
            ) -> OSStatus;
        }
        unsafe { SecKeychainItemCreatePersistentReference(self, persistent_item_ref) }
    }

    /// Returns a SecKeychainItemRef, given a persistent reference previously obtained by calling SecKeychainItemCreatePersistentReference().
    ///
    /// Parameter `persistentItemRef`: A CFDataRef containing a persistent reference to a keychain item.
    ///
    /// Parameter `itemRef`: On return, a SecKeychainItemRef for the keychain item described by the persistent reference. You must release this item reference by calling the CFRelease function.
    ///
    /// Returns: A result code. See "Security Error Codes" (SecBase.h).
    #[doc(alias = "SecKeychainItemCopyFromPersistentReference")]
    #[cfg(feature = "SecBase")]
    #[deprecated = "SecKeychain is deprecated"]
    #[inline]
    pub unsafe fn copy_from_persistent_reference(
        persistent_item_ref: &CFData,
        item_ref: NonNull<*mut SecKeychainItem>,
    ) -> OSStatus {
        extern "C-unwind" {
            fn SecKeychainItemCopyFromPersistentReference(
                persistent_item_ref: &CFData,
                item_ref: NonNull<*mut SecKeychainItem>,
            ) -> OSStatus;
        }
        unsafe { SecKeychainItemCopyFromPersistentReference(persistent_item_ref, item_ref) }
    }

    /// Returns the CSSM_DL_DB_HANDLE for a given keychain item reference.
    ///
    /// Parameter `keyItemRef`: A keychain item reference.
    ///
    /// Parameter `dldbHandle`: On return, a CSSM_DL_DB_HANDLE for the keychain database containing the given item. The handle is valid until the keychain reference is released.
    ///
    /// Returns: A result code. See "Security Error Codes" (SecBase.h).
    ///
    /// This API is deprecated for 10.7. It should no longer be needed.
    #[doc(alias = "SecKeychainItemGetDLDBHandle")]
    #[cfg(all(feature = "SecBase", feature = "cssmconfig", feature = "cssmtype"))]
    #[deprecated = "CSSM is not supported"]
    #[inline]
    pub unsafe fn dldb_handle(
        self: &SecKeychainItem,
        dldb_handle: NonNull<CSSM_DL_DB_HANDLE>,
    ) -> OSStatus {
        extern "C-unwind" {
            fn SecKeychainItemGetDLDBHandle(
                key_item_ref: &SecKeychainItem,
                dldb_handle: NonNull<CSSM_DL_DB_HANDLE>,
            ) -> OSStatus;
        }
        unsafe { SecKeychainItemGetDLDBHandle(self, dldb_handle) }
    }

    /// Returns a CSSM_DB_UNIQUE_RECORD for the given keychain item reference.
    ///
    /// Parameter `itemRef`: A keychain item reference.
    ///
    /// Parameter `uniqueRecordID`: On return, a pointer to a CSSM_DB_UNIQUE_RECORD structure for the given item. The unique record is valid until the item reference is released.
    ///
    /// Returns: A result code. See "Security Error Codes" (SecBase.h).
    ///
    /// This API is deprecated for 10.7. It should no longer be needed.
    #[doc(alias = "SecKeychainItemGetUniqueRecordID")]
    #[cfg(all(
        feature = "SecAsn1Types",
        feature = "SecBase",
        feature = "cssmconfig",
        feature = "cssmtype"
    ))]
    #[deprecated = "CSSM is not supported"]
    #[inline]
    pub unsafe fn unique_record_id(
        self: &SecKeychainItem,
        unique_record_id: NonNull<*const CSSM_DB_UNIQUE_RECORD>,
    ) -> OSStatus {
        extern "C-unwind" {
            fn SecKeychainItemGetUniqueRecordID(
                item_ref: &SecKeychainItem,
                unique_record_id: NonNull<*const CSSM_DB_UNIQUE_RECORD>,
            ) -> OSStatus;
        }
        unsafe { SecKeychainItemGetUniqueRecordID(self, unique_record_id) }
    }

    /// Copies the access of a given keychain item.
    ///
    /// Parameter `itemRef`: A reference to a keychain item.
    ///
    /// Parameter `access`: On return, a reference to the keychain item's access.
    ///
    /// Returns: A result code. See "Security Error Codes" (SecBase.h).
    #[doc(alias = "SecKeychainItemCopyAccess")]
    #[cfg(feature = "SecBase")]
    #[deprecated = "SecKeychain is deprecated"]
    #[inline]
    pub unsafe fn copy_access(self: &SecKeychainItem, access: NonNull<*mut SecAccess>) -> OSStatus {
        extern "C-unwind" {
            fn SecKeychainItemCopyAccess(
                item_ref: &SecKeychainItem,
                access: NonNull<*mut SecAccess>,
            ) -> OSStatus;
        }
        unsafe { SecKeychainItemCopyAccess(self, access) }
    }

    /// Sets the access of a given keychain item.
    ///
    /// Parameter `itemRef`: A reference to a keychain item.
    ///
    /// Parameter `access`: A reference to an access to replace the keychain item's current access.
    ///
    /// Returns: A result code. See "Security Error Codes" (SecBase.h).
    #[doc(alias = "SecKeychainItemSetAccess")]
    #[cfg(feature = "SecBase")]
    #[deprecated = "SecKeychain is deprecated"]
    #[inline]
    pub unsafe fn set_access(self: &SecKeychainItem, access: &SecAccess) -> OSStatus {
        extern "C-unwind" {
            fn SecKeychainItemSetAccess(item_ref: &SecKeychainItem, access: &SecAccess)
                -> OSStatus;
        }
        unsafe { SecKeychainItemSetAccess(self, access) }
    }
}

extern "C-unwind" {
    #[cfg(feature = "SecBase")]
    #[deprecated = "renamed to `SecKeychainItem::modify_attributes_and_data`"]
    pub fn SecKeychainItemModifyAttributesAndData(
        item_ref: &SecKeychainItem,
        attr_list: *const SecKeychainAttributeList,
        length: u32,
        data: *const c_void,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "SecBase")]
    #[deprecated = "renamed to `SecKeychainItem::create_from_content`"]
    pub fn SecKeychainItemCreateFromContent(
        item_class: SecItemClass,
        attr_list: NonNull<SecKeychainAttributeList>,
        length: u32,
        data: *const c_void,
        keychain_ref: Option<&SecKeychain>,
        initial_access: Option<&SecAccess>,
        item_ref: *mut *mut SecKeychainItem,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "SecBase")]
    #[deprecated = "renamed to `SecKeychainItem::modify_content`"]
    pub fn SecKeychainItemModifyContent(
        item_ref: &SecKeychainItem,
        attr_list: *const SecKeychainAttributeList,
        length: u32,
        data: *const c_void,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "SecBase")]
    #[deprecated = "renamed to `SecKeychainItem::copy_content`"]
    pub fn SecKeychainItemCopyContent(
        item_ref: &SecKeychainItem,
        item_class: *mut SecItemClass,
        attr_list: *mut SecKeychainAttributeList,
        length: *mut u32,
        out_data: *mut *mut c_void,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "SecBase")]
    #[deprecated = "renamed to `SecKeychainItem::free_content`"]
    pub fn SecKeychainItemFreeContent(
        attr_list: *mut SecKeychainAttributeList,
        data: *mut c_void,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "SecBase")]
    #[deprecated = "renamed to `SecKeychainItem::copy_attributes_and_data`"]
    pub fn SecKeychainItemCopyAttributesAndData(
        item_ref: &SecKeychainItem,
        info: *mut SecKeychainAttributeInfo,
        item_class: *mut SecItemClass,
        attr_list: *mut *mut SecKeychainAttributeList,
        length: *mut u32,
        out_data: *mut *mut c_void,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "SecBase")]
    #[deprecated = "renamed to `SecKeychainItem::free_attributes_and_data`"]
    pub fn SecKeychainItemFreeAttributesAndData(
        attr_list: *mut SecKeychainAttributeList,
        data: *mut c_void,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "SecBase")]
    #[deprecated = "renamed to `SecKeychainItem::delete`"]
    pub fn SecKeychainItemDelete(item_ref: &SecKeychainItem) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "SecBase")]
    #[deprecated = "renamed to `SecKeychainItem::copy_keychain`"]
    pub fn SecKeychainItemCopyKeychain(
        item_ref: &SecKeychainItem,
        keychain_ref: NonNull<*mut SecKeychain>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "SecBase")]
    #[deprecated = "renamed to `SecKeychainItem::create_copy`"]
    pub fn SecKeychainItemCreateCopy(
        item_ref: &SecKeychainItem,
        dest_keychain_ref: Option<&SecKeychain>,
        initial_access: Option<&SecAccess>,
        item_copy: NonNull<*mut SecKeychainItem>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "SecBase")]
    #[deprecated = "renamed to `SecKeychainItem::create_persistent_reference`"]
    pub fn SecKeychainItemCreatePersistentReference(
        item_ref: &SecKeychainItem,
        persistent_item_ref: NonNull<*const CFData>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "SecBase")]
    #[deprecated = "renamed to `SecKeychainItem::copy_from_persistent_reference`"]
    pub fn SecKeychainItemCopyFromPersistentReference(
        persistent_item_ref: &CFData,
        item_ref: NonNull<*mut SecKeychainItem>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(all(feature = "SecBase", feature = "cssmconfig", feature = "cssmtype"))]
    #[deprecated = "renamed to `SecKeychainItem::dldb_handle`"]
    pub fn SecKeychainItemGetDLDBHandle(
        key_item_ref: &SecKeychainItem,
        dldb_handle: NonNull<CSSM_DL_DB_HANDLE>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "SecAsn1Types",
        feature = "SecBase",
        feature = "cssmconfig",
        feature = "cssmtype"
    ))]
    #[deprecated = "renamed to `SecKeychainItem::unique_record_id`"]
    pub fn SecKeychainItemGetUniqueRecordID(
        item_ref: &SecKeychainItem,
        unique_record_id: NonNull<*const CSSM_DB_UNIQUE_RECORD>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "SecBase")]
    #[deprecated = "renamed to `SecKeychainItem::copy_access`"]
    pub fn SecKeychainItemCopyAccess(
        item_ref: &SecKeychainItem,
        access: NonNull<*mut SecAccess>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "SecBase")]
    #[deprecated = "renamed to `SecKeychainItem::set_access`"]
    pub fn SecKeychainItemSetAccess(item_ref: &SecKeychainItem, access: &SecAccess) -> OSStatus;
}
