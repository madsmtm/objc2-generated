//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Represents a Classification UI extension request's context.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/identitylookupui/ilclassificationuiextensioncontext?language=objc)
    #[unsafe(super(NSExtensionContext, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ILClassificationUIExtensionContext;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for ILClassificationUIExtensionContext {}
);

impl ILClassificationUIExtensionContext {
    extern_methods!(
        #[unsafe(method(isReadyForClassificationResponse))]
        #[unsafe(method_family = none)]
        pub unsafe fn isReadyForClassificationResponse(&self) -> bool;

        /// Setter for [`isReadyForClassificationResponse`][Self::isReadyForClassificationResponse].
        #[unsafe(method(setReadyForClassificationResponse:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setReadyForClassificationResponse(
            &self,
            ready_for_classification_response: bool,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl ILClassificationUIExtensionContext {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
