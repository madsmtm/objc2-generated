//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;

use crate::*;

/// These are externally visible identifiers for authorization sessions.
/// Different sessions have different identifiers; beyond that, you can't
/// tell anything from these values.
/// SessionIds can be compared for equality as you'd expect, but you should be careful
/// to use attribute bits wherever appropriate.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/security/securitysessionid?language=objc)
pub type SecuritySessionId = u32;

/// [Apple's documentation](https://developer.apple.com/documentation/security/nosecuritysession?language=objc)
pub const noSecuritySession: SecuritySessionId = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/security/callersecuritysession?language=objc)
pub const callerSecuritySession: SecuritySessionId = 4294967295;

/// Each Session has a set of attribute bits. You can get those from the
/// SessionGetInfo API function.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/security/sessionattributebits?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SessionAttributeBits(pub u32);
bitflags::bitflags! {
    impl SessionAttributeBits: u32 {
        const sessionIsRoot = 0x0001;
        const sessionHasGraphicAccess = 0x0010;
        const sessionHasTTY = 0x0020;
        const sessionIsRemote = 0x1000;
    }
}

#[cfg(feature = "objc2")]
unsafe impl Encode for SessionAttributeBits {
    const ENCODING: Encoding = u32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for SessionAttributeBits {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// These flags control how a new session is created by SessionCreate.
/// They have no permanent meaning beyond that.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/security/sessioncreationflags?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SessionCreationFlags(pub u32);
bitflags::bitflags! {
    impl SessionCreationFlags: u32 {
        const sessionKeepCurrentBootstrap = 0x8000;
    }
}

#[cfg(feature = "objc2")]
unsafe impl Encode for SessionCreationFlags {
    const ENCODING: Encoding = u32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for SessionCreationFlags {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/security/errsessionsuccess?language=objc)
pub const errSessionSuccess: OSStatus = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/security/errsessioninvalidid?language=objc)
pub const errSessionInvalidId: OSStatus = -60500;
/// [Apple's documentation](https://developer.apple.com/documentation/security/errsessioninvalidattributes?language=objc)
pub const errSessionInvalidAttributes: OSStatus = -60501;
/// [Apple's documentation](https://developer.apple.com/documentation/security/errsessionauthorizationdenied?language=objc)
pub const errSessionAuthorizationDenied: OSStatus = -60502;
/// [Apple's documentation](https://developer.apple.com/documentation/security/errsessionvaluenotset?language=objc)
pub const errSessionValueNotSet: OSStatus = -60503;
/// [Apple's documentation](https://developer.apple.com/documentation/security/errsessioninternal?language=objc)
pub const errSessionInternal: OSStatus = -60008;
/// [Apple's documentation](https://developer.apple.com/documentation/security/errsessioninvalidflags?language=objc)
pub const errSessionInvalidFlags: OSStatus = -60011;

extern "C-unwind" {
    /// Obtain information about a session. You can ask about any session whose
    /// identifier you know. Use the callerSecuritySession constant to ask about
    /// your own session (the one your process is in).
    ///
    ///
    /// Parameter `session`: (input) The Session you are asking about. Can be one of the
    /// special constants defined above.
    ///
    ///
    /// Parameter `sessionId`: (output/optional) The actual SecuritySessionId for the session you asked about.
    /// Will never be one of those constants.
    ///
    ///
    /// Parameter `attributes`: (output/optional) Receives the attribute bits for the session.
    ///
    ///
    /// Returns: An OSStatus indicating success (errSecSuccess) or an error cause.
    ///
    /// errSessionInvalidId -60500 Invalid session id specified
    pub fn SessionGetInfo(
        session: SecuritySessionId,
        session_id: *mut SecuritySessionId,
        attributes: *mut SessionAttributeBits,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// This (very specialized) function creates a security session.
    /// Upon completion, the new session contains the calling process (and none other).
    /// You cannot create a session for someone else, and cannot avoid being placed
    /// into the new session. This is (currently) the only call that changes a process's
    /// session membership.
    /// By default, a new bootstrap subset port is created for the calling process. The process
    /// acquires this new port as its bootstrap port, which all its children will inherit.
    /// If you happen to have created the subset port on your own, you can pass the
    /// sessionKeepCurrentBootstrap flag, and SessionCreate will use it. Note however that
    /// you cannot supersede a prior SessionCreate call that way; only a single SessionCreate
    /// call is allowed for each Session (however made).
    /// This call will discard any security information established for the calling process.
    /// In particular, any authorization handles acquired will become invalid, and so will any
    /// keychain related information. We recommend that you call SessionCreate before
    /// making any other security-related calls that establish rights of any kind, to the
    /// extent this is practical. Also, we strongly recommend that you do not perform
    /// security-related calls in any other threads while calling SessionCreate.
    ///
    ///
    /// Parameter `flags`: Flags controlling how the session is created.
    ///
    ///
    /// Parameter `attributes`: The set of attribute bits to set for the new session.
    /// Not all bits can be set this way.
    ///
    ///
    /// Returns: An OSStatus indicating success (errSecSuccess) or an error cause.
    ///
    /// errSessionInvalidAttributes -60501 Attempt to set invalid attribute bits
    /// errSessionAuthorizationDenied -60502 Attempt to re-initialize a session
    /// errSessionInvalidFlags -60011 Attempt to specify unsupported flag bits
    pub fn SessionCreate(flags: SessionCreationFlags, attributes: SessionAttributeBits)
        -> OSStatus;
}