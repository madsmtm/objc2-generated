//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetresourceloader?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetResourceLoader;
);

unsafe impl NSObjectProtocol for AVAssetResourceLoader {}

extern_methods!(
    unsafe impl AVAssetResourceLoader {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        /// The receiver's delegate.
        ///
        /// The value of this property is an object conforming to the AVAssetResourceLoaderDelegate protocol. The delegate is set using the setDelegate:queue: method. The delegate is held using a zeroing-weak reference, so this property will have a value of nil after a delegate that was previously set has been deallocated.
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn AVAssetResourceLoaderDelegate>>>;
    }
);

extern_protocol!(
    /// The AVAssetResourceLoaderDelegate protocol defines methods that allow your code to handle resource loading requests coming from an AVURLAsset.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetresourceloaderdelegate?language=objc)
    pub unsafe trait AVAssetResourceLoaderDelegate: NSObjectProtocol {
        /// Invoked when assistance is required of the application to load a resource.
        ///
        /// Parameter `resourceLoader`: The instance of AVAssetResourceLoader for which the loading request is being made.
        ///
        /// Parameter `loadingRequest`: An instance of AVAssetResourceLoadingRequest that provides information about the requested resource.
        ///
        /// Returns: YES if the delegate can load the resource indicated by the AVAssetResourceLoadingRequest; otherwise NO.
        ///
        /// Delegates receive this message when assistance is required of the application to load a resource. For example, this method is invoked to load decryption keys that have been specified using custom URL schemes.
        /// If the result is YES, the resource loader expects invocation, either subsequently or immediately, of either -[AVAssetResourceLoadingRequest finishLoading] or -[AVAssetResourceLoadingRequest finishLoadingWithError:]. If you intend to finish loading the resource after your handling of this message returns, you must retain the instance of AVAssetResourceLoadingRequest until after loading is finished.
        /// If the result is NO, the resource loader treats the loading of the resource as having failed.
        /// Note that if the delegate's implementation of -resourceLoader:shouldWaitForLoadingOfRequestedResource: returns YES without finishing the loading request immediately, it may be invoked again with another loading request before the prior request is finished; therefore in such cases the delegate should be prepared to manage multiple loading requests.
        ///
        /// If an AVURLAsset is added to an AVContentKeySession object and a delegate is set on its AVAssetResourceLoader, that delegate's resourceLoader:shouldWaitForLoadingOfRequestedResource: method must specify which custom URL requests should be handled as content keys. This is done by returning YES and passing either AVStreamingKeyDeliveryPersistentContentKeyType or AVStreamingKeyDeliveryContentKeyType into -[AVAssetResourceLoadingContentInformationRequest setContentType:] and then calling -[AVAssetResourceLoadingRequest finishLoading].
        #[optional]
        #[method(resourceLoader:shouldWaitForLoadingOfRequestedResource:)]
        unsafe fn resourceLoader_shouldWaitForLoadingOfRequestedResource(
            &self,
            resource_loader: &AVAssetResourceLoader,
            loading_request: &AVAssetResourceLoadingRequest,
        ) -> bool;

        /// Invoked when assistance is required of the application to renew a resource.
        ///
        /// Parameter `resourceLoader`: The instance of AVAssetResourceLoader for which the loading request is being made.
        ///
        /// Parameter `renewalRequest`: An instance of AVAssetResourceRenewalRequest that provides information about the requested resource.
        ///
        /// Returns: YES if the delegate can renew the resource indicated by the AVAssetResourceLoadingRequest; otherwise NO.
        ///
        /// Delegates receive this message when assistance is required of the application to renew a resource previously loaded by resourceLoader:shouldWaitForLoadingOfRequestedResource:. For example, this method is invoked to renew decryption keys that require renewal, as indicated in a response to a prior invocation of resourceLoader:shouldWaitForLoadingOfRequestedResource:.
        /// If the result is YES, the resource loader expects invocation, either subsequently or immediately, of either -[AVAssetResourceRenewalRequest finishLoading] or -[AVAssetResourceRenewalRequest finishLoadingWithError:]. If you intend to finish loading the resource after your handling of this message returns, you must retain the instance of AVAssetResourceRenewalRequest until after loading is finished.
        /// If the result is NO, the resource loader treats the loading of the resource as having failed.
        /// Note that if the delegate's implementation of -resourceLoader:shouldWaitForRenewalOfRequestedResource: returns YES without finishing the loading request immediately, it may be invoked again with another loading request before the prior request is finished; therefore in such cases the delegate should be prepared to manage multiple loading requests.
        ///
        /// If an AVURLAsset is added to an AVContentKeySession object and a delegate is set on its AVAssetResourceLoader, that delegate's resourceLoader:shouldWaitForRenewalOfRequestedResource:renewalRequest method must specify which custom URL requests should be handled as content keys. This is done by returning YES and passing either AVStreamingKeyDeliveryPersistentContentKeyType or AVStreamingKeyDeliveryContentKeyType into -[AVAssetResourceLoadingContentInformationRequest setContentType:] and then calling -[AVAssetResourceLoadingRequest finishLoading].
        #[optional]
        #[method(resourceLoader:shouldWaitForRenewalOfRequestedResource:)]
        unsafe fn resourceLoader_shouldWaitForRenewalOfRequestedResource(
            &self,
            resource_loader: &AVAssetResourceLoader,
            renewal_request: &AVAssetResourceRenewalRequest,
        ) -> bool;

        /// Informs the delegate that a prior loading request has been cancelled.
        ///
        /// Parameter `loadingRequest`: The loading request that has been cancelled.
        ///
        /// Previously issued loading requests can be cancelled when data from the resource is no longer required or when a loading request is superseded by new requests for data from the same resource. For example, if to complete a seek operation it becomes necessary to load a range of bytes that's different from a range previously requested, the prior request may be cancelled while the delegate is still handling it.
        #[optional]
        #[method(resourceLoader:didCancelLoadingRequest:)]
        unsafe fn resourceLoader_didCancelLoadingRequest(
            &self,
            resource_loader: &AVAssetResourceLoader,
            loading_request: &AVAssetResourceLoadingRequest,
        );

        /// Invoked when assistance is required of the application to respond to an authentication challenge.
        ///
        /// Parameter `resourceLoader`: The instance of AVAssetResourceLoader asking for help with an authentication challenge.
        ///
        /// Parameter `authenticationChallenge`: An instance of NSURLAuthenticationChallenge.
        ///
        /// Delegates receive this message when assistance is required of the application to respond to an authentication challenge.
        /// If the result is YES, the resource loader expects you to send an appropriate response, either subsequently or immediately, to the NSURLAuthenticationChallenge's sender, i.e. [authenticationChallenge sender], via use of one of the messages defined in the NSURLAuthenticationChallengeSender protocol (see NSAuthenticationChallenge.h). If you intend to respond to the authentication challenge after your handling of -resourceLoader:shouldWaitForResponseToAuthenticationChallenge: returns, you must retain the instance of NSURLAuthenticationChallenge until after your response has been made.
        #[optional]
        #[method(resourceLoader:shouldWaitForResponseToAuthenticationChallenge:)]
        unsafe fn resourceLoader_shouldWaitForResponseToAuthenticationChallenge(
            &self,
            resource_loader: &AVAssetResourceLoader,
            authentication_challenge: &NSURLAuthenticationChallenge,
        ) -> bool;

        /// Informs the delegate that a prior authentication challenge has been cancelled.
        ///
        /// Parameter `authenticationChallenge`: The authentication challenge that has been cancelled.
        #[optional]
        #[method(resourceLoader:didCancelAuthenticationChallenge:)]
        unsafe fn resourceLoader_didCancelAuthenticationChallenge(
            &self,
            resource_loader: &AVAssetResourceLoader,
            authentication_challenge: &NSURLAuthenticationChallenge,
        );
    }
);

