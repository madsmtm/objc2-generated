//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2_core_foundation::*;

use crate::*;

#[cfg(feature = "CSCommon")]
unsafe impl ConcreteType for SecStaticCode {
    /// Returns the type identifier of all SecStaticCode instances.
    #[doc(alias = "SecStaticCodeGetTypeID")]
    #[inline]
    fn type_id() -> CFTypeID {
        extern "C-unwind" {
            fn SecStaticCodeGetTypeID() -> CFTypeID;
        }
        unsafe { SecStaticCodeGetTypeID() }
    }
}

extern "C-unwind" {
    /// Given a path to a file system object, create a SecStaticCode object representing
    /// the code at that location, if possible. Such a SecStaticCode is not inherently
    /// linked to running code in the system.
    ///
    /// It is possible to create a SecStaticCode object from an unsigned code object.
    /// Most uses of such an object will return the errSecCSUnsigned error. However,
    /// SecCodeCopyPath and SecCodeCopySigningInformation can be safely applied to such objects.
    ///
    ///
    /// Parameter `path`: A path to a location in the file system. Only file:// URLs are
    /// currently supported. For bundles, pass a URL to the root directory of the
    /// bundle. For single files, pass a URL to the file. If you pass a URL to the
    /// main executable of a bundle, the bundle as a whole will be generally recognized.
    /// Caution: Paths containing embedded // or /../ within a bundle's directory
    /// may cause the bundle to be misconstrued. If you expect to submit such paths,
    /// first clean them with realpath(3) or equivalent.
    ///
    /// Parameter `flags`: Optional flags. Pass kSecCSDefaultFlags for standard behavior.
    ///
    /// Parameter `staticCode`: On successful return, contains a reference to the StaticCode object
    /// representing the code at path. Unchanged on error.
    ///
    /// Returns: Upon success, errSecSuccess. Upon error, an OSStatus value documented in
    /// CSCommon.h or certain other Security framework headers.
    #[cfg(feature = "CSCommon")]
    pub fn SecStaticCodeCreateWithPath(
        path: &CFURL,
        flags: SecCSFlags,
        static_code: NonNull<*mut SecStaticCode>,
    ) -> OSStatus;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/security/kseccodeattributearchitecture?language=objc)
    pub static kSecCodeAttributeArchitecture: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/security/kseccodeattributesubarchitecture?language=objc)
    pub static kSecCodeAttributeSubarchitecture: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/security/kseccodeattributeuniversalfileoffset?language=objc)
    pub static kSecCodeAttributeUniversalFileOffset: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/security/kseccodeattributebundleversion?language=objc)
    pub static kSecCodeAttributeBundleVersion: &'static CFString;
}

extern "C-unwind" {
    /// Given a path to a file system object, create a SecStaticCode object representing
    /// the code at that location, if possible. Such a SecStaticCode is not inherently
    /// linked to running code in the system.
    ///
    /// It is possible to create a SecStaticCode object from an unsigned code object.
    /// Most uses of such an object will return the errSecCSUnsigned error. However,
    /// SecCodeCopyPath and SecCodeCopySigningInformation can be safely applied to such objects.
    ///
    ///
    /// Parameter `path`: A path to a location in the file system. Only file:// URLs are
    /// currently supported. For bundles, pass a URL to the root directory of the
    /// bundle. For single files, pass a URL to the file. If you pass a URL to the
    /// main executable of a bundle, the bundle as a whole will be generally recognized.
    /// Caution: Paths containing embedded // or /../ within a bundle's directory
    /// may cause the bundle to be misconstrued. If you expect to submit such paths,
    /// first clean them with realpath(3) or equivalent.
    ///
    /// Parameter `flags`: Optional flags. Pass kSecCSDefaultFlags for standard behavior.
    ///
    /// Parameter `attributes`: A CFDictionary containing additional attributes of the code sought.
    ///
    /// Parameter `staticCode`: On successful return, contains a reference to the StaticCode object
    /// representing the code at path. Unchanged on error.
    ///
    /// Returns: Upon success, errSecSuccess. Upon error, an OSStatus value documented in
    /// CSCommon.h or certain other Security framework headers.
    ///
    ///
    /// This can be a CFString containing a canonical architecture name ("i386" etc.), or a CFNumber
    /// specifying an architecture numerically (see mach/machine.h). This key is ignored if the code
    /// is not in Mach-O binary form. If the code is Mach-O but not universal ("thin"), the architecture
    /// specified must agree with the actual file contents.
    ///
    /// (using the kSecCodeAttributeArchitecture key), specifies any sub-architecture by number.
    /// This key is ignored if no main architecture is specified; if it is specified by name; or
    /// if the code is not in Mach-O form.
    ///
    ///
    /// then select the specified framework version. This key is otherwise ignored.
    #[cfg(feature = "CSCommon")]
    pub fn SecStaticCodeCreateWithPathAndAttributes(
        path: &CFURL,
        flags: SecCSFlags,
        attributes: &CFDictionary,
        static_code: NonNull<*mut SecStaticCode>,
    ) -> OSStatus;
}

