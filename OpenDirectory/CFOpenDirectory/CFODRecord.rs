//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

#[cfg(feature = "objc2-core-foundation")]
unsafe impl ConcreteType for ODRecordRef {
    /// Standard GetTypeID function support for CF-based objects
    ///
    /// Returns the typeID for the ODRecord object
    ///
    /// Returns: a valid CFTypeID for the ODRecord object
    #[doc(alias = "ODRecordGetTypeID")]
    #[inline]
    fn type_id() -> CFTypeID {
        extern "C-unwind" {
            fn ODRecordGetTypeID() -> CFTypeID;
        }
        unsafe { ODRecordGetTypeID() }
    }
}

extern "C-unwind" {
    /// Similar to calling ODNodeSetCredentials except credentials are only set for this particular record's node
    ///
    /// Sets the credentials if necessary on the ODNodeRef referenced by this ODRecordRef.  Very similar to
    /// calling ODNodeSetCredentials except other records referencing the underlying ODNodeRef will not get
    /// authenticated, therefore inadvertant changes cannot occur.  If all records referencing a particular
    /// ODNodeRef need to be updated, then use ODNodeSetCredentials on the original ODNodeRef instead.  If the
    /// ODNodeRef is already authenticated with the same name and password, this will be a NOOP call.  The original
    /// ODNodeRef held by an ODRecordRef will be released and a new ODNodeRef will be created when the credentials
    /// are set for this ODRecordRef.  Calling this on multiple records could result in multiple References into the
    /// OpenDirectory daemon, which could cause errors logged into /var/log/system.log if a threshold is reached.
    ///
    /// Parameter `record`: an ODRecordRef to use
    ///
    /// Parameter `username`: a CFStringRef of the username used to authenticate
    ///
    /// Parameter `password`: a CFStringRef of the password used to authenticate
    ///
    /// Parameter `error`: an optional CFErrorRef reference for error details
    ///
    /// Returns: returns true on success, otherwise outError can be checked for details.  Upon failure the original node
    /// will still be intact.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn ODRecordSetNodeCredentials(
        record: &ODRecordRef,
        username: Option<&CFString>,
        password: Option<&CFString>,
        error: *mut *mut CFError,
    ) -> bool;
}

extern "C-unwind" {
    /// Similar to calling ODNodeSetCredentialsExtended except credentials are only set for this particular record's
    /// node
    ///
    /// Allows the caller to use other types of authentications that are available in Open Directory, that may
    /// require response-request loops, etc.  Not all OD plugins will support this call, look for
    /// kODErrorCredentialsMethodNotSupported in outError.  Same behavior as ODRecordSetNodeCredentials.
    ///
    /// Parameter `record`: an ODRecordRef to use
    ///
    /// Parameter `recordType`: a ODRecordTypeRef of the type of record to do the authentication with
    ///
    /// Parameter `authType`: a ODAuthenticationTypeRef of the type of authentication to be used (e.g., kDSStdAuthNTLMv2)
    ///
    /// Parameter `authItems`: a CFArrayRef of CFData or CFString items that will be sent in order to the auth process
    ///
    /// Parameter `outAuthItems`: a pointer to CFArrayRef that will be assigned to a CFArrayRef of CFData items if the
    /// call returned any values followup values
    ///
    /// Parameter `outContext`: a pointer to ODContextRef if the call requires further calls for response-request auths.
    ///
    /// Parameter `error`: an optional CFErrorRef reference for error details
    ///
    /// Returns: a bool will be returned with the result of the operation and outAuthItems set with response items
    /// and outContext set for any needed continuation.  Upon failure the original node will still be intact.
    #[cfg(all(
        feature = "CFOpenDirectoryConstants",
        feature = "objc2-core-foundation"
    ))]
    pub fn ODRecordSetNodeCredentialsExtended(
        record: &ODRecordRef,
        record_type: Option<&ODRecordType>,
        auth_type: Option<&ODAuthenticationType>,
        auth_items: Option<&CFArray>,
        out_auth_items: *mut *const CFArray,
        out_context: *mut *const ODContextRef,
        error: *mut *mut CFError,
    ) -> bool;
}

extern "C-unwind" {
    /// Unsupported function.
    ///
    /// Unsupported function.
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated]
    pub fn ODRecordSetNodeCredentialsUsingKerberosCache(
        record: &ODRecordRef,
        cache_name: Option<&CFString>,
        error: *mut *mut CFError,
    ) -> bool;
}

