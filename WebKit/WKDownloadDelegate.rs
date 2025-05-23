//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkdownloadredirectpolicy?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WKDownloadRedirectPolicy(pub NSInteger);
impl WKDownloadRedirectPolicy {
    #[doc(alias = "WKDownloadRedirectPolicyCancel")]
    pub const Cancel: Self = Self(0);
    #[doc(alias = "WKDownloadRedirectPolicyAllow")]
    pub const Allow: Self = Self(1);
}

unsafe impl Encode for WKDownloadRedirectPolicy {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for WKDownloadRedirectPolicy {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkdownloadplaceholderpolicy?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WKDownloadPlaceholderPolicy(pub NSInteger);
impl WKDownloadPlaceholderPolicy {
    #[doc(alias = "WKDownloadPlaceholderPolicyDisable")]
    pub const Disable: Self = Self(0);
    #[doc(alias = "WKDownloadPlaceholderPolicyEnable")]
    pub const Enable: Self = Self(1);
}

unsafe impl Encode for WKDownloadPlaceholderPolicy {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for WKDownloadPlaceholderPolicy {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkdownloaddelegate?language=objc)
    pub unsafe trait WKDownloadDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(all(feature = "WKDownload", feature = "block2"))]
        #[unsafe(method(download:decideDestinationUsingResponse:suggestedFilename:completionHandler:))]
        #[unsafe(method_family = none)]
        unsafe fn download_decideDestinationUsingResponse_suggestedFilename_completionHandler(
            &self,
            download: &WKDownload,
            response: &NSURLResponse,
            suggested_filename: &NSString,
            completion_handler: &block2::DynBlock<dyn Fn(*mut NSURL)>,
        );

        #[cfg(all(feature = "WKDownload", feature = "block2"))]
        #[optional]
        #[unsafe(method(download:willPerformHTTPRedirection:newRequest:decisionHandler:))]
        #[unsafe(method_family = none)]
        unsafe fn download_willPerformHTTPRedirection_newRequest_decisionHandler(
            &self,
            download: &WKDownload,
            response: &NSHTTPURLResponse,
            request: &NSURLRequest,
            decision_handler: &block2::DynBlock<dyn Fn(WKDownloadRedirectPolicy)>,
        );

        #[cfg(all(feature = "WKDownload", feature = "block2"))]
        #[optional]
        #[unsafe(method(download:didReceiveAuthenticationChallenge:completionHandler:))]
        #[unsafe(method_family = none)]
        unsafe fn download_didReceiveAuthenticationChallenge_completionHandler(
            &self,
            download: &WKDownload,
            challenge: &NSURLAuthenticationChallenge,
            completion_handler: &block2::DynBlock<
                dyn Fn(NSURLSessionAuthChallengeDisposition, *mut NSURLCredential),
            >,
        );

        #[cfg(feature = "WKDownload")]
        #[optional]
        #[unsafe(method(downloadDidFinish:))]
        #[unsafe(method_family = none)]
        unsafe fn downloadDidFinish(&self, download: &WKDownload);

        #[cfg(feature = "WKDownload")]
        #[optional]
        #[unsafe(method(download:didFailWithError:resumeData:))]
        #[unsafe(method_family = none)]
        unsafe fn download_didFailWithError_resumeData(
            &self,
            download: &WKDownload,
            error: &NSError,
            resume_data: Option<&NSData>,
        );

        #[cfg(all(feature = "WKDownload", feature = "block2"))]
        #[optional]
        #[unsafe(method(download:decidePlaceholderPolicy:))]
        #[unsafe(method_family = none)]
        unsafe fn download_decidePlaceholderPolicy(
            &self,
            download: &WKDownload,
            completion_handler: &block2::DynBlock<dyn Fn(WKDownloadPlaceholderPolicy, *mut NSURL)>,
        );

        #[cfg(all(feature = "WKDownload", feature = "block2"))]
        #[optional]
        #[unsafe(method(download:didReceivePlaceholderURL:completionHandler:))]
        #[unsafe(method_family = none)]
        unsafe fn download_didReceivePlaceholderURL_completionHandler(
            &self,
            download: &WKDownload,
            url: &NSURL,
            completion_handler: &block2::DynBlock<dyn Fn()>,
        );

        #[cfg(feature = "WKDownload")]
        #[optional]
        #[unsafe(method(download:didReceiveFinalURL:))]
        #[unsafe(method_family = none)]
        unsafe fn download_didReceiveFinalURL(&self, download: &WKDownload, url: &NSURL);
    }
);
