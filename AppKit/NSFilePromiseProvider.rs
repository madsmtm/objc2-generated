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

unsafe impl NSObjectProtocol for NSFilePromiseProvider {}

#[cfg(feature = "NSPasteboard")]
unsafe impl NSPasteboardWriting for NSFilePromiseProvider {}

extern_methods!(
    unsafe impl NSFilePromiseProvider {
        #[unsafe(method_family(none))]
        #[method_id(fileType)]
        pub unsafe fn fileType(&self) -> Retained<NSString>;

        /// Setter for [`fileType`][Self::fileType].
        #[method(setFileType:)]
        pub unsafe fn setFileType(&self, file_type: &NSString);

        #[unsafe(method_family(none))]
        #[method_id(delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSFilePromiseProviderDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSFilePromiseProviderDelegate>>,
        );

        #[unsafe(method_family(none))]
        #[method_id(userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Retained<AnyObject>>;

        /// Setter for [`userInfo`][Self::userInfo].
        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, user_info: Option<&AnyObject>);

        #[unsafe(method_family(init))]
        #[method_id(initWithFileType:delegate:)]
        pub unsafe fn initWithFileType_delegate(
            this: Allocated<Self>,
            file_type: &NSString,
            delegate: &ProtocolObject<dyn NSFilePromiseProviderDelegate>,
        ) -> Retained<Self>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSFilePromiseProvider {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsfilepromiseproviderdelegate?language=objc)
    pub unsafe trait NSFilePromiseProviderDelegate: NSObjectProtocol {
        #[unsafe(method_family(none))]
        #[method_id(filePromiseProvider:fileNameForType:)]
        unsafe fn filePromiseProvider_fileNameForType(
            &self,
            file_promise_provider: &NSFilePromiseProvider,
            file_type: &NSString,
            mtm: MainThreadMarker,
        ) -> Retained<NSString>;

        #[cfg(feature = "block2")]
        #[method(filePromiseProvider:writePromiseToURL:completionHandler:)]
        unsafe fn filePromiseProvider_writePromiseToURL_completionHandler(
            &self,
            file_promise_provider: &NSFilePromiseProvider,
            url: &NSURL,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[optional]
        #[unsafe(method_family(none))]
        #[method_id(operationQueueForFilePromiseProvider:)]
        unsafe fn operationQueueForFilePromiseProvider(
            &self,
            file_promise_provider: &NSFilePromiseProvider,
            mtm: MainThreadMarker,
        ) -> Retained<NSOperationQueue>;
    }
);