/// Returns a CFDictionaryRef of the effective policy for the user if available
///
/// Returns a CFDictionaryRef of the effective policy for the user if available
///
/// Parameter `allocator`: a CFAllocatorRef to use
///
/// Parameter `record`: an ODRecordRef to use
///
/// Parameter `error`: an optional CFErrorRef reference for error details
///
/// Returns: a CFDictionaryRef of the password policies for the supplied record, or NULL if no policy set
#[cfg(feature = "objc2-core-foundation")]
#[deprecated = "use ODRecordCopyEffectivePolicies"]
#[inline]
pub unsafe extern "C-unwind" fn ODRecordCopyPasswordPolicy(
    allocator: Option<&CFAllocator>,
    record: Option<&ODRecordRef>,
    error: *mut *mut CFError,
) -> Option<CFRetained<CFDictionary>> {
    extern "C-unwind" {
        fn ODRecordCopyPasswordPolicy(
            allocator: Option<&CFAllocator>,
            record: Option<&ODRecordRef>,
            error: *mut *mut CFError,
        ) -> Option<NonNull<CFDictionary>>;
    }
    let ret = unsafe { ODRecordCopyPasswordPolicy(allocator, record, error) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

extern "C-unwind" {
    /// Verifies the password provided is valid for the record
    ///
    /// Verifies the password provided is valid for the record.
    ///
    /// Parameter `record`: an ODRecordRef to use
    ///
    /// Parameter `password`: a CFStringRef of the password that is being verified
    ///
    /// Parameter `error`: an optional CFErrorRef reference for error details
    ///
    /// Returns: returns true on success, otherwise outError can be checked for details
    #[cfg(feature = "objc2-core-foundation")]
    pub fn ODRecordVerifyPassword(
        record: &ODRecordRef,
        password: Option<&CFString>,
        error: *mut *mut CFError,
    ) -> bool;
}

extern "C-unwind" {
    /// Allows use of other Open Directory types of authentications to verify a record password
    ///
    /// Allows the caller to use other types of authentications that are available in Open Directory, that may
    /// require response-request loops, etc.
    ///
    /// Parameter `record`: an ODRecordRef to use
    ///
    /// Parameter `authType`: a ODAuthenticationTypeRef of the type of authentication to be used (e.g., kODAuthenticationTypeCRAM_MD5)
    ///
    /// Parameter `authItems`: a CFArrayRef of CFData or CFString items that will be sent in order to the auth process
    ///
    /// Parameter `outAuthItems`: a pointer to CFArrayRef that will be assigned to a CFArrayRef of CFData items if the
    /// call returned any values followup values
    ///
    /// Parameter `outContext`: a pointer to ODContextRef if the call requires further calls for response-request auths.
    ///
    /// Parameter `error`: an optional CFErrorRef reference for error details
    ///
    /// Returns: a bool will be returned with the result of the operation and outAuthItems set with response items
    /// and outContext set for any needed continuation.  Some ODNodes may not support the call so an error of
    /// eNotHandledByThisNode or eNotYetImplemented may be returned.
    #[cfg(all(
        feature = "CFOpenDirectoryConstants",
        feature = "objc2-core-foundation"
    ))]
    pub fn ODRecordVerifyPasswordExtended(
        record: &ODRecordRef,
        auth_type: Option<&ODAuthenticationType>,
        auth_items: Option<&CFArray>,
        out_auth_items: *mut *const CFArray,
        out_context: *mut *const ODContextRef,
        error: *mut *mut CFError,
    ) -> bool;
}

extern "C-unwind" {
    /// Changes the password of an ODRecord
    ///
    /// Changes the password of an ODRecord.  If NULL is passed into inOldPassword, then an attempt to set
    /// the password will be tried.  If changing a password, then both old and new passwords should be supplied.
    ///
    /// Parameter `record`: an ODRecordRef to use
    ///
    /// Parameter `oldPassword`: a CFString of the record's old password (NULL is optional).
    ///
    /// Parameter `newPassword`: a CFString of the record's new password
    ///
    /// Parameter `error`: an optional CFErrorRef reference for error details
    ///
    /// Returns: returns true on success, otherwise outError can be checked for details
    #[cfg(feature = "objc2-core-foundation")]
    pub fn ODRecordChangePassword(
        record: &ODRecordRef,
        old_password: Option<&CFString>,
        new_password: Option<&CFString>,
        error: *mut *mut CFError,
    ) -> bool;
}

