//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-ui-kit")]
use objc2_ui_kit::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/watchkit/wkinterfacedate?language=objc)
    #[unsafe(super(WKInterfaceObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WKInterfaceObject")]
    pub struct WKInterfaceDate;
);

#[cfg(feature = "WKInterfaceObject")]
unsafe impl NSObjectProtocol for WKInterfaceDate {}

extern_methods!(
    #[cfg(feature = "WKInterfaceObject")]
    unsafe impl WKInterfaceDate {
        #[cfg(feature = "objc2-ui-kit")]
        #[method(setTextColor:)]
        pub unsafe fn setTextColor(&self, color: Option<&UIColor>);

        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, time_zone: Option<&NSTimeZone>);

        #[method(setCalendar:)]
        pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>);
    }
);

extern_methods!(
    /// Methods declared on superclass `WKInterfaceObject`
    #[cfg(feature = "WKInterfaceObject")]
    unsafe impl WKInterfaceDate {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WKInterfaceObject")]
    unsafe impl WKInterfaceDate {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);