extern_class!(
    /// AVAssetResourceLoadingRequestor represents the originator of loading request
    ///
    ///
    /// Information about the originator of a loading request, in order to decide whether or how to fulfill the request.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetresourceloadingrequestor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetResourceLoadingRequestor;
);

unsafe impl NSObjectProtocol for AVAssetResourceLoadingRequestor {}

extern_methods!(
    unsafe impl AVAssetResourceLoadingRequestor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        /// Whether the requestor provides expired session reports (see AVContentKeySession)
        #[method(providesExpiredSessionReports)]
        pub unsafe fn providesExpiredSessionReports(&self) -> bool;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetresourceloadingrequest?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetResourceLoadingRequest;
);

unsafe impl NSObjectProtocol for AVAssetResourceLoadingRequest {}

extern_methods!(
    unsafe impl AVAssetResourceLoadingRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        /// An NSURLRequest for the requested resource.
        #[method_id(@__retain_semantics Other request)]
        pub unsafe fn request(&self) -> Retained<NSURLRequest>;

        /// Indicates whether loading of the resource has been finished.
        ///
        /// The value of this property becomes YES only in response to an invocation of either -finishLoading or -finishLoadingWithError:.
        #[method(isFinished)]
        pub unsafe fn isFinished(&self) -> bool;

        /// Indicates whether the request has been cancelled.
        ///
        /// The value of this property becomes YES when the resource loader cancels the loading of a request, just prior to sending the message -resourceLoader:didCancelLoadingRequest: to its delegate.
        #[method(isCancelled)]
        pub unsafe fn isCancelled(&self) -> bool;

        /// An instance of AVAssetResourceLoadingContentInformationRequest that you must populate with information about the resource before responding to any AVAssetResourceLoadingDataRequests for the resource.  The value of this property will be nil if no such information is being requested.
        #[method_id(@__retain_semantics Other contentInformationRequest)]
        pub unsafe fn contentInformationRequest(
            &self,
        ) -> Option<Retained<AVAssetResourceLoadingContentInformationRequest>>;

        /// An instance of AVAssetResourceLoadingDataRequest that indicates the range of resource data that's being requested.  If an AVAssetResourceLoadingContentInformationRequest has been provided, you must set its properties appropriately before responding to any AVAssetResourceLoadingDataRequests.  The value of this property will be nil if no data is being requested.
        #[method_id(@__retain_semantics Other dataRequest)]
        pub unsafe fn dataRequest(&self) -> Option<Retained<AVAssetResourceLoadingDataRequest>>;

        /// Set the value of this property to an instance of NSURLResponse indicating a response to the loading request. If no response is needed, leave the value of this property set to nil.
        #[method_id(@__retain_semantics Other response)]
        pub unsafe fn response(&self) -> Option<Retained<NSURLResponse>>;

        /// Setter for [`response`][Self::response].
        #[method(setResponse:)]
        pub unsafe fn setResponse(&self, response: Option<&NSURLResponse>);

        /// Set the value of this property to an instance of NSURLRequest indicating a redirection of the loading request to another URL. If no redirection is needed, leave the value of this property set to nil.
        ///
        /// AVAssetResourceLoader supports redirects to HTTP URLs only. Redirects to other URLs will result in a loading failure.
        #[method_id(@__retain_semantics Other redirect)]
        pub unsafe fn redirect(&self) -> Option<Retained<NSURLRequest>>;

        /// Setter for [`redirect`][Self::redirect].
        #[method(setRedirect:)]
        pub unsafe fn setRedirect(&self, redirect: Option<&NSURLRequest>);

        /// The AVAssetResourceLoadingRequestor that made this request
        #[method_id(@__retain_semantics Other requestor)]
        pub unsafe fn requestor(&self) -> Retained<AVAssetResourceLoadingRequestor>;

        /// Causes the receiver to treat the processing of the request as complete.
        ///
        /// If a dataRequest is present, and the resource does not contain the full extent of the data that has been requested according to the values of the requestedOffset and requestedLength properties of the dataRequest, or if requestsAllDataToEndOfResource has a value of YES, -finishLoading may be invoked after providing as much of the requested data as the resource contains. If the contentInformationRequest property is not nil and specifies a non-empty allowedContentTypes array, the contentInformationRequest's contentType property must be set to a value within allowedContentTypes. Otherwise, this method will throw an exception.
        #[method(finishLoading)]
        pub unsafe fn finishLoading(&self);

        /// Causes the receiver to treat the request as having failed.
        ///
        /// Parameter `error`: An instance of NSError indicating the reason for failure.
        #[method(finishLoadingWithError:)]
        pub unsafe fn finishLoadingWithError(&self, error: Option<&NSError>);
    }
);

