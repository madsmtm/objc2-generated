//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// NSScripting.
#[cfg(all(feature = "NSApplication", feature = "NSResponder"))]
impl NSApplication {
    extern_methods!(
        #[cfg(feature = "NSDocument")]
        #[unsafe(method(orderedDocuments))]
        #[unsafe(method_family = none)]
        pub unsafe fn orderedDocuments(&self) -> Retained<NSArray<NSDocument>>;

        #[cfg(feature = "NSWindow")]
        #[unsafe(method(orderedWindows))]
        #[unsafe(method_family = none)]
        pub unsafe fn orderedWindows(&self) -> Retained<NSArray<NSWindow>>;
    );
}
