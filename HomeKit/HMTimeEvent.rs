//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// This class is used to represent a generic time event.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/homekit/hmtimeevent?language=objc)
    #[unsafe(super(HMEvent, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HMEvent")]
    pub struct HMTimeEvent;
);

#[cfg(feature = "HMEvent")]
unsafe impl Send for HMTimeEvent {}

#[cfg(feature = "HMEvent")]
unsafe impl Sync for HMTimeEvent {}

#[cfg(feature = "HMEvent")]
unsafe impl NSObjectProtocol for HMTimeEvent {}

extern_methods!(
    #[cfg(feature = "HMEvent")]
    unsafe impl HMTimeEvent {}
);

extern_methods!(
    /// Methods declared on superclass `HMEvent`
    #[cfg(feature = "HMEvent")]
    unsafe impl HMTimeEvent {
        #[deprecated = "HMEvent is abstract"]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[deprecated = "HMEvent is abstract"]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);