extern_class!(
    /// AVAssetResourceRenewalRequest encapsulates information about a resource request issued by a resource loader for the purpose of renewing a request previously issued.
    ///
    ///
    /// When an AVURLAsset needs to renew a resource (because contentInformationRequest.renewalDate has been set on a previous loading request), it asks its AVAssetResourceLoader object to assist. The resource loader encapsulates the request information by creating an instance of this object, which it then hands to its delegate for processing. The delegate uses the information in this object to perform the request and report on the success or failure of the operation.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetresourcerenewalrequest?language=objc)
    #[unsafe(super(AVAssetResourceLoadingRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetResourceRenewalRequest;
);

unsafe impl NSObjectProtocol for AVAssetResourceRenewalRequest {}

extern_methods!(
    unsafe impl AVAssetResourceRenewalRequest {}
);

extern_methods!(
    /// Methods declared on superclass `AVAssetResourceLoadingRequest`
    unsafe impl AVAssetResourceRenewalRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetresourceloadingcontentinformationrequest?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetResourceLoadingContentInformationRequest;
);

unsafe impl NSObjectProtocol for AVAssetResourceLoadingContentInformationRequest {}

extern_methods!(
    unsafe impl AVAssetResourceLoadingContentInformationRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        /// A UTI that indicates the type of data contained by the requested resource.
        ///
        /// Before you finish loading an AVAssetResourceLoadingRequest, if its contentInformationRequest is not nil, you should set the value of this property to a UTI indicating the type of data contained by the requested resource.
        #[method_id(@__retain_semantics Other contentType)]
        pub unsafe fn contentType(&self) -> Option<Retained<NSString>>;

        /// Setter for [`contentType`][Self::contentType].
        #[method(setContentType:)]
        pub unsafe fn setContentType(&self, content_type: Option<&NSString>);

        /// An array showing the types of data which will be accepted as a valid response for the requested resource.
        ///
        /// If an AVAssetResourceLoadingRequest's contentInformationRequest is not nil, ensure that the value assigned to the contentType property is in this array. Otherwise, calling -finishLoading on the associated request will result in an exception.
        #[method_id(@__retain_semantics Other allowedContentTypes)]
        pub unsafe fn allowedContentTypes(&self) -> Option<Retained<NSArray<NSString>>>;

        /// Indicates the length of the requested resource, in bytes.
        ///
        /// Before you finish loading an AVAssetResourceLoadingRequest, if its contentInformationRequest is not nil, you should set the value of this property to the number of bytes contained by the requested resource.
        #[method(contentLength)]
        pub unsafe fn contentLength(&self) -> c_longlong;

        /// Setter for [`contentLength`][Self::contentLength].
        #[method(setContentLength:)]
        pub unsafe fn setContentLength(&self, content_length: c_longlong);

        /// Indicates whether random access to arbitrary ranges of bytes of the resource is supported. Such support also allows portions of the resource to be requested more than once.
        ///
        /// Before you finish loading an AVAssetResourceLoadingRequest, if its contentInformationRequest is not nil, you should set the value of this property to YES if you support random access to arbitrary ranges of bytes of the resource. If you do not set this property to YES for resources that must be loaded incrementally, loading of the resource may fail. Such resources include anything that contains media data.
        #[method(isByteRangeAccessSupported)]
        pub unsafe fn isByteRangeAccessSupported(&self) -> bool;

        /// Setter for [`isByteRangeAccessSupported`][Self::isByteRangeAccessSupported].
        #[method(setByteRangeAccessSupported:)]
        pub unsafe fn setByteRangeAccessSupported(&self, byte_range_access_supported: bool);

        /// For resources that expire, the date at which a new AVAssetResourceLoadingRequest will be issued for a renewal of this resource, if the media system still requires it.
        ///
        /// Before you finish loading an AVAssetResourceLoadingRequest, if the resource is prone to expiry you should set the value of this property to the date at which a renewal should be triggered. This value should be set sufficiently early enough to allow an AVAssetResourceRenewalRequest, delivered to your delegate via -resourceLoader:shouldWaitForRenewalOfRequestedResource:, to finish before the actual expiry time. Otherwise media playback may fail.
        #[method_id(@__retain_semantics Other renewalDate)]
        pub unsafe fn renewalDate(&self) -> Option<Retained<NSDate>>;

        /// Setter for [`renewalDate`][Self::renewalDate].
        #[method(setRenewalDate:)]
        pub unsafe fn setRenewalDate(&self, renewal_date: Option<&NSDate>);

        /// Indicates whether asset data loading can expect data to be produced immediately.
        ///
        /// Before you finish loading an AVAssetResourceLoadingRequest, if its contentInformationRequest is not nil, you may set this property to YES to indicate that all asset data can be produced immediately, e.g., because the data is fully cached, or because the custom URL scheme ultimately refers to files on local storage. This allows significant data flow optimizations. For backward compatibility, this property defaults to NO.
        #[method(isEntireLengthAvailableOnDemand)]
        pub unsafe fn isEntireLengthAvailableOnDemand(&self) -> bool;

        /// Setter for [`isEntireLengthAvailableOnDemand`][Self::isEntireLengthAvailableOnDemand].
        #[method(setEntireLengthAvailableOnDemand:)]
        pub unsafe fn setEntireLengthAvailableOnDemand(
            &self,
            entire_length_available_on_demand: bool,
        );
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetresourceloadingdatarequest?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetResourceLoadingDataRequest;
);

