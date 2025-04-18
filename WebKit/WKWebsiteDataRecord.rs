//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkwebsitedatatypefetchcache?language=objc)
    pub static WKWebsiteDataTypeFetchCache: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkwebsitedatatypediskcache?language=objc)
    pub static WKWebsiteDataTypeDiskCache: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkwebsitedatatypememorycache?language=objc)
    pub static WKWebsiteDataTypeMemoryCache: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkwebsitedatatypeofflinewebapplicationcache?language=objc)
    pub static WKWebsiteDataTypeOfflineWebApplicationCache: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkwebsitedatatypecookies?language=objc)
    pub static WKWebsiteDataTypeCookies: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkwebsitedatatypesessionstorage?language=objc)
    pub static WKWebsiteDataTypeSessionStorage: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkwebsitedatatypelocalstorage?language=objc)
    pub static WKWebsiteDataTypeLocalStorage: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkwebsitedatatypewebsqldatabases?language=objc)
    pub static WKWebsiteDataTypeWebSQLDatabases: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkwebsitedatatypeindexeddbdatabases?language=objc)
    pub static WKWebsiteDataTypeIndexedDBDatabases: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkwebsitedatatypeserviceworkerregistrations?language=objc)
    pub static WKWebsiteDataTypeServiceWorkerRegistrations: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkwebsitedatatypefilesystem?language=objc)
    pub static WKWebsiteDataTypeFileSystem: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkwebsitedatatypesearchfieldrecentsearches?language=objc)
    pub static WKWebsiteDataTypeSearchFieldRecentSearches: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkwebsitedatatypemediakeys?language=objc)
    pub static WKWebsiteDataTypeMediaKeys: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/wkwebsitedatatypehashsalt?language=objc)
    pub static WKWebsiteDataTypeHashSalt: &'static NSString;
}

extern_class!(
    /// A WKWebsiteDataRecord represents website data, grouped by domain name using the public suffix list.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/webkit/wkwebsitedatarecord?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKWebsiteDataRecord;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for WKWebsiteDataRecord {}
);

impl WKWebsiteDataRecord {
    extern_methods!(
        /// The display name for the data record. This is usually the domain name.
        #[unsafe(method(displayName))]
        #[unsafe(method_family = none)]
        pub unsafe fn displayName(&self) -> Retained<NSString>;

        /// The various types of website data that exist for this data record.
        #[unsafe(method(dataTypes))]
        #[unsafe(method_family = none)]
        pub unsafe fn dataTypes(&self) -> Retained<NSSet<NSString>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl WKWebsiteDataRecord {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