/// Returns the record type of an ODRecordRef
///
/// Returns the record type of an ODRecordRef
///
/// Parameter `record`: an ODRecordRef to use
///
/// Returns: a CFStringRef of the record type for this ODRecordRef
#[cfg(feature = "objc2-core-foundation")]
#[inline]
pub unsafe extern "C-unwind" fn ODRecordGetRecordType(
    record: &ODRecordRef,
) -> Option<CFRetained<CFString>> {
    extern "C-unwind" {
        fn ODRecordGetRecordType(record: &ODRecordRef) -> Option<NonNull<CFString>>;
    }
    let ret = unsafe { ODRecordGetRecordType(record) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

/// Returns the official record name of an ODRecordRef
///
/// Returns the official record name of an ODRecordRef which typically corresponds to the first value
/// of the kODAttributeTypeRecordName attribute, but not always.  This name should be a valid name in either case.
///
/// Parameter `record`: an ODRecordRef to use
///
/// Returns: a CFStringRef of the record name for this ODRecordRef
#[cfg(feature = "objc2-core-foundation")]
#[inline]
pub unsafe extern "C-unwind" fn ODRecordGetRecordName(
    record: &ODRecordRef,
) -> Option<CFRetained<CFString>> {
    extern "C-unwind" {
        fn ODRecordGetRecordName(record: &ODRecordRef) -> Option<NonNull<CFString>>;
    }
    let ret = unsafe { ODRecordGetRecordName(record) };
    ret.map(|ret| unsafe { CFRetained::retain(ret) })
}

/// Returns the value of an attribute as an array of CFStringRef or CFDataRef types
///
/// Returns the value of an attribute as an array of CFStringRef or CFDataRef, depending on
/// whether the data is Binary or not.  If the value has been fetched from the directory previously
/// a copy of the internal storage will be returned without going to the directory.  If it has not been fetched
/// previously, then it will be fetched at that time.
///
/// Parameter `record`: an ODRecordRef to use
///
/// Parameter `attribute`: a CFStringRef or ODAttributeType of the attribute (e.g., kODAttributeTypeRecordName, etc.)
///
/// Parameter `error`: an optional CFErrorRef reference for error details
///
/// Returns: a CFArrayRef of the attribute requested if possible, or NULL if the attribute doesn't exist
#[cfg(all(
    feature = "CFOpenDirectoryConstants",
    feature = "objc2-core-foundation"
))]
#[inline]
pub unsafe extern "C-unwind" fn ODRecordCopyValues(
    record: &ODRecordRef,
    attribute: Option<&ODAttributeType>,
    error: *mut *mut CFError,
) -> Option<CFRetained<CFArray>> {
    extern "C-unwind" {
        fn ODRecordCopyValues(
            record: &ODRecordRef,
            attribute: Option<&ODAttributeType>,
            error: *mut *mut CFError,
        ) -> Option<NonNull<CFArray>>;
    }
    let ret = unsafe { ODRecordCopyValues(record, attribute, error) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

extern "C-unwind" {
    /// Will take a CFDataRef or CFStringRef or a CFArrayRef of either type and set it for the attribute
    ///
    /// Will take a CFDataRef or CFStringRef or a CFArrayRef of either type and set it for the attribute.
    /// Any mixture of the types CFData and CFString are accepted.
    ///
    /// Parameter `record`: an ODRecordRef to use
    ///
    /// Parameter `attribute`: a CFStringRef of the attribute for values to be added too
    ///
    /// Parameter `valueOrValues`: a CFArrayRef of CFStringRef or CFDataRef types or either of the individual types, passing
    /// an empty CFArray deletes the attribute.  The underlying implementation will do this in the most efficient manner,
    /// either by adding only new values or completely replacing the values depending on the capabilities of the
    /// particular plugin.
    ///
    /// Parameter `error`: an optional CFErrorRef reference for error details
    ///
    /// Returns: returns true on success, otherwise outError can be checked for details
    #[cfg(all(
        feature = "CFOpenDirectoryConstants",
        feature = "objc2-core-foundation"
    ))]
    pub fn ODRecordSetValue(
        record: &ODRecordRef,
        attribute: Option<&ODAttributeType>,
        value_or_values: Option<&CFType>,
        error: *mut *mut CFError,
    ) -> bool;
}

