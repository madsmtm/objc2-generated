//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnetserviceserrorcode?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSNetServicesErrorCode: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnetserviceserrordomain?language=objc)
    #[cfg(all(feature = "NSError", feature = "NSString"))]
    pub static NSNetServicesErrorDomain: &'static NSErrorDomain;
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnetserviceserror?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSNetServicesError(pub NSInteger);
impl NSNetServicesError {
    #[doc(alias = "NSNetServicesUnknownError")]
    pub const UnknownError: Self = Self(-72000);
    #[doc(alias = "NSNetServicesCollisionError")]
    pub const CollisionError: Self = Self(-72001);
    #[doc(alias = "NSNetServicesNotFoundError")]
    pub const NotFoundError: Self = Self(-72002);
    #[doc(alias = "NSNetServicesActivityInProgress")]
    pub const ActivityInProgress: Self = Self(-72003);
    #[doc(alias = "NSNetServicesBadArgumentError")]
    pub const BadArgumentError: Self = Self(-72004);
    #[doc(alias = "NSNetServicesCancelledError")]
    pub const CancelledError: Self = Self(-72005);
    #[doc(alias = "NSNetServicesInvalidError")]
    pub const InvalidError: Self = Self(-72006);
    #[doc(alias = "NSNetServicesTimeoutError")]
    pub const TimeoutError: Self = Self(-72007);
    #[doc(alias = "NSNetServicesMissingRequiredConfigurationError")]
    pub const MissingRequiredConfigurationError: Self = Self(-72008);
}

unsafe impl Encode for NSNetServicesError {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSNetServicesError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnetserviceoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSNetServiceOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSNetServiceOptions: NSUInteger {
        #[doc(alias = "NSNetServiceNoAutoRename")]
        const NoAutoRename = 1<<0;
        #[doc(alias = "NSNetServiceListenForConnections")]
        const ListenForConnections = 1<<1;
    }
}

unsafe impl Encode for NSNetServiceOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSNetServiceOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnetservice?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
    pub struct NSNetService;
);

unsafe impl NSObjectProtocol for NSNetService {}

