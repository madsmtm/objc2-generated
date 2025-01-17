//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiprinterdestination?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPrinterDestination;
);

unsafe impl NSCoding for UIPrinterDestination {}

unsafe impl NSObjectProtocol for UIPrinterDestination {}

unsafe impl NSSecureCoding for UIPrinterDestination {}

extern_methods!(
    unsafe impl UIPrinterDestination {
        #[unsafe(method_family(init))]
        #[method_id(initWithURL:)]
        pub unsafe fn initWithURL(this: Allocated<Self>, url: &NSURL) -> Retained<Self>;

        #[unsafe(method_family(none))]
        #[method_id(URL)]
        pub unsafe fn URL(&self) -> Retained<NSURL>;

        /// Setter for [`URL`][Self::URL].
        #[method(setURL:)]
        pub unsafe fn setURL(&self, url: &NSURL);

        #[unsafe(method_family(none))]
        #[method_id(displayName)]
        pub unsafe fn displayName(&self) -> Option<Retained<NSString>>;

        /// Setter for [`displayName`][Self::displayName].
        #[method(setDisplayName:)]
        pub unsafe fn setDisplayName(&self, display_name: Option<&NSString>);

        #[unsafe(method_family(none))]
        #[method_id(txtRecord)]
        pub unsafe fn txtRecord(&self) -> Option<Retained<NSData>>;

        /// Setter for [`txtRecord`][Self::txtRecord].
        #[method(setTxtRecord:)]
        pub unsafe fn setTxtRecord(&self, txt_record: Option<&NSData>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIPrinterDestination {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiprintserviceextension?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPrintServiceExtension;
);

unsafe impl NSObjectProtocol for UIPrintServiceExtension {}

extern_methods!(
    unsafe impl UIPrintServiceExtension {
        #[cfg(feature = "UIPrintInfo")]
        #[unsafe(method_family(none))]
        #[method_id(printerDestinationsForPrintInfo:)]
        pub unsafe fn printerDestinationsForPrintInfo(
            &self,
            print_info: &UIPrintInfo,
        ) -> Retained<NSArray<UIPrinterDestination>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIPrintServiceExtension {
        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