unsafe impl NSObjectProtocol for AVAssetResourceLoadingDataRequest {}

extern_methods!(
    unsafe impl AVAssetResourceLoadingDataRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        /// The position within the resource of the first byte requested.
        #[method(requestedOffset)]
        pub unsafe fn requestedOffset(&self) -> c_longlong;

        /// The length of the data requested.
        ///
        /// Note that requestsAllDataToEndOfResource will be set to YES when the entire remaining length of the resource is being requested from requestedOffset to the end of the resource. This can occur even when the content length has not yet been reported by you via a prior finished loading request.
        /// When requestsAllDataToEndOfResource has a value of YES, you should disregard the value of requestedLength and incrementally provide as much data starting from the requestedOffset as the resource contains, until you have provided all of the available data successfully and invoked -finishLoading, until you have encountered a failure and invoked -finishLoadingWithError:, or until you have received -resourceLoader:didCancelLoadingRequest: for the AVAssetResourceLoadingRequest from which the AVAssetResourceLoadingDataRequest was obtained.
        /// When requestsAllDataToEndOfResource is YES and the content length has not yet been provided by you via a prior finished loading request, the value of requestedLength is set to NSIntegerMax. Starting in macOS 10.11 and iOS 9.0, in 32-bit applications requestedLength is also set to NSIntegerMax when all of the remaining resource data is being requested and the known length of the remaining data exceeds NSIntegerMax.
        #[method(requestedLength)]
        pub unsafe fn requestedLength(&self) -> NSInteger;

        /// Specifies that the entire remaining length of the resource from requestedOffset to the end of the resource is being requested.
        ///
        /// When requestsAllDataToEndOfResource has a value of YES, you should disregard the value of requestedLength and incrementally provide as much data starting from the requestedOffset as the resource contains, until you have provided all of the available data successfully and invoked -finishLoading, until you have encountered a failure and invoked -finishLoadingWithError:, or until you have received -resourceLoader:didCancelLoadingRequest: for the AVAssetResourceLoadingRequest from which the AVAssetResourceLoadingDataRequest was obtained.
        #[method(requestsAllDataToEndOfResource)]
        pub unsafe fn requestsAllDataToEndOfResource(&self) -> bool;

        /// The position within the resource of the next byte within the resource following the bytes that have already been provided via prior invocations of -respondWithData.
        #[method(currentOffset)]
        pub unsafe fn currentOffset(&self) -> c_longlong;

        /// Provides data to the receiver.
        ///
        /// Parameter `data`: An instance of NSData containing some or all of the requested bytes.
        ///
        /// May be invoked multiple times on the same instance of AVAssetResourceLoadingDataRequest to provide the full range of requested data incrementally. Upon each invocation, the value of currentOffset will be updated to accord with the amount of data provided.
        /// The instance of NSData that you provide may be retained for use in parsing or other processing for an indefinite period of time after this method returns. For this reason, if you are providing an instance of NSMutableData, you should avoid mutating it further after sharing its contents. If you are managing your own memory pool for I/O and resource loading, consider using -[NSData initWithBytesNoCopy:length:deallocator:] in order to receive notification of the earliest opportunity for safe recycling of the underlying memory.
        #[method(respondWithData:)]
        pub unsafe fn respondWithData(&self, data: &NSData);
    }
);

