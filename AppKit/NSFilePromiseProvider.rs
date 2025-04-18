//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfilepromiseprovider?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFilePromiseProvider;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSFilePromiseProvider {}
);

#[cfg(feature = "NSPasteboard")]
extern_conformance!(
    unsafe impl NSPasteboardWriting for NSFilePromiseProvider {}
);

impl NSFilePromiseProvider {
    extern_methods!(
        #[unsafe(method(fileType))]
        #[unsafe(method_family = none)]
        pub unsafe fn fileType(&self) -> Retained<NSString>;

        /// Setter for [`fileType`][Self::fileType].
        #[unsafe(method(setFileType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setFileType(&self, file_type: &NSString);

        #[unsafe(method(delegate))]
        #[unsafe(method_family = none)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSFilePromiseProviderDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[unsafe(method(setDelegate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSFilePromiseProviderDelegate>>,
        );

        #[unsafe(method(userInfo))]
        #[unsafe(method_family = none)]
        pub unsafe fn userInfo(&self) -> Option<Retained<AnyObject>>;

        /// Setter for [`userInfo`][Self::userInfo].
        #[unsafe(method(setUserInfo:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setUserInfo(&self, user_info: Option<&AnyObject>);

        #[unsafe(method(initWithFileType:delegate:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithFileType_delegate(
            this: Allocated<Self>,
            file_type: &NSString,
            delegate: &ProtocolObject<dyn NSFilePromiseProviderDelegate>,
        ) -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSFilePromiseProvider {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfilepromiseproviderdelegate?language=objc)
    pub unsafe trait NSFilePromiseProviderDelegate: NSObjectProtocol {
        #[unsafe(method(filePromiseProvider:fileNameForType:))]
        #[unsafe(method_family = none)]
        unsafe fn filePromiseProvider_fileNameForType(
            &self,
            file_promise_provider: &NSFilePromiseProvider,
            file_type: &NSString,
            mtm: MainThreadMarker,
        ) -> Retained<NSString>;

        #[cfg(feature = "block2")]
        #[unsafe(method(filePromiseProvider:writePromiseToURL:completionHandler:))]
        #[unsafe(method_family = none)]
        unsafe fn filePromiseProvider_writePromiseToURL_completionHandler(
            &self,
            file_promise_provider: &NSFilePromiseProvider,
            url: &NSURL,
            completion_handler: &block2::DynBlock<dyn Fn(*mut NSError)>,
        );

        #[optional]
        #[unsafe(method(operationQueueForFilePromiseProvider:))]
        #[unsafe(method_family = none)]
        unsafe fn operationQueueForFilePromiseProvider(
            &self,
            file_promise_provider: &NSFilePromiseProvider,
            mtm: MainThreadMarker,
        ) -> Retained<NSOperationQueue>;
    }
);
