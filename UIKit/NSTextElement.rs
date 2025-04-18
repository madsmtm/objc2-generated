//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/nstextelement?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextElement;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSTextElement {}
);

impl NSTextElement {
    extern_methods!(
        #[cfg(feature = "NSTextContentManager")]
        #[unsafe(method(initWithTextContentManager:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithTextContentManager(
            this: Allocated<Self>,
            text_content_manager: Option<&NSTextContentManager>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSTextContentManager")]
        #[unsafe(method(textContentManager))]
        #[unsafe(method_family = none)]
        pub unsafe fn textContentManager(&self) -> Option<Retained<NSTextContentManager>>;

        #[cfg(feature = "NSTextContentManager")]
        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`textContentManager`][Self::textContentManager].
        #[unsafe(method(setTextContentManager:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setTextContentManager(
            &self,
            text_content_manager: Option<&NSTextContentManager>,
        );

        #[cfg(feature = "NSTextRange")]
        #[unsafe(method(elementRange))]
        #[unsafe(method_family = none)]
        pub unsafe fn elementRange(&self) -> Option<Retained<NSTextRange>>;

        #[cfg(feature = "NSTextRange")]
        /// Setter for [`elementRange`][Self::elementRange].
        #[unsafe(method(setElementRange:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setElementRange(&self, element_range: Option<&NSTextRange>);

        #[unsafe(method(childElements))]
        #[unsafe(method_family = none)]
        pub unsafe fn childElements(&self) -> Retained<NSArray<NSTextElement>>;

        #[unsafe(method(parentElement))]
        #[unsafe(method_family = none)]
        pub unsafe fn parentElement(&self) -> Option<Retained<NSTextElement>>;

        #[unsafe(method(isRepresentedElement))]
        #[unsafe(method_family = none)]
        pub unsafe fn isRepresentedElement(&self) -> bool;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSTextElement {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/nstextparagraph?language=objc)
    #[unsafe(super(NSTextElement, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextParagraph;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NSTextParagraph {}
);

impl NSTextParagraph {
    extern_methods!(
        #[unsafe(method(initWithAttributedString:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithAttributedString(
            this: Allocated<Self>,
            attributed_string: Option<&NSAttributedString>,
        ) -> Retained<Self>;

        #[unsafe(method(attributedString))]
        #[unsafe(method_family = none)]
        pub unsafe fn attributedString(&self) -> Retained<NSAttributedString>;

        #[cfg(feature = "NSTextRange")]
        #[unsafe(method(paragraphContentRange))]
        #[unsafe(method_family = none)]
        pub unsafe fn paragraphContentRange(&self) -> Option<Retained<NSTextRange>>;

        #[cfg(feature = "NSTextRange")]
        #[unsafe(method(paragraphSeparatorRange))]
        #[unsafe(method_family = none)]
        pub unsafe fn paragraphSeparatorRange(&self) -> Option<Retained<NSTextRange>>;
    );
}

/// Methods declared on superclass `NSTextElement`.
impl NSTextParagraph {
    extern_methods!(
        #[cfg(feature = "NSTextContentManager")]
        #[unsafe(method(initWithTextContentManager:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithTextContentManager(
            this: Allocated<Self>,
            text_content_manager: Option<&NSTextContentManager>,
        ) -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl NSTextParagraph {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
