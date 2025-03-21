//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaextension/mediaextensionerrordomain?language=objc)
    pub static MediaExtensionErrorDomain: Option<&'static NSErrorDomain>;
}

/// MediaExtension framework error codes.
///
/// These error codes are returned in the NSError object in the event a method fails.
///
/// Returned if the extension does not support an aspect of the media.
///
/// Returned if the extension cannot allocate memory.
///
/// Returned if the extension received an invalid parameter.
///
/// Returned if any type of error occurred while parsing the media.
///
/// Returned if the extension encountered an internal operation failure (e.g. code loading).
///
/// Returned if the extension encountered a property it does not support reading/writing to.
///
/// Returned if the plugin track reader is requested to return an edit that is out of range.
///
/// Returned if the plugin sample cursor cannot be created because there are no samples in the track, or if a loadSampleBufferContainingSamplesToEndCursor request is made that cannot be satisfied.
///
/// Returned to indicate a specific sample is not contiguous, spans more than one file, or is for some other reason unsuitable for reading directly from a file. For such samples, clients must call loadSampleBufferContainingSamplesToEndCursor.
///
/// Returned if the end of the source file has been reached.
///
/// Returned if an invalid operation is requested by the client on a byte source.
///
/// Returned if a decoder is asked to decode a sample without decoding the required reference frame dependencies first.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/mediaextension/meerror?language=objc)
// NS_ERROR_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MEError(pub NSInteger);
impl MEError {
    #[doc(alias = "MEErrorUnsupportedFeature")]
    pub const UnsupportedFeature: Self = Self(-19320);
    #[doc(alias = "MEErrorAllocationFailure")]
    pub const AllocationFailure: Self = Self(-19321);
    #[doc(alias = "MEErrorInvalidParameter")]
    pub const InvalidParameter: Self = Self(-19322);
    #[doc(alias = "MEErrorParsingFailure")]
    pub const ParsingFailure: Self = Self(-19323);
    #[doc(alias = "MEErrorInternalFailure")]
    pub const InternalFailure: Self = Self(-19324);
    #[doc(alias = "MEErrorPropertyNotSupported")]
    pub const PropertyNotSupported: Self = Self(-19325);
    #[doc(alias = "MEErrorNoSuchEdit")]
    pub const NoSuchEdit: Self = Self(-19326);
    #[doc(alias = "MEErrorNoSamples")]
    pub const NoSamples: Self = Self(-19327);
    #[doc(alias = "MEErrorLocationNotAvailable")]
    pub const LocationNotAvailable: Self = Self(-19328);
    #[doc(alias = "MEErrorEndOfStream")]
    pub const EndOfStream: Self = Self(-19329);
    #[doc(alias = "MEErrorPermissionDenied")]
    pub const PermissionDenied: Self = Self(-19330);
    #[doc(alias = "MEErrorReferenceMissing")]
    pub const ReferenceMissing: Self = Self(-19331);
}

unsafe impl Encode for MEError {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MEError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
