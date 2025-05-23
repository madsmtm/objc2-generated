//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::cell::UnsafeCell;
use core::ffi::*;
use core::marker::{PhantomData, PhantomPinned};
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/security/kauthorizationexternalformlength?language=objc)
pub const kAuthorizationExternalFormLength: c_uint = 32;
/// [Apple's documentation](https://developer.apple.com/documentation/security/errauthorizationsuccess?language=objc)
pub const errAuthorizationSuccess: OSStatus = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/security/errauthorizationinvalidset?language=objc)
pub const errAuthorizationInvalidSet: OSStatus = -60001;
/// [Apple's documentation](https://developer.apple.com/documentation/security/errauthorizationinvalidref?language=objc)
pub const errAuthorizationInvalidRef: OSStatus = -60002;
/// [Apple's documentation](https://developer.apple.com/documentation/security/errauthorizationinvalidtag?language=objc)
pub const errAuthorizationInvalidTag: OSStatus = -60003;
/// [Apple's documentation](https://developer.apple.com/documentation/security/errauthorizationinvalidpointer?language=objc)
pub const errAuthorizationInvalidPointer: OSStatus = -60004;
/// [Apple's documentation](https://developer.apple.com/documentation/security/errauthorizationdenied?language=objc)
pub const errAuthorizationDenied: OSStatus = -60005;
/// [Apple's documentation](https://developer.apple.com/documentation/security/errauthorizationcanceled?language=objc)
pub const errAuthorizationCanceled: OSStatus = -60006;
/// [Apple's documentation](https://developer.apple.com/documentation/security/errauthorizationinteractionnotallowed?language=objc)
pub const errAuthorizationInteractionNotAllowed: OSStatus = -60007;
/// [Apple's documentation](https://developer.apple.com/documentation/security/errauthorizationinternal?language=objc)
pub const errAuthorizationInternal: OSStatus = -60008;
/// [Apple's documentation](https://developer.apple.com/documentation/security/errauthorizationexternalizenotallowed?language=objc)
pub const errAuthorizationExternalizeNotAllowed: OSStatus = -60009;
/// [Apple's documentation](https://developer.apple.com/documentation/security/errauthorizationinternalizenotallowed?language=objc)
pub const errAuthorizationInternalizeNotAllowed: OSStatus = -60010;
/// [Apple's documentation](https://developer.apple.com/documentation/security/errauthorizationinvalidflags?language=objc)
pub const errAuthorizationInvalidFlags: OSStatus = -60011;
/// [Apple's documentation](https://developer.apple.com/documentation/security/errauthorizationtoolexecutefailure?language=objc)
pub const errAuthorizationToolExecuteFailure: OSStatus = -60031;
/// [Apple's documentation](https://developer.apple.com/documentation/security/errauthorizationtoolenvironmenterror?language=objc)
pub const errAuthorizationToolEnvironmentError: OSStatus = -60032;
/// [Apple's documentation](https://developer.apple.com/documentation/security/errauthorizationbadaddress?language=objc)
pub const errAuthorizationBadAddress: OSStatus = -60033;

/// Optional flags passed in to several Authorization APIs.
/// See the description of AuthorizationCreate, AuthorizationCopyRights and AuthorizationFree for a description of how they affect those calls.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/security/authorizationflags?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AuthorizationFlags(pub u32);
bitflags::bitflags! {
    impl AuthorizationFlags: u32 {
        #[doc(alias = "kAuthorizationFlagDefaults")]
        const Defaults = 0;
        #[doc(alias = "kAuthorizationFlagInteractionAllowed")]
        const InteractionAllowed = 1<<0;
        #[doc(alias = "kAuthorizationFlagExtendRights")]
        const ExtendRights = 1<<1;
        #[doc(alias = "kAuthorizationFlagPartialRights")]
        const PartialRights = 1<<2;
        #[doc(alias = "kAuthorizationFlagDestroyRights")]
        const DestroyRights = 1<<3;
        #[doc(alias = "kAuthorizationFlagPreAuthorize")]
        const PreAuthorize = 1<<4;
        #[doc(alias = "kAuthorizationFlagSkipInternalAuth")]
        const SkipInternalAuth = 1<<9;
        #[doc(alias = "kAuthorizationFlagNoData")]
        const NoData = 1<<20;
    }
}

