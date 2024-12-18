//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn ODRecordGetTypeID() -> CFTypeID;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFOpenDirectory", feature = "objc2-core-foundation"))]
    pub fn ODRecordSetNodeCredentials(
        record: ODRecordRef,
        username: CFStringRef,
        password: CFStringRef,
        error: *mut CFErrorRef,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CFOpenDirectory",
        feature = "CFOpenDirectoryConstants",
        feature = "objc2-core-foundation"
    ))]
    pub fn ODRecordSetNodeCredentialsExtended(
        record: ODRecordRef,
        record_type: Option<&ODRecordType>,
        auth_type: Option<&ODAuthenticationType>,
        auth_items: CFArrayRef,
        out_auth_items: *mut CFArrayRef,
        out_context: *mut ODContextRef,
        error: *mut CFErrorRef,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFOpenDirectory", feature = "objc2-core-foundation"))]
    #[deprecated]
    pub fn ODRecordSetNodeCredentialsUsingKerberosCache(
        record: ODRecordRef,
        cache_name: CFStringRef,
        error: *mut CFErrorRef,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFOpenDirectory", feature = "objc2-core-foundation"))]
    #[deprecated = "use ODRecordCopyEffectivePolicies"]
    pub fn ODRecordCopyPasswordPolicy(
        allocator: CFAllocatorRef,
        record: ODRecordRef,
        error: *mut CFErrorRef,
    ) -> CFDictionaryRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFOpenDirectory", feature = "objc2-core-foundation"))]
    pub fn ODRecordVerifyPassword(
        record: ODRecordRef,
        password: CFStringRef,
        error: *mut CFErrorRef,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CFOpenDirectory",
        feature = "CFOpenDirectoryConstants",
        feature = "objc2-core-foundation"
    ))]
    pub fn ODRecordVerifyPasswordExtended(
        record: ODRecordRef,
        auth_type: Option<&ODAuthenticationType>,
        auth_items: CFArrayRef,
        out_auth_items: *mut CFArrayRef,
        out_context: *mut ODContextRef,
        error: *mut CFErrorRef,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFOpenDirectory", feature = "objc2-core-foundation"))]
    pub fn ODRecordChangePassword(
        record: ODRecordRef,
        old_password: CFStringRef,
        new_password: CFStringRef,
        error: *mut CFErrorRef,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFOpenDirectory", feature = "objc2-core-foundation"))]
    pub fn ODRecordGetRecordType(record: ODRecordRef) -> CFStringRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFOpenDirectory", feature = "objc2-core-foundation"))]
    pub fn ODRecordGetRecordName(record: ODRecordRef) -> CFStringRef;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CFOpenDirectory",
        feature = "CFOpenDirectoryConstants",
        feature = "objc2-core-foundation"
    ))]
    pub fn ODRecordCopyValues(
        record: ODRecordRef,
        attribute: Option<&ODAttributeType>,
        error: *mut CFErrorRef,
    ) -> CFArrayRef;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CFOpenDirectory",
        feature = "CFOpenDirectoryConstants",
        feature = "objc2-core-foundation"
    ))]
    pub fn ODRecordSetValue(
        record: ODRecordRef,
        attribute: Option<&ODAttributeType>,
        value_or_values: CFTypeRef,
        error: *mut CFErrorRef,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CFOpenDirectory",
        feature = "CFOpenDirectoryConstants",
        feature = "objc2-core-foundation"
    ))]
    pub fn ODRecordAddValue(
        record: ODRecordRef,
        attribute: Option<&ODAttributeType>,
        value: CFTypeRef,
        error: *mut CFErrorRef,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CFOpenDirectory",
        feature = "CFOpenDirectoryConstants",
        feature = "objc2-core-foundation"
    ))]
    pub fn ODRecordRemoveValue(
        record: ODRecordRef,
        attribute: Option<&ODAttributeType>,
        value: CFTypeRef,
        error: *mut CFErrorRef,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFOpenDirectory", feature = "objc2-core-foundation"))]
    pub fn ODRecordCopyDetails(
        record: ODRecordRef,
        attributes: CFArrayRef,
        error: *mut CFErrorRef,
    ) -> CFDictionaryRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFOpenDirectory", feature = "objc2-core-foundation"))]
    pub fn ODRecordSynchronize(record: ODRecordRef, error: *mut CFErrorRef) -> bool;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFOpenDirectory", feature = "objc2-core-foundation"))]
    pub fn ODRecordDelete(record: ODRecordRef, error: *mut CFErrorRef) -> bool;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFOpenDirectory", feature = "objc2-core-foundation"))]
    pub fn ODRecordAddMember(
        group: ODRecordRef,
        member: ODRecordRef,
        error: *mut CFErrorRef,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFOpenDirectory", feature = "objc2-core-foundation"))]
    pub fn ODRecordRemoveMember(
        group: ODRecordRef,
        member: ODRecordRef,
        error: *mut CFErrorRef,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFOpenDirectory", feature = "objc2-core-foundation"))]
    pub fn ODRecordContainsMember(
        group: ODRecordRef,
        member: ODRecordRef,
        error: *mut CFErrorRef,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFOpenDirectory", feature = "objc2-core-foundation"))]
    #[deprecated = "use ODRecordCopyAccountPolicies"]
    pub fn ODRecordCopyPolicies(record: ODRecordRef, error: *mut CFErrorRef) -> CFDictionaryRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFOpenDirectory", feature = "objc2-core-foundation"))]
    #[deprecated = "use ODRecordAuthenticationAllowed and similar functions"]
    pub fn ODRecordCopyEffectivePolicies(
        record: ODRecordRef,
        error: *mut CFErrorRef,
    ) -> CFDictionaryRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFOpenDirectory", feature = "objc2-core-foundation"))]
    #[deprecated]
    pub fn ODRecordCopySupportedPolicies(
        record: ODRecordRef,
        error: *mut CFErrorRef,
    ) -> CFDictionaryRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFOpenDirectory", feature = "objc2-core-foundation"))]
    #[deprecated = "use ODRecordSetAccountPolicies"]
    pub fn ODRecordSetPolicies(
        record: ODRecordRef,
        policies: CFDictionaryRef,
        error: *mut CFErrorRef,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CFOpenDirectory",
        feature = "CFOpenDirectoryConstants",
        feature = "objc2-core-foundation"
    ))]
    #[deprecated = "use ODRecordAddAccountPolicy"]
    pub fn ODRecordSetPolicy(
        record: ODRecordRef,
        policy: Option<&ODPolicyType>,
        value: CFTypeRef,
        error: *mut CFErrorRef,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CFOpenDirectory",
        feature = "CFOpenDirectoryConstants",
        feature = "objc2-core-foundation"
    ))]
    #[deprecated = "use ODRecordRemoveAccountPolicy"]
    pub fn ODRecordRemovePolicy(
        record: ODRecordRef,
        policy: Option<&ODPolicyType>,
        error: *mut CFErrorRef,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CFOpenDirectory",
        feature = "CFOpenDirectoryConstants",
        feature = "objc2-core-foundation"
    ))]
    pub fn ODRecordAddAccountPolicy(
        record: ODRecordRef,
        policy: CFDictionaryRef,
        category: Option<&ODPolicyCategoryType>,
        error: *mut CFErrorRef,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "CFOpenDirectory",
        feature = "CFOpenDirectoryConstants",
        feature = "objc2-core-foundation"
    ))]
    pub fn ODRecordRemoveAccountPolicy(
        record: ODRecordRef,
        policy: CFDictionaryRef,
        category: Option<&ODPolicyCategoryType>,
        error: *mut CFErrorRef,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFOpenDirectory", feature = "objc2-core-foundation"))]
    pub fn ODRecordSetAccountPolicies(
        record: ODRecordRef,
        policies: CFDictionaryRef,
        error: *mut CFErrorRef,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFOpenDirectory", feature = "objc2-core-foundation"))]
    pub fn ODRecordCopyAccountPolicies(
        record: ODRecordRef,
        error: *mut CFErrorRef,
    ) -> CFDictionaryRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFOpenDirectory", feature = "objc2-core-foundation"))]
    pub fn ODRecordAuthenticationAllowed(record: ODRecordRef, error: *mut CFErrorRef) -> bool;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFOpenDirectory", feature = "objc2-core-foundation"))]
    pub fn ODRecordPasswordChangeAllowed(
        record: ODRecordRef,
        new_password: CFStringRef,
        error: *mut CFErrorRef,
    ) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "CFOpenDirectory")]
    pub fn ODRecordWillPasswordExpire(record: ODRecordRef, will_expire_in: u64) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "CFOpenDirectory")]
    pub fn ODRecordWillAuthenticationsExpire(record: ODRecordRef, will_expire_in: u64) -> bool;
}

extern "C-unwind" {
    #[cfg(feature = "CFOpenDirectory")]
    pub fn ODRecordSecondsUntilPasswordExpires(record: ODRecordRef) -> i64;
}

extern "C-unwind" {
    #[cfg(feature = "CFOpenDirectory")]
    pub fn ODRecordSecondsUntilAuthenticationsExpire(record: ODRecordRef) -> i64;
}
