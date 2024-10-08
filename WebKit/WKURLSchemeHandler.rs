//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;

use crate::*;

extern_protocol!(
    pub unsafe trait WKURLSchemeHandler: NSObjectProtocol + MainThreadOnly {
        #[cfg(all(
            feature = "WKURLSchemeTask",
            feature = "WKWebView",
            feature = "objc2-app-kit"
        ))]
        #[cfg(target_os = "macos")]
        #[method(webView:startURLSchemeTask:)]
        unsafe fn webView_startURLSchemeTask(
            &self,
            web_view: &WKWebView,
            url_scheme_task: &ProtocolObject<dyn WKURLSchemeTask>,
        );

        #[cfg(all(
            feature = "WKURLSchemeTask",
            feature = "WKWebView",
            feature = "objc2-app-kit"
        ))]
        #[cfg(target_os = "macos")]
        #[method(webView:stopURLSchemeTask:)]
        unsafe fn webView_stopURLSchemeTask(
            &self,
            web_view: &WKWebView,
            url_scheme_task: &ProtocolObject<dyn WKURLSchemeTask>,
        );
    }

    unsafe impl ProtocolType for dyn WKURLSchemeHandler {}
);
