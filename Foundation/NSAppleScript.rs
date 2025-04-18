//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsapplescripterrormessage?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSAppleScriptErrorMessage: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsapplescripterrornumber?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSAppleScriptErrorNumber: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsapplescripterrorappname?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSAppleScriptErrorAppName: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsapplescripterrorbriefmessage?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSAppleScriptErrorBriefMessage: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsapplescripterrorrange?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSAppleScriptErrorRange: &'static NSString;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsapplescript?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAppleScript;
);

#[cfg(feature = "NSObject")]
extern_conformance!(
    unsafe impl NSCopying for NSAppleScript {}
);

#[cfg(feature = "NSObject")]
unsafe impl CopyingHelper for NSAppleScript {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for NSAppleScript {}
);

impl NSAppleScript {
    extern_methods!(
        #[cfg(all(feature = "NSDictionary", feature = "NSString", feature = "NSURL"))]
        #[unsafe(method(initWithContentsOfURL:error:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithContentsOfURL_error(
            this: Allocated<Self>,
            url: &NSURL,
            error_info: Option<&mut Option<Retained<NSDictionary<NSString, AnyObject>>>>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSString")]
        #[unsafe(method(initWithSource:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithSource(
            this: Allocated<Self>,
            source: &NSString,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSString")]
        #[unsafe(method(source))]
        #[unsafe(method_family = none)]
        pub unsafe fn source(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(isCompiled))]
        #[unsafe(method_family = none)]
        pub unsafe fn isCompiled(&self) -> bool;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[unsafe(method(compileAndReturnError:))]
        #[unsafe(method_family = none)]
        pub unsafe fn compileAndReturnError(
            &self,
            error_info: Option<&mut Option<Retained<NSDictionary<NSString, AnyObject>>>>,
        ) -> bool;

        #[cfg(all(
            feature = "NSAppleEventDescriptor",
            feature = "NSDictionary",
            feature = "NSString"
        ))]
        #[unsafe(method(executeAndReturnError:))]
        #[unsafe(method_family = none)]
        pub unsafe fn executeAndReturnError(
            &self,
            error_info: Option<&mut Option<Retained<NSDictionary<NSString, AnyObject>>>>,
        ) -> Retained<NSAppleEventDescriptor>;

        #[cfg(all(
            feature = "NSAppleEventDescriptor",
            feature = "NSDictionary",
            feature = "NSString"
        ))]
        #[unsafe(method(executeAppleEvent:error:))]
        #[unsafe(method_family = none)]
        pub unsafe fn executeAppleEvent_error(
            &self,
            event: &NSAppleEventDescriptor,
            error_info: Option<&mut Option<Retained<NSDictionary<NSString, AnyObject>>>>,
        ) -> Retained<NSAppleEventDescriptor>;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSAppleScript {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
