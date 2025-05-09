//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/naturallanguage/nllanguagerecognizer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NLLanguageRecognizer;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for NLLanguageRecognizer {}
);

impl NLLanguageRecognizer {
    extern_methods!(
        #[cfg(feature = "NLLanguage")]
        #[unsafe(method(dominantLanguageForString:))]
        #[unsafe(method_family = none)]
        pub unsafe fn dominantLanguageForString(string: &NSString) -> Option<Retained<NLLanguage>>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(processString:))]
        #[unsafe(method_family = none)]
        pub unsafe fn processString(&self, string: &NSString);

        #[unsafe(method(reset))]
        #[unsafe(method_family = none)]
        pub unsafe fn reset(&self);

        #[cfg(feature = "NLLanguage")]
        #[unsafe(method(dominantLanguage))]
        #[unsafe(method_family = none)]
        pub unsafe fn dominantLanguage(&self) -> Option<Retained<NLLanguage>>;

        #[cfg(feature = "NLLanguage")]
        #[unsafe(method(languageHypothesesWithMaximum:))]
        #[unsafe(method_family = none)]
        pub unsafe fn languageHypothesesWithMaximum(
            &self,
            max_hypotheses: NSUInteger,
        ) -> Retained<NSDictionary<NLLanguage, NSNumber>>;

        #[cfg(feature = "NLLanguage")]
        #[unsafe(method(languageHints))]
        #[unsafe(method_family = none)]
        pub unsafe fn languageHints(&self) -> Retained<NSDictionary<NLLanguage, NSNumber>>;

        #[cfg(feature = "NLLanguage")]
        /// Setter for [`languageHints`][Self::languageHints].
        #[unsafe(method(setLanguageHints:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLanguageHints(&self, language_hints: &NSDictionary<NLLanguage, NSNumber>);

        #[cfg(feature = "NLLanguage")]
        #[unsafe(method(languageConstraints))]
        #[unsafe(method_family = none)]
        pub unsafe fn languageConstraints(&self) -> Retained<NSArray<NLLanguage>>;

        #[cfg(feature = "NLLanguage")]
        /// Setter for [`languageConstraints`][Self::languageConstraints].
        #[unsafe(method(setLanguageConstraints:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setLanguageConstraints(&self, language_constraints: &NSArray<NLLanguage>);
    );
}

/// Methods declared on superclass `NSObject`.
impl NLLanguageRecognizer {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