#[cfg(feature = "objc2")]
unsafe impl Encode for AuthorizationFlags {
    const ENCODING: Encoding = u32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for AuthorizationFlags {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/security/kauthorizationflagcannotpreauthorize?language=objc)
pub const kAuthorizationFlagCanNotPreAuthorize: c_uint = 1;

/// [Apple's documentation](https://developer.apple.com/documentation/security/authorizationopaqueref?language=objc)
#[repr(C)]
#[derive(Debug)]
pub struct AuthorizationOpaqueRef {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for AuthorizationOpaqueRef {
    const ENCODING_REF: Encoding =
        Encoding::Pointer(&Encoding::Struct("AuthorizationOpaqueRef", &[]));
}

/// Opaque reference to an authorization object.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/security/authorizationref?language=objc)
pub type AuthorizationRef = *const AuthorizationOpaqueRef;

/// A zero terminated string in UTF-8 encoding.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/security/authorizationstring?language=objc)
pub type AuthorizationString = *const c_char;

/// Each AuthorizationItem describes a single string-named item with optional
/// parameter value. The value must be contiguous memory of valueLength bytes;
/// internal structure is defined separately for each name.
///
/// Field: name name of the item, as an AuthorizationString. Mandatory.
/// Field: valueLength Number of bytes in parameter value. Must be 0 if no parameter value.
/// Field: value Pointer to the optional parameter value associated with name.
/// Must be NULL if no parameter value.
/// Field: flags Reserved field. Must be set to 0 on creation. Do not modify after that.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/security/authorizationitem?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AuthorizationItem {
    pub name: AuthorizationString,
    pub valueLength: usize,
    pub value: *mut c_void,
    pub flags: u32,
}

#[cfg(feature = "objc2")]
unsafe impl Encode for AuthorizationItem {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <AuthorizationString>::ENCODING,
            <usize>::ENCODING,
            <*mut c_void>::ENCODING,
            <u32>::ENCODING,
        ],
    );
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for AuthorizationItem {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// An AuthorizationItemSet structure represents a set of zero or more AuthorizationItems.  Since it is a set it should not contain any identical AuthorizationItems.
///
/// Field: count Number of items identified by items.
/// Field: items Pointer to an array of items.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/security/authorizationitemset?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AuthorizationItemSet {
    pub count: u32,
    pub items: *mut AuthorizationItem,
}

