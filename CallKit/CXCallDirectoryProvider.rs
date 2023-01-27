//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CallKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CallKit_CXCallDirectoryProvider")]
    pub struct CXCallDirectoryProvider;

    #[cfg(feature = "CallKit_CXCallDirectoryProvider")]
    unsafe impl ClassType for CXCallDirectoryProvider {
        type Super = NSObject;
    }
);

#[cfg(feature = "CallKit_CXCallDirectoryProvider")]
unsafe impl NSExtensionRequestHandling for CXCallDirectoryProvider {}

#[cfg(feature = "CallKit_CXCallDirectoryProvider")]
unsafe impl NSObjectProtocol for CXCallDirectoryProvider {}

extern_methods!(
    #[cfg(feature = "CallKit_CXCallDirectoryProvider")]
    unsafe impl CXCallDirectoryProvider {
        #[cfg(feature = "CallKit_CXCallDirectoryExtensionContext")]
        #[method(beginRequestWithExtensionContext:)]
        pub unsafe fn beginRequestWithExtensionContext(
            &self,
            context: &CXCallDirectoryExtensionContext,
        );
    }
);