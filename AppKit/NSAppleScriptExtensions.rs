//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

mod private_NSAppleScriptNSExtensions {
    pub trait Sealed {}
}

/// Category "NSExtensions" on [`NSAppleScript`].
#[doc(alias = "NSExtensions")]
pub unsafe trait NSAppleScriptNSExtensions:
    ClassType + Sized + private_NSAppleScriptNSExtensions::Sealed
{
    extern_methods!(
        #[unsafe(method(richTextSource))]
        #[unsafe(method_family = none)]
        unsafe fn richTextSource(&self) -> Option<Retained<NSAttributedString>>;
    );
}

impl private_NSAppleScriptNSExtensions::Sealed for NSAppleScript {}
unsafe impl NSAppleScriptNSExtensions for NSAppleScript {}
