//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsvalidateduserinterfaceitem?language=objc)
    pub unsafe trait NSValidatedUserInterfaceItem: MainThreadOnly {
        #[unsafe(method(action))]
        #[unsafe(method_family = none)]
        unsafe fn action(&self) -> Option<Sel>;

        #[unsafe(method(tag))]
        #[unsafe(method_family = none)]
        unsafe fn tag(&self) -> NSInteger;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsuserinterfacevalidations?language=objc)
    pub unsafe trait NSUserInterfaceValidations: MainThreadOnly {
        #[unsafe(method(validateUserInterfaceItem:))]
        #[unsafe(method_family = none)]
        unsafe fn validateUserInterfaceItem(
            &self,
            item: &ProtocolObject<dyn NSValidatedUserInterfaceItem>,
        ) -> bool;
    }
);
