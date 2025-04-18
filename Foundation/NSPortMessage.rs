//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsportmessage?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPortMessage;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSPortMessage {}
);

impl NSPortMessage {
    extern_methods!(
        #[cfg(all(feature = "NSArray", feature = "NSPort"))]
        #[unsafe(method(initWithSendPort:receivePort:components:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithSendPort_receivePort_components(
            this: Allocated<Self>,
            send_port: Option<&NSPort>,
            reply_port: Option<&NSPort>,
            components: Option<&NSArray>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSArray")]
        #[unsafe(method(components))]
        #[unsafe(method_family = none)]
        pub unsafe fn components(&self) -> Option<Retained<NSArray>>;

        #[cfg(feature = "NSPort")]
        #[unsafe(method(receivePort))]
        #[unsafe(method_family = none)]
        pub unsafe fn receivePort(&self) -> Option<Retained<NSPort>>;

        #[cfg(feature = "NSPort")]
        #[unsafe(method(sendPort))]
        #[unsafe(method_family = none)]
        pub unsafe fn sendPort(&self) -> Option<Retained<NSPort>>;

        #[cfg(feature = "NSDate")]
        #[unsafe(method(sendBeforeDate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn sendBeforeDate(&self, date: &NSDate) -> bool;

        #[unsafe(method(msgid))]
        #[unsafe(method_family = none)]
        pub unsafe fn msgid(&self) -> u32;

        /// Setter for [`msgid`][Self::msgid].
        #[unsafe(method(setMsgid:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setMsgid(&self, msgid: u32);
    );
}

/// Methods declared on superclass `NSObject`.
impl NSPortMessage {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
