//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/callkit/cxcall?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CXCall;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for CXCall {}
);

impl CXCall {
    extern_methods!(
        #[unsafe(method(UUID))]
        #[unsafe(method_family = none)]
        pub unsafe fn UUID(&self) -> Retained<NSUUID>;

        #[unsafe(method(isOutgoing))]
        #[unsafe(method_family = none)]
        pub unsafe fn isOutgoing(&self) -> bool;

        #[unsafe(method(isOnHold))]
        #[unsafe(method_family = none)]
        pub unsafe fn isOnHold(&self) -> bool;

        #[unsafe(method(hasConnected))]
        #[unsafe(method_family = none)]
        pub unsafe fn hasConnected(&self) -> bool;

        #[unsafe(method(hasEnded))]
        #[unsafe(method_family = none)]
        pub unsafe fn hasEnded(&self) -> bool;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(isEqualToCall:))]
        #[unsafe(method_family = none)]
        pub unsafe fn isEqualToCall(&self, call: &CXCall) -> bool;
    );
}

/// Methods declared on superclass `NSObject`.
impl CXCall {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
