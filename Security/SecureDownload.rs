//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::cell::UnsafeCell;
use core::ffi::*;
use core::marker::{PhantomData, PhantomPinned};
#[cfg(feature = "objc2")]
use objc2::__framework_prelude::*;
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/security/opaquesecuredownload?language=objc)
#[repr(C)]
#[derive(Debug)]
pub struct OpaqueSecureDownload {
    inner: [u8; 0],
    _p: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for OpaqueSecureDownload {
    const ENCODING_REF: Encoding =
        Encoding::Pointer(&Encoding::Struct("OpaqueSecureDownload", &[]));
}

/// [Apple's documentation](https://developer.apple.com/documentation/security/securedownloadref?language=objc)
pub type SecureDownloadRef = *mut OpaqueSecureDownload;

/// [Apple's documentation](https://developer.apple.com/documentation/security/errsecuredownloadinvalidticket?language=objc)
pub const errSecureDownloadInvalidTicket: c_int = -20052;
/// [Apple's documentation](https://developer.apple.com/documentation/security/errsecuredownloadinvaliddownload?language=objc)
pub const errSecureDownloadInvalidDownload: c_int = -20053;

/// This type is used to indicate whether or not a
/// signer should be evaluated.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/security/securedownloadtrustcallbackresult?language=objc)
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SecureDownloadTrustCallbackResult(pub c_uint);
impl SecureDownloadTrustCallbackResult {
    #[doc(alias = "kSecureDownloadDoNotEvaluateSigner")]
    pub const DoNotEvaluateSigner: Self = Self(0);
    #[doc(alias = "kSecureDownloadEvaluateSigner")]
    pub const EvaluateSigner: Self = Self(1);
    #[doc(alias = "kSecureDownloadFailEvaluation")]
    pub const FailEvaluation: Self = Self(2);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for SecureDownloadTrustCallbackResult {
    const ENCODING: Encoding = c_uint::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for SecureDownloadTrustCallbackResult {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// This callback is used to determine whether trust for a particular
/// signer should be evaluated.
///
/// Parameter `trustRef`: The trustRef for this evaluation
///
/// Parameter `setupContext`: user defined.
///
/// Returns: A SecureDownloadTrustCallbackResult (see).
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/security/securedownloadtrustsetupcallback?language=objc)
#[cfg(feature = "SecTrust")]
pub type SecureDownloadTrustSetupCallback = Option<
    unsafe extern "C-unwind" fn(*mut SecTrust, *mut c_void) -> SecureDownloadTrustCallbackResult,
>;

/// This callback is used called after trust has been evaluated.
///
/// Parameter `trustRef`: The trustRef for this evaluation
///
/// Parameter `result`: The result of the evaluation (See the SecTrust documentation).
///
/// Parameter `evaluateContext`: user defined.
///
/// Returns: A SecTrustResultType.  Return the value passed in result if you
/// do not want to change the evaluation result.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/security/securedownloadtrustevaluatecallback?language=objc)
#[cfg(feature = "SecTrust")]
pub type SecureDownloadTrustEvaluateCallback = Option<
    unsafe extern "C-unwind" fn(
        *mut SecTrust,
        SecTrustResultType,
        *mut c_void,
    ) -> SecTrustResultType,
>;

extern "C-unwind" {
    /// Create a SecureDownloadRef for use during the Secure Download process.
    ///
    /// Parameter `ticket`: The download ticket.
    ///
    /// Parameter `setup`: Called before trust is verified for each signer of the ticket.
    /// This allows the user to modify the SecTrustRef if needed
    /// (see the SecTrust documentation).  Returns a SecureDownloadTrustCallbackResult (see).
    ///
    /// Parameter `setupContext`: User defined.  Passed as a parameter to the setupCallback.
    ///
    /// Parameter `evaluate`: Called after SecTrustEvaluate has been called for a
    /// signer if the result was not trusted. This allows
    /// the developer to query the user as to whether or not
    /// to trust the signer.  Returns a SecTrustResultType
    ///
    /// Parameter `evaluateContext`: User defined.  Passed as a parameter to the evaluate callback.
    ///
    /// Parameter `downloadRef`: The returned reference.
    ///
    /// Returns: Returns errSecureDownloadInvalidTicket if the ticket was invalid.  Otherwise
    /// see "Security Error Codes" (SecBase.h).
    /// .
    #[cfg(feature = "SecTrust")]
    #[deprecated = "SecureDownload is not supported"]
    pub fn SecureDownloadCreateWithTicket(
        ticket: Option<&CFData>,
        setup: SecureDownloadTrustSetupCallback,
        setup_context: *mut c_void,
        evaluate: SecureDownloadTrustEvaluateCallback,
        evaluate_context: *mut c_void,
        download_ref: *mut SecureDownloadRef,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Return a list of URL's from which the data can be downloaded.  The first
    /// URL in the list is the preferred download location.  The other URL's are
    /// backup locations in case earlier locations in the list could not be
    /// accessed.
    ///
    /// Parameter `downloadRef`: A SecureDownloadRef instance.
    ///
    /// Parameter `urls`: On return, the list of URL's to download.  Format is a CFArray of CFURL's.
    ///
    /// Returns: A result code.  See "Security Error Codes" (SecBase.h).
    #[deprecated = "SecureDownload is not supported"]
    pub fn SecureDownloadCopyURLs(
        download_ref: SecureDownloadRef,
        urls: *mut *const CFArray,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Return the printable name of this download ticket.
    ///
    /// Parameter `downloadRef`: A SecureDownloadRef instance.
    ///
    /// Parameter `name`: On output, the download name.
    ///
    /// Returns: A result code.  See "Security Error Codes" (SecBase.h).
    #[deprecated = "SecureDownload is not supported"]
    pub fn SecureDownloadCopyName(
        download_ref: SecureDownloadRef,
        name: *mut *const CFString,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Return the date the downlooad ticket was created.
    ///
    /// Parameter `downloadRef`: A SecureDownloadRef instance.
    ///
    /// Returns: A result code.
    #[deprecated = "SecureDownload is not supported"]
    pub fn SecureDownloadCopyCreationDate(
        download_ref: SecureDownloadRef,
        date: *mut *const CFDate,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Return the size of the expected download.
    ///
    /// Parameter `downloadRef`: A SecureDownloadRef instance.
    ///
    /// Parameter `downloadSize`: On output, the size of the download.
    ///
    /// Returns: A result code.  See "Security Error Codes" (SecBase.h).
    #[deprecated = "SecureDownload is not supported"]
    pub fn SecureDownloadGetDownloadSize(
        download_ref: SecureDownloadRef,
        download_size: *mut i64,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Check data received during Secure Download for validity.
    /// Call this function each time data is received.
    ///
    /// Parameter `downloadRef`: A SecureDownloadRef instance.
    ///
    /// Parameter `data`: The data to check.
    ///
    /// Returns: Returns errSecureDownloadInvalidDownload if data is invalid.  Otherwise
    /// see "Security Error Codes" (SecBase.h).
    #[deprecated = "SecureDownload is not supported"]
    pub fn SecureDownloadUpdateWithData(
        download_ref: SecureDownloadRef,
        data: Option<&CFData>,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Concludes the secure download process.  Call this after all data has been received.
    ///
    /// Parameter `downloadRef`: A SecureDownloadRef instance.
    ///
    /// Returns: Returns errSecureDownloadInvalidDownload if data is invalid.  Otherwise
    /// see "Security Error Codes" (SecBase.h).
    #[deprecated = "SecureDownload is not supported"]
    pub fn SecureDownloadFinished(download_ref: SecureDownloadRef) -> OSStatus;
}

extern "C-unwind" {
    /// Releases a SecureDownloadRef.
    ///
    /// Parameter `downloadRef`: The SecureDownloadRef to release.
    ///
    /// Returns: A result code.  See "Security Error Codes" (SecBase.h).
    #[deprecated = "SecureDownload is not supported"]
    pub fn SecureDownloadRelease(download_ref: SecureDownloadRef) -> OSStatus;
}

extern "C-unwind" {
    /// Copies the ticket location from an x-securedownload URL.
    ///
    /// Parameter `url`: The x-securedownload URL.
    ///
    /// Parameter `ticketLocation`: On exit, the URL of the ticket.
    ///
    /// Returns: A result code.  See "Security Error Codes" (SecBase.h).
    #[deprecated = "SecureDownload is not supported"]
    pub fn SecureDownloadCopyTicketLocation(
        url: Option<&CFURL>,
        ticket_location: *mut *const CFURL,
    ) -> OSStatus;
}
