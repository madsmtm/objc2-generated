//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// This class is used to represent a generic HomeKit event.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/homekit/hmevent?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct HMEvent;
);

unsafe impl Send for HMEvent {}

unsafe impl Sync for HMEvent {}

unsafe impl NSObjectProtocol for HMEvent {}

extern_methods!(
    unsafe impl HMEvent {
        #[deprecated = "HMEvent is abstract"]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[deprecated = "HMEvent is abstract"]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        /// A unique identifier for the event.
        #[method_id(@__retain_semantics Other uniqueIdentifier)]
        pub unsafe fn uniqueIdentifier(&self) -> Retained<NSUUID>;

        #[cfg(feature = "HMHome")]
        /// Specifies whether the HMEvent can be added to HMEventTrigger on the given home.
        #[method(isSupportedForHome:)]
        pub unsafe fn isSupportedForHome(home: &HMHome) -> bool;
    }
);