extern "C-unwind" {
    /// Adds a value to an attribute
    ///
    /// Adds a value to an attribute.
    ///
    /// Parameter `record`: an ODRecordRef to use
    ///
    /// Parameter `attribute`: a CFStringRef of the attribute for values to be added too
    ///
    /// Parameter `value`: a CFTypeRef of the value to be added to the attribute, either CFStringRef or CFDataRef
    ///
    /// Parameter `error`: an optional CFErrorRef reference for error details
    ///
    /// Returns: returns true on success, otherwise outError can be checked for details
    #[cfg(all(
        feature = "CFOpenDirectoryConstants",
        feature = "objc2-core-foundation"
    ))]
    pub fn ODRecordAddValue(
        record: &ODRecordRef,
        attribute: Option<&ODAttributeType>,
        value: Option<&CFType>,
        error: *mut *mut CFError,
    ) -> bool;
}

extern "C-unwind" {
    /// Removes a particular value from an attribute.
    ///
    /// Removes a particular value from an attribute.
    ///
    /// Parameter `record`: an ODRecordRef to use
    ///
    /// Parameter `attribute`: a CFStringRef of the attribute to remove the value from
    ///
    /// Parameter `value`: a CFTypeRef of the value to be removed from the attribute.  Either CFStringRef or CFDataRef.
    /// If the value does not exist, true will be returned and no error will be set.
    ///
    /// Parameter `error`: an optional CFErrorRef reference for error details
    ///
    /// Returns: returns true on success, otherwise outError can be checked for details
    #[cfg(all(
        feature = "CFOpenDirectoryConstants",
        feature = "objc2-core-foundation"
    ))]
    pub fn ODRecordRemoveValue(
        record: &ODRecordRef,
        attribute: Option<&ODAttributeType>,
        value: Option<&CFType>,
        error: *mut *mut CFError,
    ) -> bool;
}

