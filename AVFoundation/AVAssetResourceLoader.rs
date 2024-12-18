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

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn AVAssetResourceLoaderDelegate>>>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetresourceloaderdelegate?language=objc)
    pub unsafe trait AVAssetResourceLoaderDelegate: NSObjectProtocol {
        #[optional]
        #[method(resourceLoader:shouldWaitForLoadingOfRequestedResource:)]
        unsafe fn resourceLoader_shouldWaitForLoadingOfRequestedResource(
            &self,
            resource_loader: &AVAssetResourceLoader,
            loading_request: &AVAssetResourceLoadingRequest,
        ) -> bool;

        #[optional]
        #[method(resourceLoader:shouldWaitForRenewalOfRequestedResource:)]
        unsafe fn resourceLoader_shouldWaitForRenewalOfRequestedResource(
            &self,
            resource_loader: &AVAssetResourceLoader,
            renewal_request: &AVAssetResourceRenewalRequest,
        ) -> bool;

        #[optional]
        #[method(resourceLoader:didCancelLoadingRequest:)]
        unsafe fn resourceLoader_didCancelLoadingRequest(
            &self,
            resource_loader: &AVAssetResourceLoader,
            loading_request: &AVAssetResourceLoadingRequest,
        );

        #[optional]
        #[method(resourceLoader:shouldWaitForResponseToAuthenticationChallenge:)]
        unsafe fn resourceLoader_shouldWaitForResponseToAuthenticationChallenge(
            &self,
            resource_loader: &AVAssetResourceLoader,
            authentication_challenge: &NSURLAuthenticationChallenge,
        ) -> bool;

        #[optional]
        #[method(resourceLoader:didCancelAuthenticationChallenge:)]
        unsafe fn resourceLoader_didCancelAuthenticationChallenge(
            &self,
            resource_loader: &AVAssetResourceLoader,
            authentication_challenge: &NSURLAuthenticationChallenge,
        );
    }

    unsafe impl ProtocolType for dyn AVAssetResourceLoaderDelegate {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetresourceloadingrequestor?language=objc)
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

        #[method_id(@__retain_semantics Other request)]
        pub unsafe fn request(&self) -> Retained<NSURLRequest>;

        #[method(isFinished)]
        pub unsafe fn isFinished(&self) -> bool;

        #[method(isCancelled)]
        pub unsafe fn isCancelled(&self) -> bool;

        #[method_id(@__retain_semantics Other contentInformationRequest)]
        pub unsafe fn contentInformationRequest(
            &self,
        ) -> Option<Retained<AVAssetResourceLoadingContentInformationRequest>>;

        #[method_id(@__retain_semantics Other dataRequest)]
        pub unsafe fn dataRequest(&self) -> Option<Retained<AVAssetResourceLoadingDataRequest>>;

        #[method_id(@__retain_semantics Other response)]
        pub unsafe fn response(&self) -> Option<Retained<NSURLResponse>>;

        #[method(setResponse:)]
        pub unsafe fn setResponse(&self, response: Option<&NSURLResponse>);

        #[method_id(@__retain_semantics Other redirect)]
        pub unsafe fn redirect(&self) -> Option<Retained<NSURLRequest>>;

        #[method(setRedirect:)]
        pub unsafe fn setRedirect(&self, redirect: Option<&NSURLRequest>);

        #[method_id(@__retain_semantics Other requestor)]
        pub unsafe fn requestor(&self) -> Retained<AVAssetResourceLoadingRequestor>;

        #[method(finishLoading)]
        pub unsafe fn finishLoading(&self);

        #[method(finishLoadingWithError:)]
        pub unsafe fn finishLoadingWithError(&self, error: Option<&NSError>);
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetresourcerenewalrequest?language=objc)
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

        #[method_id(@__retain_semantics Other contentType)]
        pub unsafe fn contentType(&self) -> Option<Retained<NSString>>;

        #[method(setContentType:)]
        pub unsafe fn setContentType(&self, content_type: Option<&NSString>);

        #[method_id(@__retain_semantics Other allowedContentTypes)]
        pub unsafe fn allowedContentTypes(&self) -> Option<Retained<NSArray<NSString>>>;

        #[method(contentLength)]
        pub unsafe fn contentLength(&self) -> c_longlong;

        #[method(setContentLength:)]
        pub unsafe fn setContentLength(&self, content_length: c_longlong);

        #[method(isByteRangeAccessSupported)]
        pub unsafe fn isByteRangeAccessSupported(&self) -> bool;

        #[method(setByteRangeAccessSupported:)]
        pub unsafe fn setByteRangeAccessSupported(&self, byte_range_access_supported: bool);

        #[method_id(@__retain_semantics Other renewalDate)]
        pub unsafe fn renewalDate(&self) -> Option<Retained<NSDate>>;

        #[method(setRenewalDate:)]
        pub unsafe fn setRenewalDate(&self, renewal_date: Option<&NSDate>);

        #[method(isEntireLengthAvailableOnDemand)]
        pub unsafe fn isEntireLengthAvailableOnDemand(&self) -> bool;

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

        #[method(requestedOffset)]
        pub unsafe fn requestedOffset(&self) -> c_longlong;

        #[method(requestedLength)]
        pub unsafe fn requestedLength(&self) -> NSInteger;

        #[method(requestsAllDataToEndOfResource)]
        pub unsafe fn requestsAllDataToEndOfResource(&self) -> bool;

        #[method(currentOffset)]
        pub unsafe fn currentOffset(&self) -> c_longlong;

        #[method(respondWithData:)]
        pub unsafe fn respondWithData(&self, data: &NSData);
    }
);

extern_methods!(
    /// AVAssetResourceLoaderContentKeySupport
    unsafe impl AVAssetResourceLoader {
        #[method(preloadsEligibleContentKeys)]
        pub unsafe fn preloadsEligibleContentKeys(&self) -> bool;

        #[method(setPreloadsEligibleContentKeys:)]
        pub unsafe fn setPreloadsEligibleContentKeys(&self, preloads_eligible_content_keys: bool);
    }
);

extern_methods!(
    /// AVAssetResourceLoaderCommonMediaClientDataSupport
    unsafe impl AVAssetResourceLoader {
        #[method(sendsCommonMediaClientDataAsHTTPHeaders)]
        pub unsafe fn sendsCommonMediaClientDataAsHTTPHeaders(&self) -> bool;

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
        #[deprecated = "Use -[AVContentKeyRequest makeStreamingContentKeyRequestDataForApp:contentIdentifier:options:completionHandler:] instead"]
        #[method_id(@__retain_semantics Other streamingContentKeyRequestDataForApp:contentIdentifier:options:error:_)]
        pub unsafe fn streamingContentKeyRequestDataForApp_contentIdentifier_options_error(
            &self,
            app_identifier: &NSData,
            content_identifier: &NSData,
            options: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Result<Retained<NSData>, Retained<NSError>>;

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
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetresourceloadingrequeststreamingcontentkeyrequestrequirespersistentkey?language=objc)
    pub static AVAssetResourceLoadingRequestStreamingContentKeyRequestRequiresPersistentKey:
        &'static NSString;
}

extern_methods!(
    /// AVAssetResourceLoadingRequestDeprecated
    unsafe impl AVAssetResourceLoadingRequest {
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
