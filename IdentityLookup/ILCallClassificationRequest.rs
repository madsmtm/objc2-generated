//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/identitylookup/ilcallclassificationrequest?language=objc)
    #[unsafe(super(ILClassificationRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ILClassificationRequest")]
    pub struct ILCallClassificationRequest;
);

#[cfg(feature = "ILClassificationRequest")]
unsafe impl NSCoding for ILCallClassificationRequest {}

#[cfg(feature = "ILClassificationRequest")]
unsafe impl NSObjectProtocol for ILCallClassificationRequest {}

#[cfg(feature = "ILClassificationRequest")]
unsafe impl NSSecureCoding for ILCallClassificationRequest {}

extern_methods!(
    #[cfg(feature = "ILClassificationRequest")]
    unsafe impl ILCallClassificationRequest {
        #[cfg(all(feature = "ILCallCommunication", feature = "ILCommunication"))]
        #[unsafe(method_family(none))]
        #[method_id(callCommunications)]
        pub unsafe fn callCommunications(&self) -> Retained<NSArray<ILCallCommunication>>;

        #[unsafe(method_family(init))]
        #[method_id(init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "ILClassificationRequest")]
    unsafe impl ILCallClassificationRequest {
        #[unsafe(method_family(new))]
        #[method_id(new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