/// Returns the attributes and values in the form of a key-value pair set for this record.
///
/// Returns the attributes and values in the form of a key-value pair set for this record.  The key is a
/// CFStringRef or ODAttributeType of the attribute name (e.g., kODAttributeTypeRecordName, etc.) and the
/// value is an CFArrayRef of either CFDataRef or CFStringRef depending on the type of data.  Binary data will
/// be returned as CFDataRef.
///
/// Parameter `record`: an ODRecordRef to use
///
/// Parameter `attributes`: a CFArrayRef of attributes.  If an attribute has not been fetched previously, it will be
/// fetched in order to return the value.  If this parameter is NULL then all currently fetched attributes
/// will be returned.
///
/// Parameter `error`: an optional CFErrorRef reference for error details
///
/// Returns: a CFDictionaryRef of the attributes for the record
#[cfg(feature = "objc2-core-foundation")]
#[inline]
pub unsafe extern "C-unwind" fn ODRecordCopyDetails(
    record: &ODRecordRef,
    attributes: Option<&CFArray>,
    error: *mut *mut CFError,
) -> Option<CFRetained<CFDictionary>> {
    extern "C-unwind" {
        fn ODRecordCopyDetails(
            record: &ODRecordRef,
            attributes: Option<&CFArray>,
            error: *mut *mut CFError,
        ) -> Option<NonNull<CFDictionary>>;
    }
    let ret = unsafe { ODRecordCopyDetails(record, attributes, error) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

extern "C-unwind" {
    /// Synchronizes the record from the Directory in order to get current data and commit pending changes
    ///
    /// Synchronizes the record from the Directory in order to get current data.  Any previously fetched attributes
    /// will be refetched from the Directory.  This will not refetch the entire record, unless the entire record
    /// has been accessed.  Additionally, any changes made to the record will be committed to the directory
    /// if the node does not do immediate commits.
    ///
    /// Parameter `record`: an ODRecordRef to use
    ///
    /// Parameter `error`: an optional CFErrorRef reference for error details
    #[cfg(feature = "objc2-core-foundation")]
    pub fn ODRecordSynchronize(record: &ODRecordRef, error: *mut *mut CFError) -> bool;
}

extern "C-unwind" {
    /// Deletes the record from the node and invalidates the record.
    ///
    /// Deletes the record from the node and invalidates the record.  The ODRecordRef should be
    /// released after deletion.
    ///
    /// Parameter `record`: an ODRecordRef to use
    ///
    /// Parameter `error`: an optional CFErrorRef reference for error details
    ///
    /// Returns: returns true on success, otherwise outError can be checked for details
    #[cfg(feature = "objc2-core-foundation")]
    pub fn ODRecordDelete(record: &ODRecordRef, error: *mut *mut CFError) -> bool;
}

extern "C-unwind" {
    /// Will add the record as a member of the group record that is provided
    ///
    /// Will add the record as a member of the group record that is provided in an appopriate manner
    /// based on what the directory will store.  An error will be returned if the record is not a group record.
    /// Additionally, if the member record is not an appropriate type allowed as part of a group an
    /// error will be returned.
    ///
    /// Parameter `group`: an ODRecordRef of the group record to modify
    ///
    /// Parameter `member`: an ODRecordRef of the record to add to the group record
    ///
    /// Parameter `error`: an optional CFErrorRef reference for error details
    ///
    /// Returns: returns true on success, otherwise outError can be checked for details
    #[cfg(feature = "objc2-core-foundation")]
    pub fn ODRecordAddMember(
        group: &ODRecordRef,
        member: Option<&ODRecordRef>,
        error: *mut *mut CFError,
    ) -> bool;
}

extern "C-unwind" {
    /// Will remove the record as a member from the group record that is provided
    ///
    /// Will remove the record as a member from the group record that is provided.  If the record type
    /// of group is not a Group, false will be returned with an appropriate error.
    ///
    /// Parameter `group`: an ODRecordRef of the group record to modify
    ///
    /// Parameter `member`: an ODRecordRef of the record to remove from the group record
    ///
    /// Parameter `error`: an optional CFErrorRef reference for error details
    ///
    /// Returns: returns true on success, otherwise outError can be checked for details
    #[cfg(feature = "objc2-core-foundation")]
    pub fn ODRecordRemoveMember(
        group: &ODRecordRef,
        member: Option<&ODRecordRef>,
        error: *mut *mut CFError,
    ) -> bool;
}

extern "C-unwind" {
    /// Will use membership APIs to resolve group membership based on Group and Member record combination
    ///
    /// Will use membership APIs to resolve group membership based on Group and Member record combination.
    /// This API does not check attributes values directly, instead uses system APIs to deal with nested
    /// memberships.
    ///
    /// Parameter `group`: an ODRecordRef of the group to be checked for membership
    ///
    /// Parameter `member`: an ODRecordRef of the member to be checked against the group
    ///
    /// Parameter `error`: an optional CFErrorRef reference for error details
    ///
    /// Returns: returns true or false depending on result
    #[cfg(feature = "objc2-core-foundation")]
    pub fn ODRecordContainsMember(
        group: &ODRecordRef,
        member: Option<&ODRecordRef>,
        error: *mut *mut CFError,
    ) -> bool;
}

/// This will copy any policies configured for the record.
///
/// This will copy any policies configured for the record.
///
/// Parameter `record`: an ODRecordRef to use
///
/// Parameter `error`: an optional CFErrorRef reference for error details
///
/// Returns: a CFDictionaryRef containing all currently configured policies
#[cfg(feature = "objc2-core-foundation")]
#[deprecated = "use ODRecordCopyAccountPolicies"]
#[inline]
pub unsafe extern "C-unwind" fn ODRecordCopyPolicies(
    record: &ODRecordRef,
    error: *mut *mut CFError,
) -> Option<CFRetained<CFDictionary>> {
    extern "C-unwind" {
        fn ODRecordCopyPolicies(
            record: &ODRecordRef,
            error: *mut *mut CFError,
        ) -> Option<NonNull<CFDictionary>>;
    }
    let ret = unsafe { ODRecordCopyPolicies(record, error) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

/// This will copy the effective policies for the record (merging any node-level policies).
///
/// This will copy the effective policies for the record (merging any node-level policies).
///
/// Parameter `record`: an ODRecordRef to use
///
/// Parameter `error`: an optional CFErrorRef reference for error details
///
/// Returns: a CFDictionaryRef containing all currently configured policies (merging any node-level policies)
#[cfg(feature = "objc2-core-foundation")]
#[deprecated = "use ODRecordAuthenticationAllowed and similar functions"]
#[inline]
pub unsafe extern "C-unwind" fn ODRecordCopyEffectivePolicies(
    record: &ODRecordRef,
    error: *mut *mut CFError,
) -> Option<CFRetained<CFDictionary>> {
    extern "C-unwind" {
        fn ODRecordCopyEffectivePolicies(
            record: &ODRecordRef,
            error: *mut *mut CFError,
        ) -> Option<NonNull<CFDictionary>>;
    }
    let ret = unsafe { ODRecordCopyEffectivePolicies(record, error) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

/// This will return a dictionary of supported policies.
///
/// This will return a dictionary of supported policies, if appropriate, the value will be the maximum value allowed
/// for the policy in question.  For example, if password history is available, it will state how much history is
/// supported.
///
/// Parameter `record`: an ODRecordRef to use
///
/// Parameter `error`: an optional CFErrorRef reference for error details
///
/// Returns: a CFDictionaryRef containing all currently supported policies
#[cfg(feature = "objc2-core-foundation")]
#[deprecated]
#[inline]
pub unsafe extern "C-unwind" fn ODRecordCopySupportedPolicies(
    record: &ODRecordRef,
    error: *mut *mut CFError,
) -> Option<CFRetained<CFDictionary>> {
    extern "C-unwind" {
        fn ODRecordCopySupportedPolicies(
            record: &ODRecordRef,
            error: *mut *mut CFError,
        ) -> Option<NonNull<CFDictionary>>;
    }
    let ret = unsafe { ODRecordCopySupportedPolicies(record, error) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

extern "C-unwind" {
    /// This will set the policy for the record.
    ///
    /// This will set the policy for the record.  Policies are evaluated in combination with node-level policies.
    ///
    /// Parameter `record`: an ODRecordRef to use
    ///
    /// Parameter `policies`: a CFDictionary of policies to be set
    ///
    /// Parameter `error`: an optional CFErrorRef reference for error details
    ///
    /// Returns: a bool which signifies if the policy set succeeded, otherwise error is set.
    #[cfg(feature = "objc2-core-foundation")]
    #[deprecated = "use ODRecordSetAccountPolicies"]
    pub fn ODRecordSetPolicies(
        record: &ODRecordRef,
        policies: Option<&CFDictionary>,
        error: *mut *mut CFError,
    ) -> bool;
}

extern "C-unwind" {
    /// This will set a specific policy setting for the record.
    ///
    /// This will set a specific policy setting for the record.
    ///
    /// Parameter `record`: an ODRecordRef to use
    ///
    /// Parameter `policy`: a valid ODPolicyType
    ///
    /// Parameter `value`: a CFTypeRef to be set (should be of appropriate type for the policy)
    ///
    /// Parameter `error`: an optional CFErrorRef reference for error details
    ///
    /// Returns: a bool which signifies if the policy set succeeded, otherwise error is set.
    #[cfg(all(
        feature = "CFOpenDirectoryConstants",
        feature = "objc2-core-foundation"
    ))]
    #[deprecated = "use ODRecordAddAccountPolicy"]
    pub fn ODRecordSetPolicy(
        record: &ODRecordRef,
        policy: Option<&ODPolicyType>,
        value: Option<&CFType>,
        error: *mut *mut CFError,
    ) -> bool;
}

extern "C-unwind" {
    /// This will remove a specific policy setting from the record.
    ///
    /// This will remove a specific policy setting from the record.
    ///
    /// Parameter `record`: an ODRecordRef to use
    ///
    /// Parameter `policy`: a valid ODPolicyType
    ///
    /// Parameter `error`: an optional CFErrorRef reference for error details
    ///
    /// Returns: a bool which signifies if the policy removal succeeded, otherwise error is set.
    #[cfg(all(
        feature = "CFOpenDirectoryConstants",
        feature = "objc2-core-foundation"
    ))]
    #[deprecated = "use ODRecordRemoveAccountPolicy"]
    pub fn ODRecordRemovePolicy(
        record: &ODRecordRef,
        policy: Option<&ODPolicyType>,
        error: *mut *mut CFError,
    ) -> bool;
}

extern "C-unwind" {
    /// This will add an account policy to the record for the specified category.
    ///
    /// This will add an account policy to the record for the specified category.
    /// The node-level and record-level policies will be combined and
    /// evaluated as appropriate, ensuring the strongest policy is enforced.
    ///
    /// Parameter `record`: an ODRecordRef to use.
    ///
    /// Parameter `policy`: a dictionary containing the specific policy to be added.
    /// The dictionary may contain the following keys:
    /// kODPolicyKeyIdentifier a required key identifying the policy.
    /// kODPolicyKeyParameters an optional key containing a dictionary of
    /// parameters that can be used for informational purposes or in
    /// the policy format string.
    /// kODPolicyKeyContent a required key specifying the policy,
    /// from which a predicate will be created for evaluating
    /// the policy.
    ///
    /// Parameter `category`: a valid ODPolicyCategoryType to which the policy will be added.
    ///
    /// Parameter `error`: is an optional CFErrorRef reference for error details.
    ///
    /// Returns: a bool which signifies if the policy addition succeeded, otherwise error is set.
    #[cfg(all(
        feature = "CFOpenDirectoryConstants",
        feature = "objc2-core-foundation"
    ))]
    pub fn ODRecordAddAccountPolicy(
        record: &ODRecordRef,
        policy: Option<&CFDictionary>,
        category: Option<&ODPolicyCategoryType>,
        error: *mut *mut CFError,
    ) -> bool;
}

