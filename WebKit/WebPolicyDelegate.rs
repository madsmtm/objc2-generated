//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// The type of action that triggered a possible navigation.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/webkit/webnavigationtype?language=objc)
// NS_ENUM
#[deprecated]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WebNavigationType(pub NSInteger);
impl WebNavigationType {
    #[doc(alias = "WebNavigationTypeLinkClicked")]
    #[deprecated]
    pub const LinkClicked: Self = Self(0);
    #[doc(alias = "WebNavigationTypeFormSubmitted")]
    #[deprecated]
    pub const FormSubmitted: Self = Self(1);
    #[doc(alias = "WebNavigationTypeBackForward")]
    #[deprecated]
    pub const BackForward: Self = Self(2);
    #[doc(alias = "WebNavigationTypeReload")]
    #[deprecated]
    pub const Reload: Self = Self(3);
    #[doc(alias = "WebNavigationTypeFormResubmitted")]
    #[deprecated]
    pub const FormResubmitted: Self = Self(4);
    #[doc(alias = "WebNavigationTypeOther")]
    #[deprecated]
    pub const Other: Self = Self(5);
}

unsafe impl Encode for WebNavigationType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for WebNavigationType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/webactionnavigationtypekey?language=objc)
    pub static WebActionNavigationTypeKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/webactionelementkey?language=objc)
    pub static WebActionElementKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/webactionbuttonkey?language=objc)
    pub static WebActionButtonKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/webactionmodifierflagskey?language=objc)
    pub static WebActionModifierFlagsKey: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/webactionoriginalurlkey?language=objc)
    pub static WebActionOriginalURLKey: Option<&'static NSString>;
}

extern_protocol!(
    /// This protocol is used to call back with the results of a
    /// policy decision. This provides the ability to make these decisions
    /// asyncrhonously, which means the decision can be made by prompting
    /// with a sheet, for example.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/webkit/webpolicydecisionlistener?language=objc)
    #[deprecated]
    pub unsafe trait WebPolicyDecisionListener: NSObjectProtocol {
        /// Use the resource
        ///
        /// If there remain more policy decisions to be made, then
        /// the next policy delegate method gets to decide. This will be
        /// either the next navigation policy delegate if there is a redirect,
        /// or the content policy delegate. If there are no more policy
        /// decisions to be made, the resource will be displayed inline if
        /// possible. If there is no view available to display the resource
        /// inline, then unableToImplementPolicyWithError:frame: will be
        /// called with an appropriate error.
        ///
        /// <p>
        /// If a new window is going to be created for this navigation as a
        /// result of frame targeting, then it will be created once you call
        /// this method.
        #[deprecated]
        #[unsafe(method(use))]
        #[unsafe(method_family = none)]
        unsafe fn r#use(&self);

        /// Download the resource instead of displaying it.
        ///
        /// This method is more than just a convenience because it
        /// allows an in-progress navigation to be converted to a download
        /// based on content type, without having to stop and restart the
        /// load.
        #[deprecated]
        #[unsafe(method(download))]
        #[unsafe(method_family = none)]
        unsafe fn download(&self);

        /// Do nothing (but the client may choose to handle the request itself)
        ///
        /// A policy of ignore prevents WebKit from doing anything
        /// further with the load, however, the client is still free to handle
        /// the request in some other way, such as opening a new window,
        /// opening a new window behind the current one, opening the URL in an
        /// external app, revealing the location in Finder if a file URL, etc.
        #[deprecated]
        #[unsafe(method(ignore))]
        #[unsafe(method_family = none)]
        unsafe fn ignore(&self);
    }
);

