//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use objc2::__framework_prelude::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitextpasteconfigurationsupporting?language=objc)
    #[cfg(feature = "UIPasteConfigurationSupporting")]
    pub unsafe trait UITextPasteConfigurationSupporting:
        UIPasteConfigurationSupporting + MainThreadOnly
    {
        #[cfg(feature = "UITextPasteDelegate")]
        #[unsafe(method(pasteDelegate))]
        #[unsafe(method_family = none)]
        unsafe fn pasteDelegate(&self)
            -> Option<Retained<ProtocolObject<dyn UITextPasteDelegate>>>;

        #[cfg(feature = "UITextPasteDelegate")]
        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`pasteDelegate`][Self::pasteDelegate].
        #[unsafe(method(setPasteDelegate:))]
        #[unsafe(method_family = none)]
        unsafe fn setPasteDelegate(
            &self,
            paste_delegate: Option<&ProtocolObject<dyn UITextPasteDelegate>>,
        );
    }
);