extern "C-unwind" {
    /// This will remove an account policy from the record for the specified category.
    ///
    /// This will remove an account policy from the record for the specified category.
    ///
    /// Parameter `record`: an ODRecordRef to use.
    ///
    /// Parameter `policy`: a dictionary containing the specific policy to be
    /// removed, with the same format as described in ODRecordAddAccountPolicy.
    ///
    /// Parameter `category`: a valid ODPolicyCategoryType from which the policy will be removed.
    ///
    /// Parameter `error`: an optional CFErrorRef reference for error details.
    ///
    /// Returns: a bool which signifies if the policy removal succeeded, otherwise error is set.
    #[cfg(all(
        feature = "CFOpenDirectoryConstants",
        feature = "objc2-core-foundation"
    ))]
    pub fn ODRecordRemoveAccountPolicy(
        record: &ODRecordRef,
        policy: Option<&CFDictionary>,
        category: Option<&ODPolicyCategoryType>,
        error: *mut *mut CFError,
    ) -> bool;
}

extern "C-unwind" {
    /// This will set the policies for the record.
    ///
    /// This will set the policies for the record, replacing any
    /// existing policies.  All of the policies in the set will be
    /// applied to the record when policies are evaluated.
    ///
    /// Parameter `record`: an ODRecordRef to use.
    ///
    /// Parameter `policies`: a dictionary containing all of the policies to be set
    /// for the node.  The dictionary may contain the following keys:
    /// kODPolicyCategoryAuthentication an optional key with a value
    /// of an array of policy dictionaries that specify when
    /// authentications should be allowed.
    /// kODPolicyCategoryPasswordContent an optional key with a
    /// value of an array of policy dictionaries the specify the
    /// required content of passwords.
    /// kODPolicyCategoryPasswordChange an optional key with a value
    /// of an array of policy dictionaries that specify when
    /// passwords are required to be changed.
    ///
    /// Parameter `error`: an optional CFErrorRef reference for error details.
    ///
    /// Returns: a bool which signifies if the policy set succeeded, otherwise error is set.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn ODRecordSetAccountPolicies(
        record: &ODRecordRef,
        policies: Option<&CFDictionary>,
        error: *mut *mut CFError,
    ) -> bool;
}