extern_protocol!(
    /// While loading a URL, WebKit asks the WebPolicyDelegate for
    /// policies that determine the action of what to do with the URL or the data that
    /// the URL represents. Typically, the policy handler methods are called in this order:
    ///
    /// decidePolicyForNewWindowAction:request:newFrameName:decisionListener: (at most once)
    /// <BR
    /// >
    /// decidePolicyForNavigationAction:request:frame:decisionListener: (zero or more times)
    /// <BR
    /// >
    /// decidePolicyForMIMEType:request:frame: (zero or more times)
    /// <BR
    /// >
    ///
    /// New window policy is always checked. Navigation policy is checked
    /// for the initial load and every redirect unless blocked by an
    /// earlier policy. Content policy is checked once the content type is
    /// known, unless an earlier policy prevented it.
    ///
    /// In rare cases, content policy might be checked more than
    /// once. This occurs when loading a "multipart/x-mixed-replace"
    /// document, also known as "server push". In this case, multiple
    /// documents come in one navigation, with each replacing the last. In
    /// this case, conent policy will be checked for each one.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/webkit/webpolicydelegate?language=objc)
    #[deprecated]
    pub unsafe trait WebPolicyDelegate: NSObjectProtocol {
        #[cfg(all(feature = "WebFrame", feature = "WebView", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        /// This method is called to decide what to do with a proposed navigation.
        ///
        /// Parameter `actionInformation`: Dictionary that describes the action that triggered this navigation.
        ///
        /// Parameter `request`: The request for the proposed navigation
        ///
        /// Parameter `frame`: The WebFrame in which the navigation is happening
        ///
        /// Parameter `listener`: The object to call when the decision is made
        ///
        /// This method will be called before loading starts, and
        /// on every redirect.
        #[deprecated]
        #[optional]
        #[unsafe(method(webView:decidePolicyForNavigationAction:request:frame:decisionListener:))]
        #[unsafe(method_family = none)]
        unsafe fn webView_decidePolicyForNavigationAction_request_frame_decisionListener(
            &self,
            web_view: Option<&WebView>,
            action_information: Option<&NSDictionary>,
            request: Option<&NSURLRequest>,
            frame: Option<&WebFrame>,
            listener: Option<&ProtocolObject<dyn WebPolicyDecisionListener>>,
        );

        #[cfg(all(feature = "WebView", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        /// This method is called to decide what to do with an targetted nagivation that would open a new window.
        ///
        /// Parameter `actionInformation`: Dictionary that describes the action that triggered this navigation.
        ///
        /// Parameter `request`: The request for the proposed navigation
        ///
        /// Parameter `frameName`: The frame in which the navigation is taking place
        ///
        /// Parameter `listener`: The object to call when the decision is made
        ///
        /// This method is provided so that modified clicks on a targetted link which
        /// opens a new frame can prevent the new window from being opened if they decide to
        /// do something else, like download or present the new frame in a specialized way.
        ///
        /// <p>
        /// If this method picks a policy of Use, the new window will be
        /// opened, and decidePolicyForNavigationAction:request:frame:decisionListner:
        /// will be called with a WebNavigationType of WebNavigationTypeOther
        /// in its action. This is to avoid possible confusion about the modifiers.
        #[deprecated]
        #[optional]
        #[unsafe(method(webView:decidePolicyForNewWindowAction:request:newFrameName:decisionListener:))]
        #[unsafe(method_family = none)]
        unsafe fn webView_decidePolicyForNewWindowAction_request_newFrameName_decisionListener(
            &self,
            web_view: Option<&WebView>,
            action_information: Option<&NSDictionary>,
            request: Option<&NSURLRequest>,
            frame_name: Option<&NSString>,
            listener: Option<&ProtocolObject<dyn WebPolicyDecisionListener>>,
        );

        #[cfg(all(feature = "WebFrame", feature = "WebView", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        /// Returns the policy for content which has been partially loaded.
        /// Sent after webView:didStartProvisionalLoadForFrame: is sent on the WebFrameLoadDelegate.
        ///
        /// Parameter `type`: MIME type for the resource.
        ///
        /// Parameter `request`: A NSURLRequest for the partially loaded content.
        ///
        /// Parameter `frame`: The frame which is loading the URL.
        ///
        /// Parameter `listener`: The object to call when the decision is made
        #[deprecated]
        #[optional]
        #[unsafe(method(webView:decidePolicyForMIMEType:request:frame:decisionListener:))]
        #[unsafe(method_family = none)]
        unsafe fn webView_decidePolicyForMIMEType_request_frame_decisionListener(
            &self,
            web_view: Option<&WebView>,
            r#type: Option<&NSString>,
            request: Option<&NSURLRequest>,
            frame: Option<&WebFrame>,
            listener: Option<&ProtocolObject<dyn WebPolicyDecisionListener>>,
        );

        #[cfg(all(feature = "WebFrame", feature = "WebView", feature = "objc2-app-kit"))]
        #[cfg(target_os = "macos")]
        /// Called when a WebPolicy could not be implemented. It is up to the client to display appropriate feedback.
        ///
        /// Parameter `error`: The error that caused the policy to not be implemented.
        ///
        /// Parameter `frame`: The frame in which the policy could not be implemented.
        #[deprecated]
        #[optional]
        #[unsafe(method(webView:unableToImplementPolicyWithError:frame:))]
        #[unsafe(method_family = none)]
        unsafe fn webView_unableToImplementPolicyWithError_frame(
            &self,
            web_view: Option<&WebView>,
            error: Option<&NSError>,
            frame: Option<&WebFrame>,
        );
    }
);
