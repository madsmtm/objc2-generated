//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsuserinterfaceitemidentifier?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type NSUserInterfaceItemIdentifier = NSString;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsuserinterfaceitemidentification?language=objc)
    pub unsafe trait NSUserInterfaceItemIdentification {
        #[unsafe(method(identifier))]
        #[unsafe(method_family = none)]
        unsafe fn identifier(&self) -> Option<Retained<NSUserInterfaceItemIdentifier>>;

        /// Setter for [`identifier`][Self::identifier].
        #[unsafe(method(setIdentifier:))]
        #[unsafe(method_family = none)]
        unsafe fn setIdentifier(&self, identifier: Option<&NSUserInterfaceItemIdentifier>);
    }
);
