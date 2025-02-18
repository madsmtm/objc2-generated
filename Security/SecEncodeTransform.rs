//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2_core_foundation::*;

use crate::*;

extern "C" {
    /// Specifies a base 64 encoding
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/security/ksecbase64encoding?language=objc)
    pub static kSecBase64Encoding: &'static CFString;
}

extern "C" {
    /// Specifies a base 32 encoding
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/security/ksecbase32encoding?language=objc)
    pub static kSecBase32Encoding: &'static CFString;
}

extern "C" {
    /// Specifies a compressed encoding.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/security/kseczlibencoding?language=objc)
    pub static kSecZLibEncoding: &'static CFString;
}

extern "C" {
    /// Used with SecTransformGetAttribute to query the attribute type.
    /// Returns one of the strings defined in the previous section.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/security/ksecencodetypeattribute?language=objc)
    pub static kSecEncodeTypeAttribute: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/security/kseclinelength64?language=objc)
    pub static kSecLineLength64: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/security/kseclinelength76?language=objc)
    pub static kSecLineLength76: &'static CFString;
}

extern "C" {
    /// Used with SecTransformSetAttribute to set the length
    /// of encoded Base32 or Base64 lines.   Some systems will
    /// not decode or otherwise deal with excessively long lines,
    /// or may be defined to limit lines to specific lengths
    /// (for example RFC1421 - 64, and RFC2045 - 76).
    ///
    /// The LineLengthAttribute may be set to any positive
    /// value (via a CFNumberRef) to limit to a specific
    /// length (values smaller then X for Base32 or Y for Base64
    /// are assume to be X or Y), or to zero for no specific
    /// limit.   Either of the string constants kSecLineLength64
    /// (RFC1421), or kSecLineLength76 (RFC2045) may be used to
    /// set line lengths of 64 or 76 bytes.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/security/ksecencodelinelengthattribute?language=objc)
    pub static kSecEncodeLineLengthAttribute: &'static CFString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/security/kseccompressionratio?language=objc)
    pub static kSecCompressionRatio: &'static CFString;
}

/// Creates an encode computation object.
///
/// Parameter `encodeType`: The type of encoding to compute.  You may pass NULL
/// for this parameter, in which case an appropriate
/// algorithm will be chosen for you.
///
/// Parameter `error`: A pointer to a CFErrorRef.  This pointer will be set
/// if an error occurred.  This value may be NULL if you
/// do not want an error returned.
///
/// Returns: A pointer to a SecTransformRef object.  This object must
/// be released with CFRelease when you are done with
/// it.  This function will return NULL if an error
/// occurred.
///
/// This function creates a transform which computes an
/// encode.
#[cfg(feature = "SecTransform")]
#[deprecated = "SecTransform is no longer supported"]
#[inline]
pub unsafe extern "C-unwind" fn SecEncodeTransformCreate(
    encode_type: &CFType,
    error: *mut *mut CFError,
) -> Option<CFRetained<SecTransform>> {
    extern "C-unwind" {
        fn SecEncodeTransformCreate(
            encode_type: &CFType,
            error: *mut *mut CFError,
        ) -> Option<NonNull<SecTransform>>;
    }
    let ret = unsafe { SecEncodeTransformCreate(encode_type, error) };
    ret.map(|ret| unsafe { CFRetained::from_raw(ret) })
}
