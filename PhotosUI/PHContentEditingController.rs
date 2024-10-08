//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
#[cfg(feature = "objc2-photos")]
use objc2_photos::*;

use crate::*;

extern_protocol!(
    pub unsafe trait PHContentEditingController: NSObjectProtocol + MainThreadOnly {
        #[cfg(feature = "objc2-photos")]
        #[method(canHandleAdjustmentData:)]
        unsafe fn canHandleAdjustmentData(&self, adjustment_data: &PHAdjustmentData) -> bool;

        #[cfg(all(feature = "objc2-app-kit", feature = "objc2-photos"))]
        #[cfg(target_os = "macos")]
        #[method(startContentEditingWithInput:placeholderImage:)]
        unsafe fn startContentEditingWithInput_placeholderImage(
            &self,
            content_editing_input: &PHContentEditingInput,
            placeholder_image: &NSImage,
        );

        #[cfg(all(feature = "block2", feature = "objc2-photos"))]
        #[method(finishContentEditingWithCompletionHandler:)]
        unsafe fn finishContentEditingWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut PHContentEditingOutput)>,
        );

        #[method(cancelContentEditing)]
        unsafe fn cancelContentEditing(&self);

        #[method(shouldShowCancelConfirmation)]
        unsafe fn shouldShowCancelConfirmation(&self) -> bool;
    }

    unsafe impl ProtocolType for dyn PHContentEditingController {}
);
