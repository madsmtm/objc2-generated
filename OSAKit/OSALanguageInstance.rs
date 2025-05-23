//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/osakit/osalanguageinstance?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct OSALanguageInstance;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for OSALanguageInstance {}
);

impl OSALanguageInstance {
    extern_methods!(
        #[cfg(feature = "OSALanguage")]
        #[unsafe(method(languageInstanceWithLanguage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn languageInstanceWithLanguage(language: &OSALanguage) -> Retained<Self>;

        #[cfg(feature = "OSALanguage")]
        #[unsafe(method(initWithLanguage:))]
        #[unsafe(method_family = init)]
        pub unsafe fn initWithLanguage(
            this: Allocated<Self>,
            language: &OSALanguage,
        ) -> Retained<Self>;

        #[cfg(feature = "OSALanguage")]
        #[unsafe(method(language))]
        #[unsafe(method_family = none)]
        pub unsafe fn language(&self) -> Retained<OSALanguage>;

        #[unsafe(method(defaultTarget))]
        #[unsafe(method_family = none)]
        pub unsafe fn defaultTarget(&self) -> Option<Retained<NSAppleEventDescriptor>>;

        /// Setter for [`defaultTarget`][Self::defaultTarget].
        #[unsafe(method(setDefaultTarget:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDefaultTarget(&self, default_target: Option<&NSAppleEventDescriptor>);

        #[unsafe(method(richTextFromDescriptor:))]
        #[unsafe(method_family = none)]
        pub unsafe fn richTextFromDescriptor(
            &self,
            descriptor: &NSAppleEventDescriptor,
        ) -> Option<Retained<NSAttributedString>>;
    );
}

/// Methods declared on superclass `NSObject`.
impl OSALanguageInstance {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
