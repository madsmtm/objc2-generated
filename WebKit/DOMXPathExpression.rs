//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/webkit/domxpathexpression?language=objc)
    #[unsafe(super(DOMObject, WebScriptObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
    #[deprecated]
    pub struct DOMXPathExpression;
);

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
extern_conformance!(
    unsafe impl NSCopying for DOMXPathExpression {}
);

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
unsafe impl CopyingHelper for DOMXPathExpression {
    type Result = Self;
}

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
extern_conformance!(
    unsafe impl NSObjectProtocol for DOMXPathExpression {}
);

#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
impl DOMXPathExpression {
    extern_methods!(
        #[cfg(all(feature = "DOMNode", feature = "DOMXPathResult"))]
        #[unsafe(method(evaluate:type:inResult:))]
        #[unsafe(method_family = none)]
        pub unsafe fn evaluate_type_inResult(
            &self,
            context_node: Option<&DOMNode>,
            r#type: c_ushort,
            in_result: Option<&DOMXPathResult>,
        ) -> Option<Retained<DOMXPathResult>>;
    );
}

/// Methods declared on superclass `DOMObject`.
#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
impl DOMXPathExpression {
    extern_methods!(
        #[deprecated]
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
impl DOMXPathExpression {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

/// DOMXPathExpressionDeprecated.
#[cfg(all(feature = "DOMObject", feature = "WebScriptObject"))]
impl DOMXPathExpression {
    extern_methods!(
        #[cfg(all(feature = "DOMNode", feature = "DOMXPathResult"))]
        #[deprecated]
        #[unsafe(method(evaluate:::))]
        #[unsafe(method_family = none)]
        pub unsafe fn evaluate(
            &self,
            context_node: Option<&DOMNode>,
            r#type: c_ushort,
            in_result: Option<&DOMXPathResult>,
        ) -> Option<Retained<DOMXPathResult>>;
    );
}