extern_methods!(
    /// AVAssetResourceLoaderContentKeySupport
    unsafe impl AVAssetResourceLoader {
        /// When YES, eligible content keys will be loaded as eagerly as possible, potentially handled by the delegate. Setting to YES may result in network activity.
        ///
        /// Any work done as a result of setting this property will be performed asynchronously.
        #[method(preloadsEligibleContentKeys)]
        pub unsafe fn preloadsEligibleContentKeys(&self) -> bool;

        /// Setter for [`preloadsEligibleContentKeys`][Self::preloadsEligibleContentKeys].
        #[method(setPreloadsEligibleContentKeys:)]
        pub unsafe fn setPreloadsEligibleContentKeys(&self, preloads_eligible_content_keys: bool);
    }
);

extern_methods!(
    /// AVAssetResourceLoaderCommonMediaClientDataSupport
    unsafe impl AVAssetResourceLoader {
        #[method(sendsCommonMediaClientDataAsHTTPHeaders)]
        pub unsafe fn sendsCommonMediaClientDataAsHTTPHeaders(&self) -> bool;

        /// Setter for [`sendsCommonMediaClientDataAsHTTPHeaders`][Self::sendsCommonMediaClientDataAsHTTPHeaders].
        #[method(setSendsCommonMediaClientDataAsHTTPHeaders:)]
        pub unsafe fn setSendsCommonMediaClientDataAsHTTPHeaders(
            &self,
            sends_common_media_client_data_as_http_headers: bool,
        );
    }
);

