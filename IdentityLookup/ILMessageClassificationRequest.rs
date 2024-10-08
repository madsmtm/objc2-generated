//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ILClassificationRequest")]
    pub struct ILMessageClassificationRequest;

    #[cfg(feature = "ILClassificationRequest")]
    unsafe impl ClassType for ILMessageClassificationRequest {
        #[inherits(NSObject)]
        type Super = ILClassificationRequest;
    }
);

#[cfg(feature = "ILClassificationRequest")]
unsafe impl NSCoding for ILMessageClassificationRequest {}

#[cfg(feature = "ILClassificationRequest")]
unsafe impl NSObjectProtocol for ILMessageClassificationRequest {}

#[cfg(feature = "ILClassificationRequest")]
unsafe impl NSSecureCoding for ILMessageClassificationRequest {}

extern_methods!(
    #[cfg(feature = "ILClassificationRequest")]
    unsafe impl ILMessageClassificationRequest {
        #[cfg(all(feature = "ILCommunication", feature = "ILMessageCommunication"))]
        #[method_id(@__retain_semantics Other messageCommunications)]
        pub unsafe fn messageCommunications(&self) -> Retained<NSArray<ILMessageCommunication>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "ILClassificationRequest")]
    unsafe impl ILMessageClassificationRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
