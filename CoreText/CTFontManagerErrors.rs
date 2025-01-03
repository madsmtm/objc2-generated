//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

extern "C" {
    /// CFError domain for CTFontManager errors
    ///
    /// CFErrors with this domain will have error codes corresponding to one of the CTFontManagerErrors above.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctfontmanagererrordomain?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTFontManagerErrorDomain: CFStringRef;
}

extern "C" {
    /// User info key to be used with CFError references returned from registration functions.
    ///
    /// The value associated with this key in the user info dictionary of a CFError is a CFArray of font URLs that failed with given error.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctfontmanagererrorfonturlskey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTFontManagerErrorFontURLsKey: CFStringRef;
}

extern "C" {
    /// User info key to be used with CFError references returned from registration functions.
    ///
    /// The value associated with this key in the user info dictionary of a CFError is a CFArray of font descriptors that failed with given error.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctfontmanagererrorfontdescriptorskey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTFontManagerErrorFontDescriptorsKey: CFStringRef;
}

extern "C" {
    /// User info key to be used with CFError references returned from registration functions.
    ///
    /// The value associated with this key in the user info dictionary of a CFError is a CFArray of font asset name strings that failed with given error.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/kctfontmanagererrorfontassetnamekey?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static kCTFontManagerErrorFontAssetNameKey: CFStringRef;
}

/// Font registration errors
///
/// Errors that would prevent registration of fonts for a specified font file URL.
///
/// The file does not exist at the specified URL.
///
/// Cannot access the file due to insufficient permissions.
///
/// The file is not a recognized or supported font file format.
///
/// The file contains invalid font data that could cause system problems.
///
/// The file has already been registered in the specified scope.
///
/// Errors that would prevent un-registration of fonts for a specified font file URL.
///
/// The file is not registered in the specified scope.
///
/// The font file is actively in use and cannot be unregistered.
///
/// The file is required by the system and cannot be unregistered.
///
/// The file could not be processed due to an unexpected FontProvider error.
///
/// The file could not be processed because the provider does not have a necessary entitlement.
///
/// The font descriptor does not have information to specify a font file.
///
/// The operation was cancelled by the user.
///
/// The file could not be registered because of a duplicated font name.
///
/// The file is not in an allowed location. It must be either in the application's bundle or an on-demand resource.
///
/// The operation failed due to a system limitation.
///
/// The specified scope is not supported.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coretext/ctfontmanagererror?language=objc)
// NS_ENUM
#[cfg(feature = "objc2-core-foundation")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CTFontManagerError(pub CFIndex);
#[cfg(feature = "objc2-core-foundation")]
impl CTFontManagerError {
    pub const kCTFontManagerErrorFileNotFound: Self = Self(101);
    pub const kCTFontManagerErrorInsufficientPermissions: Self = Self(102);
    pub const kCTFontManagerErrorUnrecognizedFormat: Self = Self(103);
    pub const kCTFontManagerErrorInvalidFontData: Self = Self(104);
    pub const kCTFontManagerErrorAlreadyRegistered: Self = Self(105);
    pub const kCTFontManagerErrorExceededResourceLimit: Self = Self(106);
    pub const kCTFontManagerErrorAssetNotFound: Self = Self(107);
    pub const kCTFontManagerErrorNotRegistered: Self = Self(201);
    pub const kCTFontManagerErrorInUse: Self = Self(202);
    pub const kCTFontManagerErrorSystemRequired: Self = Self(203);
    pub const kCTFontManagerErrorRegistrationFailed: Self = Self(301);
    pub const kCTFontManagerErrorMissingEntitlement: Self = Self(302);
    pub const kCTFontManagerErrorInsufficientInfo: Self = Self(303);
    pub const kCTFontManagerErrorCancelledByUser: Self = Self(304);
    pub const kCTFontManagerErrorDuplicatedName: Self = Self(305);
    pub const kCTFontManagerErrorInvalidFilePath: Self = Self(306);
    pub const kCTFontManagerErrorUnsupportedScope: Self = Self(307);
}

#[cfg(all(feature = "objc2", feature = "objc2-core-foundation"))]
unsafe impl Encode for CTFontManagerError {
    const ENCODING: Encoding = CFIndex::ENCODING;
}

#[cfg(all(feature = "objc2", feature = "objc2-core-foundation"))]
unsafe impl RefEncode for CTFontManagerError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
