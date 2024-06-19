//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NEAppProxyFlow")]
    pub struct NEAppProxyTCPFlow;

    #[cfg(feature = "NEAppProxyFlow")]
    unsafe impl ClassType for NEAppProxyTCPFlow {
        #[inherits(NSObject)]
        type Super = NEAppProxyFlow;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "NEAppProxyFlow")]
unsafe impl NSObjectProtocol for NEAppProxyTCPFlow {}

extern_methods!(
    #[cfg(feature = "NEAppProxyFlow")]
    unsafe impl NEAppProxyTCPFlow {
        #[cfg(feature = "block2")]
        #[method(readDataWithCompletionHandler:)]
        pub unsafe fn readDataWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut NSData, *mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(writeData:withCompletionHandler:)]
        pub unsafe fn writeData_withCompletionHandler(
            &self,
            data: &NSData,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "NWEndpoint")]
        #[method_id(@__retain_semantics Other remoteEndpoint)]
        pub unsafe fn remoteEndpoint(&self) -> Retained<NWEndpoint>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NEAppProxyFlow")]
    unsafe impl NEAppProxyTCPFlow {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);