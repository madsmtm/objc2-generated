//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/browserenginekit/betextinteractiondelegate?language=objc)
    pub unsafe trait BETextInteractionDelegate {
        #[cfg(feature = "BETextInteraction")]
        /// Invoked by the system when the selection is about to change in the document.
        #[unsafe(method(systemWillChangeSelectionForInteraction:))]
        #[unsafe(method_family = none)]
        unsafe fn systemWillChangeSelectionForInteraction(
            &self,
            text_interaction: &BETextInteraction,
        );

        #[cfg(feature = "BETextInteraction")]
        /// Invoked by the system when the selection is about to change in the document.
        #[unsafe(method(systemDidChangeSelectionForInteraction:))]
        #[unsafe(method_family = none)]
        unsafe fn systemDidChangeSelectionForInteraction(
            &self,
            text_interaction: &BETextInteraction,
        );
    }
);