/// [Apple's documentation](https://developer.apple.com/documentation/security/kseccscheckallarchitectures?language=objc)
pub const kSecCSCheckAllArchitectures: u32 = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/security/kseccsdonotvalidateexecutable?language=objc)
pub const kSecCSDoNotValidateExecutable: u32 = 2;
/// [Apple's documentation](https://developer.apple.com/documentation/security/kseccsdonotvalidateresources?language=objc)
pub const kSecCSDoNotValidateResources: u32 = 4;
/// [Apple's documentation](https://developer.apple.com/documentation/security/kseccsbasicvalidateonly?language=objc)
pub const kSecCSBasicValidateOnly: u32 = 6;
/// [Apple's documentation](https://developer.apple.com/documentation/security/kseccschecknestedcode?language=objc)
pub const kSecCSCheckNestedCode: u32 = 8;
/// [Apple's documentation](https://developer.apple.com/documentation/security/kseccsstrictvalidate?language=objc)
pub const kSecCSStrictValidate: u32 = 16;
/// [Apple's documentation](https://developer.apple.com/documentation/security/kseccsfullreport?language=objc)
pub const kSecCSFullReport: u32 = 32;
/// [Apple's documentation](https://developer.apple.com/documentation/security/kseccscheckgatekeeperarchitectures?language=objc)
pub const kSecCSCheckGatekeeperArchitectures: u32 = 65;
/// [Apple's documentation](https://developer.apple.com/documentation/security/kseccsrestrictsymlinks?language=objc)
pub const kSecCSRestrictSymlinks: u32 = 128;
/// [Apple's documentation](https://developer.apple.com/documentation/security/kseccsrestricttoapplike?language=objc)
pub const kSecCSRestrictToAppLike: u32 = 256;
/// [Apple's documentation](https://developer.apple.com/documentation/security/kseccsrestrictsidebanddata?language=objc)
pub const kSecCSRestrictSidebandData: u32 = 512;
/// [Apple's documentation](https://developer.apple.com/documentation/security/kseccsusesoftwaresigningcert?language=objc)
pub const kSecCSUseSoftwareSigningCert: u32 = 1024;
/// [Apple's documentation](https://developer.apple.com/documentation/security/kseccsvalidatepeh?language=objc)
pub const kSecCSValidatePEH: u32 = 2048;
/// [Apple's documentation](https://developer.apple.com/documentation/security/kseccssinglethreaded?language=objc)
pub const kSecCSSingleThreaded: u32 = 4096;
/// [Apple's documentation](https://developer.apple.com/documentation/security/kseccsallownetworkaccess?language=objc)
pub const kSecCSAllowNetworkAccess: u32 = 65536;
/// [Apple's documentation](https://developer.apple.com/documentation/security/kseccsfastexecutablevalidation?language=objc)
pub const kSecCSFastExecutableValidation: u32 = 131072;

extern "C-unwind" {
    #[cfg(feature = "CSCommon")]
    pub fn SecStaticCodeCheckValidity(
        static_code: &SecStaticCode,
        flags: SecCSFlags,
        requirement: Option<&SecRequirement>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "CSCommon")]
    pub fn SecStaticCodeCheckValidityWithErrors(
        static_code: &SecStaticCode,
        flags: SecCSFlags,
        requirement: Option<&SecRequirement>,
        errors: *mut *mut CFError,
    ) -> OSStatus;
}