extern_methods!(
    /// AVAssetResourceLoadingRequestContentKeyRequestSupport
    unsafe impl AVAssetResourceLoadingRequest {
        /// Obtains a streaming content key request for a specific combination of application and content.
        ///
        /// Parameter `appIdentifier`: An opaque identifier for the application. The value of this identifier depends on the particular system used to provide the decryption key.
        ///
        /// Parameter `contentIdentifier`: An opaque identifier for the content. The value of this identifier depends on the particular system used to provide the decryption key.
        ///
        /// Parameter `options`: Additional information necessary to obtain the key, or nil if none.
        ///
        /// Parameter `outError`: If obtaining the streaming content key request fails, will be set to an instance of NSError describing the failure.
        ///
        /// Returns: The key request data that must be transmitted to the key vendor to obtain the content key.
        #[deprecated = "Use -[AVContentKeyRequest makeStreamingContentKeyRequestDataForApp:contentIdentifier:options:completionHandler:] instead"]
        #[method_id(@__retain_semantics Other streamingContentKeyRequestDataForApp:contentIdentifier:options:error:_)]
        pub unsafe fn streamingContentKeyRequestDataForApp_contentIdentifier_options_error(
            &self,
            app_identifier: &NSData,
            content_identifier: &NSData,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Result<Retained<NSData>, Retained<NSError>>;

        /// Obtains a persistable content key from a context.
        ///
        /// Parameter `keyVendorResponse`: The response returned from the key vendor as a result of a request generated from streamingContentKeyRequestDataForApp:contentIdentifier:options:error:.
        ///
        /// Parameter `options`: Additional information necessary to obtain the persistable content key, or nil if none.
        ///
        /// Parameter `outError`: If obtaining the persistable content key fails, will be set to an instance of NSError describing the failure.
        ///
        /// Returns: The persistable content key data that may be stored offline to answer future loading requests of the same content key.
        ///
        /// The data returned from this method may be used to immediately satisfy an AVAssetResourceLoadingDataRequest, as well as any subsequent requests for the same key url. The value of AVAssetResourceLoadingContentInformationRequest.contentType must be set to AVStreamingKeyDeliveryPersistentContentKeyType when responding with data created with this method.
        #[deprecated = "Use -[AVPersistableContentKeyRequest persistableContentKeyFromKeyVendorResponse:options:error:] instead"]
        #[method_id(@__retain_semantics Other persistentContentKeyFromKeyVendorResponse:options:error:_)]
        pub unsafe fn persistentContentKeyFromKeyVendorResponse_options_error(
            &self,
            key_vendor_response: &NSData,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Result<Retained<NSData>, Retained<NSError>>;
    }
);

extern "C" {
    /// Specifies whether the content key request should require a persistable key to be returned from the key vendor. Value should be a NSNumber created with +[NSNumber numberWithBool:].
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetresourceloadingrequeststreamingcontentkeyrequestrequirespersistentkey?language=objc)
    pub static AVAssetResourceLoadingRequestStreamingContentKeyRequestRequiresPersistentKey:
        &'static NSString;
}

extern_methods!(
    /// AVAssetResourceLoadingRequestDeprecated
    unsafe impl AVAssetResourceLoadingRequest {
        /// Causes the receiver to finish loading a resource that a delegate has previously assumed responsibility for loading by returning YES as the result of -resourceLoader:shouldWaitForLoadingOfRequestedResource:.
        ///
        /// Parameter `response`: The NSURLResponse for the NSURLRequest of the receiver. Should be nil if no response is required.
        ///
        /// Parameter `data`: An instance of NSData containing the data of the resource. Should be nil if no such data is available.
        ///
        /// Parameter `redirect`: An instance of NSURLRequest indicating a redirect of the loading request. Should be nil if no redirect is needed.
        ///
        /// This method is deprecated. Use the following methods instead.
        /// -[AVAssetResourceLoadingRequest setResponse:] to set the response property,
        /// -[AVAssetResourceLoadingRequest setRedirect:] to set the redirect property,
        /// -[AVAssetResourceLoadingDataRequest respondWithData:] to provide data, and
        /// -[AVAssetResourceLoadingRequest finishLoading] to indicate that loading is finished.
        #[deprecated = "Use -[AVAssetResourceLoadingRequest setResponse:], -[AVAssetResourceLoadingRequest setRedirect:], -[AVAssetResourceLoadingDataRequest respondWithData:], -[AVAssetResourceLoadingRequest finishLoading]"]
        #[method(finishLoadingWithResponse:data:redirect:)]
        pub unsafe fn finishLoadingWithResponse_data_redirect(
            &self,
            response: Option<&NSURLResponse>,
            data: Option<&NSData>,
            redirect: Option<&NSURLRequest>,
        );
    }
);