#[cfg(feature = "objc2")]
unsafe impl Encode for AuthorizationItemSet {
    const ENCODING: Encoding =
        Encoding::Struct("?", &[<u32>::ENCODING, <*mut AuthorizationItem>::ENCODING]);
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for AuthorizationItemSet {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// An AuthorizationExternalForm structure can hold the externalized form of
/// an AuthorizationRef. As such, it can be transmitted across IPC channels
/// to other processes, which can re-internalize it to recover a valid AuthorizationRef
/// handle.
/// The data contained in an AuthorizationExternalForm should be considered opaque.
///
/// SECURITY NOTE: Applications should take care to not disclose the AuthorizationExternalForm to
/// potential attackers since it would authorize rights to them.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/security/authorizationexternalform?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AuthorizationExternalForm {
    pub bytes: [c_char; 32],
}

#[cfg(feature = "objc2")]
unsafe impl Encode for AuthorizationExternalForm {
    const ENCODING: Encoding = Encoding::Struct("?", &[<[c_char; 32]>::ENCODING]);
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for AuthorizationExternalForm {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// An AuthorizationItemSet representing a set of rights each with an associated argument (value).
/// Each argument value is as defined for the specific right they belong to.  Argument values may not contain pointers as the should be copyable to different address spaces.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/security/authorizationrights?language=objc)
pub type AuthorizationRights = AuthorizationItemSet;

/// An AuthorizationItemSet representing environmental information of potential use
/// to authorization decisions.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/security/authorizationenvironment?language=objc)
pub type AuthorizationEnvironment = AuthorizationItemSet;

extern "C-unwind" {
    /// Create a new autorization object which can be used in other authorization calls.  When the authorization is no longer needed AuthorizationFree should be called.
    ///
    /// When the kAuthorizationFlagInteractionAllowed flag is set, user interaction will happen when required.  Failing to set this flag will result in this call failing with a errAuthorizationInteractionNotAllowed status when interaction is required.
    ///
    /// Setting the kAuthorizationFlagExtendRights flag will extend the currently available rights. If this flag is set the returned AuthorizationRef will grant all the rights requested when errAuthorizationSuccess is returned. If this flag is not set the operation will almost certainly succeed, but no attempt will be made to make the requested rights availible.
    /// Call AuthorizationCopyRights to figure out which of the requested rights are granted by the returned AuthorizationRef.
    ///
    /// Setting the kAuthorizationFlagPartialRights flag will cause this call to succeed if only some of the requested rights are being granted by the returned AuthorizationRef. Unless this flag is set this API will fail if not all the requested rights could be obtained.
    ///
    /// Setting the kAuthorizationFlagDestroyRights flag will prevent any rights obtained during this call from being preserved after returning from this API (This is most useful when the authorization parameter is NULL and the caller doesn't want to affect the session state in any way).
    ///
    /// Setting the kAuthorizationFlagPreAuthorize flag will pre authorize the requested rights so that at a later time -- by calling AuthorizationMakeExternalForm() follow by AuthorizationCreateFromExternalForm() -- the obtained rights can be used in a different process.  Rights that can't be preauthorized will be treated as if they were authorized for the sake of returning an error (in other words if all rights are either authorized or could not be preauthorized this call will still succeed).
    /// The rights which could not be preauthorized are not currently authorized and may fail to authorize when a later call to AuthorizationCopyRights() is made, unless the kAuthorizationFlagExtendRights and kAuthorizationFlagInteractionAllowed flags are set.  Even then they might still fail if the user does not supply the correct credentials.
    /// The reason for passing in this flag is to provide correct audit trail information and to avoid unnecessary user interaction.
    ///
    ///
    /// Parameter `rights`: (input/optional) An AuthorizationItemSet containing rights for which authorization is being requested.  If none are specified the resulting AuthorizationRef will authorize nothing at all.
    ///
    /// Parameter `environment`: (input/optional) An AuthorizationItemSet containing environment state used when making the autorization decision.  See the AuthorizationEnvironment type for details.
    ///
    /// Parameter `flags`: (input) options specified by the AuthorizationFlags enum.  set all unused bits to zero to allow for future expansion.
    ///
    /// Parameter `authorization`: (output optional) A pointer to an AuthorizationRef to be returned.  When the returned AuthorizationRef is no longer needed AuthorizationFree should be called to prevent anyone from using the acquired rights.  If NULL is specified no new rights are returned, but the system will attempt to authorize all the requested rights and return the appropriate status.
    ///
    ///
    /// Returns: errAuthorizationSuccess 0 authorization or all requested rights succeeded.
    ///
    /// errAuthorizationDenied -60005 The authorization for one or more of the requested rights was denied.
    ///
    /// errAuthorizationCanceled -60006 The authorization was canceled by the user.
    ///
    /// errAuthorizationInteractionNotAllowed -60007 The authorization was denied since no interaction with the user was allowed.
    pub fn AuthorizationCreate(
        rights: *const AuthorizationRights,
        environment: *const AuthorizationEnvironment,
        flags: AuthorizationFlags,
        authorization: *mut AuthorizationRef,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Destroy an AutorizationRef object. If the kAuthorizationFlagDestroyRights flag is passed,
    /// any rights associated with the authorization are lost. Otherwise, only local resources
    /// are released, and the rights may still be available to other clients.
    ///
    /// Setting the kAuthorizationFlagDestroyRights flag will prevent any rights that were obtained by the specified authorization object to be preserved after returning from this API.  This effectivaly locks down all potentially shared authorizations.
    ///
    ///
    /// Parameter `authorization`: (input) The authorization object on which this operation is performed.
    ///
    ///
    /// Parameter `flags`: (input) Bit mask of option flags to this call.
    ///
    ///
    /// Returns: errAuthorizationSuccess 0 No error.
    ///
    /// errAuthorizationInvalidRef -60002 The authorization parameter is invalid.
    pub fn AuthorizationFree(
        authorization: AuthorizationRef,
        flags: AuthorizationFlags,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Given a set of rights, return the subset that is currently authorized
    /// by the AuthorizationRef given.
    ///
    /// When the kAuthorizationFlagInteractionAllowed flag is set, user interaction will happen when required.  Failing to set this flag will result in this call failing with a errAuthorizationInteractionNotAllowed status when interaction is required.
    ///
    /// Setting the kAuthorizationFlagExtendRights flag will extend the currently available rights.
    ///
    /// Setting the kAuthorizationFlagPartialRights flag will cause this call to succeed if only some of the requested rights are being granted by the returned AuthorizationRef.  Unless this flag is set this API will fail if not all the requested rights could be obtained.
    ///
    /// Setting the kAuthorizationFlagDestroyRights flag will prevent any additional rights obtained during this call from being preserved after returning from this API.
    ///
    /// Setting the kAuthorizationFlagPreAuthorize flag will pre authorize the requested rights so that at a later time -- by calling AuthorizationMakeExternalForm() follow by AuthorizationCreateFromExternalForm() -- the obtained rights can be used in a different process.  Rights that can't be preauthorized will be treated as if they were authorized for the sake of returning an error (in other words if all rights are either authorized or could not be preauthorized this call will still succeed), and they will be returned in authorizedRights with their kAuthorizationFlagCanNotPreAuthorize bit in the flags field set to 1.
    /// The rights which could not be preauthorized are not currently authorized and may fail to authorize when a later call to AuthorizationCopyRights() is made, unless the kAuthorizationFlagExtendRights and kAuthorizationFlagInteractionAllowed flags are set.  Even then they might still fail if the user does not supply the correct credentials.
    /// The reason for passing in this flag is to provide correct audit trail information and to avoid unnecessary user interaction.
    ///
    ///
    /// Parameter `authorization`: (input) The authorization object on which this operation is performed.
    ///
    /// Parameter `rights`: (input) A rights set (see AuthorizationCreate).
    ///
    /// Parameter `environment`: (input/optional) An AuthorizationItemSet containing environment state used when making the autorization decision.  See the AuthorizationEnvironment type for details.
    ///
    /// Parameter `flags`: (input) options specified by the AuthorizationFlags enum.  set all unused bits to zero to allow for future expansion.
    ///
    /// Parameter `authorizedRights`: (output/optional) A pointer to a newly allocated AuthorizationInfoSet in which the authorized subset of rights are returned (authorizedRights should be deallocated by calling AuthorizationFreeItemSet() when it is no longer needed).  If NULL the only information returned is the status.  Note that if the kAuthorizationFlagPreAuthorize flag was specified rights that could not be preauthorized are returned in authorizedRights, but their flags contains the kAuthorizationFlagCanNotPreAuthorize bit.
    ///
    ///
    /// Returns: errAuthorizationSuccess 0 No error.
    ///
    /// errAuthorizationInvalidRef -60002 The authorization parameter is invalid.
    ///
    /// errAuthorizationInvalidSet -60001 The rights parameter is invalid.
    ///
    /// errAuthorizationInvalidPointer -60004 The authorizedRights parameter is invalid.
    pub fn AuthorizationCopyRights(
        authorization: AuthorizationRef,
        rights: NonNull<AuthorizationRights>,
        environment: *const AuthorizationEnvironment,
        flags: AuthorizationFlags,
        authorized_rights: *mut *mut AuthorizationRights,
    ) -> OSStatus;
}

/// Callback block passed to AuthorizationCopyRightsAsync.
///
///
/// Parameter `err`: (output) The result of the AuthorizationCopyRights call.
///
/// Parameter `blockAuthorizedRights`: (output) The authorizedRights from the AuthorizationCopyRights call to be deallocated by calling AuthorizationFreeItemSet() when it is no longer needed.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/security/authorizationasynccallback?language=objc)
#[cfg(feature = "block2")]
pub type AuthorizationAsyncCallback =
    *mut block2::DynBlock<dyn Fn(OSStatus, *mut AuthorizationRights)>;

extern "C-unwind" {
    /// An asynchronous version of AuthorizationCopyRights.
    ///
    ///
    /// Parameter `callbackBlock`: (input) The callback block to be called upon completion.
    #[cfg(feature = "block2")]
    pub fn AuthorizationCopyRightsAsync(
        authorization: AuthorizationRef,
        rights: NonNull<AuthorizationRights>,
        environment: *const AuthorizationEnvironment,
        flags: AuthorizationFlags,
        callback_block: AuthorizationAsyncCallback,
    );
}

extern "C-unwind" {
    /// Returns sideband information (e.g. access credentials) obtained from a call to AuthorizationCreate.  The format of this data depends of the tag specified.
    ///
    ///
    /// Parameter `authorization`: (input) The authorization object on which this operation is performed.
    ///
    /// Parameter `tag`: (input/optional) An optional string tag specifing which sideband information should be returned.  When NULL is specified all available information is returned.
    ///
    /// Parameter `info`: (output) A pointer to a newly allocated AuthorizationInfoSet in which the requested sideband infomation is returned (info should be deallocated by calling AuthorizationFreeItemSet() when it is no longer needed).
    ///
    ///
    /// Returns: errAuthorizationSuccess 0 No error.
    ///
    /// errAuthorizationInvalidRef -60002 The authorization parameter is invalid.
    ///
    /// errAuthorizationInvalidTag -60003 The tag parameter is invalid.
    ///
    /// errAuthorizationInvalidPointer -60004 The info parameter is invalid.
    pub fn AuthorizationCopyInfo(
        authorization: AuthorizationRef,
        tag: AuthorizationString,
        info: NonNull<*mut AuthorizationItemSet>,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Turn an Authorization into an external "byte blob" form so it can be
    /// transmitted to another process.
    /// Note that *storing* the external form somewhere will probably not do what
    /// you want, since authorizations are bounded by sessions, processes, and possibly
    /// time limits. This is for online transmission of authorizations.
    ///
    ///
    /// Parameter `authorization`: The (valid) authorization reference to externalize
    ///
    /// Parameter `extForm`: Pointer to an AuthorizationExternalForm variable to fill.
    ///
    ///
    /// Returns: errAuthorizationSuccess 0 No error.
    ///
    /// errAuthorizationExternalizeNotAllowed -60009 Externalizing this authorization is not allowed.
    ///
    /// errAuthorizationInvalidRef -60002 The authorization parameter is invalid.
    pub fn AuthorizationMakeExternalForm(
        authorization: AuthorizationRef,
        ext_form: NonNull<AuthorizationExternalForm>,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Internalize the external "byte blob" form of an authorization reference.
    ///
    ///
    /// Parameter `extForm`: Pointer to an AuthorizationExternalForm value.
    ///
    /// Parameter `authorization`: Will be filled with a valid AuthorizationRef on success.
    ///
    ///
    /// Returns: errAuthorizationInternalizeNotAllowed -60010 Internalizing this authorization is not allowed.
    pub fn AuthorizationCreateFromExternalForm(
        ext_form: NonNull<AuthorizationExternalForm>,
        authorization: NonNull<AuthorizationRef>,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Release the memory allocated for an AuthorizationItemSet that was allocated
    /// by an API call.
    ///
    ///
    /// Parameter `set`: The AuthorizationItemSet to deallocate.
    ///
    ///
    /// Returns: errAuthorizationSuccess 0 No error.
    ///
    /// errAuthorizationInvalidSet -60001 The set parameter is invalid.
    pub fn AuthorizationFreeItemSet(set: NonNull<AuthorizationItemSet>) -> OSStatus;
}

extern "C-unwind" {
    /// From within a tool launched via the AuthorizationExecuteWithPrivileges function
    /// ONLY, retrieve the AuthorizationRef originally passed to that function.
    /// While AuthorizationExecuteWithPrivileges already verified the authorization to
    /// launch your tool, the tool may want to avail itself of any additional pre-authorizations
    /// the caller may have obtained through that reference.
    ///
    ///
    /// This function has been deprecated and should no longer be used.
    /// Use a launchd-launched helper tool and/or the Service Mangement framework
    /// for this functionality.
    #[deprecated]
    pub fn AuthorizationCopyPrivilegedReference(
        authorization: NonNull<AuthorizationRef>,
        flags: AuthorizationFlags,
    ) -> OSStatus;
}
