//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A WKSecurityOrigin object contains information about a security origin.
    ///
    /// An instance of this class is a transient, data-only object;
    /// it does not uniquely identify a security origin across multiple delegate method
    /// calls.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/webkit/wksecurityorigin?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKSecurityOrigin;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for WKSecurityOrigin {}
);

impl WKSecurityOrigin {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// The security origin's protocol.
        #[unsafe(method(protocol))]
        #[unsafe(method_family = none)]
        pub unsafe fn protocol(&self) -> Retained<NSString>;

        /// The security origin's host.
        #[unsafe(method(host))]
        #[unsafe(method_family = none)]
        pub unsafe fn host(&self) -> Retained<NSString>;

        /// The security origin's port.
        #[unsafe(method(port))]
        #[unsafe(method_family = none)]
        pub unsafe fn port(&self) -> NSInteger;
    );
}

/// Methods declared on superclass `NSObject`.
impl WKSecurityOrigin {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    );
}