extern_methods!(
    unsafe impl NSNetService {
        #[cfg(feature = "NSString")]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[unsafe(method_family(init))]
        #[method_id(initWithDomain:type:name:port:)]
        pub unsafe fn initWithDomain_type_name_port(
            this: Allocated<Self>,
            domain: &NSString,
            r#type: &NSString,
            name: &NSString,
            port: c_int,
        ) -> Retained<Self>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[unsafe(method_family(init))]
        #[method_id(initWithDomain:type:name:)]
        pub unsafe fn initWithDomain_type_name(
            this: Allocated<Self>,
            domain: &NSString,
            r#type: &NSString,
            name: &NSString,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSObjCRuntime", feature = "NSRunLoop", feature = "NSString"))]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method(scheduleInRunLoop:forMode:)]
        pub unsafe fn scheduleInRunLoop_forMode(
            &self,
            a_run_loop: &NSRunLoop,
            mode: &NSRunLoopMode,
        );

        #[cfg(all(feature = "NSObjCRuntime", feature = "NSRunLoop", feature = "NSString"))]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method(removeFromRunLoop:forMode:)]
        pub unsafe fn removeFromRunLoop_forMode(
            &self,
            a_run_loop: &NSRunLoop,
            mode: &NSRunLoopMode,
        );

        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[unsafe(method_family(none))]
        #[method_id(delegate)]
        pub unsafe fn delegate(&self)
            -> Option<Retained<ProtocolObject<dyn NSNetServiceDelegate>>>;

        /// Setter for [`delegate`][Self::delegate].
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSNetServiceDelegate>>,
        );

        #[method(includesPeerToPeer)]
        pub unsafe fn includesPeerToPeer(&self) -> bool;

        /// Setter for [`includesPeerToPeer`][Self::includesPeerToPeer].
        #[method(setIncludesPeerToPeer:)]
        pub unsafe fn setIncludesPeerToPeer(&self, includes_peer_to_peer: bool);

        #[cfg(feature = "NSString")]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[unsafe(method_family(none))]
        #[method_id(name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[unsafe(method_family(none))]
        #[method_id(type)]
        pub unsafe fn r#type(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[unsafe(method_family(none))]
        #[method_id(domain)]
        pub unsafe fn domain(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[unsafe(method_family(none))]
        #[method_id(hostName)]
        pub unsafe fn hostName(&self) -> Option<Retained<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSData"))]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[unsafe(method_family(none))]
        #[method_id(addresses)]
        pub unsafe fn addresses(&self) -> Option<Retained<NSArray<NSData>>>;

        #[method(port)]
        pub unsafe fn port(&self) -> NSInteger;

        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method(publish)]
        pub unsafe fn publish(&self);

        #[method(publishWithOptions:)]
        pub unsafe fn publishWithOptions(&self, options: NSNetServiceOptions);

        #[deprecated = "Not supported"]
        #[method(resolve)]
        pub unsafe fn resolve(&self);

        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method(stop)]
        pub unsafe fn stop(&self);

        #[cfg(all(feature = "NSData", feature = "NSDictionary", feature = "NSString"))]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[unsafe(method_family(none))]
        #[method_id(dictionaryFromTXTRecordData:)]
        pub unsafe fn dictionaryFromTXTRecordData(
            txt_data: &NSData,
        ) -> Retained<NSDictionary<NSString, NSData>>;

        #[cfg(all(feature = "NSData", feature = "NSDictionary", feature = "NSString"))]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[unsafe(method_family(none))]
        #[method_id(dataFromTXTRecordDictionary:)]
        pub unsafe fn dataFromTXTRecordDictionary(
            txt_dictionary: &NSDictionary<NSString, NSData>,
        ) -> Retained<NSData>;

        #[cfg(feature = "NSDate")]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method(resolveWithTimeout:)]
        pub unsafe fn resolveWithTimeout(&self, timeout: NSTimeInterval);

        #[cfg(feature = "NSStream")]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method(getInputStream:outputStream:)]
        pub unsafe fn getInputStream_outputStream(
            &self,
            input_stream: *mut *mut NSInputStream,
            output_stream: *mut *mut NSOutputStream,
        ) -> bool;

        #[cfg(feature = "NSData")]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method(setTXTRecordData:)]
        pub unsafe fn setTXTRecordData(&self, record_data: Option<&NSData>) -> bool;

        #[cfg(feature = "NSData")]
        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[unsafe(method_family(none))]
        #[method_id(TXTRecordData)]
        pub unsafe fn TXTRecordData(&self) -> Option<Retained<NSData>>;

        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method(startMonitoring)]
        pub unsafe fn startMonitoring(&self);

        #[deprecated = "Use nw_connection_t or nw_listener_t in Network framework instead"]
        #[method(stopMonitoring)]
        pub unsafe fn stopMonitoring(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSNetService {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnetservicebrowser?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use nw_browser_t in Network framework instead"]
    pub struct NSNetServiceBrowser;
);

unsafe impl NSObjectProtocol for NSNetServiceBrowser {}

extern_methods!(
    unsafe impl NSNetServiceBrowser {
        #[deprecated = "Use nw_browser_t in Network framework instead"]
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[deprecated = "Use nw_browser_t in Network framework instead"]
        #[unsafe(method_family(none))]
        #[method_id(delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSNetServiceBrowserDelegate>>>;

        /// Setter for [`delegate`][Self::delegate].
        #[deprecated = "Use nw_browser_t in Network framework instead"]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSNetServiceBrowserDelegate>>,
        );

        #[method(includesPeerToPeer)]
        pub unsafe fn includesPeerToPeer(&self) -> bool;

        /// Setter for [`includesPeerToPeer`][Self::includesPeerToPeer].
        #[method(setIncludesPeerToPeer:)]
        pub unsafe fn setIncludesPeerToPeer(&self, includes_peer_to_peer: bool);

        #[cfg(all(feature = "NSObjCRuntime", feature = "NSRunLoop", feature = "NSString"))]
        #[deprecated = "Use nw_browser_t in Network framework instead"]
        #[method(scheduleInRunLoop:forMode:)]
        pub unsafe fn scheduleInRunLoop_forMode(
            &self,
            a_run_loop: &NSRunLoop,
            mode: &NSRunLoopMode,
        );

        #[cfg(all(feature = "NSObjCRuntime", feature = "NSRunLoop", feature = "NSString"))]
        #[deprecated = "Use nw_browser_t in Network framework instead"]
        #[method(removeFromRunLoop:forMode:)]
        pub unsafe fn removeFromRunLoop_forMode(
            &self,
            a_run_loop: &NSRunLoop,
            mode: &NSRunLoopMode,
        );

        #[deprecated = "Use nw_browser_t in Network framework instead"]
        #[method(searchForBrowsableDomains)]
        pub unsafe fn searchForBrowsableDomains(&self);

        #[deprecated = "Use nw_browser_t in Network framework instead"]
        #[method(searchForRegistrationDomains)]
        pub unsafe fn searchForRegistrationDomains(&self);

        #[cfg(feature = "NSString")]
        #[deprecated = "Use nw_browser_t in Network framework instead"]
        #[method(searchForServicesOfType:inDomain:)]
        pub unsafe fn searchForServicesOfType_inDomain(
            &self,
            r#type: &NSString,
            domain_string: &NSString,
        );

        #[deprecated = "Use nw_browser_t in Network framework instead"]
        #[method(stop)]
        pub unsafe fn stop(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSNetServiceBrowser {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnetservicedelegate?language=objc)
    pub unsafe trait NSNetServiceDelegate: NSObjectProtocol {
        #[optional]
        #[method(netServiceWillPublish:)]
        unsafe fn netServiceWillPublish(&self, sender: &NSNetService);

        #[optional]
        #[method(netServiceDidPublish:)]
        unsafe fn netServiceDidPublish(&self, sender: &NSNetService);

        #[cfg(all(feature = "NSDictionary", feature = "NSString", feature = "NSValue"))]
        #[optional]
        #[method(netService:didNotPublish:)]
        unsafe fn netService_didNotPublish(
            &self,
            sender: &NSNetService,
            error_dict: &NSDictionary<NSString, NSNumber>,
        );

        #[optional]
        #[method(netServiceWillResolve:)]
        unsafe fn netServiceWillResolve(&self, sender: &NSNetService);

        #[optional]
        #[method(netServiceDidResolveAddress:)]
        unsafe fn netServiceDidResolveAddress(&self, sender: &NSNetService);

        #[cfg(all(feature = "NSDictionary", feature = "NSString", feature = "NSValue"))]
        #[optional]
        #[method(netService:didNotResolve:)]
        unsafe fn netService_didNotResolve(
            &self,
            sender: &NSNetService,
            error_dict: &NSDictionary<NSString, NSNumber>,
        );

        #[optional]
        #[method(netServiceDidStop:)]
        unsafe fn netServiceDidStop(&self, sender: &NSNetService);

        #[cfg(feature = "NSData")]
        #[optional]
        #[method(netService:didUpdateTXTRecordData:)]
        unsafe fn netService_didUpdateTXTRecordData(&self, sender: &NSNetService, data: &NSData);

        #[cfg(feature = "NSStream")]
        #[optional]
        #[method(netService:didAcceptConnectionWithInputStream:outputStream:)]
        unsafe fn netService_didAcceptConnectionWithInputStream_outputStream(
            &self,
            sender: &NSNetService,
            input_stream: &NSInputStream,
            output_stream: &NSOutputStream,
        );
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnetservicebrowserdelegate?language=objc)
    pub unsafe trait NSNetServiceBrowserDelegate: NSObjectProtocol {
        #[optional]
        #[method(netServiceBrowserWillSearch:)]
        unsafe fn netServiceBrowserWillSearch(&self, browser: &NSNetServiceBrowser);

        #[optional]
        #[method(netServiceBrowserDidStopSearch:)]
        unsafe fn netServiceBrowserDidStopSearch(&self, browser: &NSNetServiceBrowser);

        #[cfg(all(feature = "NSDictionary", feature = "NSString", feature = "NSValue"))]
        #[optional]
        #[method(netServiceBrowser:didNotSearch:)]
        unsafe fn netServiceBrowser_didNotSearch(
            &self,
            browser: &NSNetServiceBrowser,
            error_dict: &NSDictionary<NSString, NSNumber>,
        );

        #[cfg(feature = "NSString")]
        #[optional]
        #[method(netServiceBrowser:didFindDomain:moreComing:)]
        unsafe fn netServiceBrowser_didFindDomain_moreComing(
            &self,
            browser: &NSNetServiceBrowser,
            domain_string: &NSString,
            more_coming: bool,
        );

        #[optional]
        #[method(netServiceBrowser:didFindService:moreComing:)]
        unsafe fn netServiceBrowser_didFindService_moreComing(
            &self,
            browser: &NSNetServiceBrowser,
            service: &NSNetService,
            more_coming: bool,
        );

        #[cfg(feature = "NSString")]
        #[optional]
        #[method(netServiceBrowser:didRemoveDomain:moreComing:)]
        unsafe fn netServiceBrowser_didRemoveDomain_moreComing(
            &self,
            browser: &NSNetServiceBrowser,
            domain_string: &NSString,
            more_coming: bool,
        );

        #[optional]
        #[method(netServiceBrowser:didRemoveService:moreComing:)]
        unsafe fn netServiceBrowser_didRemoveService_moreComing(
            &self,
            browser: &NSNetServiceBrowser,
            service: &NSNetService,
            more_coming: bool,
        );
    }
);
