//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "NSString")]
pub type NSHTTPCookiePropertyKey = NSString;

// NS_TYPED_ENUM
#[cfg(feature = "NSString")]
pub type NSHTTPCookieStringPolicy = NSString;

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieName: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieValue: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieOriginURL: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieVersion: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieDomain: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookiePath: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieSecure: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieExpires: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieComment: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieCommentURL: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieDiscard: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieMaximumAge: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookiePort: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieSameSitePolicy: &'static NSHTTPCookiePropertyKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieSameSiteLax: &'static NSHTTPCookieStringPolicy;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSHTTPCookieSameSiteStrict: &'static NSHTTPCookieStringPolicy;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSHTTPCookie;

    unsafe impl ClassType for NSHTTPCookie {
        type Super = NSObject;
    }
);

unsafe impl Send for NSHTTPCookie {}

unsafe impl Sync for NSHTTPCookie {}

unsafe impl NSObjectProtocol for NSHTTPCookie {}

extern_methods!(
    unsafe impl NSHTTPCookie {
        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Init initWithProperties:)]
        pub unsafe fn initWithProperties(
            this: Allocated<Self>,
            properties: &NSDictionary<NSHTTPCookiePropertyKey, AnyObject>,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other cookieWithProperties:)]
        pub unsafe fn cookieWithProperties(
            properties: &NSDictionary<NSHTTPCookiePropertyKey, AnyObject>,
        ) -> Option<Retained<NSHTTPCookie>>;

        #[cfg(all(feature = "NSArray", feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other requestHeaderFieldsWithCookies:)]
        pub unsafe fn requestHeaderFieldsWithCookies(
            cookies: &NSArray<NSHTTPCookie>,
        ) -> Retained<NSDictionary<NSString, NSString>>;

        #[cfg(all(
            feature = "NSArray",
            feature = "NSDictionary",
            feature = "NSString",
            feature = "NSURL"
        ))]
        #[method_id(@__retain_semantics Other cookiesWithResponseHeaderFields:forURL:)]
        pub unsafe fn cookiesWithResponseHeaderFields_forURL(
            header_fields: &NSDictionary<NSString, NSString>,
            url: &NSURL,
        ) -> Retained<NSArray<NSHTTPCookie>>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other properties)]
        pub unsafe fn properties(
            &self,
        ) -> Option<Retained<NSDictionary<NSHTTPCookiePropertyKey, AnyObject>>>;

        #[method(version)]
        pub unsafe fn version(&self) -> NSUInteger;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Retained<NSString>;

        #[cfg(feature = "NSDate")]
        #[method_id(@__retain_semantics Other expiresDate)]
        pub unsafe fn expiresDate(&self) -> Option<Retained<NSDate>>;

        #[method(isSessionOnly)]
        pub unsafe fn isSessionOnly(&self) -> bool;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other domain)]
        pub unsafe fn domain(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other path)]
        pub unsafe fn path(&self) -> Retained<NSString>;

        #[method(isSecure)]
        pub unsafe fn isSecure(&self) -> bool;

        #[method(isHTTPOnly)]
        pub unsafe fn isHTTPOnly(&self) -> bool;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other comment)]
        pub unsafe fn comment(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Other commentURL)]
        pub unsafe fn commentURL(&self) -> Option<Retained<NSURL>>;

        #[cfg(all(feature = "NSArray", feature = "NSValue"))]
        #[method_id(@__retain_semantics Other portList)]
        pub unsafe fn portList(&self) -> Option<Retained<NSArray<NSNumber>>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other sameSitePolicy)]
        pub unsafe fn sameSitePolicy(&self) -> Option<Retained<NSHTTPCookieStringPolicy>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSHTTPCookie {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
