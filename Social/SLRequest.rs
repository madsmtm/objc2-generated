//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-accounts")]
use objc2_accounts::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/social/slrequestmethod?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SLRequestMethod(pub NSInteger);
impl SLRequestMethod {
    #[doc(alias = "SLRequestMethodGET")]
    pub const GET: Self = Self(0);
    #[doc(alias = "SLRequestMethodPOST")]
    pub const POST: Self = Self(1);
    #[doc(alias = "SLRequestMethodDELETE")]
    pub const DELETE: Self = Self(2);
    #[doc(alias = "SLRequestMethodPUT")]
    pub const PUT: Self = Self(3);
}

unsafe impl Encode for SLRequestMethod {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SLRequestMethod {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/social/slrequesthandler?language=objc)
#[cfg(feature = "block2")]
pub type SLRequestHandler =
    *mut block2::DynBlock<dyn Fn(*mut NSData, *mut NSHTTPURLResponse, *mut NSError)>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/social/slrequest?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SLRequest;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for SLRequest {}
);

impl SLRequest {
    extern_methods!(
        #[unsafe(method(requestForServiceType:requestMethod:URL:parameters:))]
        #[unsafe(method_family = none)]
        pub unsafe fn requestForServiceType_requestMethod_URL_parameters(
            service_type: Option<&NSString>,
            request_method: SLRequestMethod,
            url: Option<&NSURL>,
            parameters: Option<&NSDictionary>,
        ) -> Option<Retained<SLRequest>>;

        #[cfg(feature = "objc2-accounts")]
        #[unsafe(method(account))]
        #[unsafe(method_family = none)]
        pub unsafe fn account(&self) -> Option<Retained<ACAccount>>;

        #[cfg(feature = "objc2-accounts")]
        /// Setter for [`account`][Self::account].
        #[unsafe(method(setAccount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setAccount(&self, account: Option<&ACAccount>);

        #[unsafe(method(requestMethod))]
        #[unsafe(method_family = none)]
        pub unsafe fn requestMethod(&self) -> SLRequestMethod;

        #[unsafe(method(URL))]
        #[unsafe(method_family = none)]
        pub unsafe fn URL(&self) -> Option<Retained<NSURL>>;

        #[unsafe(method(parameters))]
        #[unsafe(method_family = none)]
        pub unsafe fn parameters(&self) -> Option<Retained<NSDictionary>>;

        #[unsafe(method(addMultipartData:withName:type:filename:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addMultipartData_withName_type_filename(
            &self,
            data: Option<&NSData>,
            name: Option<&NSString>,
            r#type: Option<&NSString>,
            filename: Option<&NSString>,
        );

        #[unsafe(method(addMultipartData:withName:type:))]
        #[unsafe(method_family = none)]
        pub unsafe fn addMultipartData_withName_type(
            &self,
            data: Option<&NSData>,
            name: Option<&NSString>,
            r#type: Option<&NSString>,
        );

        #[unsafe(method(preparedURLRequest))]
        #[unsafe(method_family = none)]
        pub unsafe fn preparedURLRequest(&self) -> Option<Retained<NSURLRequest>>;

        #[cfg(feature = "block2")]
        #[unsafe(method(performRequestWithHandler:))]
        #[unsafe(method_family = none)]
        pub unsafe fn performRequestWithHandler(&self, handler: SLRequestHandler);
    );
}

/// Methods declared on superclass `NSObject`.
impl SLRequest {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
