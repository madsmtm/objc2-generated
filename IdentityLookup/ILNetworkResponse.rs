//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A response to an HTTPS network request.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/identitylookup/ilnetworkresponse?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ILNetworkResponse;
);

extern_conformance!(
    unsafe impl NSCoding for ILNetworkResponse {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for ILNetworkResponse {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for ILNetworkResponse {}
);

impl ILNetworkResponse {
    extern_methods!(
        /// Represents the URL response itself. See documentation for NSHTTPURLResponse.
        #[unsafe(method(urlResponse))]
        #[unsafe(method_family = none)]
        pub unsafe fn urlResponse(&self) -> Retained<NSHTTPURLResponse>;

        /// Data returned in the HTTPS response.
        #[unsafe(method(data))]
        #[unsafe(method_family = none)]
        pub unsafe fn data(&self) -> Retained<NSData>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl ILNetworkResponse {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
