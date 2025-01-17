//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkdownload?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKDownload;
);

unsafe impl NSObjectProtocol for WKDownload {}

unsafe impl NSProgressReporting for WKDownload {}

extern_methods!(
    unsafe impl WKDownload {
        #[unsafe(method_family(none))]
        #[method_id(originalRequest)]
        pub unsafe fn originalRequest(&self) -> Option<Retained<NSURLRequest>>;

        #[cfg(all(feature = "WKWebView", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        #[unsafe(method_family(none))]
        #[method_id(webView)]
        pub unsafe fn webView(&self) -> Option<Retained<WKWebView>>;

        #[cfg(feature = "WKDownloadDelegate")]
        #[unsafe(method_family(none))]
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn WKDownloadDelegate>>>;

        #[cfg(feature = "WKDownloadDelegate")]
        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn WKDownloadDelegate>>);

        #[method(isUserInitiated)]
        pub unsafe fn isUserInitiated(&self) -> bool;

        #[cfg(feature = "WKFrameInfo")]
        #[unsafe(method_family(none))]
        #[method_id(originatingFrame)]
        pub unsafe fn originatingFrame(&self) -> Retained<WKFrameInfo>;

        #[cfg(feature = "block2")]
        #[method(cancel:)]
        pub unsafe fn cancel(
            &self,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSData)>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKDownload {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
