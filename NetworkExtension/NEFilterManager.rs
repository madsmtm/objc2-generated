//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NEFilterManagerError(pub NSInteger);
impl NEFilterManagerError {
    #[doc(alias = "NEFilterManagerErrorConfigurationInvalid")]
    pub const ConfigurationInvalid: Self = Self(1);
    #[doc(alias = "NEFilterManagerErrorConfigurationDisabled")]
    pub const ConfigurationDisabled: Self = Self(2);
    #[doc(alias = "NEFilterManagerErrorConfigurationStale")]
    pub const ConfigurationStale: Self = Self(3);
    #[doc(alias = "NEFilterManagerErrorConfigurationCannotBeRemoved")]
    pub const ConfigurationCannotBeRemoved: Self = Self(4);
    #[doc(alias = "NEFilterManagerErrorConfigurationPermissionDenied")]
    pub const ConfigurationPermissionDenied: Self = Self(5);
    #[doc(alias = "NEFilterManagerErrorConfigurationInternalError")]
    pub const ConfigurationInternalError: Self = Self(6);
}

unsafe impl Encode for NEFilterManagerError {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NEFilterManagerError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    pub static NEFilterErrorDomain: &'static NSString;
}

extern "C" {
    pub static NEFilterConfigurationDidChangeNotification: &'static NSString;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NEFilterManagerGrade(pub NSInteger);
impl NEFilterManagerGrade {
    #[doc(alias = "NEFilterManagerGradeFirewall")]
    pub const Firewall: Self = Self(1);
    #[doc(alias = "NEFilterManagerGradeInspector")]
    pub const Inspector: Self = Self(2);
}

unsafe impl Encode for NEFilterManagerGrade {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NEFilterManagerGrade {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEFilterManager;

    unsafe impl ClassType for NEFilterManager {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NEFilterManager {}

extern_methods!(
    unsafe impl NEFilterManager {
        #[method_id(@__retain_semantics Other sharedManager)]
        pub unsafe fn sharedManager() -> Retained<NEFilterManager>;

        #[cfg(feature = "block2")]
        #[method(loadFromPreferencesWithCompletionHandler:)]
        pub unsafe fn loadFromPreferencesWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(removeFromPreferencesWithCompletionHandler:)]
        pub unsafe fn removeFromPreferencesWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(saveToPreferencesWithCompletionHandler:)]
        pub unsafe fn saveToPreferencesWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[method_id(@__retain_semantics Other localizedDescription)]
        pub unsafe fn localizedDescription(&self) -> Option<Retained<NSString>>;

        #[method(setLocalizedDescription:)]
        pub unsafe fn setLocalizedDescription(&self, localized_description: Option<&NSString>);

        #[cfg(feature = "NEFilterProviderConfiguration")]
        #[method_id(@__retain_semantics Other providerConfiguration)]
        pub unsafe fn providerConfiguration(
            &self,
        ) -> Option<Retained<NEFilterProviderConfiguration>>;

        #[cfg(feature = "NEFilterProviderConfiguration")]
        #[method(setProviderConfiguration:)]
        pub unsafe fn setProviderConfiguration(
            &self,
            provider_configuration: Option<&NEFilterProviderConfiguration>,
        );

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(grade)]
        pub unsafe fn grade(&self) -> NEFilterManagerGrade;

        #[method(setGrade:)]
        pub unsafe fn setGrade(&self, grade: NEFilterManagerGrade);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEFilterManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);