/// This will copy any policies configured for the record.
///
/// This will copy any policies configured for the record.  Does not
/// copy any policies set for the node.
///
/// Parameter `record`: an ODRecordRef to use.
///
/// Parameter `error`: an optional CFErrorRef reference for error details.
///
/// Returns: a CFDictionaryRef containing all currently set policies.  The
/// format of the dictionary is the same as described in
/// ODRecordSetAccountPolicies().
#[cfg(feature = "objc2-core-foundation")]
#[inline]
pub unsafe extern "C-unwind" fn ODRecordCopyAccountPolicies(
    record: &ODRecordRef,
    error: *mut *mut CFError,
) -> Option<CFRetained<CFDictionary>> {
    extern "C-unwind" {
        fn ODRecordCopyAccountPolicies(
            record: &ODRecordRef,
            error: *mut *mut CFError,
        ) -> Option<NonNull<CFDictionary>>;
    }
    let ret = unsafe { ODRecordCopyAccountPolicies(record, error) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}

extern "C-unwind" {
    /// Determines if policies allow the account to authenticate.
    ///
    /// Determines if policies allow the account to authenticate.
    /// Authentication and password change policies are evaluated.
    /// Record-level and node-level policies are evaluated in
    /// combination, with record-level taking precedence over node-level
    /// policies.  The failure of any single policy will deny the
    /// authentication.
    ///
    /// This check is only definitive at the time it was requested. The
    /// policy or the environment could change before the authentication
    /// is actually requested.  Errors from the authentication request
    /// should be consulted.
    ///
    /// It is not necessary to call this function when callingg
    /// ODRecordVerifyPassword or ODRecordVerifyPasswordExtended
    /// since those functions perform same policy evaluation.
    ///
    ///
    /// Parameter `record`: an ODRecordRef to use.
    ///
    /// Parameter `error`: an optional CFErrorRef reference for error details.
    ///
    /// Returns: a bool which signifies if the authentication is allowed, otherwise error is set.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn ODRecordAuthenticationAllowed(record: &ODRecordRef, error: *mut *mut CFError) -> bool;
}

extern "C-unwind" {
    /// Determines if policies allow the password change.
    ///
    /// Determines if policies allow the password change.  Password
    /// content policies are evaluated. Record-level and node-level
    /// policies are evaluated in combination, with record-level taking
    /// precedence over node-level policies.  The failure of any single
    /// policy will deny the password change.
    ///
    /// This check is only definitive at the time it was requested. The
    /// policy or the environment could change before the password change
    /// is actually requested.  Errors from the password change request
    /// should be consulted.
    ///
    ///
    /// Parameter `record`: an ODRecordRef to use.
    ///
    /// Parameter `newPassword`: contains the password to be evaluated.
    ///
    /// Parameter `error`: an optional CFErrorRef reference for error details.
    ///
    /// Returns: a bool which signifies if the password change is allowed, otherwise error is set.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn ODRecordPasswordChangeAllowed(
        record: &ODRecordRef,
        new_password: Option<&CFString>,
        error: *mut *mut CFError,
    ) -> bool;
}

