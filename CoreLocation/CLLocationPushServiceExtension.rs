//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/corelocation/cllocationpushserviceextension?language=objc)
    pub unsafe trait CLLocationPushServiceExtension: NSObjectProtocol {
        #[cfg(feature = "block2")]
        #[unsafe(method(didReceiveLocationPushPayload:completion:))]
        #[unsafe(method_family = none)]
        unsafe fn didReceiveLocationPushPayload_completion(
            &self,
            payload: &NSDictionary<NSString, AnyObject>,
            completion: &block2::DynBlock<dyn Fn()>,
        );

        #[optional]
        #[unsafe(method(serviceExtensionWillTerminate))]
        #[unsafe(method_family = none)]
        unsafe fn serviceExtensionWillTerminate(&self);
    }
);
