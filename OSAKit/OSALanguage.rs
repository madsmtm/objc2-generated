//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/osakit/osalanguagefeatures?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OSALanguageFeatures(pub NSUInteger);
bitflags::bitflags! {
    impl OSALanguageFeatures: NSUInteger {
        #[doc(alias = "OSASupportsCompiling")]
        const SupportsCompiling = 0x0002;
        #[doc(alias = "OSASupportsGetSource")]
        const SupportsGetSource = 0x0004;
        #[doc(alias = "OSASupportsAECoercion")]
        const SupportsAECoercion = 0x0008;
        #[doc(alias = "OSASupportsAESending")]
        const SupportsAESending = 0x0010;
        #[doc(alias = "OSASupportsRecording")]
        const SupportsRecording = 0x0020;
        #[doc(alias = "OSASupportsConvenience")]
        const SupportsConvenience = 0x0040;
        #[doc(alias = "OSASupportsDialects")]
        const SupportsDialects = 0x0080;
        #[doc(alias = "OSASupportsEventHandling")]
        const SupportsEventHandling = 0x0100;
    }
}

unsafe impl Encode for OSALanguageFeatures {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for OSALanguageFeatures {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/osakit/osalanguage?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct OSALanguage;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for OSALanguage {}
);

impl OSALanguage {
    extern_methods!(
        #[unsafe(method(availableLanguages))]
        #[unsafe(method_family = none)]
        pub unsafe fn availableLanguages() -> Retained<NSArray<OSALanguage>>;

        #[unsafe(method(languageForName:))]
        #[unsafe(method_family = none)]
        pub unsafe fn languageForName(name: &NSString) -> Option<Retained<OSALanguage>>;

        #[unsafe(method(languageForScriptDataDescriptor:))]
        #[unsafe(method_family = none)]
        pub unsafe fn languageForScriptDataDescriptor(
            descriptor: &NSAppleEventDescriptor,
        ) -> Option<Retained<OSALanguage>>;

        #[unsafe(method(defaultLanguage))]
        #[unsafe(method_family = none)]
        pub unsafe fn defaultLanguage() -> Option<Retained<OSALanguage>>;

        #[unsafe(method(setDefaultLanguage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn setDefaultLanguage(default_language: &OSALanguage);

        #[cfg(feature = "OSALanguageInstance")]
        #[unsafe(method(sharedLanguageInstance))]
        #[unsafe(method_family = none)]
        pub unsafe fn sharedLanguageInstance(&self) -> Retained<OSALanguageInstance>;

        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        pub unsafe fn name(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(info))]
        #[unsafe(method_family = none)]
        pub unsafe fn info(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(version))]
        #[unsafe(method_family = none)]
        pub unsafe fn version(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(type))]
        #[unsafe(method_family = none)]
        pub unsafe fn r#type(&self) -> OSType;

        #[unsafe(method(subType))]
        #[unsafe(method_family = none)]
        pub unsafe fn subType(&self) -> OSType;

        #[unsafe(method(manufacturer))]
        #[unsafe(method_family = none)]
        pub unsafe fn manufacturer(&self) -> OSType;

        #[unsafe(method(features))]
        #[unsafe(method_family = none)]
        pub unsafe fn features(&self) -> OSALanguageFeatures;

        #[unsafe(method(isThreadSafe))]
        #[unsafe(method_family = none)]
        pub unsafe fn isThreadSafe(&self) -> bool;
    );
}

/// Methods declared on superclass `NSObject`.
impl OSALanguage {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
