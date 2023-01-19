//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMProcessingInstruction")]
    #[deprecated]
    pub struct DOMProcessingInstruction;

    #[cfg(feature = "WebKit_DOMProcessingInstruction")]
    unsafe impl ClassType for DOMProcessingInstruction {
        #[inherits(DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMCharacterData;
    }
);

extern_methods!(
    #[cfg(feature = "WebKit_DOMProcessingInstruction")]
    unsafe impl DOMProcessingInstruction {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "WebKit_DOMStyleSheet")]
        #[method_id(@__retain_semantics Other sheet)]
        pub unsafe fn sheet(&self) -> Option<Id<DOMStyleSheet, Shared>>;
    }
);