extern "C-unwind" {
    /// Determines if the password will expire within the specified time.
    ///
    /// Determines if the password will expire (i.e. need to be changed)
    /// between now and the specified number of seconds in the future.
    /// Record-level and node-level policies are evaluated
    /// together, with record-level taking precedence over node-level
    /// policies.
    ///
    /// Parameter `record`: an ODRecordRef to use.
    ///
    /// Parameter `willExpireIn`: the number of seconds from the current time to be
    /// used as the upper-bound for the password expiration period.
    ///
    /// Returns: a bool which signifies if the password will expire within the
    /// specified time.
    pub fn ODRecordWillPasswordExpire(record: &ODRecordRef, will_expire_in: u64) -> bool;
}

extern "C-unwind" {
    /// Determines if authentications will expire within the specified time.
    ///
    /// Determines if authentications will expire (i.e. session and/or
    /// account expires) between now and the specified number of seconds
    /// in the future.  Record-level and node-level policies are evaluated
    /// together, with record-level taking precedence over node-level
    /// policies.
    ///
    /// Parameter `record`: an ODRecordRef to use.
    ///
    /// Parameter `willExpireIn`: the number of seconds from the current time to be
    /// used as the upper-bound for the authentication expiration period.
    ///
    /// Returns: a bool which signifies if authentications will expire within the
    /// specified time.
    pub fn ODRecordWillAuthenticationsExpire(record: &ODRecordRef, will_expire_in: u64) -> bool;
}

extern "C-unwind" {
    /// Determines how many seconds until the password expires.
    ///
    /// Determines how many seconds until the password expires (i.e.
    /// needs changing).  Password change policies are evaluated.
    /// Record-level and node-level policies are evaluated in
    /// combination, with record-level taking precedence over node-level
    /// policies.
    ///
    /// Parameter `record`: an ODRecordRef to use.
    ///
    /// Returns: the number of seconds until the password expires.  If multiple
    /// policies will cause the password to expire, the soonest
    /// expiration time is returned.  If already expired,
    /// kODExpirationTimeExpired is returned.  If there are no password
    /// change policies, kODExpirationTimeNeverExpires is returned.
    pub fn ODRecordSecondsUntilPasswordExpires(record: &ODRecordRef) -> i64;
}

extern "C-unwind" {
    /// Determines how many seconds until authentications expire.
    ///
    /// Determines how many seconds until authentications expire (i.e.
    /// session and/or account expires). Record-level and node-level
    /// policies are evaluated together, with record-level taking
    /// precedence over node-level policies
    ///
    /// Parameter `record`: an ODRecordRef to use.
    ///
    /// Returns: the number of seconds until authentications expire.  If multiple
    /// policies will cause authentications to expire, the soonest
    /// expiration time is returned. If already expired,
    /// kODExpirationTimeExpired is returned.  If there are no
    /// authentication policies controlling expiration,
    /// kODExpirationTimeNeverExpires is returned.
    pub fn ODRecordSecondsUntilAuthenticationsExpire(record: &ODRecordRef) -> i64;
}
